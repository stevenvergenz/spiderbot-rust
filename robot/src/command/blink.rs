use core::ops::ShlAssign;

use crate::led::Led;

use super::Command;
use super::SpiderBot;

const SIZE: usize = 2;

pub struct BlinkCommand {
    pattern: [u8; SIZE],
}

impl BlinkCommand {
    pub fn new(pattern: &'static str) -> Self {
        let mut blink = Self {
            pattern: [0; SIZE],
        };
        blink.pattern[0] = 1;

        for c in pattern.chars() {
            let mut overflow = match c {
                '.' => false,
                '-' => true,
                _ => panic!("Invalid character found: {}", c),
            };

            for byte in blink.pattern.as_mut_slice() {
                let (new_byte, new_overflow) = byte.overflowing_shl(1);
                *byte = (new_byte & 0xfe) | if overflow { 1 } else { 0 };
                overflow = new_overflow;

                if !overflow {
                    break;
                }
            }

            assert!(!overflow, "Pattern {} exceeds storage capacity", pattern);
        }

        blink
    }
}

impl Command for BlinkCommand {
    fn setup(&mut self, robot: &mut SpiderBot, clock: usize) {

    }

    fn exec(&mut self, robot: &mut SpiderBot, clock: usize) {

    }

    fn is_done(&self) -> bool {
        todo!()
    }

    fn teardown(&mut self, clock: usize) {

    }
}
