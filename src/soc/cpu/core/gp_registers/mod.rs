use log::trace;

use crate::constants::arch::{
    RISCV_NUM_OF_REGISTERS, RISCV_NUM_OF_REGISTERS_EMBEDDED,
    RISCV_X0_ZERO_REGISTER_VALUE, XLen,
};
use crate::soc::cpu::core::registers::{
    RegisterFileInterface, RegisterInterface,
};
use constants::GPRegisterName;

pub mod constants;

#[derive(Default, Copy, Clone)]
pub struct Register {
    value: XLen,
}

#[derive(Clone)]
pub struct RegisterFile<const REGS_NUM: usize> {
    registers: [Register; REGS_NUM],
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

    fn length() -> usize {
        REGS_NUM
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

impl<'a, const REGS_NUM: usize> IntoIterator for &'a RegisterFile<REGS_NUM> {
    type Item = &'a Register;
    type IntoIter = std::slice::Iter<'a, Register>;

    fn into_iter(self) -> Self::IntoIter {
        self.registers.iter()
    }
}

impl<'a, const REGS_NUM: usize> IntoIterator
    for &'a mut RegisterFile<REGS_NUM>
{
    type Item = &'a mut Register;
    type IntoIter = std::slice::IterMut<'a, Register>;

    fn into_iter(self) -> Self::IntoIter {
        self.registers.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register() {
        let mut reg = Register::default();

        assert_eq!(reg.value, 0);
        assert_eq!(reg.read(), 0);

        for i in 0..17 {
            reg.write(i);
            assert_eq!(reg.value, i);
            assert_eq!(reg.read(), i);
        }
    }

    #[test]
    fn register_file() {
        const REGS_NUM: usize = RISCV_NUM_OF_REGISTERS;
        let mut reg_file = RegisterFile::<REGS_NUM> {
            registers: [Register::default(); REGS_NUM],
        };

        for reg in &reg_file {
            assert_eq!(reg.value, 0);
        }
        for reg in &reg_file {
            assert_eq!(reg.read(), 0);
        }

        for i in 0..REGS_NUM {
            reg_file.write(i, 7);
        }

        assert_eq!(reg_file.read(0), 0);
        for i in 1..RegisterFile::<REGS_NUM>::length() {
            assert_eq!(reg_file.read(i), 7);
        }
    }

    #[test]
    fn register_file_rv_embedded() {
        const REGS_NUM: usize = RISCV_NUM_OF_REGISTERS_EMBEDDED;
        let mut reg_file = RegisterFile::<REGS_NUM> {
            registers: [Register::default(); REGS_NUM],
        };

        for reg in &reg_file {
            assert_eq!(reg.value, 0);
        }
        for reg in &reg_file {
            assert_eq!(reg.read(), 0);
        }

        for i in 0..REGS_NUM {
            reg_file.write(i, 7);
        }

        assert_eq!(reg_file.read(0), 0);
        for i in 1..RegisterFile::<REGS_NUM>::length() {
            assert_eq!(reg_file.read(i), 7);
        }
    }
}
