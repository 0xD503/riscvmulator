use crate::constants::arch::RISCV_MAX_INSTRUCTION_SIZE;

// memory padding is needed to prevent panics on reading instructions near the
// RAM_END zone
pub const MEMORY_ARRAY_PADDING: usize = RISCV_MAX_INSTRUCTION_SIZE;
