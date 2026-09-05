pub mod constants;

use self::constants::ExecutableCommand;

use super::ArchState;
use super::decoder::Decoder;
use super::fetch_unit::FetchUnit;
use super::gp_registers::Register;
use super::registers::{RegisterFileInterface, RegisterInterface};

use crate::constants::arch::{
    Byte, DoubleWord, HalfWord, RISCV_NUM_OF_CSR, SignedImmediate, SignedXLen,
    SignedXLen32, UnsignedImmediate, UnsignedXLen, Word, XLen, XLen32,
};
use crate::soc::hart::extensions::zicsr::constants::CSRRegisterName;
use crate::soc::hart::extensions::zicsr::{CSRFile, ExecutorZicsrContext};
use crate::soc::hart::privileged::constants::{
    MCause_Code, MSTATUS_MPP_MASK, MSTATUS_MPP_OFFSET, MStatus_MPP,
};
use crate::soc::hart::privileged::machine::misa;
use crate::soc::memory::{MemoryValueType, MemoryValueTypeEnum};

use log::{debug, error, trace};

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
    pub(crate) memory_value_type: MemoryValueTypeEnum,

    pub(crate) need_to_branch: bool,

    pub(crate) zicsr: ExecutorZicsrContext,
}

impl Executor {
    pub fn new(pc_reset_value: XLen) -> Self {
        Self::get_reset_instance(pc_reset_value)
    }

    pub fn reset(&mut self, pc_reset_value: XLen) {
        *self = Self::get_reset_instance(pc_reset_value);
    }

    fn get_reset_instance(pc_reset_value: XLen) -> Self {
        Self {
            result: XLen::MIN,
            branch_target_address: pc_reset_value,
            jump_target_address: pc_reset_value,
            return_target_address: pc_reset_value,
            memory_address: XLen::MIN,
            value_to_store_u8: Default::default(),
            value_to_store_u16: Default::default(),
            value_to_store_u32: Default::default(),
            value_to_store_u64: Default::default(),
            memory_value_type: MemoryValueTypeEnum::DOUBLE_WORD,
            need_to_branch: false,
            zicsr: ExecutorZicsrContext {
                csr_value: Default::default(),
                trap_cause: Default::default(),
                trap_vector: Default::default(),
                epc: Default::default(),
            },
        }
    }
}

