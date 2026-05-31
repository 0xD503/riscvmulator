use super::core::core::Context;

use crate::constants::arch::XLen;
use crate::soc::cpu::core::registers::RegisterInterface;

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct RISC_V_CPU {
    pub(crate) context: Context,
}

impl RISC_V_CPU {
    pub fn reset(&mut self) {
        self.context.reset();
    }

    pub(crate) fn set_pc(&mut self, value: XLen) {
        self.context.PC.write(value);
    }
}
