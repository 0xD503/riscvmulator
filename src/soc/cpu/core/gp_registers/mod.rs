use log::trace;

use crate::constants::arch::{RISCV_X0_ZERO_REGISTER_VALUE, XLen};
use crate::soc::cpu::core::registers::{
    RegisterFileInterface, RegisterInterface,
};
use constants::GPRegisterName;

pub mod constants;

#[derive(Copy, Clone)]
pub struct Register {
    pub(in crate::soc::cpu) value: XLen,
}

#[derive(Clone)]
pub struct RegisterFile<const REGS_NUM: usize> {
    pub(in crate::soc::cpu) registers: [Register; REGS_NUM],
}

impl RegisterInterface for Register {
    fn read(&self) -> XLen {
        self.value
    }
    fn write(&mut self, value: XLen) {
        self.value = value;
    }
}

impl<const REGS_NUM: usize> RegisterFileInterface for RegisterFile<REGS_NUM> {
    fn new() -> Self {
        Self {
            registers: [Register {
                value: RISCV_X0_ZERO_REGISTER_VALUE,
            }; REGS_NUM],
        }
    }

    fn read(&self, offset: usize) -> XLen {
        trace!("Reading register x{offset:#?}...");

        self.registers[offset].read()
    }

    fn write(&mut self, offset: usize, value: XLen) {
        trace!("Writing register x{offset:#?}...");

        // register `x0_zero` is hardwired with all bits equal to 0, so do nothing with `x0_zero`
        if offset != (GPRegisterName::x0_zero as usize) {
            self.registers[offset].write(value);
        } else {
            trace!(
                "An attempt to write value {value} to register `x0_zero` was silently discarded"
            );
        }
    }
}