impl Executor {
    fn dispatch(
        &mut self,
        decoder: &Decoder,
        control_status_registers: &CSRFile<RISCV_NUM_OF_CSR>,
        pc: &Register,
        fetch_unit: &FetchUnit,
    ) {
        match decoder.command_to_execute {
            // RV32I
            // I_3
            ExecutableCommand::LB => self.instruction_load_byte(decoder),
            ExecutableCommand::LH => self.instruction_load_half(decoder),
            ExecutableCommand::LW => self.instruction_load_word(decoder),
            ExecutableCommand::LBU => {
                self.instruction_load_byte_unsigned(decoder)
            }
            ExecutableCommand::LHU => {
                self.instruction_load_half_unsigned(decoder)
            }
            // I_19
            ExecutableCommand::ADDI => self.instruction_addi(decoder),
            ExecutableCommand::SLLI => self.instruction_slli(decoder),
            ExecutableCommand::SLTI => self.instruction_slti(decoder),
            ExecutableCommand::SLTIU => self.instruction_sltiu(decoder),
            ExecutableCommand::XORI => self.instruction_xori(decoder),
            ExecutableCommand::SRLI => self.instruction_srli(decoder),
            ExecutableCommand::SRAI => self.instruction_srai(decoder),
            ExecutableCommand::ORI => self.instruction_ori(decoder),
            ExecutableCommand::ANDI => self.instruction_andi(decoder),
            // U_23
            ExecutableCommand::AUIPC => self.instruction_auipc(decoder, pc),
            // S_35
            ExecutableCommand::SB => self.instruction_store_byte(decoder),
            ExecutableCommand::SH => self.instruction_store_half_word(decoder),
            ExecutableCommand::SW => self.instruction_store_word(decoder),
            // R_51
            ExecutableCommand::ADD => self.instruction_add(decoder),
            ExecutableCommand::SUB => self.instruction_sub(decoder),
            ExecutableCommand::SLL => self.instruction_sll(decoder),
            ExecutableCommand::SLT => self.instruction_slt(decoder),
            ExecutableCommand::SLTU => self.instruction_sltu(decoder),
            ExecutableCommand::XOR => self.instruction_xor(decoder),
            ExecutableCommand::SRL => self.instruction_srl(decoder),
            ExecutableCommand::SRA => self.instruction_sra(decoder),
            ExecutableCommand::OR => self.instruction_or(decoder),
            ExecutableCommand::AND => self.instruction_and(decoder),
            // U_55
            ExecutableCommand::LUI => self.instruction_lui(decoder),
            // B_99
            ExecutableCommand::BEQ => self.instruction_beq(decoder, pc),
            ExecutableCommand::BNE => self.instruction_bne(decoder, pc),
            ExecutableCommand::BLT => self.instruction_blt(decoder, pc),
            ExecutableCommand::BGE => self.instruction_bge(decoder, pc),
            ExecutableCommand::BLTU => self.instruction_bltu(decoder, pc),
            ExecutableCommand::BGEU => self.instruction_bgeu(decoder, pc),
            // I_103
            ExecutableCommand::JALR => {
                self.instruction_jalr(decoder, pc, fetch_unit)
            }
            // J_111
            ExecutableCommand::JAL => {
                self.instruction_jal(decoder, pc, fetch_unit)
            }

            // RV64I
            // I_3
            ExecutableCommand::LD => self.instruction_load_double_word(decoder),
            ExecutableCommand::LWU => {
                self.instruction_load_word_unsigned(decoder)
            }
            // I_27
            ExecutableCommand::ADDIW => {
                self.instruction_add_immediate_word(decoder);
            }
            ExecutableCommand::SLLIW => {
                self.instruction_shift_left_logical_immediate_word(decoder);
            }
            ExecutableCommand::SRLIW => {
                self.instruction_shift_right_logical_immediate_word(decoder);
            }
            ExecutableCommand::SRAIW => {
                self.instruction_shift_right_arith_immediate_word(decoder);
            }
            // S_35
            ExecutableCommand::SD => {
                self.instruction_store_double_word(decoder)
            }
            // R_59
            ExecutableCommand::ADDW => self.instruction_add_word(decoder),
            ExecutableCommand::SUBW => self.instruction_sub_word(decoder),
            ExecutableCommand::SLLW => {
                self.instruction_shift_left_logical_word(decoder);
            }
            ExecutableCommand::SRLW => {
                self.instruction_shift_right_logical_word(decoder);
            }
            ExecutableCommand::SRAW => {
                self.instruction_shift_right_arith_word(decoder);
            }

            ExecutableCommand::FENCE => {
                self.instruction_fence(decoder);
            }

            ExecutableCommand::FENCE_I => {
                self.instruction_fence_i(decoder);
            }

            ExecutableCommand::ECALL => {
                self.instruction_execution_call(control_status_registers, pc);
            }
            ExecutableCommand::EBREAK => {
                self.instruction_execution_break(decoder);
            }

            ExecutableCommand::URET => {
                self.instruction_user_return(decoder);
            }
            ExecutableCommand::SRET => {
                self.instruction_supervisor_return(decoder);
            }
            ExecutableCommand::MRET => {
                self.instruction_machine_return(control_status_registers);
            }

            // Zicsr extension
            ExecutableCommand::CSRRW => {
                self.instruction_csr_read_write(
                    decoder,
                    control_status_registers,
                );
            }
            ExecutableCommand::CSRRS => {
                self.instruction_csr_read_set(
                    decoder,
                    control_status_registers,
                );
            }
            ExecutableCommand::CSRRC => {
                self.instruction_csr_read_clear(
                    decoder,
                    control_status_registers,
                );
            }
            ExecutableCommand::CSRRWI => {
                self.instruction_csr_read_write_immediate(
                    decoder,
                    control_status_registers,
                );
            }
            ExecutableCommand::CSRRSI => {
                self.instruction_csr_read_set_immediate(
                    decoder,
                    control_status_registers,
                );
            }
            ExecutableCommand::CSRRCI => {
                self.instruction_csr_read_clear_immediate(
                    decoder,
                    control_status_registers,
                );
            }

            /// `M` extension instructoins
            ExecutableCommand::MUL => {
                self.instruction_multiply_low(decoder);
            }
            ExecutableCommand::MULH => {
                self.instruction_multiply_high(decoder);
            }
            ExecutableCommand::MULHSU => {
                self.instruction_multiply_high_signed_unsigned(decoder);
            }
            ExecutableCommand::MULHU => {
                self.instruction_multiply_high_unsigned(decoder);
            }
            ExecutableCommand::DIV => {
                self.instruction_divide(decoder);
            }
            ExecutableCommand::DIVU => {
                self.instruction_divide_unsigned(decoder);
            }
            ExecutableCommand::REM => {
                self.instruction_remainder(decoder);
            }
            ExecutableCommand::REMU => {
                self.instruction_remainder_unsigned(decoder);
            }
            // M32
            ExecutableCommand::MULW => {
                self.instruction_multiply_low_word(decoder);
            }
            ExecutableCommand::DIVW => {
                self.instruction_divide_word(decoder);
            }
            ExecutableCommand::DIVUW => {
                self.instruction_divide_unsigned_word(decoder);
            }
            ExecutableCommand::REMW => {
                self.instruction_remainder_word(decoder);
            }
            ExecutableCommand::REMUW => {
                self.instruction_remainder_unsigned_word(decoder);
            }

            /// `C` extension instructions
            // CIW
            ExecutableCommand::C_ADDI4SPN => {
                self.instruction_add_immediate_to_stack_pointer_compressed(
                    decoder,
                );
            }
            // CL
            ExecutableCommand::C_FLD => {
                self.instruction_load_double_precision_fp_compressed(decoder);
            }
            ExecutableCommand::C_LW => {
                self.instruction_load_word_compressed(decoder);
            }
            ExecutableCommand::C_FLW => {
                self.instruction_load_single_precision_fp_compressed(decoder);
            }
            ExecutableCommand::C_LD => {
                self.instruction_load_double_word_compressed(decoder);
            }
            // CS
            ExecutableCommand::C_FSD => {
                self.instruction_store_double_precision_fp_compressed(decoder);
            }
            ExecutableCommand::C_SW => {
                self.instruction_store_word_compressed(decoder);
            }
            ExecutableCommand::C_FSW => {
                self.instruction_store_single_precision_fp_compressed(decoder);
            }
            ExecutableCommand::C_SD => {
                self.instruction_store_double_word_compressed(decoder);
            }
            ExecutableCommand::C_AND => {
                self.instruction_and_compressed(decoder);
            }
            ExecutableCommand::C_OR => {
                self.instruction_or_compressed(decoder);
            }
            ExecutableCommand::C_XOR => {
                self.instruction_xor_compressed(decoder);
            }
            ExecutableCommand::C_SUB => {
                self.instruction_sub_compressed(decoder);
            }
            ExecutableCommand::C_ADDW => {
                self.instruction_add_word_compressed(decoder);
            }
            ExecutableCommand::C_SUBW => {
                self.instruction_sub_word_compressed(decoder);
            }
            ExecutableCommand::C_BEQZ => {
                self.instruction_branch_if_equals_zero_compressed(decoder);
            }
            ExecutableCommand::C_BNEZ => {
                self.instruction_branch_if_not_equals_zero_compressed(decoder);
            }
            ExecutableCommand::C_SRLI => {
                self.instruction_logical_shift_right_compressed(decoder);
            }
            ExecutableCommand::C_SRAI => {
                self.instruction_arithmetic_shift_right_compressed(decoder);
            }
            ExecutableCommand::C_ANDI => {
                self.instruction_and_immediate_compressed(decoder);
            }
            ExecutableCommand::C_J => {
                self.instruction_jump_compressed(decoder);
            }
            ExecutableCommand::C_JAL => {
                self.instruction_jal_compressed(decoder);
            }
            ExecutableCommand::C_JR => {
                self.instruction_jump_to_addr_in_register_compressed(decoder);
            }
            ExecutableCommand::C_JALR => {
                self.instruction_jalr_compressed(decoder);
            }
            ExecutableCommand::C_MV => {
                self.instruction_move_compressed(decoder);
            }
            ExecutableCommand::C_ADD => {
                self.instruction_add_compressed(decoder);
            }
            ExecutableCommand::C_EBREAK => {
                self.instruction_ebreak_compressed(decoder);
            }
            ExecutableCommand::C_LWSP => {
                self.instruction_load_word_sp_compressed(decoder);
            }
            ExecutableCommand::C_LDSP => {
                self.instruction_load_double_word_sp_compressed(decoder);
            }
            ExecutableCommand::C_FLWSP => {
                self.instruction_load_single_precision_fp_sp_compressed(
                    decoder,
                );
            }
            ExecutableCommand::C_FLDSP => {
                self.instruction_load_double_precision_fp_sp_compressed(
                    decoder,
                );
            }
            ExecutableCommand::C_LI => {
                self.instruction_load_immediate_compressed(decoder);
            }
            ExecutableCommand::C_LUI => {
                self.instruction_load_upper_immediate_compressed(decoder);
            }
            ExecutableCommand::C_ADDI => {
                self.instruction_add_immediate_compressed(decoder);
            }
            ExecutableCommand::C_ADDIW => {
                self.instruction_add_immediate_to_word_compressed(decoder);
            }
            ExecutableCommand::C_ADDI16SP => {
                self.instruction_add_immediate_16_to_sp_compressed(decoder);
            }
            ExecutableCommand::C_SLLI => {
                self.instruction_logical_shift_left_compressed(decoder);
            }
            ExecutableCommand::C_NOP => {
                self.instruction_nop_compressed(decoder);
            }
            ExecutableCommand::C_SWSP => {
                self.instruction_store_word_sp_compressed(decoder);
            }
            ExecutableCommand::C_SDSP => {
                self.instruction_store_double_word_sp_compressed(decoder);
            }
            ExecutableCommand::C_FSWSP => {
                self.instruction_store_single_precision_fp_sp_compressed(
                    decoder,
                );
            }
            ExecutableCommand::C_FSDSP => {
                self.instruction_store_double_precision_fp_compressed(decoder);
            }
        }
    }

