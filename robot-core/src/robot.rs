pub trait Robot {
    fn tick(&mut self);
    fn exec(&mut self) -> ! {
        loop {
            self.tick();
        }
    }
}
