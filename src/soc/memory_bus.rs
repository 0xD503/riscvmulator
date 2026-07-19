use log::{error, trace};

use crate::constants::arch::{Byte, XLen};
use crate::soc::hart::extensions::compressed::constants::InstructionTypeCompressed;
use crate::soc::memory::{Memory, MemoryDevice, MemoryValue};
use crate::soc::peripherals::common::device::{
    Devices, IODeviceError, RawDevice,
};

pub trait Bus<AddrType> {
    // fn memory_reset(&mut self);

    // we cannot make `read()`/`write()` methods to be generic because of the
    // `dyn` traits compatibility requirement. So we declare methods for each
    // type. Read more at:
    // https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility

    fn read_u8(&self, address: AddrType) -> Result<u8, IODeviceError>;
    fn read_u16(&self, address: AddrType) -> Result<u16, IODeviceError>;
    fn read_u32(&self, address: AddrType) -> Result<u32, IODeviceError>;
    fn read_u64(&self, address: AddrType) -> Result<u64, IODeviceError>;

    fn write_u8(
        &self,
        address: AddrType,
        value: u8,
    ) -> Result<(), IODeviceError>;
    fn write_u16(
        &self,
        address: AddrType,
        value: u16,
    ) -> Result<(), IODeviceError>;
    fn write_u32(
        &self,
        address: AddrType,
        value: u32,
    ) -> Result<(), IODeviceError>;
    fn write_u64(
        &self,
        address: AddrType,
        value: u64,
    ) -> Result<(), IODeviceError>;

    // fn read<MemValType: MemoryValue>(&self, address: AddrType) -> MemValType;
    // fn write<MemValType: MemoryValue>(
    //     &self,
    //     address: AddrType,
    //     value: MemValType,
    // );

    // instruction read/write
    fn instruction_read(&self, address: AddrType) -> InstructionTypeCompressed;
    // fn instruction_read<MemValType: MemoryValue>(
    //     &self,
    //     address: AddrType,
    // ) -> MemValType;
}

pub trait RawBus<AddrType> {
    // data read/write
    fn read_raw<MemValType: MemoryValue>(
        &self,
        address: AddrType,
    ) -> MemValType;
    // fn read_data(&self, address: XLen, size: usize) -> Vec<Byte>;

    fn write_raw<MemValType: MemoryValue>(
        &self,
        address: AddrType,
        value: MemValType,
    );
    // fn write_data(&mut self, address: XLen, data: Vec<Byte>);

    // // instruction read/write
    // fn instruction_read_raw<MemValType: MemoryValue>(
    //     &self,
    //     address: AddrType,
    // ) -> MemValType;
}

pub struct SystemBus<AddrType: Send + Sync> {
    ram_base: XLen,
    memory: Memory<AddrType>,
    ram_end: XLen,

    pub(crate) devices: Devices<AddrType>,
}

impl<AddrType: Send + Sync> SystemBus<AddrType> {
    pub fn new(fill_value: Byte, ram_base: XLen, ram_size: usize) -> Self {
        Self {
            ram_base: ram_base,
            memory: Memory::new(fill_value, ram_size),
            ram_end: ram_base + ram_size as XLen,

            devices: Devices::new(),
        }
    }

    pub fn reset(&self) {
        self.memory.reset();
    }

    // pub fn fill_with_nops(&mut self) {
    //     self.memory_reset();
    //     for addr in
    //         (self.ram_base..self.ram_end).step_by(size_of::<InstructionType>())
    //     {
    //         self.write_raw(addr, Opcode::OP_IMM as Byte);
    //     }
    // }

    // fn compute_offset_within_device(
    //     addr: AddrType,
    //     base_addr: AddrType,
    // ) -> usize { Byte, DoubleWord, HalfWord, Word
    //     (addr - base_addr) as usize
    // }
}