    pub(in crate::soc::hart) fn execute(
        &mut self,
        decoder: &Decoder,
        arch_state: &ArchState,
        fetch_unit: &FetchUnit,
    ) {
        // default executor context
        self.need_to_branch = false;

        self.dispatch(
            decoder,
            &arch_state.control_status_registers,
            &arch_state.PC,
            fetch_unit,
        );
    }

    // use wrapping operation in order to allow overruns
    fn update_memory_address(&mut self, decoder: &Decoder) {
        self.memory_address = XLen::wrapping_add(
            decoder.source_register_1,
            decoder.immediate.sign_extend(),
        );
    }

    fn instruction_load_byte(&mut self, decoder: &Decoder) {
        debug!("Executing LB...");

        self.update_memory_address(decoder);
        self.memory_value_type = MemoryValueTypeEnum::BYTE;
    }

    fn instruction_load_half(&mut self, decoder: &Decoder) {
        debug!("Executing LH...");

        self.update_memory_address(decoder);
        self.memory_value_type = MemoryValueTypeEnum::HALF_WORD;
    }

    fn instruction_load_word(&mut self, decoder: &Decoder) {
        debug!("Executing LW...");

        self.update_memory_address(decoder);
        self.memory_value_type = MemoryValueTypeEnum::WORD;
    }

