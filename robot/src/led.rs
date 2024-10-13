use arduino_hal::{
    hal::port::PB5,
    port::{mode::Output, Pin},
};

struct Timeout {
    duration: usize,
    off: usize,
}

pub struct Led {
    pin: Pin<Output, PB5>,
    off_timeout: Option<Timeout>,
}

impl Led {
    pub fn new(pin: Pin<Output, PB5>) -> Self {
        Self {
            pin,
            off_timeout: None,
        }
    }

    pub fn is_lit(&self) -> bool {
        self.pin.is_set_high()
    }

    pub fn set_lit(&mut self, state: bool) {
        if state {
            self.pin.set_high();
        }
        else {
            self.pin.set_low();
        }
        self.off_timeout = None;
    }

    pub fn toggle(&mut self) {
        self.pin.toggle();
        self.off_timeout = None;
    }

    pub fn set_lit_for(&mut self, duration_ms: usize) {
        self.off_timeout = Some(Timeout {
            duration: duration_ms,
            off: 0,
        });
        self.set_lit(true);
    }

    pub fn tick(&mut self, clock: usize) {
        if let Some(timeout) = self.off_timeout.as_mut() {
            if timeout.off == 0 {
                timeout.off = timeout.duration + clock;
            }
            else if timeout.off < clock {
                self.set_lit(false);
                self.off_timeout = None;
            }
        }
    }
}
