use std::sync::RwLock;

use log::error;

use crate::{
    BusAddrType, Device, IODeviceError, MemoryValue, XLen,
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

    fn read_impl<MemoryValType: MemoryValue>(
        &self,
        offset: usize,
    ) -> Result<MemoryValType, IODeviceError> {
        let read_guarded_state = self.state.read().map_err(|err| {
            error!("failed to read UART state: {err}");
            IODeviceError::Read(offset as XLen)
        })?;

        read_guarded_state.memory_map.read_impl(offset)
    }

    fn write_impl<MemoryValType: MemoryValue>(
        &self,
        offset: usize,
        value: MemoryValType,
    ) -> Result<(), IODeviceError> {
        let mut write_guarded_state = self.state.write().map_err(|err| {
            error!("failed to write UART state: {err}");
            IODeviceError::Write(offset as XLen)
        })?;

        write_guarded_state.memory_map.write_impl(offset, value)
    }

    // fn state_read(&self) -> &UARTState {
    //     &self.state.read().map_err(|err| {
    //         error!("failed to read UART state: {err}");
    //         IODeviceError::Read(offset as XLen)
    //     })
    // }
}

impl Device for UART {
    fn meta(&self) -> &DeviceMeta {
        &self.meta
    }

    // we cannot make `read()`/`write()` methods to be generic because of the
    // `dyn` traits compatibility requirement. So we declare methods for each
    // type. Read more at:
    // https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility

    fn read_u8(&self, offset: usize) -> Result<u8, IODeviceError> {
        self.read_impl(offset)
    }
    fn read_u16(&self, offset: usize) -> Result<u16, IODeviceError> {
        self.read_impl(offset)
    }
    fn read_u32(&self, offset: usize) -> Result<u32, IODeviceError> {
        self.read_impl(offset)
    }
    fn read_u64(&self, offset: usize) -> Result<u64, IODeviceError> {
        self.read_impl(offset)
    }

    fn write_u8(&self, offset: usize, value: u8) -> Result<(), IODeviceError> {
        self.write_impl(offset, value)
    }
    fn write_u16(
        &self,
        offset: usize,
        value: u16,
    ) -> Result<(), IODeviceError> {
        self.write_impl(offset, value)
    }
    fn write_u32(
        &self,
        offset: usize,
        value: u32,
    ) -> Result<(), IODeviceError> {
        self.write_impl(offset, value)
    }
    fn write_u64(
        &self,
        offset: usize,
        value: u64,
    ) -> Result<(), IODeviceError> {
        self.write_impl(offset, value)
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
