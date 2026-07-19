// use std::sync::RwLock;

use crate::{
    BusAddrType, Device, soc::peripherals::common::device::DeviceData,
};

#[derive(Debug)]
pub struct CLINT {
    data: DeviceData,
}

impl CLINT {
    pub fn create_dev(
        base_addr: BusAddrType,
        size: usize,
        kind: String,
        name: String,
    ) -> Box<dyn Device> {
        let clint_dev = CLINT {
            data: DeviceData::new(base_addr, size, kind, name),
        };
        Box::new(clint_dev)
    }
}

impl Device for CLINT {
    fn data(&self) -> &DeviceData {
        &self.data
    }
}
