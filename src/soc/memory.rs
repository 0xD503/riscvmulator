mod constants;

use std::marker::PhantomData;
use std::mem::size_of;
use std::sync::RwLock;

use log::trace;

use crate::IODeviceError;
use crate::constants::arch::{
    Byte, DoubleWord, HalfWord, SignedByte, SignedDoubleWord, SignedHalfWord,
    SignedWord, SignedXLen, Word, XLen,
};
use crate::constants::soc::MEM_RESET_VALUE;

use super::peripherals::common::device::RawDevice;

use self::constants::MEMORY_ARRAY_PADDING;

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(Default, Clone)]
pub enum MemoryValueTypeEnum {
    #[default]
    BYTE,
    HALF_WORD,
    WORD,
    DOUBLE_WORD,
    // QUAD_WORD,
    UNSIGNED_BYTE,
    UNSIGNED_HALF_WORD,
    UNSIGNED_WORD,
    // UNSIGNED_DOUBLE_WORD,
}

pub trait MemoryValueType: Sized {
    type Bytes: AsRef<[Byte]>;

    fn from_le_bytes(bytes: &[Byte]) -> Self;
    fn to_le_bytes(&self) -> Self::Bytes;

    fn sign_extend(&self) -> XLen;
    fn zero_extend(&self) -> XLen;

    fn to_u64(&self) -> u64;
}

impl MemoryValueType for Byte {
    type Bytes = [Byte; size_of::<Self>()];

    fn from_le_bytes(bytes: &[Byte]) -> Self {
        Self::from_le_bytes(bytes.try_into().unwrap())
    }

    fn to_le_bytes(&self) -> Self::Bytes {
        Self::to_le_bytes(*self)
    }

    fn sign_extend(&self) -> XLen {
        *self as SignedByte as SignedXLen as XLen
    }

    fn zero_extend(&self) -> XLen {
        *self as XLen
    }

    fn to_u64(&self) -> u64 {
        *self as u64
    }
}

impl MemoryValueType for HalfWord {
    type Bytes = [Byte; size_of::<Self>()];

    fn from_le_bytes(bytes: &[Byte]) -> Self {
        Self::from_le_bytes(bytes.try_into().unwrap())
    }

    fn to_le_bytes(&self) -> Self::Bytes {
        Self::to_le_bytes(*self)
    }

    fn sign_extend(&self) -> XLen {
        *self as SignedHalfWord as SignedXLen as XLen
    }

    fn zero_extend(&self) -> XLen {
        *self as XLen
    }

    fn to_u64(&self) -> u64 {
        *self as u64
    }
}
impl MemoryValueType for Word {
    type Bytes = [Byte; size_of::<Self>()];

    fn from_le_bytes(bytes: &[Byte]) -> Self {
        Self::from_le_bytes(bytes.try_into().unwrap())
    }

    fn to_le_bytes(&self) -> Self::Bytes {
        Self::to_le_bytes(*self)
    }

    fn sign_extend(&self) -> XLen {
        *self as SignedWord as SignedXLen as XLen
    }

    fn zero_extend(&self) -> XLen {
        *self as XLen
    }

    fn to_u64(&self) -> u64 {
        *self as u64
    }
}

impl MemoryValueType for DoubleWord {
    type Bytes = [Byte; size_of::<Self>()];

    fn from_le_bytes(bytes: &[Byte]) -> Self {
        Self::from_le_bytes(bytes.try_into().unwrap())
    }

    fn to_le_bytes(&self) -> Self::Bytes {
        Self::to_le_bytes(*self)
    }

    fn sign_extend(&self) -> XLen {
        *self as SignedDoubleWord as SignedXLen as XLen
    }

    fn zero_extend(&self) -> XLen {
        *self as XLen
    }

    fn to_u64(&self) -> u64 {
        *self as u64
    }
}

// impl MemoryValue for QuadWord {
//     type Bytes = [Byte; size_of::<Self>()];

//     fn from_le_bytes(bytes: &[Byte]) -> Self {
//         Self::from_le_bytbyte_array_ptres(bytes.try_into().unwrap())
//     }

//     fn to_le_bytes(&self) -> Self::Bytes {
//         Self::to_le_bytes(*self)
//     }

//     // NOTICE: truncates value
//     fn sign_extend(&self) -> XLen {
//         *self as SignedQuadWord as SignedXLen as XLen
//     }

//     // NOTICE: truncates value
//     fn zero_extend(&self) -> XLen {
//         *self as XLen
//     }

//     fn to_u64(&self) -> u64 {
//         *self as u64
//     }
// }

pub trait MemoryDevice<AddrType>: Send + Sync {
    // we cannot make `read()`/`write()` methods to be generic because of the
    // `dyn` traits compatibility requirement. So we declare methods for each
    // type. Read more at:
    // https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility

    fn read_u8(&self, offset: usize) -> u8;
    fn read_u16(&self, offset: usize) -> u16;
    fn read_u32(&self, offset: usize) -> u32;
    fn read_u64(&self, offset: usize) -> u64;

    fn write_u8(&self, offset: usize, value: u8);
    fn write_u16(&self, offset: usize, value: u16);
    fn write_u32(&self, offset: usize, value: u32);
    fn write_u64(&self, offset: usize, value: u64);

    fn read(
        &self,
        offset: usize,
        dest: &mut [Byte],
    ) -> Result<(), IODeviceError>;
    fn write(&self, offset: usize, value: &[Byte])
    -> Result<(), IODeviceError>;
}

