#![no_std]
#![no_main]

use panic_halt as _;

use robot::SpiderBot;

#[arduino_hal::entry]
fn main() -> ! {
    let mut robot = SpiderBot::new();
    robot.exec(20)
}
