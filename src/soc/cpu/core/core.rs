use super::super::extensions::zicsr::CSRFile;
use super::decoder::decoder::Decoder;
use super::executor::executor::ExecutionContext;
use super::gp_registers::{Register, RegisterFile};
use crate::constants::arch::{
    InstructionType, RISCV_NUM_OF_CSR, RISCV_NUM_OF_REGISTERS,
};

#[allow(non_snake_case)]
#[derive(Clone)]
pub struct Context {
    pub(crate) gp_register_file: RegisterFile<RISCV_NUM_OF_REGISTERS>,
    pub(crate) PC: Register,
    pub(crate) control_status_registers: CSRFile<RISCV_NUM_OF_CSR>,
    pub(crate) current_instruction: InstructionType,
    pub(crate) decoder: Decoder,
    pub(crate) execution: ExecutionContext,
}
