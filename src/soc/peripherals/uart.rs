// use std::sync::RwLock;

use std::sync::RwLock;

use log::error;

use crate::{
    BusAddrType, Device, IODeviceError, XLen,
    soc::peripherals::common::{
        device::DeviceMeta,
        memory_map::{self, MemoryMap},
    },
};

#[derive(Debug)]
pub struct UART
where
    Self: Device,
{
    meta: DeviceMeta,
    state: RwLock<UARTState>,
}

impl UART {
    pub fn create_dev(
        base_addr: BusAddrType,
        size: usize,
        kind: String,
        name: String,
    ) -> Box<dyn Device> {
        let uart_dev = UART {
            meta: DeviceMeta::new(base_addr, size, kind, name),
            state: RwLock::new(UARTState::new(size)),
        };
        Box::new(uart_dev)
    }
}

impl Device for UART {
    fn data(&self) -> &DeviceMeta {
        &self.meta
    }

    // fn read<MemValType: MemoryValue>(&self, offset: usize) -> MemValType;
    // fn write<MemValType: MemoryValue>(&self, offset: usize, value: MemValType);

    // we cannot make `read()`/`write()` methods to be generic because of the
    // `dyn` traits compatibility requirement. So we declare methods for each
    // type. Read more at:
    // https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility

    // fn read_u8(&self, offset: usize) -> Result<u8, IODeviceError> {
    //     self.state
    //         .read()
    //         .map_err(|err| {
    //             error!("failed to read device state: {err}");
    //             IODeviceError::Read(offset as XLen)
    //         })
    //         .
    //         .read_impl(offset)
    // }
    fn read_u8(&self, offset: usize) -> Result<u8, IODeviceError> {
        self.data().read_impl(offset)
    }
    fn read_u16(&self, offset: usize) -> Result<u16, IODeviceError> {
        self.data().read_impl(offset)
    }
    fn read_u32(&self, offset: usize) -> Result<u32, IODeviceError> {
        self.data().read_impl(offset)
    }
    fn read_u64(&self, offset: usize) -> Result<u64, IODeviceError> {
        self.data().read_impl(offset)
    }

    fn write_u8(&self, offset: usize, value: u8) -> Result<(), IODeviceError> {
        self.data().write_impl(offset, value)
    }
    fn write_u16(
        &self,
        offset: usize,
        value: u16,
    ) -> Result<(), IODeviceError> {
        self.data().write_impl(offset, value)
    }
    fn write_u32(
        &self,
        offset: usize,
        value: u32,
    ) -> Result<(), IODeviceError> {
        self.data().write_impl(offset, value)
    }
    fn write_u64(
        &self,
        offset: usize,
        value: u64,
    ) -> Result<(), IODeviceError> {
        self.data().write_impl(offset, value)
    }
}

#[derive(Debug)]
struct UARTState {
    memory_map: MemoryMap,
}

impl UARTState {
    fn new(memory_map_size: usize) -> Self {
        UARTState {
            memory_map: MemoryMap::new(memory_map_size),
        }
    }
}
