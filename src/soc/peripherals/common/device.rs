use std::{collections::BTreeMap, marker::PhantomData, sync::RwLock};

use crate::{constants::arch::XLen, soc::memory::MemoryValue};

pub struct Devices<AddrType> {
    // TODO: not sure what is better: `RwLock` vs `Mutex`
    // because `RwLock` may cause writer starvation
    tree: RwLock<BTreeMap<XLen, Box<dyn Device>>>,

    _phantom_marker: PhantomData<AddrType>,
}

impl<AddrType> Devices<AddrType> {
    pub fn new() -> Self {
        Devices::get_reset_instance()
    }

    fn get_reset_instance() -> Self {
        Devices {
            tree: RwLock::new(Default::default()),
            _phantom_marker: Default::default(),
        }
    }
}

pub trait Device: Send + Sync {
    // we cannot make `read()`/`write()` methods to be generic because of the
    // `dyn` traits compatibility requirement. So we declare methods for each
    // type. Read more at:
    // https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility

    // fn read<MemValType: MemoryValue>(&self, offset: usize) -> MemValType;
    // fn write<MemValType: MemoryValue>(&self, offset: usize, value: MemValType);

    fn read_u8(&self, offset: usize) -> u8;
    fn read_u16(&self, offset: usize) -> u16;
    fn read_u32(&self, offset: usize) -> u32;
    fn read_u64(&self, offset: usize) -> u64;

    fn write_u8(&self, offset: usize, value: u8);
    fn write_u16(&self, offset: usize, value: u16);
    fn write_u32(&self, offset: usize, value: u32);
    fn write_u64(&self, offset: usize, value: u64);

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
