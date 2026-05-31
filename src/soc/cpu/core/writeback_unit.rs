// use super::

use crate::constants::arch::XLen;
use crate::soc::cpu::RISC_V_CPU;
use crate::soc::cpu::core::registers::RegisterFileInterface;
use crate::soc::cpu::extensions::zicsr::constants::CSRRegisterName;
use crate::soc::cpu::priveleged::machine::MCause_Code;

use log::{error, trace};

impl RISC_V_CPU {
    // write back handlers
    fn write_back_to_reg(&mut self) {
        let register_offset = self.context.decoder.destination_register_offset;
        trace!(
            "Writing back to register x{register_offset:#?},\
                value: {:#018X?}...",
            self.context.execution.result
        );

        self.context
            .gp_register_file
            .write(register_offset, self.context.execution.result);
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
            self.context.execution.return_target_address
        );

        self.set_pc_to_target_address(
            self.context.execution.return_target_address,
        );
    }

    fn branch_to_target_address(&mut self) {
        trace!(
            "Updating PC to BTA: {:#018X?}...",
            self.context.execution.branch_target_address
        );

        self.set_pc_to_target_address(
            self.context.execution.branch_target_address,
        );
    }

    fn jump_to_target_address(&mut self) {
        trace!(
            "Updating PC to JTA: {:#018X?}...",
            self.context.execution.jump_target_address
        );

        self.set_pc_to_target_address(
            self.context.execution.jump_target_address,
        );
    }

    fn trap_to_target_address(&mut self) {
        trace!(
            "Calling trap handler at addr: {:#018X?}...",
            self.context.decoder.zicsr.trap_vector
        );

        self.set_pc_to_target_address(self.context.decoder.zicsr.trap_vector);
    }

    fn call_trap_handler(&mut self) {
        match self.context.decoder.zicsr.trap_cause {
            MCause_Code::EXCEPTION_ENV_CALL_FROM_U_MODE => {
                self.context.control_status_registers.write(
                    CSRRegisterName::mcause as usize,
                    self.context.decoder.zicsr.trap_cause.clone() as XLen,
                );
                self.context.control_status_registers.write(
                    CSRRegisterName::mepc as usize,
                    self.context.decoder.zicsr.epc as XLen,
                );
                error!("TODO: Probably implemented incorrectly");
            }
            MCause_Code::EXCEPTION_ENV_CALL_FROM_S_MODE => {
                todo!();
            }
            MCause_Code::EXCEPTION_ENV_CALL_FROM_M_MODE => {
                self.context.control_status_registers.write(
                    CSRRegisterName::mcause as usize,
                    self.context.decoder.zicsr.trap_cause.clone() as XLen,
                );
                self.context.control_status_registers.write(
                    CSRRegisterName::mepc as usize,
                    self.context.decoder.zicsr.epc as XLen,
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
            .control_status_registers
            .write(register_offset, self.context.decoder.zicsr.csr_value);
    }

    pub fn write_back(&mut self) {
        // check if write to general purpose register is necessary
        if self.context.decoder.need_to_write_back {
            self.write_back_to_reg();
            // jumps always incur register write-back
            if self.context.decoder.need_to_jump {
                self.jump_to_target_address();
                return;
            }
        } else if self.context.decoder.need_to_branch {
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