    fn instruction_load_byte_unsigned(&mut self, decoder: &Decoder) {
        debug!("Executing LBU...");

        self.update_memory_address(decoder);
        self.memory_value_type = MemoryValueTypeEnum::UNSIGNED_BYTE;
    }

    fn instruction_load_half_unsigned(&mut self, decoder: &Decoder) {
        debug!("Executing LHU...");

        self.update_memory_address(decoder);
        self.memory_value_type = MemoryValueTypeEnum::UNSIGNED_HALF_WORD;
    }

    fn instruction_addi(&mut self, decoder: &Decoder) {
        debug!("Executing ADDI...");

        // use wrapping operation in order to allow overruns
        self.result = XLen::wrapping_add(
            decoder.source_register_1,
            decoder.immediate.sign_extend(),
        );
    }

    fn instruction_slli(&mut self, decoder: &Decoder) {
        debug!("Executing SLLI...");

        self.result = decoder.source_register_1 << decoder.shamt;
    }

    fn instruction_slti(&mut self, decoder: &Decoder) {
        debug!("Executing SLTI...");

        self.result = if (decoder.source_register_1 as SignedXLen)
            < (decoder.immediate as SignedImmediate as SignedXLen)
        {
            1
        } else {
            0
        };
    }

    fn instruction_sltiu(&mut self, decoder: &Decoder) {
        debug!("Executing SLTIU...");

        self.result = if (decoder.source_register_1 as UnsignedXLen)
            < decoder.immediate.sign_extend()
        {
            1
        } else {
            0
        };
    }

