use arduino_hal::{
    clock::Clock,
    pac::TC2,
    port::{
        mode::Output,
        Pin,
    }, usart::{UsartOps, UsartWriter},
};
use embedded_hal::digital::OutputPin;
use embedded_io::{
    ErrorType,
    Write,
    WriteReady,
};
use core::{
    mem::MaybeUninit,
    ops::RangeInclusive,
};
use ufmt::{derive::uDebug, uWrite};
use crate::error::SerialError;

/// A software serial implementation that uses two GPIO pins for RX and TX.
/// High level (1) on TX is idle.
/// Start bit is 0, then 8 data bits, then stop bit (1)

struct InterruptState {
    pin: Pin<Output>,
    next_bit: Option<bool>,
}

static mut INTERRUPT_STATE: MaybeUninit<InterruptState> = MaybeUninit::uninit();

#[avr_device::interrupt(atmega328p)]
unsafe fn TIMER2_COMPA() {
    avr_device::interrupt::CriticalSection::new();

    #[allow(static_mut_refs)]
    let state = &mut *INTERRUPT_STATE.as_mut_ptr();

    if let Some(next_bit) = state.next_bit.take() {
        // write next bit to pin
        state.pin.set_state(next_bit.into()).unwrap();
    }
}

#[derive(uDebug)]
pub enum WriterState {
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
    pub fn new(pin: Pin<Output>, mut clock: TC2, baud: u32) -> Result<Self, &'static str> {
        // set timer to "clear timer on compare match" (CTC) mode
        clock.tccr2a.write(|w| w.wgm2().ctc());

        let prescale = Self::calc_prescale(baud, &mut clock)?;
        Self::calc_compare(baud, prescale, &mut clock);

        unsafe {
            INTERRUPT_STATE = MaybeUninit::new(InterruptState {
                pin: pin,
                next_bit: None,
            });
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        }

        Ok(Self {
            clock,
            head: 0,
            tail: 0,
            size: 0,
            buf: [0; B],
            next_send: WriterState::Idle,
        })
    }

    pub fn process<T>(&mut self, serial: &mut T) -> Result<(), T::Error> where T: ufmt::uWrite {
        if self.size == 0 {
            // disable overflow clock interrupt
            self.clock.timsk2.write(|w| w.ocie2a().clear_bit());
            return Ok(());
        }
        else {
            self.clock.timsk2.write(|w| w.ocie2a().set_bit());
        }

        let int_state = unsafe {
            #[allow(static_mut_refs)]
            &mut *INTERRUPT_STATE.as_mut_ptr()
        };

        if int_state.next_bit.is_none() {
            //ufmt::uwriteln!(serial, "Sending next bit...")?;
            int_state.next_bit = self.send_next_bit();
        }

        Ok(())
    }

    pub fn debug<T>(&self, serial: &mut T) -> Result<(), T::Error> where T: ufmt::uWrite {
        ufmt::uwriteln!(serial,
            "SerialWriter: head={}, tail={}, size={}, next_send={:?}",
            self.head, self.tail, self.size, self.next_send)
    }

    fn calc_prescale(baud: u32, clock: &mut TC2) -> Result<u32, &'static str> {
        const NATIVE_TICK_S: f32 = 1f32 / arduino_hal::DefaultClock::FREQ as f32;
        const PRESCALE_1: RangeInclusive<f32>    = NATIVE_TICK_S * 1.0    ..= NATIVE_TICK_S * 1.0 * 256.0;
        const PRESCALE_8: RangeInclusive<f32>    = NATIVE_TICK_S * 8.0    ..= NATIVE_TICK_S * 8.0 * 256.0;
        const PRESCALE_32: RangeInclusive<f32>   = NATIVE_TICK_S * 32.0   ..= NATIVE_TICK_S * 32.0 * 256.0;
        const PRESCALE_64: RangeInclusive<f32>   = NATIVE_TICK_S * 64.0   ..= NATIVE_TICK_S * 64.0 * 256.0;
        const PRESCALE_128: RangeInclusive<f32>  = NATIVE_TICK_S * 128.0  ..= NATIVE_TICK_S * 128.0 * 256.0;
        const PRESCALE_256: RangeInclusive<f32>  = NATIVE_TICK_S * 256.0  ..= NATIVE_TICK_S * 256.0 * 256.0;
        const PRESCALE_1024: RangeInclusive<f32> = NATIVE_TICK_S * 1024.0 ..= NATIVE_TICK_S * 1024.0 * 256.0;

        let freq_s = 1.0 / baud as f32;
        if PRESCALE_1.contains(&freq_s) {
            // set clock prescaler to 1:1
            clock.tccr2b.write(|w| w.cs2().direct());
            Ok(1)
        }
        else if PRESCALE_8.contains(&freq_s) {
            clock.tccr2b.write(|w| w.cs2().prescale_8());
            Ok(8)
        }
        else if PRESCALE_32.contains(&freq_s) {
            clock.tccr2b.write(|w| w.cs2().prescale_32());
            Ok(32)
        }
        else if PRESCALE_64.contains(&freq_s) {
            clock.tccr2b.write(|w| w.cs2().prescale_64());
            Ok(64)
        }
        else if PRESCALE_128.contains(&freq_s) {
            clock.tccr2b.write(|w| w.cs2().prescale_128());
            Ok(128)
        }
        else if PRESCALE_256.contains(&freq_s) {
            clock.tccr2b.write(|w| w.cs2().prescale_256());
            Ok(256)
        }
        else if PRESCALE_1024.contains(&freq_s) {
            clock.tccr2b.write(|w| w.cs2().prescale_1024());
            Ok(1024)
        }
        else {
            Err("Frequency too low/high for software serial")
        }
    }

    fn calc_compare(baud: u32, prescale: u32, clock: &mut TC2) -> u8 {
        let compare = (arduino_hal::DefaultClock::FREQ / baud / prescale - 1) as u8;
        clock.ocr2a.write(|w| w.bits(compare));
        compare
    }

    fn send_next_bit(&mut self) -> Option<bool> {
        match self.next_send {
            WriterState::Idle => {
                if self.size > 0 {
                    self.next_send = WriterState::Start;
                }
                None
            },
            WriterState::Start => {
                self.next_send = WriterState::Data(0);
                Some(false)
            },
            WriterState::Data(bit) => {
                self.next_send = if bit < 7 { WriterState::Data(bit + 1) } else { WriterState::Stop };
                Some((self.buf[self.head] & (1 << (7 - bit))) != 0)
            },
            WriterState::Stop => {
                self.head = (self.head + 1) % B;
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