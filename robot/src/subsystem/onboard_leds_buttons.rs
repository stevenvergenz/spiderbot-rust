use arduino_hal::{
    hal::port::{PB0, PB1, PD7},
    port::{
        mode::{Floating, Input, Output},
        Pin, PinOps,
    },
};

use super::Subsystem;

pub struct OnboardLedsButtons {
    pin_red_a: FusedLedButton<PD7>,
    pin_green_b: FusedLedButton<PB0>,
}

impl OnboardLedsButtons {
    pub fn new(pin_red_a: Pin<Output, PD7>, pin_green_b: Pin<Output, PB0>) -> Self {
        Self {
            pin_red_a: FusedLedButton::new(pin_red_a),
            pin_green_b: FusedLedButton::new(pin_green_b),
        }
    }

    pub fn led_red(&self) -> & dyn Led {
        &self.pin_red_a
    }
    pub fn led_red_mut(&mut self) -> &mut dyn Led {
        &mut self.pin_red_a
    }
    pub fn button_a(&self) -> & dyn Button {
        &self.pin_red_a
    }
}

impl Subsystem for OnboardLedsButtons {
    fn tick(&mut self, clock: usize) {
        self.pin_red_a.refresh(clock);
    }
}

pub trait Led {
    fn is_lit(&self) -> bool;
    fn set_lit(&mut self, state: bool);
    fn toggle(&mut self);
    fn set_lit_for(&mut self, duration_ms: usize);
}

pub trait Button {
    fn is_pressed(&self) -> bool;
}

struct FusedLedButton<T> where T: PinOps {
    pin: Option<Pin<Output, T>>,
    button_pressed: bool,
    led_lit: bool,
    button_debounce: usize,
}

impl<T> FusedLedButton<T> where T: PinOps {
    pub fn new(pin: Pin<Output, T>) -> Self {
        Self {
            pin: Some(pin),
            button_pressed: false,
            led_lit: false,
            button_debounce: 0,
        }
    }

    pub fn refresh(&mut self, clock: usize) {
        let mut out_pin = self.pin.take().unwrap();

        let mut is_high = false;
        if clock > self.button_debounce {
            out_pin.set_low();
            let in_pin = out_pin.into_floating_input();

            if in_pin.is_high() {
                is_high = true;
                self.button_pressed = true;
                self.button_debounce = clock + 500;
            }
            else {
                self.button_pressed = false;
            }

            out_pin = in_pin.into_output();
        }

        if self.led_lit {
            out_pin.set_low();
        }
        else if !is_high {
            out_pin.set_high();
        }

        self.pin = Some(out_pin);
    }
}

impl<T> Led for FusedLedButton<T> where T: PinOps {
    fn is_lit(&self) -> bool {
        self.led_lit
    }
    fn set_lit(&mut self, state: bool) {
        self.led_lit = state;
    }
    fn toggle(&mut self) {
        self.led_lit = !self.led_lit;
    }
    fn set_lit_for(&mut self, duration_ms: usize) {

    }
}

impl<T> Button for FusedLedButton<T> where T: PinOps {
    fn is_pressed(&self) -> bool {
        self.button_pressed
    }
}
