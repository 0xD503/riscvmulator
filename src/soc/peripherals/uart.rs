// use std::sync::RwLock;

use crate::{
    BusAddrType, Device, soc::peripherals::common::device::DeviceData,
};

#[derive(Debug)]
pub struct UART {
    data: DeviceData,
}

impl UART {
    pub fn create_dev(
        base_addr: BusAddrType,
        size: usize,
        kind: String,
        name: String,
    ) -> Box<dyn Device> {
        let uart_dev = UART {
            data: DeviceData::new(base_addr, size, kind, name),
        };
        Box::new(uart_dev)
    }
}

impl Device for UART {
    fn data(&self) -> &DeviceData {
        &self.data
    }
}
