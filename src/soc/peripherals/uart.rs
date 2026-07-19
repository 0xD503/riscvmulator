use std::sync::RwLock;

use crate::{
    BusAddrType, Device, IODeviceError,
    soc::peripherals::common::device::DeviceData,
};

#[derive(Debug)]
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

    pub fn create_dev(
        base_addr: BusAddrType,
        size: usize,
        dev_type: String,
        name: String,
    ) -> Box<dyn Device> {
        // ) -> Result<Box<dyn Device>, IODeviceError> {
        let uart_dev = UART {
            data: DeviceData::new(base_addr, size, dev_type, name),
        };
        // Ok(Box::new(uart_dev))
        Box::new(uart_dev)
    }

    //
}

impl Device for UART {
    fn data(&self) -> &DeviceData {
        &self.data
    }
}
