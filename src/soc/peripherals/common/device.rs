use std::{collections::BTreeMap, marker::PhantomData, sync::RwLock};

use crate::{
    constants::arch::{BusAddrType, XLen},
    globals::exit_codes::ExitCode,
    soc::memory::MemoryValue,
};

pub struct Devices<AddrType> {
    // TODO: not sure what is better: `RwLock` vs `Mutex`
    // because `RwLock` may cause writer starvation
    pub(crate) tree: RwLock<BTreeMap<XLen, Box<dyn Device>>>,

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

    pub fn read_u8(&self, addr: XLen) -> Result<u8, DeviceIOError> {
        let result;

        let device_tree = self.tree.read().unwrap_or_else(|err| {
            todo!("Is it even possible???: {err}");
        });
        let closest_dev = device_tree.range(..=addr).next_back();

        if let Some(device_entry) = closest_dev {
            let device = device_entry.1;
            let device_addr_range = device.base_addr()
                ..=(device.end_addr() - size_of::<u8>() as XLen);
            if device_addr_range.contains(&addr) {
                let offset = (addr - device.base_addr()) as usize;
                result = device.read_u8(offset);
            } else {
                result = Err(DeviceIOError::Read(addr))
            }
        } else {
            result = Err(DeviceIOError::Read(addr))
        }

        result
    }
    pub fn read_u16(&self, addr: XLen) -> Result<u16, DeviceIOError> {
        let result;

        let device_tree = self.tree.read().unwrap_or_else(|err| {
            todo!("Is it even possible???: {err}");
        });
        let closest_dev = device_tree.range(..=addr).next_back();

        if let Some(device_entry) = closest_dev {
            let device = device_entry.1;
            let device_addr_range = device.base_addr()
                ..=(device.end_addr() - size_of::<u16>() as XLen);
            if device_addr_range.contains(&addr) {
                let offset = (addr - device.base_addr()) as usize;
                result = device.read_u16(offset);
            } else {
                result = Err(DeviceIOError::Read(addr))
            }
        } else {
            result = Err(DeviceIOError::Read(addr))
        }

        result
    }
    pub fn read_u32(&self, addr: XLen) -> Result<u32, DeviceIOError> {
        let result;

        let device_tree = self.tree.read().unwrap_or_else(|err| {
            todo!("Is it even possible???: {err}");
        });
        let closest_dev = device_tree.range(..=addr).next_back();

        if let Some(device_entry) = closest_dev {
            let device = device_entry.1;
            let device_addr_range = device.base_addr()
                ..=(device.end_addr() - size_of::<u32>() as XLen);
            if device_addr_range.contains(&addr) {
                let offset = (addr - device.base_addr()) as usize;
                result = device.read_u32(offset);
            } else {
                result = Err(DeviceIOError::Read(addr))
            }
        } else {
            result = Err(DeviceIOError::Read(addr))
        }

        result
    }
    pub fn read_u64(&self, addr: XLen) -> Result<u64, DeviceIOError> {
        let result;

        let device_tree = self.tree.read().unwrap_or_else(|err| {
            todo!("Is it even possible???: {err}");
        });
        let closest_dev = device_tree.range(..=addr).next_back();

        if let Some(device_entry) = closest_dev {
            let device = device_entry.1;
            let device_addr_range = device.base_addr()
                ..=(device.end_addr() - size_of::<u64>() as XLen);
            if device_addr_range.contains(&addr) {
                let offset = (addr - device.base_addr()) as usize;
                result = device.read_u64(offset);
            } else {
                result = Err(DeviceIOError::Read(addr))
            }
        } else {
            result = Err(DeviceIOError::Read(addr))
        }

        result
    }

    pub fn write_u8(&self, addr: XLen, value: u8) -> Result<(), DeviceIOError> {
        let result;

        let device_tree = self.tree.read().unwrap_or_else(|err| {
            todo!("Is it even possible???: {err}");
        });
        let closest_dev = device_tree.range(..=addr).next_back();

        if let Some(device_entry) = closest_dev {
            let device = device_entry.1;
            let device_addr_range = device.base_addr()
                ..=(device.end_addr() - size_of::<u8>() as XLen);
            if device_addr_range.contains(&addr) {
                let offset = (addr - device.base_addr()) as usize;
                result = device.write_u8(offset, value);
            } else {
                result = Err(DeviceIOError::Write(addr))
            }
        } else {
            result = Err(DeviceIOError::Write(addr))
        }

        result
    }

