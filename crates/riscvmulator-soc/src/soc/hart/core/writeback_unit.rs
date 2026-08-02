// use super::

use crate::soc::hart::Hart;
use crate::soc::hart::core::registers::RegisterFileInterface;
use crate::soc::hart::extensions::zicsr::constants::CSRRegisterName;
use crate::{
    constants::arch::XLen, soc::hart::privileged::constants::MCause_Code,
};

use log::{error, trace};

impl Hart {
    // write back handlers
    fn write_back_to_reg(&mut self) {
        let register_offset = self.context.decoder.destination_register_offset;
        trace!(
            "Writing back to register x{register_offset:#?},\
                value: {:#018X?}...",
            self.context.executor.result
        );

        self.context
            .arch_state
            .gp_register_file
            .write(register_offset, self.context.executor.result);
    }

    fn set_pc_to_target_address(&mut self, target_addr: XLen) {
        error!(
            "TODO: unimplemented: generate exception if BRANCH/JUMP (BTA/JTA) address is misaligned"
        );
        self.set_pc(target_addr);
    }

    fn return_to_target_address(&mut self) {
        trace!(
            "Returning from trap to target address: {:#018X?}...",
            self.context.executor.return_target_address
        );

        self.set_pc_to_target_address(
            self.context.executor.return_target_address,
        );
    }

    fn branch_to_target_address(&mut self) {
        trace!(
            "Updating PC to BTA: {:#018X?}...",
            self.context.executor.branch_target_address
        );

        self.set_pc_to_target_address(
            self.context.executor.branch_target_address,
        );
    }

    fn jump_to_target_address(&mut self) {
        trace!(
            "Updating PC to JTA: {:#018X?}...",
            self.context.executor.jump_target_address
        );

        self.set_pc_to_target_address(
            self.context.executor.jump_target_address,
        );
    }

    fn trap_to_target_address(&mut self) {
        trace!(
            "Calling trap handler at addr: {:#018X?}...",
            self.context.executor.zicsr.trap_vector
        );

        self.set_pc_to_target_address(self.context.executor.zicsr.trap_vector);
    }

    fn call_trap_handler(&mut self) {
        match self.context.executor.zicsr.trap_cause {
            MCause_Code::EXCEPTION_ENV_CALL_FROM_U_MODE => {
                self.context.arch_state.control_status_registers.write(
                    CSRRegisterName::mcause as usize,
                    self.context.executor.zicsr.trap_cause.clone() as XLen,
                );
                self.context.arch_state.control_status_registers.write(
                    CSRRegisterName::mepc as usize,
                    self.context.executor.zicsr.epc as XLen,
                );
                error!("TODO: Probably implemented incorrectly");
            }
            MCause_Code::EXCEPTION_ENV_CALL_FROM_S_MODE => {
                todo!();
            }
            MCause_Code::EXCEPTION_ENV_CALL_FROM_M_MODE => {
                self.context.arch_state.control_status_registers.write(
                    CSRRegisterName::mcause as usize,
                    self.context.executor.zicsr.trap_cause.clone() as XLen,
                );
                self.context.arch_state.control_status_registers.write(
                    CSRRegisterName::mepc as usize,
                    self.context.executor.zicsr.epc as XLen,
                );
            }

            _ => {
                error!("unimplemented!");
                todo!("unimplemented!");
            }
        }

        self.trap_to_target_address();
    }

    fn write_to_csr(&mut self) {
        let register_offset = self.context.decoder.immediate as usize;
        trace!("Writing to CSR #{register_offset:#?}...");

        self.context
            .arch_state
            .control_status_registers
            .write(register_offset, self.context.executor.zicsr.csr_value);
    }

    pub(in crate::soc::hart) fn write_back(&mut self) {
        // check if write to general purpose register is necessary
        if self.context.decoder.need_to_write_back {
            self.write_back_to_reg();
            // jumps always incur register write-back
            if self.context.decoder.need_to_jump {
                self.jump_to_target_address();
                return;
            }
        } else if self.context.executor.need_to_branch {
            self.branch_to_target_address();
            return;
        }
        // check if write to CSR is necessary
        if self.context.decoder.zicsr.need_to_write_csr {
            self.write_to_csr();
        }
        // check if return from trap is needed
        else if self.context.decoder.zicsr.need_to_return_from_trap {
            self.return_to_target_address();
            return;
        }
        if self.context.decoder.zicsr.need_to_call_trap {
            self.call_trap_handler();
        }
    }
}
