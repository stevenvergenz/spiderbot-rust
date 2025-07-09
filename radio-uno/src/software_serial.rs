use embedded_hal::digital::InputPin;
use embedded_io::{
    ErrorKind,
    ErrorType,
    Read,
    ReadReady,
};
use crate::error::SerialError;

/// A software serial implementation that uses two GPIO pins for RX and TX.
/// High level (1) on TX is idle.
/// Start bit is 0, then 8 data bits, then stop bit (1)

// 125000bps, 8N1

enum BitBangState {
    Idle,
    DataBits(u8), // Number of bits read/written so far
}

const BUFFER_SIZE: usize = 0x400;

pub struct SerialReader8N1<PinRx> where PinRx: InputPin {
    pin_rx: PinRx,
    head: usize,
    read_buf: [u8; BUFFER_SIZE],

    reader_state: BitBangState,
}

impl<PinRx> SerialReader8N1<PinRx> where PinRx: InputPin {
    pub fn new(pin_rx: PinRx) -> Self {
        Self {
            pin_rx,
            head: 0,
            read_buf: [0; BUFFER_SIZE],
            reader_state: BitBangState::Idle,
        }
    }

    pub fn is_active(&self) -> bool {
        match self.reader_state {
            BitBangState::Idle => false, // No data read
            BitBangState::DataBits(_) => true, // Data byte read
        }
    }

    pub fn process(&mut self) -> Result<(), SerialError> {
        let mut ret = Ok(());

        self.reader_state = match self.reader_state {
            // start reading
            BitBangState::Idle => {
                match self.pin_rx.is_low() {
                    Ok(true) => BitBangState::DataBits(0),
                    _ => BitBangState::Idle,
                }
            },
            // stop reading
            BitBangState::DataBits(8) => {
                self.head += 1;
                match self.pin_rx.is_high() {
                    Ok(true) => {},
                    Ok(false) => { ret = Err(SerialError::new(ErrorKind::InvalidData)); },
                    _ => { ret = Err(SerialError::new(ErrorKind::Other)); },
                }
                BitBangState::Idle
            },
            BitBangState::DataBits(bits_read) => {
                match self.pin_rx.is_high() {
                    Ok(true) => {
                        self.read_buf[self.head]
                            = self.read_buf[self.head] & !(1 << (7 - bits_read))
                            | (1 << (7 - bits_read));
                        BitBangState::DataBits(bits_read + 1)
                    },
                    Ok(false) => {
                        self.read_buf[self.head]
                            = self.read_buf[self.head] & !(1 << (7 - bits_read));
                        BitBangState::DataBits(bits_read + 1)
                    },
                    Err(_) => {
                        // Error reading pin, reset state
                        self.head += 1;
                        ret = Err(SerialError::new(ErrorKind::BrokenPipe));
                        BitBangState::Idle
                    }
                }
            }
        };
        ret
    }
}

impl<PinRx> ErrorType for SerialReader8N1<PinRx> where PinRx: InputPin {
    type Error = SerialError;
}

impl<PinRx> ReadReady for SerialReader8N1<PinRx> where PinRx: InputPin {
    fn read_ready(&mut self) -> Result<bool, Self::Error> {
        Ok(self.head > 0)
    }
}

impl<PinRx> Read for SerialReader8N1<PinRx> where PinRx: InputPin {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        match self.read_ready() {
            Ok(true) => {},
            _ => return Err(SerialError::new(ErrorKind::Unsupported)),
        };

        let bytes_read = if buf.len() <= self.head {
            buf.copy_from_slice(&self.read_buf[0..buf.len()]);
            buf.len()
        }
        else {
            for (i, x) in self.read_buf[0..self.head].iter().enumerate() {
                buf[i] = *x;
            }
            self.head
        };

        self.read_buf[0] = self.read_buf[self.head];
        self.head = 0;
        Ok(bytes_read)
    }
}
