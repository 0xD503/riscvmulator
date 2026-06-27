// use crate::constants::arch::{
//     RISCV_MAX_INSTRUCTION_SIZE, RISCV_MIN_INSTRUCTION_SIZE,
// };

// // memory padding is needed to prevent panics on reading instructions near the
// // RAM_END zone
// pub const MEMORY_ARRAY_PADDING: usize =
//     RISCV_MAX_INSTRUCTION_SIZE - RISCV_MIN_INSTRUCTION_SIZE;
pub const MEMORY_ARRAY_PADDING: usize = 0;

// it's better not to set shard size to values < 16
// it's also recommended to set shard size to be power of 2
pub const MEMORY_SHARD_SIZE: usize = 4096;
