// modules includes
mod config;
mod constants;
pub mod elf_loader;
mod logger;
mod parse_args;
pub mod soc;
mod utils;

use config::configure;
use logger::init_logger;
use parse_args::{CLI_Args, parse_args};

// addr: {address:#018X?}

// std lib imports

// external libs imports
use log::{debug, error, info, trace, warn};

// current project imports
use constants::arch::*;
use soc::cpu::core::registers::RegisterFileInterface;
use soc::cpu::cpu::RISC_V_CPU;
use soc::cpu::extensions::zicsr::constants::CSRRegisterName;
use soc::cpu::priveleged::machine::*;
use soc::memory::{MemoryValue, MemoryValueType};
use soc::memory_bus::{Bus, SystemBus};
use soc::soc::SoC;

use crate::soc::cpu::core::registers::RegisterInterface;

impl RISC_V_CPU {
    fn set_pc(&mut self, value: XLen) {
        self.context.PC.write(value);
    }

    // memop handlers
    fn read_memory_signed<ValType: MemoryValue>(
        &mut self,
        bus: &mut SystemBus<BusAddrType>,
    ) {
        self.context.execution.result = bus
            .read::<ValType>(self.context.execution.memory_address)
            .sign_extend();
    }

    fn read_memory_unsigned<ValType: MemoryValue>(
        &mut self,
        bus: &mut SystemBus<BusAddrType>,
    ) {
        self.context.execution.result = bus
            .read::<ValType>(self.context.execution.memory_address)
            .zero_extend();
    }

    fn memory_operation(&mut self, bus: &mut SystemBus<BusAddrType>) {
        if self.context.decoder.need_to_read_memory {
            trace!("Executing memory read...");

            match self.context.execution.memory_value_type {
                MemoryValueType::DOUBLE_WORD => {
                    self.read_memory_signed::<DoubleWord>(bus);
                }
                MemoryValueType::WORD => {
                    self.read_memory_signed::<Word>(bus);
                }
                MemoryValueType::HALF_WORD => {
                    self.read_memory_signed::<HalfWord>(bus);
                }
                MemoryValueType::BYTE => {
                    self.read_memory_signed::<Byte>(bus);
                }
                MemoryValueType::UNSIGNED_WORD => {
                    self.read_memory_unsigned::<Word>(bus);
                }
                MemoryValueType::UNSIGNED_HALF_WORD => {
                    self.read_memory_unsigned::<HalfWord>(bus);
                }
                MemoryValueType::UNSIGNED_BYTE => {
                    self.read_memory_unsigned::<Byte>(bus);
                }
            }
        } else if self.context.decoder.need_to_write_memory {
            trace!("Executing memory write...");

            match self.context.execution.memory_value_type {
                MemoryValueType::DOUBLE_WORD => {
                    bus.write(
                        self.context.execution.memory_address,
                        self.context.execution.value_to_store_u64,
                    );
                }
                MemoryValueType::WORD => {
                    bus.write(
                        self.context.execution.memory_address,
                        self.context.execution.value_to_store_u32,
                    );
                }
                MemoryValueType::HALF_WORD => {
                    bus.write(
                        self.context.execution.memory_address,
                        self.context.execution.value_to_store_u16,
                    );
                }
                MemoryValueType::BYTE => {
                    bus.write(
                        self.context.execution.memory_address,
                        self.context.execution.value_to_store_u8,
                    );
                }

                MemoryValueType::UNSIGNED_WORD
                | MemoryValueType::UNSIGNED_HALF_WORD
                | MemoryValueType::UNSIGNED_BYTE => {
                    error!("unreachable!");
                    unreachable!("unreachable!");
                }
            }
        }
    }

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

    fn write_back(&mut self) {
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

    // advancing PC
    fn is_need_to_advance_pc(&self) -> bool {
        !self.context.decoder.need_to_jump
            && !self.context.decoder.need_to_branch
            && !self.context.decoder.zicsr.need_to_return_from_trap
            && !self.context.decoder.zicsr.need_to_call_trap
    }

    fn advance_pc(&mut self) {
        if self.is_need_to_advance_pc() {
            trace!("Advancing PC...");

            self.context
                .PC
                .write(self.context.PC.read() + RISCV_PC_STEP);
        } else {
            trace!("Skip advancing PC...");
        }
    }
}

pub fn init() -> CLI_Args {
    init_logger();
    let args = parse_args();
    configure();

    args
}

pub fn init_test() {
    // init_logger();
    configure();
}