    fn instruction_xori(&mut self, decoder: &Decoder) {
        debug!("Executing XORI...");

        self.result =
            decoder.source_register_1 ^ decoder.immediate.sign_extend();
    }

    fn instruction_srli(&mut self, decoder: &Decoder) {
        debug!("Executing SRLI...");

        self.result = decoder.source_register_1 >> decoder.shamt;
    }

    fn instruction_srai(&mut self, decoder: &Decoder) {
        debug!("Executing SRAI...");

        self.result = ((decoder.source_register_1 as SignedXLen)
            >> decoder.shamt) as XLen;
    }

    fn instruction_ori(&mut self, decoder: &Decoder) {
        debug!("Executing ORI...");

        self.result =
            decoder.source_register_1 | decoder.immediate.sign_extend();
    }

    fn instruction_andi(&mut self, decoder: &Decoder) {
        debug!("Executing ANDI...");

        self.result =
            decoder.source_register_1 & decoder.immediate.sign_extend();
    }

    fn instruction_auipc(&mut self, decoder: &Decoder, pc: &Register) {
        debug!("Executing AUIPC...");

        self.result =
            XLen::wrapping_add(pc.read(), decoder.immediate.sign_extend());
    }

    fn instruction_store_byte(&mut self, decoder: &Decoder) {
        debug!("Executing SB...");

        self.update_memory_address(decoder);
        self.value_to_store_u8 = decoder.source_register_2 as Byte;
        self.memory_value_type = MemoryValueTypeEnum::BYTE;
    }

    fn instruction_store_half_word(&mut self, decoder: &Decoder) {
        debug!("Executing SH...");

        self.update_memory_address(decoder);
        self.value_to_store_u16 = decoder.source_register_2 as HalfWord;
        self.memory_value_type = MemoryValueTypeEnum::HALF_WORD;
    }

    fn instruction_store_word(&mut self, decoder: &Decoder) {
        debug!("Executing SW...");

        self.update_memory_address(decoder);
        self.value_to_store_u32 = decoder.source_register_2 as Word;
        self.memory_value_type = MemoryValueTypeEnum::WORD;
    }

    fn instruction_add(&mut self, decoder: &Decoder) {
        debug!("Executing ADD...");

        self.result = XLen::wrapping_add(
            decoder.source_register_1,
            decoder.source_register_2,
        );
    }

    fn instruction_sub(&mut self, decoder: &Decoder) {
        debug!("Executing SUB...");

        self.result = XLen::wrapping_sub(
            decoder.source_register_1,
            decoder.source_register_2,
        );
    }

    fn instruction_sll(&mut self, decoder: &Decoder) {
        debug!("Executing SLL...");

        self.result = XLen::wrapping_shl(
            decoder.source_register_1,
            decoder.source_register_2 as UnsignedImmediate,
        );
    }

    fn instruction_slt(&mut self, decoder: &Decoder) {
        debug!("Executing SLT...");

        self.result = if (decoder.source_register_1 as SignedXLen)
            < (decoder.source_register_2 as SignedXLen)
        {
            1
        } else {
            0
        }
    }

    fn instruction_sltu(&mut self, decoder: &Decoder) {
        debug!("Executing SLTU...");

        self.result = if decoder.source_register_1 < decoder.source_register_2 {
            1
        } else {
            0
        }
    }

    fn instruction_xor(&mut self, decoder: &Decoder) {
        debug!("Executing XOR...");

        self.result = decoder.source_register_1 ^ decoder.source_register_2;
    }

    fn instruction_srl(&mut self, decoder: &Decoder) {
        debug!("Executing SRL...");

        self.result = XLen::wrapping_shr(
            decoder.source_register_1,
            decoder.source_register_2 as UnsignedImmediate,
        );
    }

    fn instruction_sra(&mut self, decoder: &Decoder) {
        debug!("Executing SRA...");

        self.result = SignedXLen::wrapping_shr(
            decoder.source_register_1 as SignedXLen,
            decoder.source_register_2 as UnsignedImmediate,
        ) as XLen;
    }

    fn instruction_or(&mut self, decoder: &Decoder) {
        debug!("Executing OR...");

        self.result = decoder.source_register_1 | decoder.source_register_2;
    }

    fn instruction_and(&mut self, decoder: &Decoder) {
        debug!("Executing AND...");

        self.result = decoder.source_register_1 & decoder.source_register_2;
    }

