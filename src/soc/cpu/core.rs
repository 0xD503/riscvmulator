pub mod decoder;
pub mod executor;
pub mod gp_registers;
pub mod memory_operator;
pub mod pc_advancer;
pub mod registers;
pub mod writeback_unit;

use self::decoder::Decoder;
use self::executor::Executor;
use self::gp_registers::{Register, RegisterFile};

use super::extensions::zicsr::CSRFile;

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
    pub(crate) execution: Executor,
}

impl Context {
    pub fn reset(&mut self) {
        self.decoder.reset();
        self.execution.reset();
    }
}
