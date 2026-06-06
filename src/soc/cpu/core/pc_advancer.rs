// use super::fetch_unit::FetchUnit;

use crate::constants::arch::XLen;
use crate::soc::cpu::Hart;
use crate::soc::cpu::core::registers::RegisterInterface;

use log::trace;

// #[allow(non_camel_case_types)]
// struct PC_Advancer;

// impl PC_Advancer {
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

            self.context.arch_state.PC.write(
                self.context.arch_state.PC.read()
                    + self.context.fetch_unit.instruction_size as XLen,
            );
        } else {
            trace!("Skip advancing PC...");
        }
    }
}
