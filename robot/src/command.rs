pub mod blink;

use crate::spiderbot::SpiderBot;

pub trait Command {
    fn setup(&mut self, robot: &mut SpiderBot, clock: usize);
    fn exec(&mut self, robot: &mut SpiderBot, clock: usize);
    fn is_done(&self) -> bool;
    fn teardown(&mut self, clock: usize);
}

pub enum CommandState<'a> {
    Disabled,
    Idle,
    Active(&'a dyn Command),
}
