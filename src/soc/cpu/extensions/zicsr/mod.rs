use log::trace;

use crate::constants::arch::{RISCV_CSR_INITIAL_VALUE, XLen};
use crate::soc::cpu::core::registers::{
    RegisterFileInterface, RegisterInterface,
};

pub mod constants;
use crate::soc::cpu::priveleged::machine::MCause_Code;

#[allow(clippy::upper_case_acronyms)]
#[derive(Default, Copy, Clone)]
pub struct CSR {
    pub(in crate::soc::cpu) value: XLen,
}

#[derive(Clone)]
pub struct CSRFile<const REGS_NUM: usize> {
    pub(in crate::soc::cpu) registers: [CSR; REGS_NUM],
}

impl<const REGS_NUM: usize> Default for CSRFile<REGS_NUM> {
    fn default() -> Self {
        Self {
            registers: [Default::default(); REGS_NUM],
        }
    }
}

impl RegisterInterface for CSR {
    fn read(&self) -> XLen {
        self.value
    }
    fn write(&mut self, value: XLen) {
        self.value = value;
    }
}

impl<const REGS_NUM: usize> RegisterFileInterface for CSRFile<REGS_NUM> {
    fn new() -> Self {
        Self {
            registers: [CSR {
                value: RISCV_CSR_INITIAL_VALUE,
            }; REGS_NUM],
        }
    }

    fn length() -> usize {
        REGS_NUM
    }

    fn read(&self, offset: usize) -> XLen {
        trace!("Reading CSR #{offset:#?}...");

        self.registers[offset].read()
    }

    fn write(&mut self, offset: usize, value: XLen) {
        trace!("Writing CSR #{offset:#?}...");

        self.registers[offset].write(value);
    }
}

// #[derive(Default, Clone)]
// pub struct ZicsrContext {
//     pub(in crate::soc::cpu) need_to_read_csr: bool,
//     pub(in crate::soc::cpu) need_to_write_csr: bool,
//     pub(in crate::soc::cpu) need_to_return_from_trap: bool,
//     pub(in crate::soc::cpu) csr_value: XLen,
//     pub(in crate::soc::cpu) uimm: XLen,
// }

#[derive(Default, Clone)]
pub struct ZicsrContext {
    pub(crate) need_to_read_csr: bool,
    pub(crate) need_to_write_csr: bool,
    pub(crate) need_to_return_from_trap: bool,
    pub(crate) need_to_call_trap: bool,
    pub(crate) csr_value: XLen,
    pub(crate) trap_cause: MCause_Code,
    pub(crate) trap_vector: XLen,
    pub(crate) epc: XLen, // exception program counter
    pub(crate) uimm: XLen,
}
