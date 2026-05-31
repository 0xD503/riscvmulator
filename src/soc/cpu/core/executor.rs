pub mod constants;

use self::constants::ExecutableCommand;

use crate::constants::arch::{
    Byte, DoubleWord, HalfWord, RISCV_PC_STEP, SignedImmediate, SignedXLen,
    SignedXLen32, UnsignedXLen, Word, XLen, XLen32,
};
use crate::soc::cpu::RISC_V_CPU;
use crate::soc::cpu::core::registers::{
    RegisterFileInterface, RegisterInterface,
};
use crate::soc::cpu::extensions::zicsr::constants::CSRRegisterName;
use crate::soc::cpu::priveleged::machine::{
    MCause_Code, MSTATUS_MPP_MASK, MSTATUS_MPP_OFFSET, MStatus_MPP,
};
use crate::soc::memory::MemoryValueType;

use log::{error, trace};

#[derive(Default, Clone)]
pub struct Executor {
    pub(crate) result: XLen,
    pub(crate) branch_target_address: XLen,
    pub(crate) jump_target_address: XLen,
    pub(crate) return_target_address: XLen,
    pub(crate) memory_address: XLen,
    pub(crate) value_to_store_u8: Byte,
    pub(crate) value_to_store_u16: HalfWord,
    pub(crate) value_to_store_u32: Word,
    pub(crate) value_to_store_u64: DoubleWord,
    // value_to_store_u128: u128,
    pub(crate) memory_value_type: MemoryValueType,
}

impl Executor {
    pub fn reset(&mut self) {
        *self = Default::default();
    }
}

impl RISC_V_CPU {
    pub fn execute(&mut self) {
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

        self.context.executor.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.executor.memory_value_type = MemoryValueType::BYTE;
    }

    fn instruction_load_half(&mut self) {
        trace!("Executing LH...");

        self.context.executor.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.executor.memory_value_type = MemoryValueType::HALF_WORD;
    }

    fn instruction_load_word(&mut self) {
        trace!("Executing LW...");

        self.context.executor.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.executor.memory_value_type = MemoryValueType::WORD;
    }

    fn instruction_load_byte_unsigned(&mut self) {
        trace!("Executing LBU...");

        self.context.executor.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.executor.memory_value_type =
            MemoryValueType::UNSIGNED_BYTE;
    }

    fn instruction_load_half_unsigned(&mut self) {
        trace!("Executing LHU...");

        self.context.executor.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.executor.memory_value_type =
            MemoryValueType::UNSIGNED_HALF_WORD;
    }

    fn instruction_addi(&mut self) {
        trace!("Executing ADDI...");

        self.context.executor.result = self.context.decoder.source_register_1
            + self.context.decoder.immediate as SignedImmediate as SignedXLen
                as XLen;
    }

    fn instruction_slli(&mut self) {
        trace!("Executing SLLI...");

        self.context.executor.result = self.context.decoder.source_register_1
            << self.context.decoder.shamt;
    }

    fn instruction_slti(&mut self) {
        trace!("Executing SLTI...");

        self.context.executor.result =
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

        self.context.executor.result =
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

        self.context.executor.result = self.context.decoder.source_register_1
            ^ self.context.decoder.immediate as SignedImmediate as SignedXLen
                as XLen;
    }

    fn instruction_srli(&mut self) {
        trace!("Executing SRLI...");

        self.context.executor.result = self.context.decoder.source_register_1
            >> self.context.decoder.shamt;
    }

    fn instruction_srai(&mut self) {
        trace!("Executing SRAI...");

        self.context.executor.result =
            ((self.context.decoder.source_register_1 as SignedXLen)
                >> self.context.decoder.shamt) as XLen;
    }

    fn instruction_ori(&mut self) {
        trace!("Executing ORI...");

        self.context.executor.result = self.context.decoder.source_register_1
            | self.context.decoder.immediate as SignedImmediate as SignedXLen
                as XLen;
    }

    fn instruction_andi(&mut self) {
        trace!("Executing ANDI...");

        self.context.executor.result = self.context.decoder.source_register_1
            & self.context.decoder.immediate as SignedImmediate as SignedXLen
                as XLen;
    }

