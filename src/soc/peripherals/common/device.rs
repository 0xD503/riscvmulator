use std::{
    collections::BTreeMap,
    fmt::{self, Debug, Formatter},
    marker::PhantomData,
    sync::RwLock,
};

use log::{error, trace};

use crate::{
    constants::arch::{BusAddrType, Byte, XLen},
    soc::memory::MemoryValueType,
};

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

    pub fn attach(&self, device: Box<dyn Device>) -> Result<(), IODeviceError> {
        let mut device_tree_guarded_write =
            self.tree.write().map_err(|err| {
                error!("failed to attach device: {err}");
                IODeviceError::Poisoned
            })?;

        // device validation
        if (device.end_addr() - size_of::<XLen>() as BusAddrType)
            < device.base_addr()
        {
            let dev_name = device.name();
            error!(
                "{dev_name} device memory space size should be >= size of XLen"
            );
        }
        let device_base_addr = device.base_addr();
        let device_error_attach =
            IODeviceError::Attach(device.kind().to_string());
        if device_tree_guarded_write.contains_key(&device_base_addr) {
            error!(
                "device with base address {device_base_addr} already exists"
            );
            return Err(device_error_attach);
        }
        // TODO: make some checks for interleaving/existing ranges
        // TODO

        // add device to devices list
        let device_was_created = device_tree_guarded_write
            .insert(device_base_addr, device)
            .is_none();
        if !device_was_created {
            error!(
                "failed to attach device with base address {device_base_addr}"
            );
            return Err(device_error_attach);
        }

        Ok(())
    }

    fn detach(
        &self,
        device: Box<dyn Device>,
    ) -> Result<Option<Box<dyn Device>>, IODeviceError> {
        let mut device_tree_guarded_write =
            self.tree.write().map_err(|err| {
                error!("failed to detach device: {err}");
                IODeviceError::Poisoned
            })?;
        let device_base_addr = device.base_addr();
        // let device_error_detach = IODeviceError::Detach(device.kind().to_string());

        Ok(device_tree_guarded_write.remove(&device_base_addr))
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

    // TODO: remake read/write methods to generics

    // pub fn read_bytes(
    //     &self,
    //     addr: XLen,
    //     dest: &mut [Byte],
    // ) -> Result<(), IODeviceError> {
    //     let result;

    //     let device_tree_read_guard = self.tree.read().unwrap_or_else(|err| {
    //         todo!("Is it even possible???: {err}");
    //     });
    //     let closest_dev = device_tree_read_guard.range(..=addr).next_back();

    //     if let Some(device_entry) = closest_dev {
    //         let device = device_entry.1;
    //         let device_addr_range =
    //             device.base_addr()..=(device.end_addr() - dest.len() as XLen);
    //         if device_addr_range.contains(&addr) {
    //             let offset = (addr - device.base_addr()) as usize;
    //             result = device.read_bytes(offset, dest);
    //         } else {
    //             result = Err(IODeviceError::Read(addr))
    //         }
    //     } else {
    //         result = Err(IODeviceError::Read(addr))
    //     }

    //     result
    // }

    // pub fn write(
    //     &self,
    //     addr: XLen,
    //     value: &[Byte],
    // ) -> Result<(), IODeviceError> {
    //     let result;

    //     let device_tree_read_guard = self.tree.read().unwrap_or_else(|err| {
    //         todo!("Is it even possible???: {err}");
    //     });
    //     let closest_dev = device_tree_read_guard.range(..=addr).next_back();

    //     if let Some(device_entry) = closest_dev {
    //         let device = device_entry.1;
    //         let device_addr_range =
    //             device.base_addr()..=(device.end_addr() - value.len() as XLen);
    //         if device_addr_range.contains(&addr) {
    //             let offset = (addr - device.base_addr()) as usize;
    //             result = device.write_bytes(offset, value);
    //         } else {
    //             result = Err(IODeviceError::Write(addr))
    //         }
    //     } else {
    //         result = Err(IODeviceError::Write(addr))
    //     }

    //     result
    // }

    fn read(
        &self,
        addr: XLen, // TODO: fix to usize????
        dest: &mut [Byte],
    ) -> Result<(), IODeviceError> {
        let result;
        // let mut call_trap = false;

        let device_tree = self.tree.read().unwrap_or_else(|err| {
            todo!("Is it even possible???: {err}");
        });
        let closest_dev = device_tree.range(..=addr).next_back();

        if let Some(device_entry) = closest_dev {
            let device = device_entry.1;
            let device_addr_range = device.base_addr()
                ..=(device.end_addr() - dest.len() as BusAddrType);
            if device_addr_range.contains(&addr) {
                let offset = (addr - device.base_addr()) as usize;

                result = device.read(offset, dest);
            } else {
                // Load Access Fault
                result = Err(IODeviceError::LoadAccessFault(addr));
                // call_trap = true;
            }
        } else {
            // Load Access Fault
            result = Err(IODeviceError::LoadAccessFault(addr));
            // call_trap = true;
        }

        // if call_trap {
        //     todo!();
        // }

        result
    }

    pub fn write(&self, addr: XLen, value: u8) -> Result<(), IODeviceError> {
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
    LoadAccessFault(XLen),
    StoreAMOAccessFault(XLen),
    Attach(String),
    Detach(String),
    Poisoned,
    LazyError,
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
            Self::LoadAccessFault(addr) => {
                writeln!(formatter, "load access fault: {addr}")
            }
            Self::StoreAMOAccessFault(addr) => {
                writeln!(formatter, "store/AMO access fault: {addr}")
            }
            Self::Attach(kind) => {
                writeln!(formatter, "failed to attach device of kind: {kind}")
            }
            Self::Detach(kind) => {
                writeln!(formatter, "failed to detach device of kind: {kind}")
            }

            Self::Poisoned => {
                writeln!(
                    formatter,
                    "failed to access device tree because of poisoning"
                )
            }

            Self::LazyError => {
                todo!(
                    "I was too lazy to implement other type, change it to one of the above"
                )
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

    fn read(
        &self,
        offset: usize,
        dest: &mut [Byte],
    ) -> Result<(), IODeviceError>;

    fn write(&self, offset: usize, value: &[Byte])
    -> Result<(), IODeviceError>;

    // we cannot make `read()`/`write()` methods to be generic because of the
    // `dyn` traits compatibility requirement. So we declare methods for each
    // type. Read more at:
    // https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility

    fn read_u8(&self, offset: usize) -> Result<u8, IODeviceError> {
        let mut dest = [0u8; size_of::<u8>()];
        self.read(offset, &mut dest)?;

        Ok(u8::from_le_bytes(dest))
    }
    fn read_u16(&self, offset: usize) -> Result<u16, IODeviceError> {
        let mut dest = [0u8; size_of::<u16>()];
        self.read(offset, &mut dest)?;

        Ok(u16::from_le_bytes(dest))
    }
    fn read_u32(&self, offset: usize) -> Result<u32, IODeviceError> {
        let mut dest = [0u8; size_of::<u32>()];
        self.read(offset, &mut dest)?;

        Ok(u32::from_le_bytes(dest))
    }
    fn read_u64(&self, offset: usize) -> Result<u64, IODeviceError> {
        let mut dest = [0u8; size_of::<u64>()];
        self.read(offset, &mut dest)?;

        Ok(u64::from_le_bytes(dest))
    }

    fn write_u8(&self, offset: usize, value: u8) -> Result<(), IODeviceError> {
        let src = u8::to_le_bytes(value);

        self.write(offset, &src)
    }
    fn write_u16(
        &self,
        offset: usize,
        value: u16,
    ) -> Result<(), IODeviceError> {
        let src = u16::to_le_bytes(value);

        self.write(offset, &src)
    }
    fn write_u32(
        &self,
        offset: usize,
        value: u32,
    ) -> Result<(), IODeviceError> {
        let src = u32::to_le_bytes(value);

        self.write(offset, &src)
    }
    fn write_u64(
        &self,
        offset: usize,
        value: u64,
    ) -> Result<(), IODeviceError> {
        let src = u64::to_le_bytes(value);

        self.write(offset, &src)
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
    fn read_raw<MemValType: MemoryValueType>(
        &self,
        offset: usize,
    ) -> MemValType;
    fn write_raw<MemValType: MemoryValueType>(
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