    fn instruction_lui(&mut self, decoder: &Decoder) {
        debug!("Executing LUI...");

        self.result = decoder.immediate.sign_extend();
    }

    fn instruction_beq(&mut self, decoder: &Decoder, pc: &Register) {
        debug!("Executing BEQ...");

        if decoder.source_register_1 == decoder.source_register_2 {
            self.branch_target_address =
                XLen::wrapping_add(pc.read(), decoder.immediate.sign_extend());
            self.need_to_branch = true;
        }
    }

    fn instruction_bne(&mut self, decoder: &Decoder, pc: &Register) {
        trace!(
            "Executing BNE: {:#018X}, {:#018X}...",
            decoder.source_register_1, decoder.source_register_2
        );

        if decoder.source_register_1 != decoder.source_register_2 {
            // use wrapping operation in order to allow overruns
            self.branch_target_address =
                XLen::wrapping_add(pc.read(), decoder.immediate.sign_extend());
            self.need_to_branch = true;
        } else {
            trace!("Skip BNE...");
        }
    }

    fn instruction_blt(&mut self, decoder: &Decoder, pc: &Register) {
        debug!("Executing BLT...");

        if (decoder.source_register_1 as SignedXLen)
            < (decoder.source_register_2 as SignedXLen)
        {
            self.branch_target_address =
                XLen::wrapping_add(pc.read(), decoder.immediate.sign_extend());
            self.need_to_branch = true;
        }
    }

    fn instruction_bge(&mut self, decoder: &Decoder, pc: &Register) {
        debug!("Executing BGE...");

        if (decoder.source_register_1 as SignedXLen)
            >= (decoder.source_register_2 as SignedXLen)
        {
            self.branch_target_address =
                XLen::wrapping_add(pc.read(), decoder.immediate.sign_extend());
            self.need_to_branch = true;
        }
    }

    fn instruction_bltu(&mut self, decoder: &Decoder, pc: &Register) {
        debug!("Executing BLTU...");

        if (decoder.source_register_1 as UnsignedXLen)
            < (decoder.source_register_2 as UnsignedXLen)
        {
            self.branch_target_address =
                XLen::wrapping_add(pc.read(), decoder.immediate.sign_extend());
            self.need_to_branch = true;
        }
    }

    fn instruction_bgeu(&mut self, decoder: &Decoder, pc: &Register) {
        debug!("Executing BGEU...");

        if (decoder.source_register_1 as UnsignedXLen)
            >= (decoder.source_register_2 as UnsignedXLen)
        {
            self.branch_target_address =
                XLen::wrapping_add(pc.read(), decoder.immediate.sign_extend());
            self.need_to_branch = true;
        }
    }

    fn instruction_jalr(
        &mut self,
        decoder: &Decoder,
        pc: &Register,
        fetch_unit: &FetchUnit,
    ) {
        debug!("Executing JALR...");

        self.jump_target_address = (XLen::wrapping_add(
            decoder.source_register_1,
            decoder.immediate.sign_extend(),
        )) & !(XLen::MAX & 0x01);
        self.result =
            XLen::wrapping_add(pc.read(), fetch_unit.instruction_size as XLen);
    }

    fn instruction_jal(
        &mut self,
        decoder: &Decoder,
        pc: &Register,
        fetch_unit: &FetchUnit,
    ) {
        debug!("Executing JAL...");

        self.jump_target_address =
            XLen::wrapping_add(pc.read(), decoder.immediate.sign_extend());
        self.result = pc.read() + fetch_unit.instruction_size as XLen;
    }

    fn instruction_load_double_word(&mut self, decoder: &Decoder) {
        debug!("Executing LD...");

        self.update_memory_address(decoder);
        self.memory_value_type = MemoryValueTypeEnum::DOUBLE_WORD;
    }

    fn instruction_load_word_unsigned(&mut self, decoder: &Decoder) {
        debug!("Executing LWU...");

        self.update_memory_address(decoder);
        self.memory_value_type = MemoryValueTypeEnum::UNSIGNED_WORD;
    }

    fn instruction_add_immediate_word(&mut self, decoder: &Decoder) {
        debug!("Executing ADDIW...");

        // use wrapping operation in order to allow overruns
        let result32 = XLen32::wrapping_add(
            decoder.source_register_1 as XLen32,
            decoder.immediate,
        );
        self.result = result32.sign_extend();
    }

