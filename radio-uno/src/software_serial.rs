use arduino_hal::{
    clock::Clock,
    pac::TC2,
    port::{
        Pin,
        mode::Output,
    },
};
use embedded_hal::digital::OutputPin;
use embedded_io::{
    ErrorType,
    Write,
    WriteReady,
};
use core::mem::MaybeUninit;
use ufmt::uWrite;
use crate::error::SerialError;

/// A software serial implementation that uses two GPIO pins for RX and TX.
/// High level (1) on TX is idle.
/// Start bit is 0, then 8 data bits, then stop bit (1)

// 16e6 ticks   1 increments   1 overflow
// ---------- x ------------ x --------------
// 1s           X tick         256 increments

const PRESCALE_1_MIN_HZ: u32 = arduino_hal::DefaultClock::FREQ / 1 / 256;
const PRESCALE_8_MIN_HZ: u32 = arduino_hal::DefaultClock::FREQ / 8 / 256;
const PRESCALE_32_MIN_HZ: u32 = arduino_hal::DefaultClock::FREQ / 32 / 256;
const PRESCALE_64_MIN_HZ: u32 = arduino_hal::DefaultClock::FREQ / 64 / 256;
const PRESCALE_128_MIN_HZ: u32 = arduino_hal::DefaultClock::FREQ / 128 / 256;
const PRESCALE_256_MIN_HZ: u32 = arduino_hal::DefaultClock::FREQ / 256 / 256;
const PRESCALE_1024_MIN_HZ: u32 = arduino_hal::DefaultClock::FREQ / 1024 / 256;

struct InterruptState {
    pin: Pin<Output>,
    next_bit: Option<bool>,
}

static mut INTERRUPT_STATE: MaybeUninit<InterruptState> = MaybeUninit::uninit();

#[avr_device::interrupt(atmega328p)]
fn TIMER2_COMPA() {
    let state = unsafe {
        #[allow(static_mut_refs)]
        &mut *INTERRUPT_STATE.as_mut_ptr()
    };
    if let Some(next_bit) = state.next_bit.take() {
        // write next bit to pin
        state.pin.set_state(next_bit.into()).unwrap();
    }
}

enum WriterState {
    Idle,
    Start,
    Data(u8),
    Stop,
}

pub struct SerialWriter<const B: usize> {
    clock: TC2,
    head: usize,
    tail: usize,
    size: usize,
    buf: [u8; B],
    next_send: WriterState,
}

impl<const B: usize> SerialWriter<B> {
    pub fn new(pin: Pin<Output>, clock: TC2, baud: u32) -> Option<Self> {
        // timer/counter control register 2
        clock.tccr2a.write(|w| w.wgm2().ctc());

        let preload =
        if baud < PRESCALE_1_MIN_HZ {
            // set clock prescaler to 1:1
            clock.tccr2b.write(|w| w.cs2().direct());
            Self::preload(baud, 1)
        }
        else if baud < PRESCALE_8_MIN_HZ {
            clock.tccr2b.write(|w| w.cs2().prescale_8());
            Self::preload(baud, 8)
        }
        else if baud < PRESCALE_32_MIN_HZ {
            clock.tccr2b.write(|w| w.cs2().prescale_32());
            Self::preload(baud, 32)
        }
        else if baud < PRESCALE_64_MIN_HZ {
            clock.tccr2b.write(|w| w.cs2().prescale_64());
            Self::preload(baud, 64)
        }
        else if baud < PRESCALE_128_MIN_HZ {
            clock.tccr2b.write(|w| w.cs2().prescale_128());
            Self::preload(baud, 128)
        }
        else if baud < PRESCALE_256_MIN_HZ {
            clock.tccr2b.write(|w| w.cs2().prescale_256());
            Self::preload(baud, 256)
        }
        else if baud < PRESCALE_1024_MIN_HZ {
            clock.tccr2b.write(|w| w.cs2().prescale_1024());
            Self::preload(baud, 1024)
        }
        else {
            return None;
        };

        // set output compare register A to preload value
        clock.ocr2a.write(|w| w.bits(preload));

        unsafe {
            INTERRUPT_STATE = MaybeUninit::new(InterruptState {
                pin: pin,
                next_bit: None,
            });
        }

        Some(Self {
            clock,
            head: 0,
            tail: 0,
            size: 0,
            buf: [0; B],
            next_send: WriterState::Idle,
        })
    }

    pub fn process(&mut self) {
        if self.size == 0 {
            // disable overflow clock interrupt
            self.clock.timsk2.write(|w| w.ocie2a().clear_bit());
            return;
        }

        let int_state = unsafe {
            #[allow(static_mut_refs)]
            &mut *INTERRUPT_STATE.as_mut_ptr()
        };

        if int_state.next_bit.is_none() {
            int_state.next_bit = self.next_bit();
        }
    }

    fn preload(baud: u32, prescale: u32) -> u8 {
        ((arduino_hal::DefaultClock::FREQ as f32) / (baud as f32) / (prescale as f32)) as u8 - 1
    }

    fn next_bit(&mut self) -> Option<bool> {
        match self.next_send {
            WriterState::Idle => {
                None
            },
            WriterState::Start => {
                self.next_send = WriterState::Data(0);
                Some(false)
            },
            WriterState::Data(bit) => {
                self.next_send = if bit < 7 { WriterState::Data(bit + 1) } else { WriterState::Stop };
                Some(self.buf[self.head] & (1 << (7 - bit)) != 0)
            },
            WriterState::Stop => {
                self.head += 1;
                self.size -= 1;
                self.next_send = if self.size > 0 { WriterState::Start } else { WriterState::Idle };
                Some(true)
            }
        }
    }
}

impl<const B: usize> ErrorType for SerialWriter<B> {
    type Error = SerialError;
}

impl<const B: usize> Write for SerialWriter<B> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        if self.size + buf.len() > B {
            return Err(SerialError::BufferFull);
        }

        for byte in buf {
            self.buf[self.tail] = *byte;
            self.tail = (self.tail + 1) % B;
            self.size += 1;
        }

        // enable overflow clock interrupt
        if self.size > 1 {
            if let WriterState::Idle = self.next_send {
                // if we are idle, start with the start bit
                self.next_send = WriterState::Start;
            }
            self.clock.timsk2.write(|w| w.ocie2a().set_bit());
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Err(SerialError::NotImplemented)
    }
}

impl<const B: usize> WriteReady for SerialWriter<B> {
    fn write_ready(&mut self) -> Result<bool, Self::Error> {
        Ok(self.size < B)
    }
}

impl<const B: usize> uWrite for SerialWriter<B> {
    type Error = SerialError;
    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        match self.write(s.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}