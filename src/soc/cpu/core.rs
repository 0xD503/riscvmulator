pub mod decoder;
pub mod executor;
pub mod fetch_unit;
pub mod gp_registers;
pub mod memory_operator;
pub mod pc_advancer;
pub mod registers;
pub mod writeback_unit;

use self::decoder::Decoder;
use self::executor::Executor;
use self::gp_registers::{Register, RegisterFile};

use super::extensions::zicsr::CSRFile;

use crate::constants::arch::{RISCV_NUM_OF_CSR, RISCV_NUM_OF_REGISTERS};
use crate::soc::cpu::core::fetch_unit::FetchUnit;

#[allow(non_snake_case)]
#[derive(Clone)]
pub struct Context {
    pub(crate) gp_register_file: RegisterFile<RISCV_NUM_OF_REGISTERS>,
    pub(crate) PC: Register,
    pub(crate) control_status_registers: CSRFile<RISCV_NUM_OF_CSR>,
    // pub(crate) current_instruction: InstructionType,
    pub(crate) fetch_unit: FetchUnit,
    pub(crate) decoder: Decoder,
    pub(crate) executor: Executor,
}

impl Context {
    pub fn reset(&mut self) {
        self.decoder.reset();
        self.executor.reset();
    }
}