    fn instruction_auipc(&mut self) {
        trace!("Executing AUIPC...");

        self.context.executor.result = self.context.PC.read()
            + self.context.decoder.immediate as SignedImmediate as SignedXLen
                as XLen;
    }

    fn instruction_store_byte(&mut self) {
        trace!("Executing SB...");

        self.context.executor.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.executor.value_to_store_u8 =
            self.context.decoder.source_register_2 as Byte;
        self.context.executor.memory_value_type = MemoryValueType::BYTE;
    }

    fn instruction_store_half_word(&mut self) {
        trace!("Executing SH...");

        self.context.executor.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.executor.value_to_store_u16 =
            self.context.decoder.source_register_2 as HalfWord;
        self.context.executor.memory_value_type = MemoryValueType::HALF_WORD;
    }

    fn instruction_store_word(&mut self) {
        trace!("Executing SW...");

        self.context.executor.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.executor.value_to_store_u32 =
            self.context.decoder.source_register_2 as Word;
        self.context.executor.memory_value_type = MemoryValueType::WORD;
    }

    fn instruction_add(&mut self) {
        trace!("Executing ADD...");

        self.context.executor.result = self.context.decoder.source_register_1
            + self.context.decoder.source_register_2;
    }

    fn instruction_sub(&mut self) {
        trace!("Executing SUB...");

        self.context.executor.result = self.context.decoder.source_register_1
            - self.context.decoder.source_register_2;
    }

    fn instruction_sll(&mut self) {
        trace!("Executing SLL...");

        self.context.executor.result = self.context.decoder.source_register_1
            << self.context.decoder.source_register_2;
    }

    fn instruction_slt(&mut self) {
        trace!("Executing SLT...");

        self.context.executor.result =
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

        self.context.executor.result = if self.context.decoder.source_register_1
            < self.context.decoder.source_register_2
        {
            1
        } else {
            0
        }
    }

    fn instruction_xor(&mut self) {
        trace!("Executing XOR...");

        self.context.executor.result = self.context.decoder.source_register_1
            ^ self.context.decoder.source_register_2;
    }

    fn instruction_srl(&mut self) {
        trace!("Executing SRL...");

        self.context.executor.result = self.context.decoder.source_register_1
            >> self.context.decoder.source_register_2;
    }

    fn instruction_sra(&mut self) {
        trace!("Executing SRA...");

        self.context.executor.result =
            ((self.context.decoder.source_register_1 as SignedXLen)
                >> self.context.decoder.source_register_2) as XLen;
    }

    fn instruction_or(&mut self) {
        trace!("Executing OR...");

        self.context.executor.result = self.context.decoder.source_register_1
            | self.context.decoder.source_register_2;
    }

    fn instruction_and(&mut self) {
        trace!("Executing AND...");

        self.context.executor.result = self.context.decoder.source_register_1
            & self.context.decoder.source_register_2;
    }

    fn instruction_lui(&mut self) {
        trace!("Executing LUI...");

        self.context.executor.result = self.context.decoder.immediate
            as SignedImmediate
            as SignedXLen as XLen;
    }

