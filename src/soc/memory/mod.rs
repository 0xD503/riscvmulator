use std::marker::PhantomData;
use std::mem::size_of;

use log::trace;

use crate::constants::arch::{
    Byte, DoubleWord, HalfWord, SignedByte, SignedDoubleWord, SignedHalfWord,
    SignedWord, SignedXLen, Word, XLen,
};
use crate::constants::soc::{MEM_RESET_VALUE, RAM_BASE};

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(Default, Clone)]
pub enum MemoryValueType {
    BYTE,
    HALF_WORD,
    WORD,
    #[default]
    DOUBLE_WORD,
    // QUAD_WORD,
    UNSIGNED_BYTE,
    UNSIGNED_HALF_WORD,
    UNSIGNED_WORD,
    // UNSIGNED_DOUBLE_WORD,
}

pub trait MemoryValue: Sized {
    fn from_le_bytes(bytes: &[Byte]) -> Self;
    fn to_le_bytes(&self) -> Vec<Byte>;

    fn sign_extend(&self) -> XLen;
    fn zero_extend(&self) -> XLen;
}

impl MemoryValue for Byte {
    fn from_le_bytes(bytes: &[Byte]) -> Self {
        Self::from_le_bytes(bytes.try_into().unwrap())
    }

    fn to_le_bytes(&self) -> Vec<Byte> {
        Self::to_le_bytes(*self).to_vec()
    }

    fn sign_extend(&self) -> XLen {
        *self as SignedByte as SignedXLen as XLen
    }

    fn zero_extend(&self) -> XLen {
        *self as XLen
    }
}

impl MemoryValue for HalfWord {
    fn from_le_bytes(bytes: &[Byte]) -> Self {
        Self::from_le_bytes(bytes.try_into().unwrap())
    }

    fn to_le_bytes(&self) -> Vec<Byte> {
        Self::to_le_bytes(*self).to_vec()
    }

    fn sign_extend(&self) -> XLen {
        *self as SignedHalfWord as SignedXLen as XLen
    }

    fn zero_extend(&self) -> XLen {
        *self as XLen
    }
}
impl MemoryValue for Word {
    fn from_le_bytes(bytes: &[Byte]) -> Self {
        Self::from_le_bytes(bytes.try_into().unwrap())
    }

    fn to_le_bytes(&self) -> Vec<Byte> {
        Self::to_le_bytes(*self).to_vec()
    }

    fn sign_extend(&self) -> XLen {
        *self as SignedWord as SignedXLen as XLen
    }

    fn zero_extend(&self) -> XLen {
        *self as XLen
    }
}

impl MemoryValue for DoubleWord {
    fn from_le_bytes(bytes: &[Byte]) -> Self {
        Self::from_le_bytes(bytes.try_into().unwrap())
    }

    fn to_le_bytes(&self) -> Vec<Byte> {
        Self::to_le_bytes(*self).to_vec()
    }

    fn sign_extend(&self) -> XLen {
        *self as SignedDoubleWord as SignedXLen as XLen
    }

    fn zero_extend(&self) -> XLen {
        *self as XLen
    }
}

// impl MemoryValue for QuadWord {
//     fn from_le_bytes(bytes: &[Byte]) -> Self {
//         Self::from_le_bytes(bytes.try_into().unwrap())
//     }

//     fn to_le_bytes(&self) -> Vec<Byte> {
//         Self::to_le_bytes(*self).to_vec()
//     }

//     // NOTICE: truncates value
//     fn sign_extend(&self) -> XLen {
//         *self as SignedQuadWord as SignedXLen as XLen
//     }

//     // NOTICE: truncates value
//     fn zero_extend(&self) -> XLen {
//         *self as XLen
//     }
// }

pub struct Memory<AddrType> {
    byte_array: Vec<Byte>,

    _phantom_marker: PhantomData<AddrType>,
}

impl<AddrType> Memory<AddrType> {
    pub fn new(fill_value: Byte, size: usize) -> Self {
        Self {
            byte_array: vec![fill_value; size],
            _phantom_marker: Default::default(),
        }
    }

    pub fn read<MemValType: MemoryValue>(&self, address: XLen) -> MemValType {
        trace!("Reading memory at addr: {address:#018X?}");

        let address = address - RAM_BASE;
        let bytes = (self.byte_array)
            [address as usize..address as usize + size_of::<MemValType>()]
            .try_into()
            .unwrap();
        // TODO: notice, so far emulator only supports little-endian
        MemValType::from_le_bytes(bytes)
    }

    pub fn write<MemValType: MemoryValue>(
        &mut self,
        address: XLen,
        value: MemValType,
    ) {
        trace!("Writing memory at addr: {address:#018X?}");

        let address = address - RAM_BASE;
        // TODO: notice, so far emulator only supports little-endian
        let bytes = value.to_le_bytes();
        self.byte_array
            [address as usize..address as usize + size_of::<MemValType>()]
            .copy_from_slice(&bytes);
    }

    fn fill(&mut self, value: Byte) {
        self.byte_array.fill(value);
    }

    pub fn reset(&mut self) {
        self.fill(MEM_RESET_VALUE);
    }
}
