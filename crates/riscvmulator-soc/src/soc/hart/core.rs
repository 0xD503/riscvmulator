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

use crate::config::SocCfg;
use crate::constants::arch::{RISCV_NUM_OF_CSR, RISCV_NUM_OF_REGISTERS};
use crate::soc::hart::core::fetch_unit::FetchUnit;
use crate::soc::hart::core::registers::{
    RawRegisterInterface, RegisterFileInterface,
};

#[allow(non_snake_case)]
#[derive(Clone)]
pub struct ArchState {
    pub(crate) gp_register_file: RegisterFile<RISCV_NUM_OF_REGISTERS>,
    pub(crate) PC: Register,
    pub(crate) control_status_registers: CSRFile<RISCV_NUM_OF_CSR>,
}

impl ArchState {
    pub(crate) fn reset(&mut self, cfg: &SocCfg, hartid: usize) {
        self.gp_register_file.reset();
        self.PC.write_raw(cfg.hart.pc_reset_value);
        self.control_status_registers.bringup_hw_state(hartid);
    }
}

#[derive(Clone)]
pub struct Context {
    pub(crate) arch_state: ArchState,
    pub(crate) fetch_unit: FetchUnit,
    pub(crate) decoder: Decoder,
    pub(crate) executor: Executor,
}

impl Context {
    pub fn reset(&mut self, cfg: &SocCfg, hartid: usize) {
        self.fetch_unit.reset();
        self.decoder.reset();
        self.executor.reset(cfg.hart.pc_reset_value);
        self.arch_state.reset(cfg, hartid);
    }
}