    fn instruction_shift_left_logical_immediate_word(
        &mut self,
        decoder: &Decoder,
    ) {
        debug!("Executing SLLIW...");

        let result32 =
            (decoder.source_register_1 as XLen32) << decoder.immediate;
        self.result = result32.sign_extend();
    }

    fn instruction_shift_right_logical_immediate_word(
        &mut self,
        decoder: &Decoder,
    ) {
        debug!("Executing SRLIW...");

        let result32 =
            (decoder.source_register_1 as XLen32) >> decoder.immediate;
        self.result = result32.sign_extend();
    }

    fn instruction_shift_right_arith_immediate_word(
        &mut self,
        decoder: &Decoder,
    ) {
        trace!(
            "Executing SRAIW: {:#018X} >> {}...",
            decoder.source_register_1 as SignedXLen32, decoder.immediate
        );

        let result32 = SignedXLen32::wrapping_shr(
            decoder.source_register_1 as SignedXLen32,
            decoder.immediate,
        ) as XLen32;
        self.result = result32.sign_extend();
    }

    fn instruction_store_double_word(&mut self, decoder: &Decoder) {
        debug!("Executing SD...");

        self.update_memory_address(decoder);
        self.value_to_store_u64 = decoder.source_register_2;
        self.memory_value_type = MemoryValueTypeEnum::DOUBLE_WORD;
    }

    fn instruction_add_word(&mut self, decoder: &Decoder) {
        debug!("Executing ADDW...");

        let result32 = SignedXLen32::wrapping_add(
            decoder.source_register_1 as SignedXLen32,
            decoder.source_register_2 as SignedXLen32,
        ) as XLen32;
        self.result = result32.sign_extend();
    }

    fn instruction_sub_word(&mut self, decoder: &Decoder) {
        debug!("Executing SUBW...");

        let result32 = SignedXLen32::wrapping_sub(
            decoder.source_register_1 as SignedXLen32,
            decoder.source_register_2 as SignedXLen32,
        ) as XLen32;
        self.result = result32.sign_extend();
    }

    fn instruction_shift_left_logical_word(&mut self, decoder: &Decoder) {
        debug!("Executing SLLW...");

        let result32 = (decoder.source_register_1 as XLen32) << decoder.shamt;
        self.result = result32.sign_extend();
    }

    fn instruction_shift_right_logical_word(&mut self, decoder: &Decoder) {
        debug!("Executing SRLW...");

        let result32 = (decoder.source_register_1 as XLen32) >> decoder.shamt;
        self.result = result32.sign_extend();
    }

    fn instruction_shift_right_arith_word(&mut self, decoder: &Decoder) {
        trace!(
            "Executing SRAW: {:#018} >> {:?}...",
            decoder.source_register_1 as SignedXLen32, decoder.shamt
        );

        let result32 = ((decoder.source_register_1 as SignedXLen32)
            >> decoder.shamt) as XLen32;
        self.result = result32.sign_extend();
    }

    fn instruction_fence(&mut self, decoder: &Decoder) {
        debug!("Executing FENCE...");

        //
        error!("TODO: unimplemented instruction");
    }

    fn instruction_fence_i(&mut self, decoder: &Decoder) {
        debug!("Executing FENCE_I...");

        //
        error!("TODO: unimplemented instruction");
    }

    fn instruction_execution_call(
        &mut self,
        control_status_registers: &CSRFile<RISCV_NUM_OF_CSR>,
        pc: &Register,
    ) {
        debug!("Executing ECALL...");

        let current_privilege_mode =
            self.read_mstatus_mpp(control_status_registers);
        match current_privilege_mode {
            MStatus_MPP::USER => {
                self.zicsr.trap_cause =
                    MCause_Code::EXCEPTION_ENV_CALL_FROM_U_MODE;
                self.zicsr.trap_vector = control_status_registers
                    .read(CSRRegisterName::mtvec as usize);
            }
            MStatus_MPP::SUPERVISOR => {
                self.zicsr.trap_cause =
                    MCause_Code::EXCEPTION_ENV_CALL_FROM_S_MODE;
                todo!();
            }
            MStatus_MPP::MACHINE => {
                self.zicsr.trap_cause =
                    MCause_Code::EXCEPTION_ENV_CALL_FROM_M_MODE;
                self.zicsr.trap_vector = control_status_registers
                    .read(CSRRegisterName::mtvec as usize);
            }

            _ => {
                error!("unimplemented!");
                todo!("unimplemented!");
            }
        }
        self.zicsr.epc = pc.read();
    }

