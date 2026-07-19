// use std::sync::RwLock;

use crate::{
    BusAddrType, Device, soc::peripherals::common::device::DeviceData,
};

#[derive(Debug)]
pub struct PLIC {
    data: DeviceData,
}

impl PLIC {
    pub fn create_dev(
        base_addr: BusAddrType,
        size: usize,
        kind: String,
        name: String,
    ) -> Box<dyn Device> {
        let plic_dev = PLIC {
            data: DeviceData::new(base_addr, size, kind, name),
        };
        Box::new(plic_dev)
    }
}

impl Device for PLIC {
    fn data(&self) -> &DeviceData {
        &self.data
    }
}
