#![no_std]
#![no_main]

use arduino_hal::{
    Peripherals,
    pins,
};
use panic_halt as _;
use radio_uno::software_serial::SerialWriter;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = pins!(dp);
    let mut serial = SerialWriter::<1024>::new(
        pins.d1.into_output().downgrade(),
        dp.TC2,
        9600,
    );

    unsafe { avr_device::interrupt::enable(); }

    let mut counter = 0u32;
    loop {
        if let Some(serial) = serial.as_mut() {
            if counter == 0 {
                ufmt::uwriteln!(serial, "Testing! ...").unwrap();
            }
            counter = (counter + 1) % 1000;
            serial.process();
        }
        arduino_hal::delay_us(10);
    }
}
