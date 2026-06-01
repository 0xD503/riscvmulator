use crate::constants::arch::RISCV_PC_STEP;
use crate::soc::cpu::Hart;
use crate::soc::cpu::core::registers::RegisterInterface;

use log::trace;

impl Hart {
    // advancing PC
    fn is_need_to_advance_pc(&self) -> bool {
        !self.context.decoder.need_to_jump
            && !self.context.executor.need_to_branch
            && !self.context.decoder.zicsr.need_to_return_from_trap
            && !self.context.decoder.zicsr.need_to_call_trap
    }

    pub fn advance_pc(&mut self) {
        if self.is_need_to_advance_pc() {
            trace!("Advancing PC...");

            self.context
                .arch_state
                .PC
                .write(self.context.arch_state.PC.read() + RISCV_PC_STEP);
        } else {
            trace!("Skip advancing PC...");
        }
    }
}
