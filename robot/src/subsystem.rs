pub mod onboard_leds_buttons;

pub trait Subsystem {
    fn tick(&mut self, clock: usize);
}
