use std::sync::RwLock;

use crate::{
    BusAddrType, Device, soc::peripherals::common::device::DeviceData,
};

pub struct UART {
    data: DeviceData,
}

impl UART {
    fn new(
        base_addr: BusAddrType,
        size: usize,
        dev_type: String,
        name: String,
    ) -> Self {
        UART {
            data: DeviceData::new(base_addr, size, dev_type, name),
        }
    }

    //
}

impl Device for UART {
    fn data(&self) -> &DeviceData {
        &self.data
    }
}
