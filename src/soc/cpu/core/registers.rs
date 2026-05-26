use crate::constants::arch::XLen;

pub trait RegisterInterface {
    fn read(&self) -> XLen;
    fn write(&mut self, value: XLen);
}

pub trait RegisterFileInterface {
    fn new() -> Self;

    fn read(&self, offset: usize) -> XLen;
    fn write(&mut self, offset: usize, value: XLen);

    fn length() -> usize;

    fn reset(&mut self);
}