#[derive(Default)]
pub struct Memory<AddrType>
where
    AddrType: Send + Sync,
{
    // TODO: not sure what is better: `RwLock` vs `Mutex`
    // because `RwLock` may cause writer starvation
    byte_array: RwLock<Vec<Byte>>,

    _phantom_marker: PhantomData<AddrType>,
}

impl<AddrType: Send + Sync> Memory<AddrType> {
    pub fn new(fill_value: Byte, size: usize) -> Self {
        Self {
            byte_array: RwLock::new(vec![
                fill_value;
                size + MEMORY_ARRAY_PADDING
            ]),
            _phantom_marker: Default::default(),
        }
    }

    fn fill(&self, value: Byte) {
        let mut byte_array = self
            .byte_array
            .write()
            .expect("sibling hart panicked while holding lock to memory");
        byte_array.fill(value);
    }

    pub fn reset(&self) {
        self.fill(MEM_RESET_VALUE);
    }

    fn read_impl<MemValType: MemoryValueType>(
        &self,
        offset: usize,
    ) -> MemValType {
        trace!("Reading memory at offset: {offset:#018X?}");

        let byte_array = self.byte_array.read().unwrap_or_else(|err| {
            todo!("Is it even possible???: {err}");
        });

        // bus makes bounds check, so we assume that `offset` is always correct
        debug_assert!(offset + size_of::<MemValType>() <= byte_array.len());

        let bytes = byte_array[offset..offset + size_of::<MemValType>()]
            .try_into()
            .unwrap();
        // TODO: notice, so far emulator only supports little-endian
        MemValType::from_le_bytes(bytes)
    }

    fn write_impl<MemValType: MemoryValueType>(
        &self,
        offset: usize,
        value: MemValType,
    ) {
        trace!("Writing memory at offset: {offset:#018X?}");

        let mut byte_array = self.byte_array.write().unwrap_or_else(|err| {
            todo!("Is it even possible???: {err}");
        });

        // bus makes bounds check, so we assume that `offset` is always correct
        debug_assert!(offset + size_of::<MemValType>() <= byte_array.len());

        // TODO: notice, so far emulator only supports little-endian
        let bytes = value.to_le_bytes();
        byte_array[offset..offset + size_of::<MemValType>()]
            .copy_from_slice(bytes.as_ref());
    }
}

impl<AddrType: Send + Sync> MemoryDevice<AddrType> for Memory<AddrType> {
    fn read(
        &self,
        offset: usize,
        dest: &mut [Byte],
    ) -> Result<(), IODeviceError> {
        trace!("Reading memory at offset: {offset:#018X?}");

        let byte_array_guard_read =
            self.byte_array.read().unwrap_or_else(|err| {
                todo!("Is it even possible???: {err}");
            });

        // bus makes bounds check, so we assume that `offset` is always correct
        debug_assert!(offset + dest.len() <= byte_array_guard_read.len());

        let src = byte_array_guard_read[offset..offset + dest.len()]
            .try_into()
            .unwrap();
        // TODO: notice, so far emulator only supports little-endian
        dest.copy_from_slice(src);

        Ok(())
    }
    fn write(
        &self,
        offset: usize,
        value: &[Byte],
    ) -> Result<(), IODeviceError> {
        trace!("Writing memory at offset: {offset:#018X?}");

        let mut byte_array_guard_write =
            self.byte_array.write().unwrap_or_else(|err| {
                todo!("Is it even possible???: {err}");
            });

        // bus makes bounds check, so we assume that `offset` is always correct
        debug_assert!(offset + value.len() <= byte_array_guard_write.len());

        byte_array_guard_write[offset..offset + value.len()]
            .copy_from_slice(value);

        Ok(())
    }

    fn read_u8(&self, offset: usize) -> u8 {
        self.read_impl(offset)
    }
    fn read_u16(&self, offset: usize) -> u16 {
        self.read_impl(offset)
    }
    fn read_u32(&self, offset: usize) -> u32 {
        self.read_impl(offset)
    }
    fn read_u64(&self, offset: usize) -> u64 {
        self.read_impl(offset)
    }

    fn write_u8(&self, offset: usize, value: u8) {
        self.write_impl(offset, value);
    }
    fn write_u16(&self, offset: usize, value: u16) {
        self.write_impl(offset, value);
    }
    fn write_u32(&self, offset: usize, value: u32) {
        self.write_impl(offset, value);
    }
    fn write_u64(&self, offset: usize, value: u64) {
        self.write_impl(offset, value);
    }
}

impl<AddrType: Send + Sync> RawDevice for Memory<AddrType> {
    fn read_raw<MemValType: MemoryValueType>(
        &self,
        offset: usize,
    ) -> MemValType {
        let byte_array = self.byte_array.read().unwrap_or_else(|err| {
            todo!("Is it even possible???: {err}");
        });

        // bus makes bounds check, so we assume that `offset` is always correct
        debug_assert!(offset + size_of::<MemValType>() <= byte_array.len());

        let bytes = byte_array[offset..offset + size_of::<MemValType>()]
            .try_into()
            .unwrap();
        // TODO: notice, so far emulator only supports little-endian
        MemValType::from_le_bytes(bytes)
    }

    fn write_raw<MemValType: MemoryValueType>(
        &self,
        offset: usize,
        value: MemValType,
    ) {
        let mut byte_array = self.byte_array.write().unwrap_or_else(|err| {
            todo!("Is it even possible???: {err}");
        });

        // bus makes bounds check, so we assume that `offset` is always correct
        debug_assert!(offset + size_of::<MemValType>() <= byte_array.len());

        // TODO: notice, so far emulator only supports little-endian
        let bytes = value.to_le_bytes();
        byte_array[offset..offset + size_of::<MemValType>()]
            .copy_from_slice(bytes.as_ref());
    }
}