    pub fn write_u16(
        &self,
        addr: XLen,
        value: u16,
    ) -> Result<(), DeviceIOError> {
        let result;

        let device_tree = self.tree.read().unwrap_or_else(|err| {
            todo!("Is it even possible???: {err}");
        });
        let closest_dev = device_tree.range(..=addr).next_back();

        if let Some(device_entry) = closest_dev {
            let device = device_entry.1;
            let device_addr_range = device.base_addr()
                ..=(device.end_addr() - size_of::<u16>() as XLen);
            if device_addr_range.contains(&addr) {
                let offset = (addr - device.base_addr()) as usize;
                result = device.write_u16(offset, value);
            } else {
                result = Err(DeviceIOError::Write(addr))
            }
        } else {
            result = Err(DeviceIOError::Write(addr))
        }

        result
    }

    pub fn write_u32(
        &self,
        addr: XLen,
        value: u32,
    ) -> Result<(), DeviceIOError> {
        let result;

        let device_tree = self.tree.read().unwrap_or_else(|err| {
            todo!("Is it even possible???: {err}");
        });
        let closest_dev = device_tree.range(..=addr).next_back();

        if let Some(device_entry) = closest_dev {
            let device = device_entry.1;
            let device_addr_range = device.base_addr()
                ..=(device.end_addr() - size_of::<u32>() as XLen);
            if device_addr_range.contains(&addr) {
                let offset = (addr - device.base_addr()) as usize;
                result = device.write_u32(offset, value);
            } else {
                result = Err(DeviceIOError::Write(addr))
            }
        } else {
            result = Err(DeviceIOError::Write(addr))
        }

        result
    }

    pub fn write_u64(
        &self,
        addr: XLen,
        value: u64,
    ) -> Result<(), DeviceIOError> {
        let result;

        let device_tree = self.tree.read().unwrap_or_else(|err| {
            todo!("Is it even possible???: {err}");
        });
        let closest_dev = device_tree.range(..=addr).next_back();

        if let Some(device_entry) = closest_dev {
            let device = device_entry.1;
            let device_addr_range = device.base_addr()
                ..=(device.end_addr() - size_of::<u64>() as XLen);
            if device_addr_range.contains(&addr) {
                let offset = (addr - device.base_addr()) as usize;
                result = device.write_u64(offset, value);
            } else {
                result = Err(DeviceIOError::Write(addr))
            }
        } else {
            result = Err(DeviceIOError::Write(addr))
        }

        result
    }
}

// impl<AddrType> Device for Device<AddrType> {
//     //
// }

pub enum DeviceIOError {
    Read(XLen),
    Write(XLen),
}

impl DeviceIOError {
    // pub fn exit_code(&self) -> i32 {
    //     match self {
    //         Self::Read(_) => ExitCode::DeviceReadError as i32,
    //     }
    // }
}

pub trait Device: Send + Sync {
    // we cannot make `read()`/`write()` methods to be generic because of the
    // `dyn` traits compatibility requirement. So we declare methods for each
    // type. Read more at:
    // https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility

    // fn read<MemValType: MemoryValue>(&self, offset: usize) -> MemValType;
    // fn write<MemValType: MemoryValue>(&self, offset: usize, value: MemValType);

    fn read_u8(&self, offset: usize) -> Result<u8, DeviceIOError>;
    fn read_u16(&self, offset: usize) -> Result<u16, DeviceIOError>;
    fn read_u32(&self, offset: usize) -> Result<u32, DeviceIOError>;
    fn read_u64(&self, offset: usize) -> Result<u64, DeviceIOError>;

    fn write_u8(&self, offset: usize, value: u8) -> Result<(), DeviceIOError>;
    fn write_u16(&self, offset: usize, value: u16)
    -> Result<(), DeviceIOError>;
    fn write_u32(&self, offset: usize, value: u32)
    -> Result<(), DeviceIOError>;
    fn write_u64(&self, offset: usize, value: u64)
    -> Result<(), DeviceIOError>;

    fn base_addr(&self) -> BusAddrType;
    fn mem_space_size(&self) -> usize;
    fn end_addr(&self) -> BusAddrType {
        // non-inclusive end addr
        self.base_addr() + self.mem_space_size() as BusAddrType
    }
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
