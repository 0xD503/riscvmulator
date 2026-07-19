use std::sync::RwLock;

use log::{error, trace};

use riscvmulator::{
    BusAddrType, Byte, Device, DeviceMeta, IODeviceError, MemoryValue, XLen,
};

pub const HTIF_SUCCESS: u32 = 1;

#[derive(Debug)]
pub struct HTIF {
    // memory_map: RwLock<Vec<u8>>,
    // base_addr: BusAddrType,
    data: DeviceMeta,
}

impl HTIF {
    // fn new(
    //     base_addr: BusAddrType,
    //     size: usize,
    //     kind: String,
    //     name: String,
    // ) -> Self {
    //     HTIF {
    //         data: DeviceMeta::new(base_addr, size, kind, name),
    //         //
    //         // memory_map: RwLock::new(vec![0; size]),
    //         // base_addr,
    //     }
    // }

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
    fn meta(&self) -> &DeviceMeta {
        &self.data
    }

    fn read_bytes(
        &self,
        offset: usize,
        dest: &mut [Byte],
    ) -> Result<(), IODeviceError> {
        // let guarded_state_read = self.guarded_state.read().map_err(|err| {
        //     error!("failed to read device state: {err}");
        //     IODeviceError::Read(offset as XLen)
        // })?;

        // let read_result = guarded_state_read.memory_map.read_impl(offset)?;
        // dest = MemoryValue::from_le_bytes(read_result);

        Ok(())
    }

    fn write_bytes(
        &self,
        offset: usize,
        src: &[Byte],
    ) -> Result<(), IODeviceError> {
        Ok(())
    }

    // // we cannot make `read()`/`write()` methods to be generic because of the
    // // `dyn` traits compatibility requirement. So we declare methods for each
    // // type. Read more at:
    // // https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility

    // fn read_u8(&self, offset: usize) -> Result<u8, IODeviceError> {
    //     self.meta().read_impl(offset)
    // }
    // fn read_u16(&self, offset: usize) -> Result<u16, IODeviceError> {
    //     self.meta().read_impl(offset)
    // }
    // fn read_u32(&self, offset: usize) -> Result<u32, IODeviceError> {
    //     self.meta().read_impl(offset)
    // }
    // fn read_u64(&self, offset: usize) -> Result<u64, IODeviceError> {
    //     self.meta().read_impl(offset)
    // }

    // fn write_u8(&self, offset: usize, value: u8) -> Result<(), IODeviceError> {
    //     self.meta().write_impl(offset, value)
    // }
    // fn write_u16(
    //     &self,
    //     offset: usize,
    //     value: u16,
    // ) -> Result<(), IODeviceError> {
    //     self.meta().write_impl(offset, value)
    // }
    // fn write_u32(
    //     &self,
    //     offset: usize,
    //     value: u32,
    // ) -> Result<(), IODeviceError> {
    //     self.meta().write_impl(offset, value)
    // }
    // fn write_u64(
    //     &self,
    //     offset: usize,
    //     value: u64,
    // ) -> Result<(), IODeviceError> {
    //     self.meta().write_impl(offset, value)
    // }
}
