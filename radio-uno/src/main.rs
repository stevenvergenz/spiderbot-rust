#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use panic_halt as _;
use core::sync::atomic::{AtomicBool, Ordering};
use arduino_hal::prelude::*;

static BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);

#[avr_device::interrupt(atmega328p)]
fn PCINT2() {
    // This function is called when a pin change interrupt occurs on PCINT0
    // You can handle the interrupt here, e.g., read a pin state or toggle an LED
    BUTTON_PRESSED.store(true, Ordering::SeqCst);
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    dp.EXINT.pcicr.write(|w| unsafe { w.bits(0b100) });
    dp.EXINT.pcmsk2.write(|w| w.bits(0b100));

    unsafe { avr_device::interrupt::enable() };

    loop {
        if BUTTON_PRESSED.load(Ordering::SeqCst) {
            // Button was pressed, handle the event
            serial.write(b'B').unwrap(); // Send 'B' over serial
            BUTTON_PRESSED.store(false, Ordering::SeqCst); // Reset the flag
        }

        // You can add other logic here, like reading from sensors or controlling outputs
        arduino_hal::delay_ms(100); // Delay to avoid busy-waiting
    }
}