    fn instruction_execution_break(&mut self, decoder: &Decoder) {
        debug!("Executing EBREAK...");

        //
        todo!();
    }

    fn instruction_user_return(&mut self, decoder: &Decoder) {
        debug!("Executing URET...");

        //
        todo!();
    }

    fn instruction_supervisor_return(&mut self, decoder: &Decoder) {
        debug!("Executing SRET...");

        //
        todo!();
    }

    fn instruction_machine_return(
        &mut self,
        control_status_registers: &CSRFile<RISCV_NUM_OF_CSR>,
    ) {
        debug!("Executing MRET...");

        self.return_target_address =
            control_status_registers.read(CSRRegisterName::mepc as usize);
    }

    // CSR instructions
    fn instruction_csr_read_clear(
        &mut self,
        decoder: &Decoder,
        control_status_registers: &CSRFile<RISCV_NUM_OF_CSR>,
    ) {
        debug!("Executing CSRRC...");

        self.read_csr_to_result(decoder, control_status_registers);
        if decoder.zicsr.need_to_write_csr {
            self.prepare_clear_bits_for_csr(decoder.source_register_1);
        }
    }

    fn instruction_csr_read_clear_immediate(
        &mut self,
        decoder: &Decoder,
        control_status_registers: &CSRFile<RISCV_NUM_OF_CSR>,
    ) {
        debug!("Executing CSRRCI...");

        self.read_csr_to_result(decoder, control_status_registers);
        if decoder.zicsr.need_to_write_csr {
            self.prepare_clear_bits_for_csr(decoder.zicsr.uimm);
        }
    }

    fn instruction_csr_read_write_immediate(
        &mut self,
        decoder: &Decoder,
        control_status_registers: &CSRFile<RISCV_NUM_OF_CSR>,
    ) {
        debug!("Executing CSRRWI...");

        if decoder.zicsr.need_to_read_csr {
            self.read_csr_to_result(decoder, control_status_registers);
        }
        self.prepare_value_for_csr(decoder.zicsr.uimm);
    }

    fn instruction_csr_read_write(
        &mut self,
        decoder: &Decoder,
        control_status_registers: &CSRFile<RISCV_NUM_OF_CSR>,
    ) {
        debug!("Executing CSRRW...");

        if decoder.zicsr.need_to_read_csr {
            self.read_csr_to_result(decoder, control_status_registers);
        }
        self.prepare_value_for_csr(decoder.source_register_1);
    }

    fn instruction_csr_read_set(
        &mut self,
        decoder: &Decoder,
        control_status_registers: &CSRFile<RISCV_NUM_OF_CSR>,
    ) {
        debug!("Executing CSRRS...");

        self.read_csr_to_result(decoder, control_status_registers);
        if decoder.zicsr.need_to_write_csr {
            self.prepare_set_bits_for_csr(decoder.source_register_1);
        }
    }

    fn instruction_csr_read_set_immediate(
        &mut self,
        decoder: &Decoder,
        control_status_registers: &CSRFile<RISCV_NUM_OF_CSR>,
    ) {
        debug!("Executing CSRRSI...");

        self.read_csr_to_result(decoder, control_status_registers);
        if decoder.zicsr.need_to_write_csr {
            self.prepare_set_bits_for_csr(decoder.zicsr.uimm);
        }
    }

    // privileged mode extractors
    fn read_mstatus_mpp(
        &self,
        control_status_registers: &CSRFile<RISCV_NUM_OF_CSR>,
    ) -> MStatus_MPP {
        let mstatus =
            control_status_registers.read(CSRRegisterName::mstatus as usize);
        MStatus_MPP::from((mstatus >> MSTATUS_MPP_OFFSET) & MSTATUS_MPP_MASK)
    }

    fn read_csr_to_result(
        &mut self,
        decoder: &Decoder,
        control_status_registers: &CSRFile<RISCV_NUM_OF_CSR>,
    ) {
        self.result = control_status_registers.read(decoder.immediate as usize);
    }

    fn prepare_value_for_csr(&mut self, value: XLen) {
        self.zicsr.csr_value = value;
    }

    fn prepare_set_bits_for_csr(&mut self, value: XLen) {
        self.zicsr.csr_value |= value;
    }

    fn prepare_clear_bits_for_csr(&mut self, value: XLen) {
        self.zicsr.csr_value &= !value;
    }
}
