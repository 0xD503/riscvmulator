use std::{
    collections::BTreeMap,
    fmt::{self, Debug, Formatter},
    marker::PhantomData,
    sync::RwLock,
};

use log::{error, trace};

use crate::{
    constants::arch::{BusAddrType, Byte, XLen},
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

    pub fn attach_device(&mut self, device: Box<dyn Device>) {
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

    // fn find_closest_device(
    //     &self,
    //     addr: XLen,
    // ) -> Option<(&XLen, &Box<dyn Device>)> {
    //     let device_tree_read_guard = self.tree.read().unwrap_or_else(|err| {
    //         todo!("Is it even possible???: {err}");
    //     });

    //     device_tree_read_guard.range(..=addr).next_back();
    // }

    pub fn read_bytes(
        &self,
        addr: XLen,
        dest: &mut [Byte],
    ) -> Result<(), IODeviceError> {
        let result;

        let device_tree_read_guard = self.tree.read().unwrap_or_else(|err| {
            todo!("Is it even possible???: {err}");
        });
        let closest_dev = device_tree_read_guard.range(..=addr).next_back();

        if let Some(device_entry) = closest_dev {
            let device = device_entry.1;
            let device_addr_range =
                device.base_addr()..=(device.end_addr() - dest.len() as XLen);
            if device_addr_range.contains(&addr) {
                let offset = (addr - device.base_addr()) as usize;
                result = device.read_bytes(offset, dest);
            } else {
                result = Err(IODeviceError::Read(addr))
            }
        } else {
            result = Err(IODeviceError::Read(addr))
        }

        result
    }

    pub fn write(
        &self,
        addr: XLen,
        value: &[Byte],
    ) -> Result<(), IODeviceError> {
        let result;

        let device_tree_read_guard = self.tree.read().unwrap_or_else(|err| {
            todo!("Is it even possible???: {err}");
        });
        let closest_dev = device_tree_read_guard.range(..=addr).next_back();

        if let Some(device_entry) = closest_dev {
            let device = device_entry.1;
            let device_addr_range =
                device.base_addr()..=(device.end_addr() - value.len() as XLen);
            if device_addr_range.contains(&addr) {
                let offset = (addr - device.base_addr()) as usize;
                result = device.write_bytes(offset, value);
            } else {
                result = Err(IODeviceError::Write(addr))
            }
        } else {
            result = Err(IODeviceError::Write(addr))
        }

        result
    }

    pub fn read_u8(&self, addr: XLen) -> Result<u8, IODeviceError> {
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
                result = Err(IODeviceError::Read(addr))
            }
        } else {
            result = Err(IODeviceError::Read(addr))
        }

        result
    }
    pub fn read_u16(&self, addr: XLen) -> Result<u16, IODeviceError> {
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
                result = Err(IODeviceError::Read(addr))
            }
        } else {
            result = Err(IODeviceError::Read(addr))
        }

        result
    }
    pub fn read_u32(&self, addr: XLen) -> Result<u32, IODeviceError> {
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
                result = Err(IODeviceError::Read(addr))
            }
        } else {
            result = Err(IODeviceError::Read(addr))
        }

        result
    }
    pub fn read_u64(&self, addr: XLen) -> Result<u64, IODeviceError> {
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
                result = Err(IODeviceError::Read(addr))
            }
        } else {
            result = Err(IODeviceError::Read(addr))
        }

        result
    }

    pub fn write_u8(&self, addr: XLen, value: u8) -> Result<(), IODeviceError> {
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
                result = Err(IODeviceError::Write(addr))
            }
        } else {
            result = Err(IODeviceError::Write(addr))
        }

        result
    }

    pub fn write_u16(
        &self,
        addr: XLen,
        value: u16,
    ) -> Result<(), IODeviceError> {
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
                result = Err(IODeviceError::Write(addr))
            }
        } else {
            result = Err(IODeviceError::Write(addr))
        }

        result
    }

    pub fn write_u32(
        &self,
        addr: XLen,
        value: u32,
    ) -> Result<(), IODeviceError> {
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
                result = Err(IODeviceError::Write(addr))
            }
        } else {
            result = Err(IODeviceError::Write(addr))
        }

        result
    }

    pub fn write_u64(
        &self,
        addr: XLen,
        value: u64,
    ) -> Result<(), IODeviceError> {
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
                result = Err(IODeviceError::Write(addr))
            }
        } else {
            result = Err(IODeviceError::Write(addr))
        }

        result
    }
}

