use crate::constants::arch::{BusAddrType, Byte, DoubleWord, HalfWord, Word};
use crate::soc::hart::core::decoder::Decoder;
use crate::soc::hart::core::executor::Executor;
use crate::soc::memory::{MemoryValue, MemoryValueType};
use crate::soc::memory_bus::{Bus, SystemBus};

use log::{error, trace};

impl Executor {
    // memop handlers
    fn read_memory_signed<ValType: MemoryValue>(
        &mut self,
        bus: &mut SystemBus<BusAddrType>,
    ) {
        self.result = bus.read::<ValType>(self.memory_address).sign_extend();
    }

    fn read_memory_unsigned<ValType: MemoryValue>(
        &mut self,
        bus: &mut SystemBus<BusAddrType>,
    ) {
        self.result = bus.read::<ValType>(self.memory_address).zero_extend();
    }

    pub fn memory_operation(
        &mut self,
        bus: &mut SystemBus<BusAddrType>,
        decoder: &Decoder,
    ) {
        if decoder.need_to_read_memory {
            trace!("Executing memory read...");

            match self.memory_value_type {
                MemoryValueType::DOUBLE_WORD => {
                    self.read_memory_signed::<DoubleWord>(bus);
                }
                MemoryValueType::WORD => {
                    self.read_memory_signed::<Word>(bus);
                }
                MemoryValueType::HALF_WORD => {
                    self.read_memory_signed::<HalfWord>(bus);
                }
                MemoryValueType::BYTE => {
                    self.read_memory_signed::<Byte>(bus);
                }
                MemoryValueType::UNSIGNED_WORD => {
                    self.read_memory_unsigned::<Word>(bus);
                }
                MemoryValueType::UNSIGNED_HALF_WORD => {
                    self.read_memory_unsigned::<HalfWord>(bus);
                }
                MemoryValueType::UNSIGNED_BYTE => {
                    self.read_memory_unsigned::<Byte>(bus);
                }
            }
        } else if decoder.need_to_write_memory {
            trace!("Executing memory write...");

            match self.memory_value_type {
                MemoryValueType::DOUBLE_WORD => {
                    bus.write(self.memory_address, self.value_to_store_u64);
                }
                MemoryValueType::WORD => {
                    bus.write(self.memory_address, self.value_to_store_u32);
                }
                MemoryValueType::HALF_WORD => {
                    bus.write(self.memory_address, self.value_to_store_u16);
                }
                MemoryValueType::BYTE => {
                    bus.write(self.memory_address, self.value_to_store_u8);
                }

                MemoryValueType::UNSIGNED_WORD
                | MemoryValueType::UNSIGNED_HALF_WORD
                | MemoryValueType::UNSIGNED_BYTE => {
                    error!("unreachable!");
                    unreachable!("unreachable!");
                }
            }
        }
    }
}
