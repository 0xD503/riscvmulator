// use std::sync::RwLock;

// use log::{error, trace};

// use riscvmulator::{
//     BusAddrType, Byte, Device, DeviceMeta, IODeviceError, MemoryValueType, XLen,
// };

pub const HTIF_SUCCESS: u32 = 1;

// #[derive(Debug)]
// pub struct HTIF {
//     // memory_map: RwLock<Vec<u8>>,
//     // base_addr: BusAddrType,
//     data: DeviceMeta,
// }

// impl HTIF {
//     // fn new(
//     //     base_addr: BusAddrType,
//     //     size: usize,
//     //     kind: String,
//     //     name: String,
//     // ) -> Self {
//     //     HTIF {
//     //         data: DeviceMeta::new(base_addr, size, kind, name),
//     //         //
//     //       use std::sync::RwLock;

// use log::{error, trace};

// use riscvmulator::{
//     BusAddrType, Byte, Device, DeviceMeta, IODeviceError, MemoryValueType, XLen,
// };

// pub const HTIF_SUCCESS: u32 = 1;

// #[derive(Debug)]
// pub struct HTIF {
//     // memory_map: RwLock<Vec<u8>>,
//     // base_addr: BusAddrType,
//     data: DeviceMeta,
// }

// impl HTIF {
//     // fn new(
//     //     base_addr: BusAddrType,
//     //     size: usize,
//     //     kind: String,
//     //     name: String,
//     // ) -> Self {
//     //     HTIF {
//     //         data: DeviceMeta::new(base_addr, size, kind, name),
//     //         //
//     //         // memory_map: RwLock::new(vec![0; size]),
//     //         // base_addr,
//     //     }
//     // }

//     // fn read_impl<MemValType: MemoryValue>(
//     //     &self,
//     //     offset: usize,
//     // ) -> Result<MemValType, DeviceIOError> {
//     //     trace!("Reading HTIF device at offset: {offset:#018X?}");

//     //     let byte_array = self.data().memory_map.read().map_err(|err| {
//     //         error!("Failed to read HTIF device: {err}");
//     //         DeviceIOError::Read(offset as XLen)
//     //     })?;

//     //     // bus makes bounds check, so we assume that `offset` is always correct
//     //     debug_assert!(offset + size_of::<MemValType>() <= byte_array.len());

//     //     let bytes = byte_array[offset..offset + size_of::<MemValType>()]
//     //         .try_into()
//     //         .unwrap();
//     //     // TODO: notice, so far emulator only supports little-endian
//     //     Ok(MemValType::from_le_bytes(bytes))
//     // }

//     // fn write_impl<MemValType: MemoryValue>(
//     //     &self,
//     //     offset: usize,
//     //     value: MemValType,
//     // ) -> Result<(), DeviceIOError> {
//     //     trace!("Writing HTIF device at offset: {offset:#018X?}");

//     //     let mut byte_array = self.data().memory_map.write().map_err(|err| {
//     //         // todo!("Is it even possible???: {err}");
//     //         DeviceIOError::Write(offset as XLen)
//     //     })?;

//     //     // bus makes bounds check, so we assume that `offset` is always correct
//     //     debug_assert!(offset + size_of::<MemValType>() <= byte_array.len());

//     //     // TODO: notice, so far emulator only supports little-endian
//     //     let bytes = value.to_le_bytes();
//     //     byte_array[offset..offset + size_of::<MemValType>()]
//     //         .copy_from_slice(bytes.as_ref());

//     //     Ok(())
//     // }
// }

// impl Device for HTIF {
//     fn meta(&self) -> &DeviceMeta {
//         &self.data
//     }

//     fn read(
//         &self,
//         offset: usize,
//         dest: &mut [Byte],
//     ) -> Result<(), IODeviceError> {
//         // let guarded_state_read = self.guarded_state.read().map_err(|err| {
//         //     error!("failed to read device state: {err}");
//         //     IODeviceError::Read(offset as XLen)
//         // })?;

//         // let read_result = guarded_state_read.memory_map.read_impl(offset)?;
//         // dest = MemoryValue::from_le_bytes(read_result);

//         Ok(())
//     }

//     fn write(&self, offset: usize, src: &[Byte]) -> Result<(), IODeviceError> {
//         Ok(())
//     }
// }
// memory_map: RwLock::new(vec![0; size]),
//     //         // base_addr,
//     //     }
//     // }

//     // fn read_impl<MemValType: MemoryValue>(
//     //     &self,
//     //     offset: usize,
//     // ) -> Result<MemValType, DeviceIOError> {
//     //     trace!("Reading HTIF device at offset: {offset:#018X?}");

//     //     let byte_array = self.data().memory_map.read().map_err(|err| {
//     //         error!("Failed to read HTIF device: {err}");
//     //         DeviceIOError::Read(offset as XLen)
//     //     })?;

//     //     // bus makes bounds check, so we assume that `offset` is always correct
//     //     debug_assert!(offset + size_of::<MemValType>() <= byte_array.len());

//     //     let bytes = byte_array[offset..offset + size_of::<MemValType>()]
//     //         .try_into()
//     //         .unwrap();
//     //     // TODO: notice, so far emulator only supports little-endian
//     //     Ok(MemValType::from_le_bytes(bytes))
//     // }

//     // fn write_impl<MemValType: MemoryValue>(
//     //     &self,
//     //     offset: usize,
//     //     value: MemValType,
//     // ) -> Result<(), DeviceIOError> {
//     //     trace!("Writing HTIF device at offset: {offset:#018X?}");

//     //     let mut byte_array = self.data().memory_map.write().map_err(|err| {
//     //         // todo!("Is it even possible???: {err}");
//     //         DeviceIOError::Write(offset as XLen)
//     //     })?;

//     //     // bus makes bounds check, so we assume that `offset` is always correct
//     //     debug_assert!(offset + size_of::<MemValType>() <= byte_array.len());

//     //     // TODO: notice, so far emulator only supports little-endian
//     //     let bytes = value.to_le_bytes();
//     //     byte_array[offset..offset + size_of::<MemValType>()]
//     //         .copy_from_slice(bytes.as_ref());

//     //     Ok(())
//     // }
// }

// impl Device for HTIF {
//     fn meta(&self) -> &DeviceMeta {
//         &self.data
//     }

//     fn read(
//         &self,
//         offset: usize,
//         dest: &mut [Byte],
//     ) -> Result<(), IODeviceError> {
//         // let guarded_state_read = self.guarded_state.read().map_err(|err| {
//         //     error!("failed to read device state: {err}");
//         //     IODeviceError::Read(offset as XLen)
//         // })?;

//         // let read_result = guarded_state_read.memory_map.read_impl(offset)?;
//         // dest = MemoryValue::from_le_bytes(read_result);

//         Ok(())
//     }

//     fn write(&self, offset: usize, src: &[Byte]) -> Result<(), IODeviceError> {
//         Ok(())
//     }
// }
