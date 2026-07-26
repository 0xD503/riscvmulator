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
}

impl<AddrType> Addressable<BusAddrType> for Devices<AddrType> {
    fn read(
        &self,
        addr: BusAddrType, // TODO: fix to usize????
        dest: &mut [Byte],
    ) -> Result<(), IODeviceError> {
        let result;

        let device_tree = self.tree.read().unwrap_or_else(|err| {
            todo!("Is it even possible???: {err}");
        });
        let closest_dev = device_tree.range(..=addr).next_back();

        if let Some(device_entry) = closest_dev {
            let device = device_entry.1;
            let device_addr_range = device.base_addr()
                ..=(device.end_addr() - dest.len() as BusAddrType);
            if device_addr_range.contains(&addr) {
                let offset = addr - device.base_addr();
                result = device.read(offset, dest);
            } else {
                // Load Access Fault
                result = Err(IODeviceError::LoadAccessFault(addr));
            }
        } else {
            // Load Access Fault
            result = Err(IODeviceError::LoadAccessFault(addr));
        }

        result
    }

    fn write(&self, addr: XLen, value: &[Byte]) -> Result<(), IODeviceError> {
        let result;

        let device_tree = self.tree.read().unwrap_or_else(|err| {
            todo!("Is it even possible???: {err}");
        });
        let closest_dev = device_tree.range(..=addr).next_back();

        if let Some(device_entry) = closest_dev {
            let device = device_entry.1;
            let device_addr_range =
                device.base_addr()..=(device.end_addr() - value.len() as XLen);
            if device_addr_range.contains(&addr) {
                let offset = addr - device.base_addr();
                result = device.write(offset, value);
            } else {
                result = Err(IODeviceError::StoreAMOAccessFault(addr))
            }
        } else {
            result = Err(IODeviceError::StoreAMOAccessFault(addr))
        }

        result
    }
}

#[derive(Debug)]
pub enum IODeviceError {
    InstructionAddrMisaligned(BusAddrType),
    InstructionAccessFault(BusAddrType),
    Read(BusAddrType),
    Write(BusAddrType),
    ReadPoisoned(BusAddrType),
    WritePoisoned(BusAddrType),
    LoadAccessFault(BusAddrType),
    StoreAMOAccessFault(BusAddrType),
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
            Self::InstructionAddrMisaligned(addr) => {
                writeln!(formatter, "instruction address misaligned: {addr}")
            }
            Self::InstructionAccessFault(addr) => {
                writeln!(formatter, "instruction access fault: {addr}")
            }
            Self::Read(addr) => {
                writeln!(formatter, "failed to read device at addr: {addr}")
            }
            Self::Write(addr) => {
                writeln!(formatter, "failed to write device at addr: {addr}")
            }
            Self::ReadPoisoned(addr) => {
                writeln!(
                    formatter,
                    "failed to read (poisoned) device at addr: {addr}"
                )
            }
            Self::WritePoisoned(addr) => {
                writeln!(
                    formatter,
                    "failed to write (poisoned) device at addr: {addr}"
                )
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

pub trait Addressable<AddrType> {
    fn read(
        &self,
        at: AddrType,
        dest: &mut [Byte],
    ) -> Result<(), IODeviceError>;

    fn write(&self, at: AddrType, value: &[Byte]) -> Result<(), IODeviceError>;
}

pub trait Device: Addressable<BusAddrType> + Send + Sync + Debug {
    fn meta(&self) -> &DeviceMeta;

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
