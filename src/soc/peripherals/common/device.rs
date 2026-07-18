use std::{
    collections::BTreeMap,
    fmt::{self, Formatter},
    marker::PhantomData,
    sync::RwLock,
};

use log::{error, trace};

use crate::{
    constants::arch::{BusAddrType, XLen},
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

    fn attach_device(&mut self, device: Box<dyn Device>) {
        let mut dev_tree = self.tree.write().unwrap_or_else(|err| {
            todo!("Is it even possible???: {err}");
        });
        let base_addr = device.base_addr();

        if dev_tree.contains_key(&base_addr) {
            todo!("handle already existing device error");
        }
        // TODO: make some checks for interleaving/existing ranges
        // TODO

        // add device to devices list
        dev_tree.insert(base_addr, device);
    }

    fn detach_device(&mut self, device: Box<dyn Device>) {
        todo!("later");
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

#[derive(Debug)]
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

impl fmt::Display for DeviceIOError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Read(addr) => {
                writeln!(formatter, "failed to read device at addr: {addr}")
            }
            Self::Write(addr) => {
                writeln!(formatter, "failed to write device at addr: {addr}")
            }
        }
    }
}

pub struct DeviceData {
    pub memory_map: RwLock<Vec<u8>>,
    base_addr: BusAddrType,
    size: usize,
    dev_type: String,
    name: String,
}

impl DeviceData {
    pub fn new(
        base_addr: BusAddrType,
        size: usize,
        dev_type: String,
        name: String,
    ) -> Self {
        DeviceData {
            memory_map: RwLock::new(vec![0; size]),
            base_addr,
            size,
            dev_type,
            name,
        }
    }
}

impl DeviceData {
    pub fn read_impl<MemoryValType: MemoryValue>(
        &self,
        offset: usize,
    ) -> Result<MemoryValType, DeviceIOError> {
        trace!("Reading device at offset: {offset:#018X?}");

        let byte_array = self.memory_map.read().map_err(|err| {
            error!("Failed to read device: {err}");
            DeviceIOError::Read(offset as XLen)
        })?;

        // bus makes bounds check, so we assume that `offset` is always correct
        debug_assert!(offset + size_of::<MemoryValType>() <= byte_array.len());

        let bytes = byte_array[offset..offset + size_of::<MemoryValType>()]
            .try_into()
            .unwrap();
        // TODO: notice, so far emulator only supports little-endian
        Ok(MemoryValType::from_le_bytes(bytes))
    }

    pub fn write_impl<MemoryValType: MemoryValue>(
        &self,
        offset: usize,
        value: MemoryValType,
    ) -> Result<(), DeviceIOError> {
        trace!("Writing device at offset: {offset:#018X?}");

        let mut byte_array = self.memory_map.write().map_err(|err| {
            // todo!("Is it even possible???: {err}");
            DeviceIOError::Write(offset as XLen)
        })?;

        // bus makes bounds check, so we assume that `offset` is always correct
        debug_assert!(offset + size_of::<MemoryValType>() <= byte_array.len());

        // TODO: notice, so far emulator only supports little-endian
        let bytes = value.to_le_bytes();
        byte_array[offset..offset + size_of::<MemoryValType>()]
            .copy_from_slice(bytes.as_ref());

        Ok(())
    }
}

pub trait Device: Send + Sync {
    fn data(&self) -> &DeviceData;

    // fn read<MemValType: MemoryValue>(&self, offset: usize) -> MemValType;
    // fn write<MemValType: MemoryValue>(&self, offset: usize, value: MemValType);

    // we cannot make `read()`/`write()` methods to be generic because of the
    // `dyn` traits compatibility requirement. So we declare methods for each
    // type. Read more at:
    // https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility

    fn read_u8(&self, offset: usize) -> Result<u8, DeviceIOError> {
        self.data().read_impl(offset)
    }
    fn read_u16(&self, offset: usize) -> Result<u16, DeviceIOError> {
        self.data().read_impl(offset)
    }
    fn read_u32(&self, offset: usize) -> Result<u32, DeviceIOError> {
        self.data().read_impl(offset)
    }
    fn read_u64(&self, offset: usize) -> Result<u64, DeviceIOError> {
        self.data().read_impl(offset)
    }

    fn write_u8(&self, offset: usize, value: u8) -> Result<(), DeviceIOError> {
        self.data().write_impl(offset, value)
    }
    fn write_u16(
        &self,
        offset: usize,
        value: u16,
    ) -> Result<(), DeviceIOError> {
        self.data().write_impl(offset, value)
    }
    fn write_u32(
        &self,
        offset: usize,
        value: u32,
    ) -> Result<(), DeviceIOError> {
        self.data().write_impl(offset, value)
    }
    fn write_u64(
        &self,
        offset: usize,
        value: u64,
    ) -> Result<(), DeviceIOError> {
        self.data().write_impl(offset, value)
    }

    fn base_addr(&self) -> BusAddrType {
        self.data().base_addr
    }
    fn mem_space_size(&self) -> usize {
        self.data().size
    }
    fn end_addr(&self) -> BusAddrType {
        // non-inclusive end addr
        self.base_addr() + self.mem_space_size() as BusAddrType
    }

    fn dev_type(&self) -> &String {
        &self.data().dev_type
    }
    fn name(&self) -> &String {
        &self.data().name
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