impl<AddrType: Send + Sync> Bus<XLen> for SystemBus<AddrType> {
    fn read_u8(&self, address: XLen) -> Result<u8, IODeviceError> {
        if (self.ram_base..=(self.ram_end - size_of::<u8>() as XLen))
            .contains(&address)
        {
            trace!("Reading memory at addr: {address:#018X?}");

            let offset = (address - self.ram_base) as usize;
            Ok(self.memory.read_u8(offset))
        } else {
            trace!("Reading MMIO at addr: {address:#018X?}");

            self.devices.read_u8(address)
        }
    }
    fn read_u16(&self, address: XLen) -> Result<u16, IODeviceError> {
        if (self.ram_base..=(self.ram_end - size_of::<u16>() as XLen))
            .contains(&address)
        {
            trace!("Reading memory at addr: {address:#018X?}");

            let offset = (address - self.ram_base) as usize;
            Ok(self.memory.read_u16(offset))
        } else {
            trace!("Reading MMIO at addr: {address:#018X?}");

            self.devices.read_u16(address)
        }
    }
    fn read_u32(&self, address: XLen) -> Result<u32, IODeviceError> {
        if (self.ram_base..=(self.ram_end - size_of::<u32>() as XLen))
            .contains(&address)
        {
            trace!("Reading memory at addr: {address:#018X?}");

            let offset = (address - self.ram_base) as usize;
            Ok(self.memory.read_u32(offset))
        } else {
            trace!("Reading MMIO at addr: {address:#018X?}");

            self.devices.read_u32(address)
        }
    }
    fn read_u64(&self, address: XLen) -> Result<u64, IODeviceError> {
        if (self.ram_base..=(self.ram_end - size_of::<u64>() as XLen))
            .contains(&address)
        {
            trace!("Reading memory at addr: {address:#018X?}");

            let offset = (address - self.ram_base) as usize;
            Ok(self.memory.read_u64(offset))
        } else {
            trace!("Reading MMIO at addr: {address:#018X?}");

            self.devices.read_u64(address)
        }
    }

    fn write_u8(&self, address: XLen, value: u8) -> Result<(), IODeviceError> {
        if (self.ram_base..=(self.ram_end - size_of::<u8>() as XLen))
            .contains(&address)
        {
            trace!("Writing memory at addr: {address:#018X?}");

            let offset = (address - self.ram_base) as usize;
            self.memory.write_u8(offset, value);
            Ok(())
        } else {
            trace!("Writing to MMIO at addr: {address:#018X?}");

            self.devices.write_u8(address, value)
        }
    }
    fn write_u16(
        &self,
        address: XLen,
        value: u16,
    ) -> Result<(), IODeviceError> {
        if (self.ram_base..=(self.ram_end - size_of::<u16>() as XLen))
            .contains(&address)
        {
            trace!("Writing memory at addr: {address:#018X?}");

            let offset = (address - self.ram_base) as usize;
            self.memory.write_u16(offset, value);
            Ok(())
        } else {
            trace!("Writing to MMIO at addr: {address:#018X?}");

            self.devices.write_u16(address, value)
        }
    }
    fn write_u32(
        &self,
        address: XLen,
        value: u32,
    ) -> Result<(), IODeviceError> {
        if (self.ram_base..=(self.ram_end - size_of::<u32>() as XLen))
            .contains(&address)
        {
            trace!("Writing memory at addr: {address:#018X?}");

            let offset = (address - self.ram_base) as usize;
            self.memory.write_u32(offset, value);
            Ok(())
        } else {
            trace!("Writing to MMIO at addr: {address:#018X?}");

            self.devices.write_u32(address, value)
        }
    }
    fn write_u64(
        &self,
        address: XLen,
        value: u64,
    ) -> Result<(), IODeviceError> {
        if (self.ram_base..=(self.ram_end - size_of::<u64>() as XLen))
            .contains(&address)
        {
            trace!("Writing memory at addr: {address:#018X?}");

            let offset = (address - self.ram_base) as usize;
            self.memory.write_u64(offset, value);
            Ok(())
        } else {
            trace!("Writing to MMIO at addr: {address:#018X?}");

            self.devices.write_u64(address, value)
        }
    }

