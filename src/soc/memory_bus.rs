use log::{error, trace};

use crate::BusAddrType;
use crate::constants::arch::{Byte, XLen};
use crate::soc::hart::extensions::compressed::constants::InstructionTypeCompressed;
use crate::soc::memory::{Memory, MemoryDevice, MemoryValueType};
use crate::soc::peripherals::common::device::{
    Addressable, Devices, IODeviceError, RawDevice,
};

pub trait Bus<AddrType> {
    fn read(
        &self,
        address: AddrType,
        dest: &mut [Byte],
    ) -> Result<(), IODeviceError>;
    fn write(
        &self,
        address: AddrType,
        value: &[Byte],
    ) -> Result<(), IODeviceError>;

    // instruction read/write
    fn instruction_read(
        &self,
        address: AddrType,
        dest: &mut [Byte],
    ) -> Result<(), IODeviceError>;
}

pub trait RawBus<AddrType> {
    // data read/write
    fn read_raw<MemValType: MemoryValueType>(
        &self,
        address: AddrType,
    ) -> MemValType;
    fn write_raw<MemValType: MemoryValueType>(
        &self,
        address: AddrType,
        value: MemValType,
    );
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

    // fn compute_offset_within_device(
    //     addr: AddrType,
    //     base_addr: AddrType,
    // ) -> usize { Byte, DoubleWord, HalfWord, Word
    //     (addr - base_addr) as usize
    // }
}

impl<AddrType: Send + Sync> Bus<XLen> for SystemBus<AddrType> {
    fn read(
        &self,
        address: BusAddrType,
        dest: &mut [Byte],
    ) -> Result<(), IODeviceError> {
        if (self.ram_base..=(self.ram_end - dest.len() as XLen))
            .contains(&address)
        {
            trace!("Reading memory at addr: {address:#018X?}");

            let offset = (address - self.ram_base) as usize;
            self.memory.read(offset, dest)
        } else {
            trace!("Reading MMIO at addr: {address:#018X?}");

            self.devices.read(address, dest)
        }
    }
    fn write(
        &self,
        address: BusAddrType,
        value: &[Byte],
    ) -> Result<(), IODeviceError> {
        if (self.ram_base..=(self.ram_end - value.len() as XLen))
            .contains(&address)
        {
            trace!("Writing memory at addr: {address:#018X?}");

            let offset = (address - self.ram_base) as usize;
            self.memory.write(offset, value)
        } else {
            trace!("Writing to MMIO at addr: {address:#018X?}");

            self.devices.write(address, value)
        }
    }

    fn instruction_read(
        &self,
        address: BusAddrType,
        dest: &mut [Byte],
    ) -> Result<(), IODeviceError> {
        if (self.ram_base
            ..=(self.ram_end - size_of::<InstructionTypeCompressed>() as XLen))
            .contains(&address)
        {
            trace!("Reading instruction from memory at addr: {address:#018X?}");

            let offset = (address - self.ram_base) as usize;
            self.memory.read(offset, dest)
        } else {
            error!(
                "an attempt to read instruction from unimplemented address space: {address:#018X}"
            );
            Err(IODeviceError::InstructionAccessFault(address))
        }
    }
}

impl<AddrType: Send + Sync> RawBus<XLen> for SystemBus<AddrType> {
    fn read_raw<MemValType: MemoryValueType>(
        &self,
        address: XLen,
    ) -> MemValType {
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
    fn write_raw<MemValType: MemoryValueType>(
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
