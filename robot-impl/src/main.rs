#![no_std]
#![no_main]

use panic_halt as _;

use robot_core::Robot;
use robot_impl::SpiderBot;

#[arduino_hal::entry]
fn main() -> ! {
    let mut robot = SpiderBot::new();
    robot.exec()
}
