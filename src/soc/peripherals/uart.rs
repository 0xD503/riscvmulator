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
    guarded_state: RwLock<UARTState>,
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
            guarded_state: RwLock::new(UARTState::new(size)),
        };
        Box::new(uart_dev)
    }

    // fn state_locker(&self) -> &RwLock<UARTState> {
    //     &self.state
    // }

    fn read_impl<MemoryValType: MemoryValue>(
        &self,
        offset: usize,
    ) -> Result<MemoryValType, IODeviceError> {
        let guarded_state_read = self.guarded_state.read().map_err(|err| {
            error!("failed to read device state: {err}");
            IODeviceError::Read(offset as XLen)
        })?;

        guarded_state_read.memory_map.read_impl(offset)
    }

    fn write_impl<MemoryValType: MemoryValue>(
        &self,
        offset: usize,
        value: MemoryValType,
    ) -> Result<(), IODeviceError> {
        let mut guarded_state_write =
            self.guarded_state.write().map_err(|err| {
                error!("failed to write device state: {err}");
                IODeviceError::Write(offset as XLen)
            })?;

        guarded_state_write.memory_map.write_impl(offset, value)
    }
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
