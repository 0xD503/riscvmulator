use log::{error, info, trace};

use crate::constants::arch::{Byte, InstructionType, XLen};
use crate::soc::memory::{Memory, MemoryValue};

use super::cpu::core::decoder::constants::Opcode;

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
    ram_base: XLen,
    memory: Memory<AddrType>,
    ram_end: XLen,

    // testing facilities
    initialized: bool,
    htif_to_host: XLen,
}

impl<AddrType> SystemBus<AddrType> {
    pub fn new(fill_value: Byte, ram_size: usize, ram_base: XLen) -> Self {
        Self {
            ram_base,
            memory: Memory::new(fill_value, ram_size),
            ram_end: ram_base + ram_size as XLen,

            initialized: false,
            htif_to_host: 0x8000_1000,
        }
    }

    pub fn reset(&mut self) {
        self.memory.reset();

        self.initialized = false;
        self.htif_to_host = 0x8000_1000;
    }

    pub fn fill_with_nops(&mut self) {
        self.memory_reset();
        for addr in
            (self.ram_base..self.ram_end).step_by(size_of::<InstructionType>())
        {
            self.write(addr, Opcode::OP_IMM as Byte);
        }
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
        if (self.ram_base..=(self.ram_end - size_of::<MemValType>() as XLen))
            .contains(&address)
        {
            trace!("Reading memory at addr: {address:#018X?}");

            let offset = address - self.ram_base;
            self.memory.read(offset)
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
        if (self.ram_base..=(self.ram_end - size_of::<MemValType>() as XLen))
            .contains(&address)
        {
            trace!("Writing memory at addr: {address:#018X?}");

            let offset = address - self.ram_base;
            self.memory.write(offset, value);
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
        if (self.ram_base..=(self.ram_end - size_of::<MemValType>() as XLen))
            .contains(&address)
        {
            trace!("Reading instruction from memory at addr: {address:#018X?}");

            let offset = address - self.ram_base;
            self.memory.read(offset)
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
