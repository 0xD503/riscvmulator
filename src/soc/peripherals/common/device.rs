use crate::soc::memory::MemoryValue;

pub trait Device {
    fn read<MemValType: MemoryValue>(&self, offset: usize) -> MemValType;
    fn write<MemValType: MemoryValue>(&self, offset: usize, value: MemValType);

    // fn base_addr() -> BusAddrType;
    // fn mem_space_size() -> usize;
    // fn end_addr() -> BusAddrType;
}

pub trait RawDevice {
    fn read_raw<MemValType: MemoryValue>(&self, offset: usize) -> MemValType;
    fn write_raw<MemValType: MemoryValue>(
        &self,
        offset: usize,
        value: MemValType,
    );
}

// pub trait Device<ValueType> {
//     fn read<ValueType>(&self, offset: usize) -> ValueType;
//     fn write<ValueType>(&mut self, offset: usize, value: ValueType);
// }

// pub trait RawDevice<ValueType> {
//     fn read_raw<ValueType>(&self, offset: usize) -> ValueType;
//     fn write_raw<ValueType>(&mut self, offset: usize, value: ValueType);
// }
