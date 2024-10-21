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
    command::{Command, CommandState, blink::BlinkCommand},
};

pub struct SpiderBot {
    serial: arduino_hal::Usart<USART0, Pin<Input, PD0>, Pin<Output, PD1>>,
    clock_pin: TC0,
    led: Led,
    state: CommandState<'static>,
}

impl SpiderBot {
    pub fn new() -> Self {
        let dp = arduino_hal::Peripherals::take().unwrap();
        let pins = arduino_hal::pins!(dp);
        let serial = arduino_hal::default_serial!(dp, pins, 57600);

        Self {
            serial,
            clock_pin: dp.TC0,
            led: Led::new(pins.d13.into_output()),
            state: CommandState::Disabled,
        }
    }

    pub fn led(&self) -> &Led {
        &self.led
    }
    pub fn led_mut(&mut self) -> &mut Led {
        &mut self.led
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
                //ufmt::uwriteln!(self.serial, "Waiting {}ms", delay).unwrap_infallible();
                arduino_hal::delay_ms(delay as u16);
            }
            else {
                ufmt::uwriteln!(self.serial,"Overrun! Expected {}ms, took {}ms", runtime, time_budget)
                    .unwrap_infallible();
            }
        }
    }

    fn tick(&mut self, clock: usize) {
        self.led.tick(clock);
    }
}
