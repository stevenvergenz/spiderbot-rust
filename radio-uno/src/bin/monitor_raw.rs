#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::{
    pins, Peripherals
};
use panic_halt as _;

const BUFFER_SIZE: usize = 0x400;
const PRESCALER: usize = 69;
const PRINT_STEP: usize = 0x20;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut throttle = 0usize;
    let mut samples = [0u8; BUFFER_SIZE];
    let mut head = (0usize, 0usize);
    let mut offset = 0u32;

    let pin = pins.d2.into_pull_up_input();

    // Wait for a character and print current time once it is received
    loop {
        if let (BUFFER_SIZE, 0) = head {
            for line_index in (0..BUFFER_SIZE).step_by(PRINT_STEP) {
                ufmt::uwrite!(&mut serial, "{:04x} | ", offset + line_index as u32).unwrap();

                for x in &samples[line_index .. line_index + PRINT_STEP] {
                    ufmt::uwrite!(&mut serial, "{:02x} ", *x).unwrap();
                }

                ufmt::uwriteln!(&mut serial, "").unwrap();
            }
            ufmt::uwriteln!(&mut serial, "").unwrap();

            offset += BUFFER_SIZE as u32;
            head = (0, 0);
        }

        if throttle == 0 {
            let mask = 1u8 << head.1;
            let value = if pin.is_high() { 1u8 } else { 0u8 } << head.1;
            samples[head.0] = samples[head.0] & !mask | value;
            head = match head {
                (byte, bit) if bit < 7 => (byte, bit + 1),
                (byte, _) => (byte + 1, 0),
            };
        }

        throttle = (throttle + 1) % PRESCALER;
    }
}
