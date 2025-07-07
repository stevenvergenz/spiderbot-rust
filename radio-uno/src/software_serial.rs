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

pub struct SerialReader8N1<PinRx> where PinRx: InputPin {
    pin_rx: PinRx,
    read_bits: u8,
    read_buf: [u8; 16],
    read_buf_len: usize,

    reader_state: BitBangState,
}

impl<PinRx> SerialReader8N1<PinRx> where PinRx: InputPin {
    pub fn new(pin_rx: PinRx) -> Self {
        Self {
            pin_rx,
            read_bits: 0,
            read_buf: [0; 16],
            read_buf_len: 0,
            reader_state: BitBangState::Idle,
        }
    }

    pub fn process(&mut self) {
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
                match self.pin_rx.is_high() {
                    // stop bit received, commit to buffer
                    Ok(true) => {
                        self.read_buf[self.read_buf_len] = self.read_bits;
                        self.read_buf_len += 1;
                        self.read_bits = 0;
                        BitBangState::Idle
                    },
                    // stop bit not high or other error, don't commit byte to buffer
                    _ => {
                        self.read_bits = 0;
                        BitBangState::Idle
                    },
                }
            },
            BitBangState::DataBits(bits_read) => {
                match self.pin_rx.is_high() {
                    Ok(true) => {
                        self.read_bits |= 1 << (7 - bits_read);
                        BitBangState::DataBits(bits_read + 1)
                    },
                    Ok(false) => {
                        BitBangState::DataBits(bits_read + 1)
                    },
                    Err(_) => {
                        // Error reading pin, reset state
                        self.read_bits = 0;
                        BitBangState::Idle
                    }
                }
            }
        };
    }
}

impl<PinRx> ErrorType for SerialReader8N1<PinRx> where PinRx: InputPin {
    type Error = SerialError;
}

impl<PinRx> ReadReady for SerialReader8N1<PinRx> where PinRx: InputPin {
    fn read_ready(&mut self) -> Result<bool, Self::Error> {
        Ok(self.read_buf_len == self.read_buf.len())
    }
}

impl<PinRx> Read for SerialReader8N1<PinRx> where PinRx: InputPin {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        match self.read_ready() {
            Ok(true) => {},
            _ => return Err(SerialError::new(ErrorKind::Unsupported)),
        };

        if buf.len() <= self.read_buf_len {
            buf.copy_from_slice(&self.read_buf[0..buf.len()]);
            self.read_buf_len = 0;
            Ok(buf.len())
        }
        else {
            for (i, x) in self.read_buf[0..self.read_buf_len].iter().enumerate() {
                buf[i] = *x;
            }
            let len = self.read_buf_len;
            self.read_buf_len = 0;
            Ok(len)
        }
    }
}
