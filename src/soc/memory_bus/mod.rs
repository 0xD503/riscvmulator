use log::error;

use crate::constants::arch::{Byte, XLen};
use crate::constants::soc::{RAM_BASE, RAM_END};
use crate::soc::memory::{Memory, MemoryValue};

pub trait Bus<AddrType> {
    fn read<MemValType: MemoryValue>(&self, address: AddrType) -> MemValType;
    // fn read_data(&self, address: XLen, size: usize) -> Vec<Byte>;

    fn write<MemValType: MemoryValue>(
        &mut self,
        address: AddrType,
        value: MemValType,
    );
    // fn write_data(&mut self, address: XLen, data: Vec<Byte>);

    fn memory_reset(&mut self);
}

pub struct SystemBus<AddrType> {
    memory: Memory<AddrType>,
}

impl<AddrType> SystemBus<AddrType> {
    pub fn new(fill_value: Byte, size: usize) -> Self {
        Self {
            memory: Memory::new(fill_value, size),
        }
    }
}

impl<AddrType> Bus<XLen> for SystemBus<AddrType> {
    fn read<MemValType: MemoryValue>(&self, address: XLen) -> MemValType {
        if (RAM_BASE..RAM_END).contains(&address) {
            self.memory.read(address)
        } else {
            error!(
                "an attempt to read from unimplemented address space: {address:#018X}"
            );
            panic!(
                "an attempt to read from unimplemented address space: {address:#018X}"
            );
        }
    }
    fn write<MemValType: MemoryValue>(
        &mut self,
        address: XLen,
        value: MemValType,
    ) {
        if (RAM_BASE..RAM_END).contains(&address) {
            self.memory.write(address, value);
        } else {
            error!(
                "an attempt to write to unimplemented address space: {address:#018X}"
            );
            panic!(
                "an attempt to write to unimplemented address space: {address:#018X}"
            );
        }
    }

    fn memory_reset(&mut self) {
        self.memory.reset();
    }
}
