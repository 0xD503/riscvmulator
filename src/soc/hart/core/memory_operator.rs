use crate::constants::arch::BusAddrType;
use crate::soc::hart::core::decoder::Decoder;
use crate::soc::hart::core::executor::Executor;
use crate::soc::memory::{MemoryValue, MemoryValueType};
use crate::soc::memory_bus::{Bus, SystemBus};

use log::{error, trace};

impl Executor {
    // we cannot make `read()`/`write()` methods to be generic because of the
    // `dyn` traits compatibility requirement. So we declare methods for each
    // type. Read more at:
    // https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility

    // memop handlers
    fn read_memory_signed_byte(&mut self, bus: &SystemBus<BusAddrType>) {
        self.result = bus.read_u8(self.memory_address).sign_extend();
    }
    fn read_memory_signed_half_word(&mut self, bus: &SystemBus<BusAddrType>) {
        self.result = bus.read_u16(self.memory_address).sign_extend();
    }
    fn read_memory_signed_word(&mut self, bus: &SystemBus<BusAddrType>) {
        self.result = bus.read_u32(self.memory_address).sign_extend();
    }
    fn read_memory_signed_double_word(&mut self, bus: &SystemBus<BusAddrType>) {
        self.result = bus.read_u64(self.memory_address).sign_extend();
    }

    fn read_memory_unsigned_byte(&mut self, bus: &SystemBus<BusAddrType>) {
        self.result = bus.read_u8(self.memory_address).zero_extend();
    }
    fn read_memory_unsigned_half_word(&mut self, bus: &SystemBus<BusAddrType>) {
        self.result = bus.read_u16(self.memory_address).zero_extend();
    }
    fn read_memory_unsigned_word(&mut self, bus: &SystemBus<BusAddrType>) {
        self.result = bus.read_u32(self.memory_address).zero_extend();
    }
    // fn read_memory_unsigned_u64(&mut self, bus: &SystemBus<BusAddrType>) {
    //     self.result = bus.read_u64(self.memory_address).zero_extend();
    // }

    // fn read_memory_signed<ValType: MemoryValue>(
    //     &mut self,
    //     bus: &SystemBus<BusAddrType>,
    // ) {
    //     self.result = bus.read::<ValType>(self.memory_address).sign_extend();
    // }
    // fn read_memory_unsigned<ValType: MemoryValue>(
    //     &mut self,
    //     bus: &SystemBus<BusAddrType>,
    // ) {
    //     self.result = bus.read::<ValType>(self.memory_address).zero_extend();
    // }

    pub fn memory_operation(
        &mut self,
        bus: &SystemBus<BusAddrType>,
        decoder: &Decoder,
    ) {
        if decoder.need_to_read_memory {
            trace!("Executing memory read...");

            match self.memory_value_type {
                MemoryValueType::DOUBLE_WORD => {
                    self.read_memory_signed_double_word(bus);
                }
                MemoryValueType::WORD => {
                    self.read_memory_signed_word(bus);
                }
                MemoryValueType::HALF_WORD => {
                    self.read_memory_signed_half_word(bus);
                }
                MemoryValueType::BYTE => {
                    self.read_memory_signed_byte(bus);
                }
                MemoryValueType::UNSIGNED_WORD => {
                    self.read_memory_unsigned_word(bus);
                }
                MemoryValueType::UNSIGNED_HALF_WORD => {
                    self.read_memory_unsigned_half_word(bus);
                }
                MemoryValueType::UNSIGNED_BYTE => {
                    self.read_memory_unsigned_byte(bus);
                }
            }
        } else if decoder.need_to_write_memory {
            trace!("Executing memory write...");

            match self.memory_value_type {
                MemoryValueType::DOUBLE_WORD => {
                    bus.write_u64(self.memory_address, self.value_to_store_u64);
                }
                MemoryValueType::WORD => {
                    bus.write_u32(self.memory_address, self.value_to_store_u32);
                }
                MemoryValueType::HALF_WORD => {
                    bus.write_u16(self.memory_address, self.value_to_store_u16);
                }
                MemoryValueType::BYTE => {
                    bus.write_u8(self.memory_address, self.value_to_store_u8);
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
