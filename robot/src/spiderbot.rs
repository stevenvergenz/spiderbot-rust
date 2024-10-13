use arduino_hal::{
    hal::port::{PD0, PD1},
    pac::{TC0, USART0},
    port::{
        mode::{Input, Output},
        Pin,
    },
    prelude::*
};
use crate::{
    led::Led,
    millis::{millis, millis_init},
};

pub struct SpiderBot {
    serial: arduino_hal::Usart<USART0, Pin<Input, PD0>, Pin<Output, PD1>>,
    clock_pin: TC0,
    led: Led,
}

impl SpiderBot {
    pub fn new() -> Self {
        let dp = arduino_hal::Peripherals::take().unwrap();
        let pins = arduino_hal::pins!(dp);
        let serial = arduino_hal::default_serial!(dp, pins, 57600);

        Self {
            serial,
            clock_pin: dp.TC0,
            led: Led::new(pins.d13.into_output())
        }
    }

    pub fn exec(&mut self, freq_hz: usize) -> ! {
        // set up the throttle checker
        millis_init(&self.clock_pin);

        // Enable interrupts globally
        unsafe { avr_device::interrupt::enable() };

        let time_budget = 1000 / freq_hz;
        loop {
            let time_start = millis();

            self.tick(time_start);

            let runtime = millis() - time_start;
            if runtime < time_budget {
                let delay = time_budget - runtime;
                ufmt::uwriteln!(self.serial, "Waiting {}ms", delay).unwrap_infallible();
                arduino_hal::delay_ms((time_budget - runtime) as u16);
            }
        }
    }

    fn tick(&mut self, clock: usize) {
        self.led.tick(clock);
    }
}
