use arduino_hal::prelude::*;
use crate::millis::{millis, millis_init};

pub struct SpiderBot {
    dp: arduino_hal::Peripherals,
}

impl SpiderBot {
    pub fn new() -> Self {
        Self {
            dp: arduino_hal::Peripherals::take().unwrap(),
        }
    }

    pub fn exec(&mut self, freq_hz: usize) -> ! {

        millis_init(&self.dp.TC0);

        // Enable interrupts globally
        unsafe { avr_device::interrupt::enable() };

        Self::throttle(20, self.tick)
    }

    fn throttle(freq_hz: usize, body: FnMut) -> ! {
        let time_budget = 1000 / freq_hz;
        loop {
            let time_start = millis();

            body();

            let runtime_ms = millis() - time_start;
            if runtime_ms < time_budget {
                arduino_hal::delay_ms((time_budget - runtime_ms) as u16);
            }
        }
    }

    fn tick(&mut self) {

    }
}
