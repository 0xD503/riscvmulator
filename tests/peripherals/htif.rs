use std::sync::RwLock;

use log::{error, trace};

use riscvmulator::{
    BusAddrType, Device, DeviceData, DeviceIOError, MemoryValue, XLen,
};

pub const HTIF_SUCCESS: u32 = 1;

pub struct HTIF {
    // memory_map: RwLock<Vec<u8>>,
    // base_addr: BusAddrType,
    data: DeviceData,
}

impl HTIF {
    pub fn new(
        base_addr: BusAddrType,
        size: usize,
        dev_type: String,
        name: String,
    ) -> Self {
        HTIF {
            data: DeviceData::new(base_addr, size, dev_type, name),
            //
            // memory_map: RwLock::new(vec![0; size]),
            // base_addr,
        }
    }

    // fn read_impl<MemoryValType: MemoryValue>(
    //     &self,
    //     offset: usize,
    // ) -> Result<MemoryValType, DeviceIOError> {
    //     trace!("Reading HTIF device at offset: {offset:#018X?}");

    //     let byte_array = self.data().memory_map.read().map_err(|err| {
    //         error!("Failed to read HTIF device: {err}");
    //         DeviceIOError::Read(offset as XLen)
    //     })?;

    //     // bus makes bounds check, so we assume that `offset` is always correct
    //     debug_assert!(offset + size_of::<MemoryValType>() <= byte_array.len());

    //     let bytes = byte_array[offset..offset + size_of::<MemoryValType>()]
    //         .try_into()
    //         .unwrap();
    //     // TODO: notice, so far emulator only supports little-endian
    //     Ok(MemoryValType::from_le_bytes(bytes))
    // }

    // fn write_impl<MemoryValType: MemoryValue>(
    //     &self,
    //     offset: usize,
    //     value: MemoryValType,
    // ) -> Result<(), DeviceIOError> {
    //     trace!("Writing HTIF device at offset: {offset:#018X?}");

    //     let mut byte_array = self.data().memory_map.write().map_err(|err| {
    //         // todo!("Is it even possible???: {err}");
    //         DeviceIOError::Write(offset as XLen)
    //     })?;

    //     // bus makes bounds check, so we assume that `offset` is always correct
    //     debug_assert!(offset + size_of::<MemoryValType>() <= byte_array.len());

    //     // TODO: notice, so far emulator only supports little-endian
    //     let bytes = value.to_le_bytes();
    //     byte_array[offset..offset + size_of::<MemoryValType>()]
    //         .copy_from_slice(bytes.as_ref());

    //     Ok(())
    // }
}

impl Device for HTIF {
    fn data(&self) -> &DeviceData {
        &self.data
    }

    // fn read_u8(&self, offset: usize) -> Result<u8, DeviceIOError> {
    //     // let mem_map = self.memory_map.read().unwrap_or_else(|err| {
    //     //     // Err(DeviceIOError::Read(offset as XLen))
    //     //     todo!("Is it even possible???: {err}")
    //     // });
    //     // Ok(mem_map[offset])
    //     self.data().read_impl(offset)
    // }
    // fn read_u16(&self, offset: usize) -> Result<u16, DeviceIOError> {
    //     self.data().read_impl(offset)
    // }
    // fn read_u32(&self, offset: usize) -> Result<u32, DeviceIOError> {
    //     self.data().read_impl(offset)
    // }
    // fn read_u64(&self, offset: usize) -> Result<u64, DeviceIOError> {
    //     self.data().read_impl(offset)
    // }

    // fn write_u8(&self, offset: usize, value: u8) -> Result<(), DeviceIOError> {
    //     self.data().write_impl(offset, value)
    // }
    // fn write_u16(
    //     &self,
    //     offset: usize,
    //     value: u16,
    // ) -> Result<(), DeviceIOError> {
    //     self.data().write_impl(offset, value)
    // }
    // fn write_u32(
    //     &self,
    //     offset: usize,
    //     value: u32,
    // ) -> Result<(), DeviceIOError> {
    //     self.data().write_impl(offset, value)
    // }
    // fn write_u64(
    //     &self,
    //     offset: usize,
    //     value: u64,
    // ) -> Result<(), DeviceIOError> {
    //     self.data().write_impl(offset, value)
    // }

    // fn base_addr(&self) -> BusAddrType {
    //     self.base_addr
    // }
    // fn mem_space_size(&self) -> usize {
    //     // TODO: check: unwrapping probably safe here
    //     self.memory_map.read().unwrap().len()
    // }
}