    fn instruction_beq(&mut self) {
        trace!("Executing BEQ...");

        if self.context.decoder.source_register_1
            == self.context.decoder.source_register_2
        {
            self.context.executor.branch_target_address =
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
            self.context.executor.branch_target_address =
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
            self.context.executor.branch_target_address =
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
            self.context.executor.branch_target_address =
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
            self.context.executor.branch_target_address =
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
            self.context.executor.branch_target_address =
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

        self.context.executor.jump_target_address =
            (self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen))
                & !(XLen::MAX & 0x01);
        self.context.executor.result = self.context.PC.read() + RISCV_PC_STEP;
        // TODO: generate exception if address is misaligned
        error!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_jal(&mut self) {
        trace!("Executing JAL...");

        self.context.executor.jump_target_address = self.context.PC.read()
            + (self.context.decoder.immediate as SignedImmediate as SignedXLen
                as XLen);
        self.context.executor.result = self.context.PC.read() + RISCV_PC_STEP;
        // TODO: generate exception if address is misaligned
        error!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_load_double_word(&mut self) {
        trace!("Executing LD...");

        self.context.executor.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.executor.memory_value_type = MemoryValueType::DOUBLE_WORD;
    }

    fn instruction_load_word_unsigned(&mut self) {
        trace!("Executing LWU...");

        self.context.executor.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.executor.memory_value_type =
            MemoryValueType::UNSIGNED_WORD;
    }

    fn instruction_add_immediate_word(&mut self) {
        trace!("Executing ADDIW...");

        let result32 = (self.context.decoder.source_register_1 as XLen32)
            + self.context.decoder.immediate;
        self.context.executor.result =
            result32 as SignedXLen32 as SignedXLen as XLen;
    }

    fn instruction_shift_left_logical_immediate_word(&mut self) {
        trace!("Executing SLLIW...");

        let result32 = (self.context.decoder.source_register_1 as XLen32)
            << self.context.decoder.immediate;
        self.context.executor.result =
            result32 as SignedXLen32 as SignedXLen as XLen;
    }

    fn instruction_shift_right_logical_immediate_word(&mut self) {
        trace!("Executing SRLIW...");

        let result32 = (self.context.decoder.source_register_1 as XLen32)
            >> self.context.decoder.immediate;
        self.context.executor.result =
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
        self.context.executor.result = result32 as SignedXLen as XLen;
    }

    fn instruction_store_double_word(&mut self) {
        trace!("Executing SD...");

        self.context.executor.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.executor.value_to_store_u64 =
            self.context.decoder.source_register_2;
        self.context.executor.memory_value_type = MemoryValueType::DOUBLE_WORD;
    }

    fn instruction_add_word(&mut self) {
        trace!("Executing ADDW...");

        let result32 = self.context.decoder.source_register_1 as SignedXLen32
            + self.context.decoder.source_register_2 as SignedXLen32;
        self.context.executor.result = result32 as SignedXLen as XLen;
    }

    fn instruction_sub_word(&mut self) {
        trace!("Executing SUBW...");

        let result32 = self.context.decoder.source_register_1 as SignedXLen32
            - self.context.decoder.source_register_2 as SignedXLen32;
        self.context.executor.result = result32 as SignedXLen as XLen;
    }

    fn instruction_shift_left_logical_word(&mut self) {
        trace!("Executing SLLW...");

        let result32 = (self.context.decoder.source_register_1 as XLen32)
            << self.context.decoder.shamt;
        self.context.executor.result =
            result32 as SignedXLen32 as SignedXLen as XLen;
    }

    fn instruction_shift_right_logical_word(&mut self) {
        trace!("Executing SRLW...");

        let result32 = (self.context.decoder.source_register_1 as XLen32)
            >> self.context.decoder.shamt;
        self.context.executor.result =
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
        self.context.executor.result = result32 as SignedXLen as XLen;
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

        self.context.executor.return_target_address = self
            .context
            .control_status_registers
            .read(CSRRegisterName::mepc as usize);
    }

    // CSR instructions
    fn instruction_csr_read_clear(&mut self) {
        trace!("Executing CSRRC...");

        self.read_csr_to_result();
        if self.context.decoder.zicsr.need_to_write_csr {
            self.prepare_clear_bits_for_csr(
                self.context.decoder.source_register_1,
            );
        }
    }

    fn instruction_csr_read_clear_immediate(&mut self) {
        trace!("Executing CSRRCI...");

        self.read_csr_to_result();
        if self.context.decoder.zicsr.need_to_write_csr {
            self.prepare_clear_bits_for_csr(self.context.decoder.zicsr.uimm);
        }
    }

    fn instruction_csr_read_write_immediate(&mut self) {
        trace!("Executing CSRRWI...");

        if self.context.decoder.zicsr.need_to_read_csr {
            self.read_csr_to_result();
        }
        self.prepare_value_for_csr(self.context.decoder.zicsr.uimm);
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

    fn instruction_csr_read_set_immediate(&mut self) {
        trace!("Executing CSRRSI...");

        self.read_csr_to_result();
        if self.context.decoder.zicsr.need_to_write_csr {
            self.prepare_set_bits_for_csr(self.context.decoder.zicsr.uimm);
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

    fn read_csr_to_result(&mut self) {
        self.context.executor.result = self
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
}
