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
    let mut serial_hw = arduino_hal::default_serial!(dp, pins, 57600);

    let result = SerialWriter::<1024>::new(
        pins.d2.into_output().downgrade(),
        dp.TC2,
        9600,
    );
    let mut serial_sw_opt = match result {
        Ok(serial) => Some(serial),
        Err(_) => {
            ufmt::uwriteln!(&mut serial_hw, "Failed to initialize SW UART").unwrap();
            None
        }
    };

    unsafe { avr_device::interrupt::enable(); }

    let mut test_counter = 0u32;
    let mut dbg_counter = 0u32;
    let mut clear = false;
    loop {
        if let Some(serial_sw) = serial_sw_opt.as_mut() {
            if test_counter == 0 {
                if let Err(_) = ufmt::uwriteln!(serial_sw, "SW UART") {
                    ufmt::uwriteln!(&mut serial_hw, "Failed to write to SW UART").unwrap();
                    clear = true;
                }
                dbg_counter = 0;
            }
            serial_sw.process(&mut serial_hw).unwrap();
            test_counter = (test_counter + 1) % 50000;


            if dbg_counter < 250 {
                if dbg_counter % 10 == 0 {
                    serial_sw.debug(&mut serial_hw).unwrap();
                }
                dbg_counter += 1;
            }
        }
        if clear {
            serial_sw_opt = None;
        }

        // if counter == 0 {
        //     ufmt::uwriteln!(&mut serial_hw, "HW UART").unwrap();
        // }

        arduino_hal::delay_us(10);
    }
}
