#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use core::{
    convert::Infallible,
    sync::atomic::{AtomicBool, Ordering},
};

use arduino_hal::{
    Peripherals,
    pins,
};

use embedded_io::{Read, ReadReady};
use panic_halt as _;
use radio_uno::software_serial::SerialReader8N1;
use ufmt::uWrite;

// desired serial rate is 115_200 Hz
// native clock is 16_000_000 Hz
// 1:1 prescaler -> 138.88... native ticks per serial tick
// preload register to 255 - 138 = 117 so the reg overflows at 115.2kHz
const PRELOAD: usize = 117;

static TIMER_INTERRUPT: AtomicBool = AtomicBool::new(false);
#[avr_device::interrupt(atmega328p)]
fn TIMER2_COMPA() {
    TIMER_INTERRUPT.store(true, Ordering::SeqCst);
}

static MSG_START: AtomicBool = AtomicBool::new(false);
#[avr_device::interrupt(atmega328p)]
fn INT0() {
    MSG_START.store(true, Ordering::SeqCst);
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // timer/counter control register 2
    dp.TC2.tccr2a.write(|w| w.wgm2().ctc());
    // set clock prescaler to 1:1
    dp.TC2.tccr2b.write(|w| w.cs2().direct());
    // initialize clock register higher than 0 so the reg overflows at 115.2kHz
    dp.TC2.ocr2a.write(|w| w.bits(PRELOAD as u8));
    // enable overflow clock interrupt
    dp.TC2.timsk2.write(|w| w.ocie2a().set_bit());

    dp.EXINT.eicra.write(|w| w.isc0().val_0x02()); // falling edge (0=low, 1=any, 2=falling, 3=rising)
    dp.EXINT.eimsk.write(|w| w.int0().set_bit()); // enable external interrupt 0

    // Enable interrupts globall
    unsafe { avr_device::interrupt::enable() };

    let mut reader = SerialReader8N1::new(pins.d2.into_pull_up_input());
    let mut buf = [0u8; 80];

    // Wait for a character and print current time once it is received
    loop {
        if !reader.is_active() && MSG_START.load(Ordering::SeqCst)
            || reader.is_active() && TIMER_INTERRUPT.load(Ordering::SeqCst)
        {
            if let Err(e) = reader.process() {
                ufmt::uwriteln!(&mut serial, "Serial error: {}", e).unwrap();
            }
            TIMER_INTERRUPT.store(false, Ordering::SeqCst);
            dp.TC2.ocr2a.write(|w| w.bits(PRELOAD as u8));
        }

        if !reader.is_active() && reader.read_ready().unwrap() {
            let len = reader.read(&mut buf).unwrap_or(0);
            print_buf(&mut serial, &buf[..len]);
            MSG_START.store(false, Ordering::SeqCst);
        }
    }
}

fn print_buf<U>(serial: &mut U, buf: &[u8]) where U : uWrite<Error = Infallible> {
    for x in buf {
        ufmt::uwrite!(serial, "{:02x} ", *x).unwrap();
    }
    ufmt::uwriteln!(serial, "").unwrap();
}