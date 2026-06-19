mod constants;

use std::marker::PhantomData;
use std::mem::size_of;
// use std::sync::Mutex;
use std::sync::{Arc, RwLock};

use log::trace;

use crate::constants::arch::{
    Byte, DoubleWord, HalfWord, SignedByte, SignedDoubleWord, SignedHalfWord,
    SignedWord, SignedXLen, Word, XLen,
};
use crate::constants::soc::MEM_RESET_VALUE;

use super::peripherals::common::device::{Device, RawDevice};

use self::constants::MEMORY_ARRAY_PADDING;

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(Default, Clone)]
pub enum MemoryValueType {
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

pub trait MemoryValue: Sized {
    type Bytes: AsRef<[Byte]>;

    fn from_le_bytes(bytes: &[Byte]) -> Self;
    fn to_le_bytes(&self) -> Self::Bytes;

    fn sign_extend(&self) -> XLen;
    fn zero_extend(&self) -> XLen;

    fn to_u64(&self) -> u64;
}

impl MemoryValue for Byte {
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

impl MemoryValue for HalfWord {
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
impl MemoryValue for Word {
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

impl MemoryValue for DoubleWord {
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
//         Self::from_le_bytes(bytes.try_into().unwrap())
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

#[derive(Default)]
pub struct Memory<AddrType> {
    // TODO: not sure what is better: `RwLock` vs `Mutex`
    // because `RwLock` may cause writer starvation
    byte_array: Arc<RwLock<Vec<Byte>>>,

    _phantom_marker: PhantomData<AddrType>,
}

impl<AddrType> Memory<AddrType> {
    pub fn new(fill_value: Byte, size: usize) -> Self {
        Self {
            byte_array: Arc::new(RwLock::new(vec![
                fill_value;
                size + MEMORY_ARRAY_PADDING
            ])),
            _phantom_marker: Default::default(),
        }
    }

    fn fill(&mut self, value: Byte) {
        let byte_array_ptr = Arc::clone(&self.byte_array);
        let mut byte_array = byte_array_ptr.write().unwrap_or_else(|err| {
            todo!("Is it even possible???: {err}");
        });
        byte_array.fill(value);
    }

    pub fn reset(&mut self) {
        self.fill(MEM_RESET_VALUE);
    }
}

impl<AddrType> Device for Memory<AddrType> {
    fn read<MemValType: MemoryValue>(&self, offset: usize) -> MemValType {
        trace!("Reading memory at offset: {offset:#018X?}");

        let byte_array_ptr = Arc::clone(&self.byte_array);
        let byte_array = byte_array_ptr.read().unwrap_or_else(|err| {
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

    fn write<MemValType: MemoryValue>(
        &mut self,
        offset: usize,
        value: MemValType,
    ) {
        trace!("Writing memory at offset: {offset:#018X?}");

        let byte_array_ptr = Arc::clone(&self.byte_array);
        let mut byte_array = byte_array_ptr.write().unwrap_or_else(|err| {
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

impl<AddrType> RawDevice for Memory<AddrType> {
    fn read_raw<MemValType: MemoryValue>(&self, offset: usize) -> MemValType {
        let byte_array_ptr = Arc::clone(&self.byte_array);
        let byte_array = byte_array_ptr.read().unwrap_or_else(|err| {
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

    fn write_raw<MemValType: MemoryValue>(
        &mut self,
        offset: usize,
        value: MemValType,
    ) {
        let byte_array_ptr = Arc::clone(&self.byte_array);
        let mut byte_array = byte_array_ptr.write().unwrap_or_else(|err| {
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
