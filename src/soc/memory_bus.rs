use log::{error, info};

use crate::constants::arch::{Byte, XLen};
use crate::constants::soc::{RAM_BASE, RAM_END};
use crate::soc::memory::{Memory, MemoryValue};

// pub const HTIF_TO_HOST: XLen = 0x80001000;
// // pub const HTIF_FROM_HOST: XLen = 0x80001004;

pub trait Bus<AddrType> {
    fn memory_reset(&mut self);

    // data read/write
    fn read<MemValType: MemoryValue>(&self, address: AddrType) -> MemValType;
    // fn read_data(&self, address: XLen, size: usize) -> Vec<Byte>;

    fn write<MemValType: MemoryValue>(
        &mut self,
        address: AddrType,
        value: MemValType,
    );
    // fn write_data(&mut self, address: XLen, data: Vec<Byte>);

    // instruction read/write
    fn instruction_read<MemValType: MemoryValue>(
        &self,
        address: AddrType,
    ) -> MemValType;
}

#[derive(Default)]
pub struct SystemBus<AddrType> {
    memory: Memory<AddrType>,
    initialized: bool,

    // testing facilities
    htif_to_host: XLen,
}

impl<AddrType> SystemBus<AddrType> {
    pub fn new(fill_value: Byte, size: usize) -> Self {
        Self {
            memory: Memory::new(fill_value, size),
            initialized: false,
            htif_to_host: 0x80001000,
        }
    }

    pub fn reset(&mut self) {
        self.memory.reset();
        self.initialized = false;
        self.htif_to_host = 0x80001000;
    }

    // testing facilities
    pub fn ready_for_testing(&mut self) {
        self.initialized = true;
    }

    pub fn set_htif(&mut self, addr: XLen) {
        self.htif_to_host = addr;
    }
}

impl<AddrType> Bus<XLen> for SystemBus<AddrType> {
    fn read<MemValType: MemoryValue>(&self, address: XLen) -> MemValType {
        if (RAM_BASE..=(RAM_END - size_of::<MemValType>() as XLen))
            .contains(&address)
        {
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
        // TODO: test scaffolding, isolate later
        if address == self.htif_to_host {
            if self.initialized {
                if value.to_u64() == 1 {
                    info!("TEST PASSED");
                    std::process::exit(0);
                } else {
                    error!("TEST FAILED");
                    std::process::exit(value.to_u64() as i32);
                }
            }
        }
        if (RAM_BASE..=(RAM_END - size_of::<MemValType>() as XLen))
            .contains(&address)
        {
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

    fn instruction_read<MemValType: MemoryValue>(
        &self,
        address: XLen,
    ) -> MemValType {
        if (RAM_BASE..=(RAM_END - size_of::<MemValType>() as XLen))
            .contains(&address)
        {
            self.memory.read(address)
        } else {
            error!(
                "an attempt to read instruction from unimplemented address space: {address:#018X}"
            );
            panic!(
                "an attempt to read instruction from unimplemented address space: {address:#018X}"
            );
        }
    }

    fn memory_reset(&mut self) {
        self.memory.reset();
    }
}
