use std::sync::RwLock;

use log::error;

use crate::{
    BusAddrType, Device, IODeviceError, XLen,
    constants::arch::Byte,
    soc::peripherals::common::{device::DeviceMeta, memory_map::MemoryMap},
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
}

impl Device for UART {
    fn meta(&self) -> &DeviceMeta {
        &self.meta
    }

    fn read(
        &self,
        offset: usize,
        dest: &mut [Byte],
    ) -> Result<(), IODeviceError> {
        let guarded_state_read = self.guarded_state.read().map_err(|err| {
            error!("failed to read device state: {err}");
            IODeviceError::ReadPoisoned(offset as XLen)
        })?;
        guarded_state_read.memory_map.read(offset, dest)?;

        Ok(())
    }

    fn write(
        &self,
        offset: usize,
        value: &[Byte],
    ) -> Result<(), IODeviceError> {
        let mut guarded_state_write =
            self.guarded_state.write().map_err(|err| {
                error!("failed to write device state: {err}");
                IODeviceError::WritePoisoned(offset as XLen)
            })?;
        guarded_state_write.memory_map.write(offset, value)?;

        Ok(())
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
