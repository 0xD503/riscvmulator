use log::trace;

use crate::{
    IODeviceError, soc::peripherals::common::constants::DEV_MEM_INIT_VAL,
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

    pub fn read_bytes(
        &self,
        offset: usize,
        dest: &mut [u8],
    ) -> Result<(), IODeviceError> {
        trace!("Reading device at offset: {offset:#018X?}");

        let byte_array = &self.data;

        // bus makes bounds check, so we assume that `offset` is always correct
        debug_assert!(offset + dest.len() <= byte_array.len());

        let src = &byte_array[offset..offset + dest.len()];
        dest.copy_from_slice(src);

        Ok(())
    }

    pub fn write_bytes(
        &mut self,
        offset: usize,
        value: &[u8],
    ) -> Result<(), IODeviceError> {
        trace!("Writing device at offset: {offset:#018X?}");

        let byte_array = &mut self.data;

        // bus makes bounds check, so we assume that `offset` is always correct
        debug_assert!(offset + value.len() <= byte_array.len());

        byte_array[offset..offset + value.len()].copy_from_slice(value);

        Ok(())
    }
}
