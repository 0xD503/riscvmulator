use super::super::executor::constants::ExecutableCommand;
use super::constants::{
    FenceMode, Funct7, I_Funct12, OP_IMM_Funct7, OP_IMM_ShiftType, Opcode,
};
use crate::constants::arch::{InstructionType, UnsignedImmediate, XLen};
use crate::soc::cpu::extensions::zicsr::ZicsrContext;

#[derive(Default, Clone)]
pub struct Decoder {
    pub(crate) opcode: Opcode,
    pub(crate) source_register_1: XLen,
    pub(crate) source_register_2: XLen,
    pub(crate) destination_register_offset: usize,

    pub(crate) funct3: InstructionType,
    pub(crate) i_funct7: OP_IMM_Funct7,
    pub(crate) i_funct12: I_Funct12,
    pub(crate) r_funct7: Funct7,

    pub(crate) shamt: InstructionType,
    pub(crate) shift_type: OP_IMM_ShiftType,

    pub(crate) immediate: UnsignedImmediate,
    pub(crate) command_to_execute: ExecutableCommand,
    pub(crate) need_to_read_memory: bool,
    pub(crate) need_to_write_memory: bool,
    pub(crate) need_to_write_back: bool,
    pub(crate) need_to_jump: bool,
    pub(crate) need_to_branch: bool,

    pub(crate) fence: FENCE_Context,

    pub(crate) zicsr: ZicsrContext,
}

#[allow(non_camel_case_types)]
#[derive(Default, Clone)]
pub struct FENCE_Context {
    pub(crate) successor_mem_write: bool,
    pub(crate) successor_mem_read: bool,
    pub(crate) successor_device_output: bool,
    pub(crate) successor_device_input: bool,

    pub(crate) predecessor_mem_write: bool,
    pub(crate) predecessor_mem_read: bool,
    pub(crate) predecessor_device_output: bool,
    pub(crate) predecessor_device_input: bool,

    pub(crate) mode: FenceMode,
}
