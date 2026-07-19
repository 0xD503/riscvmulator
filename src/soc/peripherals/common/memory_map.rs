use log::{error, trace};

use crate::{
    IODeviceError, MemoryValue,
    soc::peripherals::common::constants::DEV_MEM_INIT_VAL,
};

#[derive(Debug)]
pub struct MemoryMap {
    data: Vec<u8>,
}

impl MemoryMap {
    pub fn new(size: usize) -> Self {
        MemoryMap {
            data: vec![DEV_MEM_INIT_VAL; size],
        }
    }

    pub fn read_impl<MemoryValType: MemoryValue>(
        &self,
        offset: usize,
    ) -> Result<MemoryValType, IODeviceError> {
        trace!("Reading device at offset: {offset:#018X?}");

        let byte_array = &self.data;

        // bus makes bounds check, so we assume that `offset` is always correct
        debug_assert!(offset + size_of::<MemoryValType>() <= byte_array.len());

        let bytes = byte_array[offset..offset + size_of::<MemoryValType>()]
            .try_into()
            .unwrap();
        // TODO: notice, so far emulator only supports little-endian
        Ok(MemoryValType::from_le_bytes(bytes))
    }

    pub fn write_impl<MemoryValType: MemoryValue>(
        &mut self,
        offset: usize,
        value: MemoryValType,
    ) -> Result<(), IODeviceError> {
        trace!("Writing device at offset: {offset:#018X?}");

        let byte_array = &mut self.data;

        // bus makes bounds check, so we assume that `offset` is always correct
        debug_assert!(offset + size_of::<MemoryValType>() <= byte_array.len());

        // TODO: notice, so far emulator only supports little-endian
        let bytes = value.to_le_bytes();
        byte_array[offset..offset + size_of::<MemoryValType>()]
            .copy_from_slice(bytes.as_ref());

        Ok(())
    }
}
