// use std::sync::RwLock;

use crate::{
    BusAddrType, Device, IODeviceError,
    soc::peripherals::common::device::DeviceMeta,
};

#[derive(Debug)]
pub struct PLIC
where
    Self: Device,
{
    meta: DeviceMeta,
}

impl PLIC {
    pub fn create_dev(
        base_addr: BusAddrType,
        size: usize,
        kind: String,
        name: String,
    ) -> Box<dyn Device> {
        let plic_dev = PLIC {
            meta: DeviceMeta::new(base_addr, size, kind, name),
        };
        Box::new(plic_dev)
    }
}

impl Device for PLIC {
    fn meta(&self) -> &DeviceMeta {
        &self.meta
    }

    // we cannot make `read()`/`write()` methods to be generic because of the
    // `dyn` traits compatibility requirement. So we declare methods for each
    // type. Read more at:
    // https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility

    fn read_u8(&self, offset: usize) -> Result<u8, IODeviceError> {
        self.meta().read_impl(offset)
    }
    fn read_u16(&self, offset: usize) -> Result<u16, IODeviceError> {
        self.meta().read_impl(offset)
    }
    fn read_u32(&self, offset: usize) -> Result<u32, IODeviceError> {
        self.meta().read_impl(offset)
    }
    fn read_u64(&self, offset: usize) -> Result<u64, IODeviceError> {
        self.meta().read_impl(offset)
    }

    fn write_u8(&self, offset: usize, value: u8) -> Result<(), IODeviceError> {
        self.meta().write_impl(offset, value)
    }
    fn write_u16(
        &self,
        offset: usize,
        value: u16,
    ) -> Result<(), IODeviceError> {
        self.meta().write_impl(offset, value)
    }
    fn write_u32(
        &self,
        offset: usize,
        value: u32,
    ) -> Result<(), IODeviceError> {
        self.meta().write_impl(offset, value)
    }
    fn write_u64(
        &self,
        offset: usize,
        value: u64,
    ) -> Result<(), IODeviceError> {
        self.meta().write_impl(offset, value)
    }
}
