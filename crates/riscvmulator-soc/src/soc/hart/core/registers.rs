use crate::constants::arch::XLen;

pub(crate) trait RegisterInterface {
    fn read(&self) -> XLen;
    fn write(&mut self, value: XLen);

    fn reset(&mut self, value: XLen);
}

pub(crate) trait RegisterFileInterface {
    fn new() -> Self;

    fn read(&self, offset: usize) -> XLen;
    fn write(&mut self, offset: usize, value: XLen);

    fn length() -> usize;

    fn reset(&mut self);
}

pub(crate) trait RawRegisterInterface {
    fn read_raw(&self) -> XLen;
    fn write_raw(&mut self, value: XLen);
}

pub(crate) trait RawRegisterFileInterface {
    fn read_raw(&self, offset: usize) -> XLen;
    fn write_raw(&mut self, offset: usize, value: XLen);
}