    // fn read<MemValType: MemoryValue>(&self, address: XLen) -> MemValType {
    //     if (self.ram_base..=(self.ram_end - size_of::<MemValType>() as XLen))
    //         .contains(&address)
    //     {
    //         trace!("Reading memory at addr: {address:#018X?}");

    //         let offset = (address - self.ram_base) as usize;
    //         self.memory.read(offset)
    //     } else {
    //         error!(
    //             "an attempt to read from unimplemented address space: {address:#018X}"
    //         );
    //         panic!(
    //             "an attempt to read from unimplemented address space: {address:#018X}"
    //         );
    //     }
    // }
    // fn write<MemValType: MemoryValue>(&self, address: XLen, value: MemValType) {
    //     if (self.ram_base..=(self.ram_end - size_of::<MemValType>() as XLen))
    //         .contains(&address)
    //     {
    //         trace!("Writing memory at addr: {address:#018X?}");

    //         let offset = (address - self.ram_base) as usize;
    //         self.memory.write(offset, value);
    //     } else {
    //         error!(
    //             "an attempt to write to unimplemented address space: {address:#018X}"
    //         );
    //         panic!(
    //             "an attempt to write to unimplemented address space: {address:#018X}"
    //         );
    //     }
    // }

    fn instruction_read(&self, address: XLen) -> InstructionTypeCompressed {
        if (self.ram_base
            ..=(self.ram_end - size_of::<InstructionTypeCompressed>() as XLen))
            .contains(&address)
        {
            trace!("Reading instruction from memory at addr: {address:#018X?}");

            let offset = (address - self.ram_base) as usize;
            self.memory.read_u16(offset)
        } else {
            error!(
                "an attempt to read instruction from unimplemented address space: {address:#018X}"
            );
            panic!(
                "an attempt to read instruction from unimplemented address space: {address:#018X}"
            );
        }
    }

    // fn instruction_read<MemValType: MemoryValue>(
    //     &self,
    //     address: XLen,
    // ) -> MemValType {
    //     if (self.ram_base..=(self.ram_end - size_of::<MemValType>() as XLen))
    //         .contains(&address)
    //     {
    //         trace!("Reading instruction from memory at addr: {address:#018X?}");

    //         let offset = (address - self.ram_base) as usize;
    //         self.memory.read(offset)
    //     } else {
    //         error!(
    //             "an attempt to read instruction from unimplemented address space: {address:#018X}"
    //         );
    //         panic!(
    //             "an attempt to read instruction from unimplemented address space: {address:#018X}"
    //         );
    //     }
    // }

    // fn memory_reset(&mut self) {
    //     self.memory.reset();
    // }
}

impl<AddrType: Send + Sync> RawBus<XLen> for SystemBus<AddrType> {
    fn read_raw<MemValType: MemoryValue>(&self, address: XLen) -> MemValType {
        if (self.ram_base..=(self.ram_end - size_of::<MemValType>() as XLen))
            .contains(&address)
        {
            trace!("Reading memory at addr: {address:#018X?}");

            let offset = (address - self.ram_base) as usize;
            self.memory.read_raw(offset)
        } else {
            error!(
                "an attempt to read from unimplemented address space: {address:#018X}"
            );
            panic!(
                "an attempt to read from unimplemented address space: {address:#018X}"
            );
        }
    }
    fn write_raw<MemValType: MemoryValue>(
        &self,
        address: XLen,
        value: MemValType,
    ) {
        if (self.ram_base..=(self.ram_end - size_of::<MemValType>() as XLen))
            .contains(&address)
        {
            trace!("Writing memory at addr: {address:#018X?}");

            let offset = (address - self.ram_base) as usize;
            self.memory.write_raw(offset, value);
        } else {
            error!(
                "an attempt to write to unimplemented address space: {address:#018X}"
            );
            panic!(
                "an attempt to write to unimplemented address space: {address:#018X}"
            );
        }
    }
}
