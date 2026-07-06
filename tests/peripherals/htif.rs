use riscvmulator::{BusAddrType, Device, DeviceIOError};

pub struct HTIF {
    //
}

impl HTIF {
    pub fn new() -> Self {
        HTIF {}
    }
}

impl Device for HTIF {
    fn read_u8(&self, offset: usize) -> Result<u8, DeviceIOError> {
        Ok(7)
    }
    fn read_u16(&self, offset: usize) -> Result<u16, DeviceIOError> {
        Ok(7)
    }
    fn read_u32(&self, offset: usize) -> Result<u32, DeviceIOError> {
        Ok(7)
    }
    fn read_u64(&self, offset: usize) -> Result<u64, DeviceIOError> {
        Ok(7)
    }

    fn write_u8(&self, offset: usize, value: u8) -> Result<(), DeviceIOError> {
        Ok(())
    }
    fn write_u16(
        &self,
        offset: usize,
        value: u16,
    ) -> Result<(), DeviceIOError> {
        Ok(())
    }
    fn write_u32(
        &self,
        offset: usize,
        value: u32,
    ) -> Result<(), DeviceIOError> {
        Ok(())
    }
    fn write_u64(
        &self,
        offset: usize,
        value: u64,
    ) -> Result<(), DeviceIOError> {
        Ok(())
    }

    fn base_addr(&self) -> BusAddrType {
        0
    }
    fn mem_space_size(&self) -> usize {
        0
    }
}
