use crate::constants::arch::{BusAddrType, Byte, DoubleWord, HalfWord, Word};
use crate::soc::hart::core::decoder::Decoder;
use crate::soc::hart::core::executor::Executor;
use crate::soc::memory::{MemoryValueType, MemoryValueTypeEnum};
use crate::soc::memory_bus::SystemBus;
use crate::soc::peripherals::common::device::{Addressable, IODeviceError};

use log::{error, trace};

impl Executor {
    // we cannot make `read()`/`write()` methods to be generic because of the
    // `dyn` traits compatibility requirement. So we declare methods for each
    // type. Read more at:
    // https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility

    // memop handlers

    fn handle_memory_read<const SIGNED: bool, ValType: MemoryValueType>(
        &mut self,
        mem_read_result: &Result<(), IODeviceError>,
        dest: &[Byte],
    ) {
        if mem_read_result.is_ok() {
            if SIGNED {
                self.result = ValType::from_le_bytes(dest).sign_extend();
            } else {
                self.result = ValType::from_le_bytes(dest).zero_extend();
            }
        } else {
            match mem_read_result.as_ref().unwrap_err() {
                IODeviceError::LoadAccessFault(address) => {
                    //
                    todo!("handle LAF");
                }
                IODeviceError::InstructionAccessFault(address) => {
                    //
                    todo!("handle IAF");
                }

                _ => {
                    error!("unknown error on memory read");
                    unreachable!("How???");
                }
            }
            // info!("");

            // self.call_trap_handler();
        }
    }

    fn read_memory_signed_byte(&mut self, bus: &SystemBus<BusAddrType>) {
        let mut dest = [0u8; size_of::<Byte>()];

        let mem_read_result = bus.read(self.memory_address, &mut dest);
        self.handle_memory_read::<true, Byte>(&mem_read_result, &dest);
    }
    fn read_memory_signed_half_word(&mut self, bus: &SystemBus<BusAddrType>) {
        let mut dest = [0u8; size_of::<HalfWord>()];

        let mem_read_result = bus.read(self.memory_address, &mut dest);
        self.handle_memory_read::<true, HalfWord>(&mem_read_result, &dest);
    }
    fn read_memory_signed_word(&mut self, bus: &SystemBus<BusAddrType>) {
        let mut dest = [0u8; size_of::<Word>()];

        let mem_read_result = bus.read(self.memory_address, &mut dest);
        self.handle_memory_read::<true, Word>(&mem_read_result, &dest);
    }
    fn read_memory_signed_double_word(&mut self, bus: &SystemBus<BusAddrType>) {
        let mut dest = [0u8; size_of::<DoubleWord>()];

        let mem_read_result = bus.read(self.memory_address, &mut dest);
        self.handle_memory_read::<true, DoubleWord>(&mem_read_result, &dest);
    }

    fn read_memory_unsigned_byte(&mut self, bus: &SystemBus<BusAddrType>) {
        let mut dest = [0u8; size_of::<Byte>()];

        let mem_read_result = bus.read(self.memory_address, &mut dest);
        self.handle_memory_read::<false, Byte>(&mem_read_result, &dest);
    }
    fn read_memory_unsigned_half_word(&mut self, bus: &SystemBus<BusAddrType>) {
        let mut dest = [0u8; size_of::<HalfWord>()];

        let mem_read_result = bus.read(self.memory_address, &mut dest);
        self.handle_memory_read::<false, HalfWord>(&mem_read_result, &dest);
    }
    fn read_memory_unsigned_word(&mut self, bus: &SystemBus<BusAddrType>) {
        let mut dest = [0u8; size_of::<Word>()];

        let mem_read_result = bus.read(self.memory_address, &mut dest);
        self.handle_memory_read::<false, Word>(&mem_read_result, &dest);
    }

    pub fn memory_operation(
        &mut self,
        bus: &SystemBus<BusAddrType>,
        decoder: &Decoder,
    ) {
        if decoder.need_to_read_memory {
            trace!("Memory read...");

            match self.memory_value_type {
                MemoryValueTypeEnum::DOUBLE_WORD => {
                    self.read_memory_signed_double_word(bus);
                }
                MemoryValueTypeEnum::WORD => {
                    self.read_memory_signed_word(bus);
                }
                MemoryValueTypeEnum::HALF_WORD => {
                    self.read_memory_signed_half_word(bus);
                }
                MemoryValueTypeEnum::BYTE => {
                    self.read_memory_signed_byte(bus);
                }
                MemoryValueTypeEnum::UNSIGNED_WORD => {
                    self.read_memory_unsigned_word(bus);
                }
                MemoryValueTypeEnum::UNSIGNED_HALF_WORD => {
                    self.read_memory_unsigned_half_word(bus);
                }
                MemoryValueTypeEnum::UNSIGNED_BYTE => {
                    self.read_memory_unsigned_byte(bus);
                }
            }
        } else if decoder.need_to_write_memory {
            trace!("Memory write...");

            let result;
            match self.memory_value_type {
                MemoryValueTypeEnum::DOUBLE_WORD => {
                    result = bus.write(
                        self.memory_address,
                        &self.value_to_store_u64.to_le_bytes(),
                    );
                }
                MemoryValueTypeEnum::WORD => {
                    result = bus.write(
                        self.memory_address,
                        &self.value_to_store_u32.to_le_bytes(),
                    );
                }
                MemoryValueTypeEnum::HALF_WORD => {
                    result = bus.write(
                        self.memory_address,
                        &self.value_to_store_u16.to_le_bytes(),
                    );
                }
                MemoryValueTypeEnum::BYTE => {
                    result = bus.write(
                        self.memory_address,
                        &self.value_to_store_u8.to_le_bytes(),
                    );
                }

                MemoryValueTypeEnum::UNSIGNED_WORD
                | MemoryValueTypeEnum::UNSIGNED_HALF_WORD
                | MemoryValueTypeEnum::UNSIGNED_BYTE => {
                    error!("unreachable!");
                    unreachable!("unreachable!");
                }
            }

            if let Err(err) = result {
                // info!("ISA exception, {err}");
                match err {
                    IODeviceError::StoreAMOAccessFault(address) => {
                        //
                        todo!("handle SAF");
                    }

                    _ => {
                        error!("unknown error on memory read");
                        unreachable!("How???");
                    }
                }
            }
        }
    }
}