#[derive(Debug)]
pub enum IODeviceError {
    Read(XLen),
    Write(XLen),
    Create(String),
}

impl IODeviceError {
    // pub fn exit_code(&self) -> i32 {
    //     match self {
    //         Self::Read(_) => ExitCode::DeviceReadError as i32,
    //     }
    // }
}

impl fmt::Display for IODeviceError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Read(addr) => {
                writeln!(formatter, "failed to read device at addr: {addr}")
            }
            Self::Write(addr) => {
                writeln!(formatter, "failed to write device at addr: {addr}")
            }
            Self::Create(kind) => {
                writeln!(formatter, "failed to create device of type: {kind}")
            }
        }
    }
}

#[derive(Debug)]
pub struct DeviceMeta {
    base_addr: BusAddrType,
    size: usize,
    kind: String,
    name: String,
}

impl DeviceMeta {
    pub fn new(
        base_addr: BusAddrType,
        size: usize,
        kind: String,
        name: String,
    ) -> Self {
        DeviceMeta {
            base_addr,
            size,
            kind,
            name,
        }
    }
}

pub trait Device: Send + Sync + Debug {
    fn meta(&self) -> &DeviceMeta;

    fn read_bytes(
        &self,
        offset: usize,
        dest: &mut [Byte],
    ) -> Result<(), IODeviceError>;

    fn write_bytes(
        &self,
        offset: usize,
        value: &[Byte],
    ) -> Result<(), IODeviceError>;

    // we cannot make `read()`/`write()` methods to be generic because of the
    // `dyn` traits compatibility requirement. So we declare methods for each
    // type. Read more at:
    // https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility

    fn read_u8(&self, offset: usize) -> Result<u8, IODeviceError> {
        let mut dest = [0u8; size_of::<u8>()];
        self.read_bytes(offset, &mut dest)?;

        Ok(u8::from_le_bytes(dest))
    }
    fn read_u16(&self, offset: usize) -> Result<u16, IODeviceError> {
        let mut dest = [0u8; size_of::<u16>()];
        self.read_bytes(offset, &mut dest)?;

        Ok(u16::from_le_bytes(dest))
    }
    fn read_u32(&self, offset: usize) -> Result<u32, IODeviceError> {
        let mut dest = [0u8; size_of::<u32>()];
        self.read_bytes(offset, &mut dest)?;

        Ok(u32::from_le_bytes(dest))
    }
    fn read_u64(&self, offset: usize) -> Result<u64, IODeviceError> {
        let mut dest = [0u8; size_of::<u64>()];
        self.read_bytes(offset, &mut dest)?;

        Ok(u64::from_le_bytes(dest))
    }

    fn write_u8(&self, offset: usize, value: u8) -> Result<(), IODeviceError> {
        let src = u8::to_le_bytes(value);

        self.write_bytes(offset, &src)
    }
    fn write_u16(
        &self,
        offset: usize,
        value: u16,
    ) -> Result<(), IODeviceError> {
        let src = u16::to_le_bytes(value);

        self.write_bytes(offset, &src)
    }
    fn write_u32(
        &self,
        offset: usize,
        value: u32,
    ) -> Result<(), IODeviceError> {
        let src = u32::to_le_bytes(value);

        self.write_bytes(offset, &src)
    }
    fn write_u64(
        &self,
        offset: usize,
        value: u64,
    ) -> Result<(), IODeviceError> {
        let src = u64::to_le_bytes(value);

        self.write_bytes(offset, &src)
    }

    fn base_addr(&self) -> BusAddrType {
        self.meta().base_addr
    }
    fn mem_space_size(&self) -> usize {
        self.meta().size
    }
    fn end_addr(&self) -> BusAddrType {
        // non-inclusive end addr
        self.base_addr() + self.mem_space_size() as BusAddrType
    }

    fn kind(&self) -> &String {
        &self.meta().kind
    }
    fn name(&self) -> &String {
        &self.meta().name
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

impl fmt::Display for dyn Device {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.name())
    }
}
