use super::arch::{Byte, INSTRUCTION_NOP, InstructionType};

pub const INITIAL_INSTRUCTION: InstructionType = INSTRUCTION_NOP;

pub const MEM_RESET_VALUE: Byte = 0x00;
// pub const RAM_SIZE: usize = 262144; // placeholder value, to be configured later
// pub const RAM_END: XLen = RAM_BASE + RAM_SIZE as XLen;
// const MEM_SPACE_SIZE: usize = XLen::MAX as usize;
