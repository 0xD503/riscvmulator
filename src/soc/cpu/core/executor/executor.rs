use crate::MemoryValueType;
use crate::constants::arch::{Byte, DoubleWord, HalfWord, Word, XLen};

#[derive(Default, Clone)]
pub struct ExecutionContext {
    pub(crate) result: XLen,
    pub(crate) branch_target_address: XLen,
    pub(crate) jump_target_address: XLen,
    pub(crate) return_target_address: XLen,
    pub(crate) memory_address: XLen,
    pub(crate) value_to_store_u8: Byte,
    pub(crate) value_to_store_u16: HalfWord,
    pub(crate) value_to_store_u32: Word,
    pub(crate) value_to_store_u64: DoubleWord,
    // value_to_store_u128: u128,
    pub(crate) memory_value_type: MemoryValueType,
}
