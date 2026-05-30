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
use std::convert::From;

// external libs imports
use log::{debug, error, info, trace, warn};

// current project imports
use constants::arch::*;
use constants::soc::*;
use soc::cpu::core::core::Context;
use soc::cpu::core::decoder::constants::*;
use soc::cpu::core::decoder::decoder::Decoder;
use soc::cpu::core::executor::constants::ExecutableCommand;
use soc::cpu::core::executor::executor::ExecutionContext;
use soc::cpu::core::gp_registers::{Register, RegisterFile};
use soc::cpu::core::registers::RegisterFileInterface;
use soc::cpu::cpu::RISC_V_CPU;
use soc::cpu::extensions::zicsr::CSRFile;
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

    fn execute(&mut self) {
        match self.context.decoder.command_to_execute {
            // RV32I
            // I_3
            ExecutableCommand::LB => self.instruction_load_byte(),
            ExecutableCommand::LH => self.instruction_load_half(),
            ExecutableCommand::LW => self.instruction_load_word(),
            ExecutableCommand::LBU => self.instruction_load_byte_unsigned(),
            ExecutableCommand::LHU => self.instruction_load_half_unsigned(),
            // I_19
            ExecutableCommand::ADDI => self.instruction_addi(),
            ExecutableCommand::SLLI => self.instruction_slli(),
            ExecutableCommand::SLTI => self.instruction_slti(),
            ExecutableCommand::SLTIU => self.instruction_sltiu(),
            ExecutableCommand::XORI => self.instruction_xori(),
            ExecutableCommand::SRLI => self.instruction_srli(),
            ExecutableCommand::SRAI => self.instruction_srai(),
            ExecutableCommand::ORI => self.instruction_ori(),
            ExecutableCommand::ANDI => self.instruction_andi(),
            // U_23
            ExecutableCommand::AUIPC => self.instruction_auipc(),
            // S_35
            ExecutableCommand::SB => self.instruction_store_byte(),
            ExecutableCommand::SH => self.instruction_store_half_word(),
            ExecutableCommand::SW => self.instruction_store_word(),
            // R_51
            ExecutableCommand::ADD => self.instruction_add(),
            ExecutableCommand::SUB => self.instruction_sub(),
            ExecutableCommand::SLL => self.instruction_sll(),
            ExecutableCommand::SLT => self.instruction_slt(),
            ExecutableCommand::SLTU => self.instruction_sltu(),
            ExecutableCommand::XOR => self.instruction_xor(),
            ExecutableCommand::SRL => self.instruction_srl(),
            ExecutableCommand::SRA => self.instruction_sra(),
            ExecutableCommand::OR => self.instruction_or(),
            ExecutableCommand::AND => self.instruction_and(),
            // U_55
            ExecutableCommand::LUI => self.instruction_lui(),
            // B_99
            ExecutableCommand::BEQ => self.instruction_beq(),
            ExecutableCommand::BNE => self.instruction_bne(),
            ExecutableCommand::BLT => self.instruction_blt(),
            ExecutableCommand::BGE => self.instruction_bge(),
            ExecutableCommand::BLTU => self.instruction_bltu(),
            ExecutableCommand::BGEU => self.instruction_bgeu(),
            // I_103
            ExecutableCommand::JALR => self.instruction_jalr(),
            // J_111
            ExecutableCommand::JAL => self.instruction_jal(),

            // RV64I
            // I_3
            ExecutableCommand::LD => self.instruction_load_double_word(),
            ExecutableCommand::LWU => self.instruction_load_word_unsigned(),
            // I_27
            ExecutableCommand::ADDIW => {
                self.instruction_add_immediate_word();
            }
            ExecutableCommand::SLLIW => {
                self.instruction_shift_left_logical_immediate_word();
            }
            ExecutableCommand::SRLIW => {
                self.instruction_shift_right_logical_immediate_word();
            }
            ExecutableCommand::SRAIW => {
                self.instruction_shift_right_arith_immediate_word();
            }
            // S_35
            ExecutableCommand::SD => self.instruction_store_double_word(),
            // R_59
            ExecutableCommand::ADDW => self.instruction_add_word(),
            ExecutableCommand::SUBW => self.instruction_sub_word(),
            ExecutableCommand::SLLW => {
                self.instruction_shift_left_logical_word();
            }
            ExecutableCommand::SRLW => {
                self.instruction_shift_right_logical_word();
            }
            ExecutableCommand::SRAW => {
                self.instruction_shift_right_arith_word();
            }

            ExecutableCommand::FENCE => {
                self.instruction_fence();
            }

            ExecutableCommand::FENCE_I => {
                self.instruction_fence_i();
            }

            ExecutableCommand::ECALL => {
                self.instruction_execution_call();
            }
            ExecutableCommand::EBREAK => {
                self.instruction_execution_break();
            }

            ExecutableCommand::URET => {
                self.instruction_user_return();
            }
            ExecutableCommand::SRET => {
                self.instruction_supervisor_return();
            }
            ExecutableCommand::MRET => {
                self.instruction_machine_return();
            }

            // Zicsr extension
            ExecutableCommand::CSRRW => {
                self.instruction_csr_read_write();
            }
            ExecutableCommand::CSRRS => {
                self.instruction_csr_read_set();
            }
            ExecutableCommand::CSRRC => {
                self.instruction_csr_read_clear();
            }
            ExecutableCommand::CSRRWI => {
                self.instruction_csr_read_write_immediate();
            }
            ExecutableCommand::CSRRSI => {
                self.instruction_csr_read_set_immediate();
            }
            ExecutableCommand::CSRRCI => {
                self.instruction_csr_read_clear_immediate();
            }
        }
    }

    fn instruction_load_byte(&mut self) {
        trace!("Executing LB...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.execution.memory_value_type = MemoryValueType::BYTE;
    }

    fn instruction_load_half(&mut self) {
        trace!("Executing LH...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.execution.memory_value_type = MemoryValueType::HALF_WORD;
    }

    fn instruction_load_word(&mut self) {
        trace!("Executing LW...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.execution.memory_value_type = MemoryValueType::WORD;
    }

    fn instruction_load_byte_unsigned(&mut self) {
        trace!("Executing LBU...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.execution.memory_value_type =
            MemoryValueType::UNSIGNED_BYTE;
    }

    fn instruction_load_half_unsigned(&mut self) {
        trace!("Executing LHU...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.execution.memory_value_type =
            MemoryValueType::UNSIGNED_HALF_WORD;
    }

    fn instruction_addi(&mut self) {
        trace!("Executing ADDI...");

        self.context.execution.result = self.context.decoder.source_register_1
            + self.context.decoder.immediate as SignedImmediate as SignedXLen
                as XLen;
    }

    fn instruction_slli(&mut self) {
        trace!("Executing SLLI...");

        self.context.execution.result = self.context.decoder.source_register_1
            << self.context.decoder.shamt;
    }

    fn instruction_slti(&mut self) {
        trace!("Executing SLTI...");

        self.context.execution.result =
            if (self.context.decoder.source_register_1 as SignedXLen)
                < (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen)
            {
                1
            } else {
                0
            };
    }

    fn instruction_sltiu(&mut self) {
        trace!("Executing SLTIU...");

        self.context.execution.result =
            if (self.context.decoder.source_register_1 as UnsignedXLen)
                < (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as UnsignedXLen)
            {
                1
            } else {
                0
            };
    }

    fn instruction_xori(&mut self) {
        trace!("Executing XORI...");

        self.context.execution.result = self.context.decoder.source_register_1
            ^ self.context.decoder.immediate as SignedImmediate as SignedXLen
                as XLen;
    }

    fn instruction_srli(&mut self) {
        trace!("Executing SRLI...");

        self.context.execution.result = self.context.decoder.source_register_1
            >> self.context.decoder.shamt;
    }

    fn instruction_srai(&mut self) {
        trace!("Executing SRAI...");

        self.context.execution.result =
            ((self.context.decoder.source_register_1 as SignedXLen)
                >> self.context.decoder.shamt) as XLen;
    }

    fn instruction_ori(&mut self) {
        trace!("Executing ORI...");

        self.context.execution.result = self.context.decoder.source_register_1
            | self.context.decoder.immediate as SignedImmediate as SignedXLen
                as XLen;
    }

    fn instruction_andi(&mut self) {
        trace!("Executing ANDI...");

        self.context.execution.result = self.context.decoder.source_register_1
            & self.context.decoder.immediate as SignedImmediate as SignedXLen
                as XLen;
    }

    fn instruction_auipc(&mut self) {
        trace!("Executing AUIPC...");

        self.context.execution.result = self.context.PC.read()
            + self.context.decoder.immediate as SignedImmediate as SignedXLen
                as XLen;
    }

    fn instruction_store_byte(&mut self) {
        trace!("Executing SB...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.execution.value_to_store_u8 =
            self.context.decoder.source_register_2 as Byte;
        self.context.execution.memory_value_type = MemoryValueType::BYTE;
    }

    fn instruction_store_half_word(&mut self) {
        trace!("Executing SH...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.execution.value_to_store_u16 =
            self.context.decoder.source_register_2 as HalfWord;
        self.context.execution.memory_value_type = MemoryValueType::HALF_WORD;
    }

    fn instruction_store_word(&mut self) {
        trace!("Executing SW...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.execution.value_to_store_u32 =
            self.context.decoder.source_register_2 as Word;
        self.context.execution.memory_value_type = MemoryValueType::WORD;
    }

    fn instruction_add(&mut self) {
        trace!("Executing ADD...");

        self.context.execution.result = self.context.decoder.source_register_1
            + self.context.decoder.source_register_2;
    }

    fn instruction_sub(&mut self) {
        trace!("Executing SUB...");

        self.context.execution.result = self.context.decoder.source_register_1
            - self.context.decoder.source_register_2;
    }

    fn instruction_sll(&mut self) {
        trace!("Executing SLL...");

        self.context.execution.result = self.context.decoder.source_register_1
            << self.context.decoder.source_register_2;
    }

    fn instruction_slt(&mut self) {
        trace!("Executing SLT...");

        self.context.execution.result =
            if (self.context.decoder.source_register_1 as SignedXLen)
                < (self.context.decoder.source_register_2 as SignedXLen)
            {
                1
            } else {
                0
            }
    }

    fn instruction_sltu(&mut self) {
        trace!("Executing SLTU...");

        self.context.execution.result =
            if self.context.decoder.source_register_1
                < self.context.decoder.source_register_2
            {
                1
            } else {
                0
            }
    }

    fn instruction_xor(&mut self) {
        trace!("Executing XOR...");

        self.context.execution.result = self.context.decoder.source_register_1
            ^ self.context.decoder.source_register_2;
    }

    fn instruction_srl(&mut self) {
        trace!("Executing SRL...");

        self.context.execution.result = self.context.decoder.source_register_1
            >> self.context.decoder.source_register_2;
    }

    fn instruction_sra(&mut self) {
        trace!("Executing SRA...");

        self.context.execution.result =
            ((self.context.decoder.source_register_1 as SignedXLen)
                >> self.context.decoder.source_register_2) as XLen;
    }

    fn instruction_or(&mut self) {
        trace!("Executing OR...");

        self.context.execution.result = self.context.decoder.source_register_1
            | self.context.decoder.source_register_2;
    }

    fn instruction_and(&mut self) {
        trace!("Executing AND...");

        self.context.execution.result = self.context.decoder.source_register_1
            & self.context.decoder.source_register_2;
    }

    fn instruction_lui(&mut self) {
        trace!("Executing LUI...");

        self.context.execution.result = self.context.decoder.immediate
            as SignedImmediate
            as SignedXLen as XLen;
    }

    fn instruction_beq(&mut self) {
        trace!("Executing BEQ...");

        if self.context.decoder.source_register_1
            == self.context.decoder.source_register_2
        {
            self.context.execution.branch_target_address =
                self.context.PC.read()
                    + (self.context.decoder.immediate as SignedImmediate
                        as SignedXLen as XLen);
            self.context.decoder.need_to_branch = true;
        }
        // TODO: generate exception if address is misaligned
        error!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_bne(&mut self) {
        trace!(
            "Executing BNE: {:#018X}, {:#018X}...",
            self.context.decoder.source_register_1,
            self.context.decoder.source_register_2
        );

        if self.context.decoder.source_register_1
            != self.context.decoder.source_register_2
        {
            self.context.execution.branch_target_address =
                self.context.PC.read()
                    + (self.context.decoder.immediate as SignedImmediate
                        as SignedXLen as XLen);
            self.context.decoder.need_to_branch = true;
        } else {
            trace!("Skip BNE...");
        }
        // TODO: generate exception if address is misaligned
        error!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_blt(&mut self) {
        trace!("Executing BLT...");

        if (self.context.decoder.source_register_1 as SignedXLen)
            < (self.context.decoder.source_register_2 as SignedXLen)
        {
            self.context.execution.branch_target_address =
                self.context.PC.read()
                    + (self.context.decoder.immediate as SignedImmediate
                        as SignedXLen as XLen);
            self.context.decoder.need_to_branch = true;
        }
        // TODO: generate exception if address is misaligned
        error!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_bge(&mut self) {
        trace!("Executing BGE...");

        if (self.context.decoder.source_register_1 as SignedXLen)
            >= (self.context.decoder.source_register_2 as SignedXLen)
        {
            self.context.execution.branch_target_address =
                self.context.PC.read()
                    + (self.context.decoder.immediate as SignedImmediate
                        as SignedXLen as XLen);
            self.context.decoder.need_to_branch = true;
        }
        // TODO: generate exception if address is misaligned
        error!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_bltu(&mut self) {
        trace!("Executing BLTU...");

        if (self.context.decoder.source_register_1 as UnsignedXLen)
            < (self.context.decoder.source_register_2 as UnsignedXLen)
        {
            self.context.execution.branch_target_address =
                self.context.PC.read()
                    + (self.context.decoder.immediate as SignedImmediate
                        as SignedXLen as XLen);
            self.context.decoder.need_to_branch = true;
        }
        // TODO: generate exception if address is misaligned
        error!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_bgeu(&mut self) {
        trace!("Executing BGEU...");

        if (self.context.decoder.source_register_1 as UnsignedXLen)
            >= (self.context.decoder.source_register_2 as UnsignedXLen)
        {
            self.context.execution.branch_target_address =
                self.context.PC.read()
                    + (self.context.decoder.immediate as SignedImmediate
                        as SignedXLen as XLen);
            self.context.decoder.need_to_branch = true;
        }
        // TODO: generate exception if address is misaligned
        error!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_jalr(&mut self) {
        trace!("Executing JALR...");

        self.context.execution.jump_target_address =
            (self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen))
                & !(XLen::MAX & 0x01);
        self.context.execution.result = self.context.PC.read() + RISCV_PC_STEP;
        // TODO: generate exception if address is misaligned
        error!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_jal(&mut self) {
        trace!("Executing JAL...");

        self.context.execution.jump_target_address = self.context.PC.read()
            + (self.context.decoder.immediate as SignedImmediate as SignedXLen
                as XLen);
        self.context.execution.result = self.context.PC.read() + RISCV_PC_STEP;
        // TODO: generate exception if address is misaligned
        error!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_load_double_word(&mut self) {
        trace!("Executing LD...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.execution.memory_value_type = MemoryValueType::DOUBLE_WORD;
    }

    fn instruction_load_word_unsigned(&mut self) {
        trace!("Executing LWU...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.execution.memory_value_type =
            MemoryValueType::UNSIGNED_WORD;
    }

    fn instruction_add_immediate_word(&mut self) {
        trace!("Executing ADDIW...");

        let result32 = (self.context.decoder.source_register_1 as XLen32)
            + self.context.decoder.immediate;
        self.context.execution.result =
            result32 as SignedXLen32 as SignedXLen as XLen;
    }

    fn instruction_shift_left_logical_immediate_word(&mut self) {
        trace!("Executing SLLIW...");

        let result32 = (self.context.decoder.source_register_1 as XLen32)
            << self.context.decoder.immediate;
        self.context.execution.result =
            result32 as SignedXLen32 as SignedXLen as XLen;
    }

    fn instruction_shift_right_logical_immediate_word(&mut self) {
        trace!("Executing SRLIW...");

        let result32 = (self.context.decoder.source_register_1 as XLen32)
            >> self.context.decoder.immediate;
        self.context.execution.result =
            result32 as SignedXLen32 as SignedXLen as XLen;
    }

    fn instruction_shift_right_arith_immediate_word(&mut self) {
        trace!(
            "Executing SRAIW: {:#018X} >> {}...",
            self.context.decoder.source_register_1 as SignedXLen32,
            self.context.decoder.immediate
        );

        let result32 = (self.context.decoder.source_register_1 as SignedXLen32)
            >> self.context.decoder.immediate;
        self.context.execution.result = result32 as SignedXLen as XLen;
    }

    fn instruction_store_double_word(&mut self) {
        trace!("Executing SD...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.execution.value_to_store_u64 =
            self.context.decoder.source_register_2;
        self.context.execution.memory_value_type = MemoryValueType::DOUBLE_WORD;
    }

    fn instruction_add_word(&mut self) {
        trace!("Executing ADDW...");

        let result32 = self.context.decoder.source_register_1 as SignedXLen32
            + self.context.decoder.source_register_2 as SignedXLen32;
        self.context.execution.result = result32 as SignedXLen as XLen;
    }

    fn instruction_sub_word(&mut self) {
        trace!("Executing SUBW...");

        let result32 = self.context.decoder.source_register_1 as SignedXLen32
            - self.context.decoder.source_register_2 as SignedXLen32;
        self.context.execution.result = result32 as SignedXLen as XLen;
    }

    fn instruction_shift_left_logical_word(&mut self) {
        trace!("Executing SLLW...");

        let result32 = (self.context.decoder.source_register_1 as XLen32)
            << self.context.decoder.shamt;
        self.context.execution.result =
            result32 as SignedXLen32 as SignedXLen as XLen;
    }

    fn instruction_shift_right_logical_word(&mut self) {
        trace!("Executing SRLW...");

        let result32 = (self.context.decoder.source_register_1 as XLen32)
            >> self.context.decoder.shamt;
        self.context.execution.result =
            result32 as SignedXLen32 as SignedXLen as XLen;
    }

    fn instruction_shift_right_arith_word(&mut self) {
        trace!(
            "Executing SRAW: {:#018} >> {:?}...",
            self.context.decoder.source_register_1 as SignedXLen32,
            self.context.decoder.shamt
        );

        let result32 = (self.context.decoder.source_register_1 as SignedXLen32)
            >> self.context.decoder.shamt;
        self.context.execution.result = result32 as SignedXLen as XLen;
    }

    fn instruction_fence(&mut self) {
        trace!("Executing FENCE...");

        //
        error!("TODO: unimplemented instruction");
    }

    fn instruction_fence_i(&mut self) {
        trace!("Executing FENCE_I...");

        //
        error!("TODO: unimplemented instruction");
    }

    fn instruction_execution_call(&mut self) {
        trace!("Executing ECALL...");

        let current_privilege_mode = self.read_mstatus_mpp();
        match current_privilege_mode {
            MStatus_MPP::USER => {
                self.context.decoder.zicsr.trap_cause =
                    MCause_Code::EXCEPTION_ENV_CALL_FROM_U_MODE;
                self.context.decoder.zicsr.trap_vector = self
                    .context
                    .control_status_registers
                    .read(CSRRegisterName::mtvec as usize);
            }
            MStatus_MPP::SUPERVISOR => {
                self.context.decoder.zicsr.trap_cause =
                    MCause_Code::EXCEPTION_ENV_CALL_FROM_S_MODE;
                todo!();
            }
            MStatus_MPP::MACHINE => {
                self.context.decoder.zicsr.trap_cause =
                    MCause_Code::EXCEPTION_ENV_CALL_FROM_M_MODE;
                self.context.decoder.zicsr.trap_vector = self
                    .context
                    .control_status_registers
                    .read(CSRRegisterName::mtvec as usize);
            }

            _ => {
                error!("unimplemented!");
                todo!("unimplemented!");
            }
        }
        self.context.decoder.zicsr.epc = self.context.PC.read();
    }

    fn instruction_execution_break(&mut self) {
        trace!("Executing EBREAK...");

        //
        todo!();
    }

    fn instruction_user_return(&mut self) {
        trace!("Executing URET...");

        //
        todo!();
    }

    fn instruction_supervisor_return(&mut self) {
        trace!("Executing SRET...");

        //
        todo!();
    }

    fn instruction_machine_return(&mut self) {
        trace!("Executing MRET...");

        self.context.execution.return_target_address = self
            .context
            .control_status_registers
            .read(CSRRegisterName::mepc as usize);
    }

    fn read_csr_to_result(&mut self) {
        self.context.execution.result = self
            .context
            .control_status_registers
            .read(self.context.decoder.immediate as usize);
    }

    fn prepare_value_for_csr(&mut self, value: XLen) {
        self.context.decoder.zicsr.csr_value = value;
    }

    fn prepare_set_bits_for_csr(&mut self, value: XLen) {
        self.context.decoder.zicsr.csr_value |= value;
    }

    fn prepare_clear_bits_for_csr(&mut self, value: XLen) {
        self.context.decoder.zicsr.csr_value &= !value;
    }

    fn instruction_csr_read_write(&mut self) {
        trace!("Executing CSRRW...");

        if self.context.decoder.zicsr.need_to_read_csr {
            self.read_csr_to_result();
        }
        self.prepare_value_for_csr(self.context.decoder.source_register_1);
    }

    fn instruction_csr_read_set(&mut self) {
        trace!("Executing CSRRS...");

        self.read_csr_to_result();
        if self.context.decoder.zicsr.need_to_write_csr {
            self.prepare_set_bits_for_csr(
                self.context.decoder.source_register_1,
            );
        }
    }

    fn instruction_csr_read_clear(&mut self) {
        trace!("Executing CSRRC...");

        self.read_csr_to_result();
        if self.context.decoder.zicsr.need_to_write_csr {
            self.prepare_clear_bits_for_csr(
                self.context.decoder.source_register_1,
            );
        }
    }

    fn instruction_csr_read_write_immediate(&mut self) {
        trace!("Executing CSRRWI...");

        if self.context.decoder.zicsr.need_to_read_csr {
            self.read_csr_to_result();
        }
        self.prepare_value_for_csr(self.context.decoder.zicsr.uimm);
    }

    fn instruction_csr_read_set_immediate(&mut self) {
        trace!("Executing CSRRSI...");

        self.read_csr_to_result();
        if self.context.decoder.zicsr.need_to_write_csr {
            self.prepare_set_bits_for_csr(self.context.decoder.zicsr.uimm);
        }
    }

    fn instruction_csr_read_clear_immediate(&mut self) {
        trace!("Executing CSRRCI...");

        self.read_csr_to_result();
        if self.context.decoder.zicsr.need_to_write_csr {
            self.prepare_clear_bits_for_csr(self.context.decoder.zicsr.uimm);
        }
    }

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
        self.context.PC.write(target_addr);
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

    // privileged mode extractors
    fn read_mstatus_mpp(&self) -> MStatus_MPP {
        let mstatus = self
            .context
            .control_status_registers
            .read(CSRRegisterName::mstatus as usize);
        MStatus_MPP::from((mstatus >> MSTATUS_MPP_OFFSET) & MSTATUS_MPP_MASK)
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
