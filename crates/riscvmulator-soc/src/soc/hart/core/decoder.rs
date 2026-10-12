pub mod constants;

use self::constants::{
    BRANCH_Funct3, FenceMode, Funct3_OPCODE_LOAD, I_Funct12, JALR_Funct3,
    MISC_MEM_Funct3, OP_Funct7, OP_IMM_Funct3, OP_IMM_Funct7, OP_IMM_ShiftType,
    Opcode, OpcodeToInstructionFormat, STORE_Funct3, SYSTEM_Funct3,
    SYSTEM_Imm_transfer_control_return_from,
};
use crate::soc::hart::core::ArchState;
use crate::soc::hart::extensions;
use crate::soc::hart::extensions::integer::constants::{
    OP_32_Funct3, OP_IMM_32_Funct3,
};
use crate::soc::hart::privileged::machine::misa;

use super::executor::constants::ExecutableCommand;

use crate::constants::arch::*;
use crate::soc::hart::core::decoder::constants::InstructionSize;
use crate::soc::hart::core::fetch_unit::FetchUnit;
use crate::soc::hart::core::gp_registers::{
    RegisterFile, constants::GPRegisterName,
};
use crate::soc::hart::core::registers::RegisterFileInterface;
use crate::soc::hart::extensions::zicsr::DecoderZicsrContext;

use log::{error, trace};

#[derive(Default, Clone)]
pub struct Decoder {
    pub(crate) opcode: Opcode,
    pub(crate) source_register_1: XLen,
    pub(crate) source_register_2: XLen,
    pub(crate) source_register_1_offset: usize,
    pub(crate) source_register_2_offset: usize,
    pub(crate) destination_register_offset: usize,

    pub(crate) funct3: InstructionType,
    pub(crate) i_funct7: OP_IMM_Funct7,
    pub(crate) i_funct12: I_Funct12,
    pub(crate) r_funct7: OP_Funct7,

    pub(crate) shamt: InstructionType,
    pub(crate) shift_type: OP_IMM_ShiftType,

    // immediate sign-extends to XLEN
    pub(crate) immediate: SignedXLen,
    pub(crate) command_to_execute: ExecutableCommand,
    pub(crate) need_to_read_memory: bool,
    pub(crate) need_to_write_memory: bool,
    pub(crate) need_to_write_back: bool,
    pub(crate) need_to_jump: bool,

    pub(crate) fence: FENCE_Context,

    pub(crate) zicsr: DecoderZicsrContext,
}

impl Decoder {
    pub fn new() -> Self {
        Self::get_reset_instance()
    }

    pub fn reset(&mut self) {
        *self = Self::get_reset_instance();
    }

    fn get_reset_instance() -> Self {
        Self {
            source_register_1: RISCV_X0_ZERO_REGISTER_VALUE,
            source_register_2: RISCV_X0_ZERO_REGISTER_VALUE,
            source_register_1_offset: GPRegisterName::x0_zero as usize,
            source_register_2_offset: GPRegisterName::x0_zero as usize,
            destination_register_offset: GPRegisterName::x0_zero as usize,
            ..Default::default()
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Default, Clone)]
pub struct FENCE_Context {
    pub(crate) successor_mem_write: bool,
    pub(crate) successor_mem_read: bool,
    pub(crate) successor_device_output: bool,
    pub(crate) successor_device_input: bool,

    pub(crate) predecessor_mem_write: bool,
    pub(crate) predecessor_mem_read: bool,
    pub(crate) predecessor_device_output: bool,
    pub(crate) predecessor_device_input: bool,

    pub(crate) mode: FenceMode,
}

impl Decoder {
    pub(crate) fn decode(
        &mut self,
        arch_state: &ArchState,
        fetch_unit: &FetchUnit,
        // expander: &Expander,
    ) {
        match fetch_unit.instruction_size {
            InstructionSize::I32 => {
                self.decode_i_32(arch_state, fetch_unit.current_instruction);
            }
            InstructionSize::I16 => {
                // if expander.instruction_was_expanded {
                // instruction was expanded during `expand()` stage
                self.decode_i_32(arch_state, fetch_unit.current_instruction);
                // } else {
                //     unreachable!(
                //         "instruction was not expanded, something gone wrong"
                //     );
                // }
            }

            _ => {
                error!(
                    "TODO: decode of given instruction size {:?} is unimplemented",
                    fetch_unit.instruction_size
                );
                todo!(
                    "TODO: decode of given instruction size {:?} is unimplemented",
                    fetch_unit.instruction_size
                );
            }
        }
    }

    /// decode 32-bit instructions
    fn decode_i_32(
        &mut self,
        arch_state: &ArchState,
        instruction: InstructionType,
    ) {
        self.opcode = self.extract_opcode(instruction);

        match From::from(self.opcode) {
            OpcodeToInstructionFormat::R_51_OP
            | OpcodeToInstructionFormat::R_59_OP_32
            | OpcodeToInstructionFormat::R_83_OP_FP =>
            self.decode_r_format(arch_state, instruction, &arch_state.gp_register_file),
            OpcodeToInstructionFormat::I_3_LOAD
            | OpcodeToInstructionFormat::I_7_LOAD_FP
            | OpcodeToInstructionFormat::I_19_OP_IMM
            | OpcodeToInstructionFormat::I_27_OP_IMM_32
            | OpcodeToInstructionFormat::I_103_JALR
            | OpcodeToInstructionFormat::I_115_SYSTEM // (Zicsr_extension)
            | OpcodeToInstructionFormat::I_15_MISC_MEM // FENCE, Zifencei
            => self.decode_i_format(instruction, &arch_state.gp_register_file),
            OpcodeToInstructionFormat::S_35_STORE
            | OpcodeToInstructionFormat::S_39_STORE_FP => {
                self.decode_s_format(instruction, &arch_state.gp_register_file)
            }
            OpcodeToInstructionFormat::B_99_BRANCH => self.decode_b_format(instruction, &arch_state.gp_register_file),
            OpcodeToInstructionFormat::U_23_AUIPC
            | OpcodeToInstructionFormat::U_55_LUI => self.decode_u_format(instruction),
            OpcodeToInstructionFormat::J_111_JAL => self.decode_j_format(instruction),
            OpcodeToInstructionFormat::R4_67_MADD
            | OpcodeToInstructionFormat::R4_71_MSUB
            | OpcodeToInstructionFormat::R4_75_NMSUB
            | OpcodeToInstructionFormat::R4_79_NMADD => self.decode_r4_format(instruction),
        }
    }

    fn decode_r_format(
        &mut self,
        arch_state: &ArchState,
        instruction: InstructionType,
        register_file: &RegisterFile<RISCV_NUM_OF_REGISTERS>,
    ) {
        trace!("Decoding R-type instruction");

        self.source_register_1 =
            self.extract_source_register_1(instruction, register_file);
        self.source_register_2 =
            self.extract_source_register_2(instruction, register_file);
        self.destination_register_offset =
            self.extract_destination_register_offset(instruction);
        self.need_to_read_memory = false;
        self.need_to_write_memory = false;
        self.need_to_write_back = true;
        self.need_to_jump = false;
        self.zicsr.need_to_read_csr = false;
        self.zicsr.need_to_write_csr = false;
        self.zicsr.need_to_return_from_trap = false;
        self.zicsr.need_to_call_trap = false;
        self.funct3 = self.extract_funct3(instruction);
        self.r_funct7 = self.extract_r_funct7(instruction);

        match self.opcode {
            Opcode::OP => match self.r_funct7 {
                OP_Funct7::F0 => {
                    match extensions::integer::constants::OP_Funct3::from(
                        self.funct3,
                    ) {
                        extensions::integer::constants::OP_Funct3::ARITH => {
                            self.command_to_execute = ExecutableCommand::ADD;
                        }
                        extensions::integer::constants::OP_Funct3::SLL => {
                            self.shamt = self.extract_r_shamt();
                            self.command_to_execute = ExecutableCommand::SLL;
                        }
                        extensions::integer::constants::OP_Funct3::SLT => {
                            self.command_to_execute = ExecutableCommand::SLT;
                        }
                        extensions::integer::constants::OP_Funct3::SLTU => {
                            self.command_to_execute = ExecutableCommand::SLTU;
                        }
                        extensions::integer::constants::OP_Funct3::XOR => {
                            self.command_to_execute = ExecutableCommand::XOR;
                        }
                        extensions::integer::constants::OP_Funct3::SRX => {
                            self.shamt = self.extract_r_shamt();
                            self.command_to_execute = ExecutableCommand::SRL;
                        }
                        extensions::integer::constants::OP_Funct3::OR => {
                            self.command_to_execute = ExecutableCommand::OR;
                        }
                        extensions::integer::constants::OP_Funct3::AND => {
                            self.command_to_execute = ExecutableCommand::AND;
                        }
                    }
                }
                OP_Funct7::F1 => {
                    match extensions::integer::constants::OP_Funct3::from(
                        self.funct3,
                    ) {
                        extensions::integer::constants::OP_Funct3::ARITH => {
                            self.command_to_execute = ExecutableCommand::SUB;
                        }
                        extensions::integer::constants::OP_Funct3::SRX => {
                            self.shamt = self.extract_r_shamt();
                            self.command_to_execute = ExecutableCommand::SRA;
                        }

                        _ => {
                            error!("unreachable!");
                            unreachable!("unreachable!");
                        }
                    }
                }
                OP_Funct7::F_MULDIV => {
                    if misa::is_m_enabled(&arch_state.control_status_registers)
                    {
                        self.decode_m();
                    } else {
                        todo!("Illegal instruction trap");
                    }
                }
            },
            Opcode::OP_32 => match self.r_funct7 {
                OP_Funct7::F0 => match OP_32_Funct3::from(self.funct3) {
                    OP_32_Funct3::ARITHW => {
                        self.command_to_execute = ExecutableCommand::ADDW;
                    }
                    OP_32_Funct3::SLLW => {
                        self.shamt = self.extract_r_shamt_rv64w();
                        self.command_to_execute = ExecutableCommand::SLLW;
                    }
                    OP_32_Funct3::SRXW => {
                        self.shamt = self.extract_r_shamt_rv64w();
                        trace!("New shamt: {}", self.shamt);
                        self.command_to_execute = ExecutableCommand::SRLW;
                    }
                },
                OP_Funct7::F1 => match OP_32_Funct3::from(self.funct3) {
                    OP_32_Funct3::ARITHW => {
                        self.command_to_execute = ExecutableCommand::SUBW;
                    }
                    // // TODO: possibly duplicate, check spec
                    // OP_32_Funct3::SLLW => {
                    //     self.shamt = self.extract_r_shamt_rv64w();
                    //     self.command_to_execute = ExecutableCommand::SLLW;
                    // }
                    OP_32_Funct3::SRXW => {
                        self.shamt = self.extract_r_shamt_rv64w();
                        trace!("New shamt: {}", self.shamt);
                        self.command_to_execute = ExecutableCommand::SRAW;
                    }

                    _ => {
                        error!("unreachable!");
                        unreachable!("unreachable!");
                    }
                },
                OP_Funct7::F_MULDIV => {
                    if misa::is_m_enabled(&arch_state.control_status_registers)
                    {
                        self.decode_m_32();
                    } else {
                        todo!("Illegal instruction trap");
                    }
                }
            },

            _ => {
                error!("unreachable!");
                unreachable!("unreachable!");
            }
        }
    }

    fn decode_i_format(
        &mut self,
        instruction: InstructionType,
        register_file: &RegisterFile<RISCV_NUM_OF_REGISTERS>,
    ) {
        trace!("Decoding I-type instruction");

        self.source_register_1 =
            self.extract_source_register_1(instruction, register_file);
        self.destination_register_offset =
            self.extract_destination_register_offset(instruction);

        self.funct3 = self.extract_funct3(instruction);

        self.need_to_read_memory = false;
        self.need_to_write_memory = false;
        self.need_to_write_back = true;
        self.need_to_jump = false;
        self.zicsr.need_to_read_csr = false;
        self.zicsr.need_to_write_csr = false;
        self.zicsr.need_to_return_from_trap = false;
        self.zicsr.need_to_call_trap = false;

        match self.opcode {
            // I_3
            Opcode::LOAD => {
                self.immediate = self.extract_imm_11_0(instruction);
                self.need_to_read_memory = true;
                match Funct3_OPCODE_LOAD::from(self.funct3) {
                    Funct3_OPCODE_LOAD::LB => {
                        self.command_to_execute = ExecutableCommand::LB
                    }
                    Funct3_OPCODE_LOAD::LH => {
                        self.command_to_execute = ExecutableCommand::LH
                    }
                    Funct3_OPCODE_LOAD::LW => {
                        self.command_to_execute = ExecutableCommand::LW
                    }
                    Funct3_OPCODE_LOAD::LD => {
                        self.command_to_execute = ExecutableCommand::LD
                    }
                    Funct3_OPCODE_LOAD::LBU => {
                        self.command_to_execute = ExecutableCommand::LBU
                    }
                    Funct3_OPCODE_LOAD::LHU => {
                        self.command_to_execute = ExecutableCommand::LHU
                    }
                    Funct3_OPCODE_LOAD::LWU => {
                        self.command_to_execute = ExecutableCommand::LWU
                    }
                }
            }
            // I_15
            Opcode::MISC_MEM => {
                match MISC_MEM_Funct3::from(self.funct3) {
                    MISC_MEM_Funct3::FENCE => {
                        self.need_to_write_back = false;

                        self.fence.successor_mem_write =
                            self.extract_fence_sw(instruction);
                        self.fence.successor_mem_read =
                            self.extract_fence_sr(instruction);
                        self.fence.successor_device_output =
                            self.extract_fence_so(instruction);
                        self.fence.successor_device_input =
                            self.extract_fence_si(instruction);

                        self.fence.predecessor_mem_write =
                            self.extract_fence_pw(instruction);
                        self.fence.predecessor_mem_read =
                            self.extract_fence_pr(instruction);
                        self.fence.predecessor_device_output =
                            self.extract_fence_po(instruction);
                        self.fence.predecessor_device_input =
                            self.extract_fence_pi(instruction);

                        self.fence.mode = self.extract_fence_fm(instruction);

                        match self.fence.mode {
                            FenceMode::NORMAL => {
                                //
                                error!("TODO: FENCE.NORMAL is unimplemented");
                            }
                            FenceMode::TSO => {
                                //
                                error!("TODO: FENCE.TSO is unimplemented");
                            }
                        }

                        self.command_to_execute = ExecutableCommand::FENCE;
                    }

                    MISC_MEM_Funct3::FENCE_I => {
                        self.need_to_write_back = false;

                        let check_if_fence_i = || -> bool {
                            self.source_register_1_offset
                                == GPRegisterName::x0_zero as usize
                                && self.extract_destination_register_offset(
                                    instruction,
                                ) == GPRegisterName::x0_zero as usize
                                && self.i_funct12 == I_Funct12::F0
                        };

                        // The unused fields in the FENCE.I instruction,
                        // funct12, rs1, and rd, are reserved for finer-grain
                        // fences in future extensions. For forward
                        // compatibility, base implementations shall ignore
                        // these fields, and standard software shall zero these
                        // fields.
                        //if check_if_fence_i() {
                        self.command_to_execute = ExecutableCommand::FENCE_I;
                        //} else {
                        //    unreachable!();
                        //}
                    }
                }
            }
            // I_19
            Opcode::OP_IMM => {
                self.immediate = self.extract_imm_11_0(instruction);
                match OP_IMM_Funct3::from(self.funct3) {
                    OP_IMM_Funct3::ADDI => {
                        self.command_to_execute = ExecutableCommand::ADDI;
                    }
                    OP_IMM_Funct3::SLLI => {
                        self.shamt = self.extract_i_shamt(instruction);
                        self.shift_type = self.extract_shift_type(instruction);
                        match self.shift_type {
                            OP_IMM_ShiftType::SXLI => {
                                // funct3 equals SLLI
                                self.command_to_execute =
                                    ExecutableCommand::SLLI
                            }

                            _ => {
                                error!("unreachable!");
                                unreachable!("unreachable!");
                            }
                        }
                    }
                    OP_IMM_Funct3::SLTI => {
                        self.command_to_execute = ExecutableCommand::SLTI;
                    }
                    OP_IMM_Funct3::SLTIU => {
                        self.command_to_execute = ExecutableCommand::SLTIU;
                    }
                    OP_IMM_Funct3::XORI => {
                        self.command_to_execute = ExecutableCommand::XORI;
                    }
                    OP_IMM_Funct3::SRXI => {
                        self.shamt = self.extract_i_shamt(instruction);
                        self.shift_type = self.extract_shift_type(instruction);
                        match self.shift_type {
                            OP_IMM_ShiftType::SXLI => {
                                // funct3 equals SRLI
                                self.command_to_execute =
                                    ExecutableCommand::SRLI;
                            }
                            OP_IMM_ShiftType::SRAI => {
                                self.command_to_execute =
                                    ExecutableCommand::SRAI;
                            }
                        }
                    }
                    OP_IMM_Funct3::ORI => {
                        self.command_to_execute = ExecutableCommand::ORI;
                    }
                    OP_IMM_Funct3::ANDI => {
                        self.command_to_execute = ExecutableCommand::ANDI;
                    }
                }
            }
            // I_27
            Opcode::OP_IMM_32 => {
                self.immediate = self.extract_imm_11_0(instruction);
                match OP_IMM_32_Funct3::from(self.funct3) {
                    OP_IMM_32_Funct3::ADDIW => {
                        self.command_to_execute = ExecutableCommand::ADDIW
                    }
                    OP_IMM_32_Funct3::SLLIW => {
                        self.shamt = self.extract_i_shamt_rv64w(instruction);
                        self.i_funct7 = self.extract_i_funct7(instruction);
                        match self.i_funct7 {
                            OP_IMM_Funct7::SXLI => {
                                // funct3 equals SLLIW
                                self.command_to_execute =
                                    ExecutableCommand::SLLIW
                            }

                            _ => {
                                error!("unreachable!");
                                unreachable!("unreachable!");
                            }
                        }
                    }
                    OP_IMM_32_Funct3::SRXIW => {
                        self.shamt = self.extract_i_shamt_rv64w(instruction);
                        self.i_funct7 = self.extract_i_funct7(instruction);
                        match self.i_funct7 {
                            OP_IMM_Funct7::SXLI => {
                                // funct3 equals SRXIW
                                self.command_to_execute =
                                    ExecutableCommand::SRLIW
                            }
                            OP_IMM_Funct7::SRAI => {
                                // funct3 equals SRXIW
                                self.command_to_execute =
                                    ExecutableCommand::SRAIW
                            }
                        }
                    }
                }
            }
            // I_103
            Opcode::JALR => {
                self.immediate = self.extract_imm_11_0(instruction);
                match JALR_Funct3::from(self.funct3) {
                    JALR_Funct3::JALR => {
                        self.need_to_jump = true;
                        self.command_to_execute = ExecutableCommand::JALR
                    }
                }
            }

            // I_115 (Zicsr_extension)
            Opcode::SYSTEM => {
                self.immediate =
                    self.extract_csr_address(instruction) as XLen as SignedXLen;
                match SYSTEM_Funct3::from(self.funct3) {
                    SYSTEM_Funct3::TRANSFER_CONTROL_RETURN_FROM => {
                        self.need_to_write_back = false;
                        let are_rd_and_rs1_zero = || -> bool {
                            (self.source_register_1 == 0)
                                && (register_file
                                    .read(self.destination_register_offset)
                                    == 0)
                        };
                        let assert_rd_and_rs1_are_zero = || {
                            if !are_rd_and_rs1_zero() {
                                error!("rs1 and rd should be equal 0");
                                unreachable!("rs1 and rd should be equal 0");
                            }
                        };

                        assert_rd_and_rs1_are_zero();
                        match From::from(self.immediate) {
                            SYSTEM_Imm_transfer_control_return_from::ECALL => {
                                self.zicsr.need_to_call_trap = true;
                                self.command_to_execute =
                                    ExecutableCommand::ECALL;
                            }
                            SYSTEM_Imm_transfer_control_return_from::EBREAK => {
                                //
                                self.command_to_execute =
                                    ExecutableCommand::EBREAK;
                                todo!("TODO: unimplemented");
                            }
                            SYSTEM_Imm_transfer_control_return_from::URET => {
                                self.zicsr.need_to_return_from_trap = true;
                                self.command_to_execute =
                                    ExecutableCommand::URET;
                                todo!("TODO: unimplemented");
                            }
                            SYSTEM_Imm_transfer_control_return_from::SRET => {
                                self.zicsr.need_to_return_from_trap = true;
                                self.command_to_execute =
                                    ExecutableCommand::SRET;
                                todo!("TODO: unimplemented");
                            }
                            SYSTEM_Imm_transfer_control_return_from::MRET => {
                                self.zicsr.need_to_return_from_trap = true;
                                self.command_to_execute =
                                    ExecutableCommand::MRET;
                            }
                        }
                    }
                    // CSR instructions are atomic
                    SYSTEM_Funct3::CSRRW => {
                        self.zicsr.need_to_write_csr = true;
                        if self.destination_register_offset
                            != GPRegisterName::x0_zero as usize
                        {
                            self.zicsr.need_to_read_csr = true;
                        }
                        self.command_to_execute = ExecutableCommand::CSRRW;
                    }
                    SYSTEM_Funct3::CSRRS => {
                        self.zicsr.need_to_read_csr = true;
                        if self.extract_source_register_1_offset(instruction)
                            != 0
                        {
                            self.zicsr.need_to_write_csr = true;
                        }
                        self.command_to_execute = ExecutableCommand::CSRRS;
                    }
                    SYSTEM_Funct3::CSRRC => {
                        self.zicsr.need_to_read_csr = true;
                        if self.extract_source_register_1_offset(instruction)
                            != 0
                        {
                            self.zicsr.need_to_write_csr = true;
                        }
                        self.command_to_execute = ExecutableCommand::CSRRC;
                    }
                    SYSTEM_Funct3::CSRRWI => {
                        self.zicsr.need_to_write_csr = true;
                        self.zicsr.uimm = self
                            .extract_source_register_1_offset(instruction)
                            as XLen;
                        if self.destination_register_offset
                            != GPRegisterName::x0_zero as usize
                        {
                            self.zicsr.need_to_read_csr = true;
                        }
                        self.command_to_execute = ExecutableCommand::CSRRWI;
                    }
                    SYSTEM_Funct3::CSRRSI => {
                        self.zicsr.need_to_read_csr = true;
                        if self.extract_source_register_1_offset(instruction)
                            != 0
                        {
                            self.zicsr.need_to_write_csr = true;
                            self.zicsr.uimm = self
                                .extract_source_register_1_offset(instruction)
                                as XLen;
                        }
                        self.command_to_execute = ExecutableCommand::CSRRSI;
                    }
                    SYSTEM_Funct3::CSRRCI => {
                        self.zicsr.need_to_read_csr = true;
                        if self.extract_source_register_1_offset(instruction)
                            != 0
                        {
                            self.zicsr.need_to_write_csr = true;
                            self.zicsr.uimm = self
                                .extract_source_register_1_offset(instruction)
                                as XLen;
                        }
                        self.command_to_execute = ExecutableCommand::CSRRCI;
                    }
                }
            }

            _ => {
                error!("unreachable!");
                unreachable!("unreachable!");
            }
        }
    }

    fn decode_s_format(
        &mut self,
        instruction: InstructionType,
        register_file: &RegisterFile<RISCV_NUM_OF_REGISTERS>,
    ) {
        trace!("Decoding S-type instruction");

        self.source_register_1 =
            self.extract_source_register_1(instruction, register_file);
        self.source_register_2 =
            self.extract_source_register_2(instruction, register_file);
        self.need_to_read_memory = false;
        self.need_to_write_memory = true;
        self.need_to_write_back = false;
        self.need_to_jump = false;
        self.zicsr.need_to_read_csr = false;
        self.zicsr.need_to_write_csr = false;
        self.zicsr.need_to_return_from_trap = false;
        self.zicsr.need_to_call_trap = false;

        // extract immediate
        self.immediate = self.extract_imm_4_0(instruction);
        self.immediate |= self.extract_imm_11_5(instruction);

        self.funct3 = self.extract_funct3(instruction);

        match self.opcode {
            // S_35
            Opcode::STORE => match STORE_Funct3::from(self.funct3) {
                STORE_Funct3::SB => {
                    self.command_to_execute = ExecutableCommand::SB;
                }
                STORE_Funct3::SH => {
                    self.command_to_execute = ExecutableCommand::SH;
                }
                STORE_Funct3::SW => {
                    self.command_to_execute = ExecutableCommand::SW;
                }
                STORE_Funct3::SD => {
                    self.command_to_execute = ExecutableCommand::SD;
                }
            },

            _ => {
                error!("unreachable!");
                unreachable!("unreachable!");
            }
        };
    }

    fn decode_b_format(
        &mut self,
        instruction: InstructionType,
        register_file: &RegisterFile<RISCV_NUM_OF_REGISTERS>,
    ) {
        trace!("Decoding B-type instruction");

        self.source_register_1 =
            self.extract_source_register_1(instruction, register_file);
        self.source_register_2 =
            self.extract_source_register_2(instruction, register_file);
        self.need_to_read_memory = false;
        self.need_to_write_memory = false;
        self.need_to_write_back = false;
        self.need_to_jump = false;
        self.zicsr.need_to_read_csr = false;
        self.zicsr.need_to_write_csr = false;
        self.zicsr.need_to_return_from_trap = false;
        self.zicsr.need_to_call_trap = false;

        // extract immediate
        self.immediate = self.extract_imm_4_1(instruction);
        self.immediate |= self.extract_imm_10_5(instruction);
        self.immediate |= self.extract_b_11(instruction);
        self.immediate |= self.extract_b_12(instruction);

        self.funct3 = self.extract_funct3(instruction);

        match self.opcode {
            Opcode::BRANCH => match BRANCH_Funct3::from(self.funct3) {
                BRANCH_Funct3::BEQ => {
                    self.command_to_execute = ExecutableCommand::BEQ
                }
                BRANCH_Funct3::BNE => {
                    self.command_to_execute = ExecutableCommand::BNE
                }
                BRANCH_Funct3::BLT => {
                    self.command_to_execute = ExecutableCommand::BLT
                }
                BRANCH_Funct3::BGE => {
                    self.command_to_execute = ExecutableCommand::BGE
                }
                BRANCH_Funct3::BLTU => {
                    self.command_to_execute = ExecutableCommand::BLTU
                }
                BRANCH_Funct3::BGEU => {
                    self.command_to_execute = ExecutableCommand::BGEU
                }
            },

            _ => {
                error!("unreachable!");
                unreachable!("unreachable!");
            }
        }
    }

    fn decode_u_format(&mut self, instruction: InstructionType) {
        trace!("Decoding U-type instruction");

        self.destination_register_offset =
            self.extract_destination_register_offset(instruction);
        self.immediate = self.extract_imm_31_12(instruction);

        self.need_to_read_memory = false;
        self.need_to_write_memory = false;
        self.need_to_write_back = true;
        self.need_to_jump = false;
        self.zicsr.need_to_read_csr = false;
        self.zicsr.need_to_write_csr = false;
        self.zicsr.need_to_return_from_trap = false;
        self.zicsr.need_to_call_trap = false;

        match self.opcode {
            Opcode::AUIPC => {
                self.command_to_execute = ExecutableCommand::AUIPC;
            }
            Opcode::LUI => {
                self.command_to_execute = ExecutableCommand::LUI;
            }

            _ => {
                error!("unreachable!");
                unreachable!("unreachable!");
            }
        };
    }

    fn decode_j_format(&mut self, instruction: InstructionType) {
        trace!("Decoding J-type instruction");

        self.destination_register_offset =
            self.extract_destination_register_offset(instruction);
        self.need_to_read_memory = false;
        self.need_to_write_memory = false;
        self.need_to_write_back = true;
        self.need_to_jump = true;
        self.zicsr.need_to_read_csr = false;
        self.zicsr.need_to_write_csr = false;
        self.zicsr.need_to_return_from_trap = false;
        self.zicsr.need_to_call_trap = false;

        // extract immediate
        self.immediate = self.extract_imm_10_1(instruction);
        self.immediate |= self.extract_j_11(instruction);
        self.immediate |= self.extract_imm_19_12(instruction);
        self.immediate |= self.extract_j_20(instruction);

        match self.opcode {
            Opcode::JAL => {
                self.command_to_execute = ExecutableCommand::JAL;
            }

            _ => {
                error!("unreachable!");
                unreachable!("unreachable!");
            }
        };
    }

    fn decode_r4_format(&mut self, instruction: InstructionType) {
        trace!("Decoding R4-type instruction");

        //
        match self.opcode {
            //
            _ => match self.funct3 {
                //
                _ => {
                    todo!("TODO: unimplemented");
                }
            },
        };
    }

    fn extract_opcode(&self, instruction: InstructionType) -> Opcode {
        Opcode::from(((instruction >> BIT_OFFSET_OPCODE) & MASK_OPCODE) as u8)
    }

    fn extract_source_register_1_offset(
        &mut self,
        instruction: InstructionType,
    ) -> usize {
        self.source_register_1_offset =
            ((instruction >> BIT_OFFSET_SOURCE_REGISTER_1)
                & MASK_SOURCE_REGISTER_1) as usize;

        self.source_register_1_offset
    }

    fn extract_source_register_1(
        &mut self,
        instruction: InstructionType,
        register_file: &RegisterFile<RISCV_NUM_OF_REGISTERS>,
    ) -> XLen {
        register_file.read(self.extract_source_register_1_offset(instruction))
    }

    fn extract_source_register_2_offset(
        &mut self,
        instruction: InstructionType,
    ) -> usize {
        self.source_register_2_offset =
            ((instruction >> BIT_OFFSET_SOURCE_REGISTER_2)
                & MASK_SOURCE_REGISTER_2) as usize;

        self.source_register_2_offset
    }

    fn extract_source_register_2(
        &mut self,
        instruction: InstructionType,
        register_file: &RegisterFile<RISCV_NUM_OF_REGISTERS>,
    ) -> XLen {
        register_file.read(self.extract_source_register_2_offset(instruction))
    }

    fn extract_destination_register_offset(
        &self,
        instruction: InstructionType,
    ) -> usize {
        ((instruction >> BIT_OFFSET_DESTINATION_REGISTER)
            & MASK_DESTINATION_REGISTER) as usize
    }

    // fn extract_destination_register(
    //     &self,
    //     instruction: InstructionType,
    //     register_file: &RegisterFile<RISCV_NUM_OF_REGISTERS>,
    // ) -> XLen {
    //     register_file
    //         .read(self.extract_destination_register_offset(instruction))
    // }

    fn extract_funct3(&self, instruction: InstructionType) -> InstructionType {
        (instruction >> BIT_OFFSET_FUNCT3) & MASK_FUNCT3
    }

    fn extract_i_funct7(&self, instruction: InstructionType) -> OP_IMM_Funct7 {
        From::from(
            (instruction >> BIT_OFFSET_R_FUNCT7_RV64W) & MASK_R_FUNCT7_RV64W,
        )
    }

    fn extract_shift_type(
        &self,
        instruction: InstructionType,
    ) -> OP_IMM_ShiftType {
        From::from((instruction >> BIT_OFFSET_SHTYPE) & MASK_SHTYPE)
    }

    fn extract_r_funct7(&self, instruction: InstructionType) -> OP_Funct7 {
        From::from((instruction >> BIT_OFFSET_R_FUNCT7) & MASK_R_FUNCT7)
    }

    fn extract_r_shamt(&self) -> InstructionType {
        (self.source_register_2 & (MASK_SHAMT as XLen)) as InstructionType
    }

    fn extract_r_shamt_rv64w(&self) -> InstructionType {
        (self.source_register_2 & (MASK_SHAMT_RV64W as XLen)) as InstructionType
    }

    fn extract_i_shamt(&self, instruction: InstructionType) -> InstructionType {
        (instruction >> BIT_OFFSET_SHAMT) & MASK_SHAMT
    }

    fn extract_i_shamt_rv64w(
        &self,
        instruction: InstructionType,
    ) -> InstructionType {
        (instruction >> BIT_OFFSET_SHAMT) & MASK_SHAMT_RV64W
    }

    fn extract_imm_11_0(&self, instruction: InstructionType) -> SignedXLen {
        // right shift for signed values compiles to arithmetic shift right
        // (ASR) with sign extension
        (instruction as SignedInstructionType
            >> (BIT_OFFSET_IMM_11_0 - BIT_OFFSET_IMM_11_0_FINAL))
            as SignedXLen32 as SignedXLen
    }

    fn extract_fence_sw(&self, instruction: InstructionType) -> bool {
        (instruction & MASK_I_FENCE_SW) != 0
    }

    fn extract_fence_sr(&self, instruction: InstructionType) -> bool {
        (instruction & MASK_I_FENCE_SR) != 0
    }

    fn extract_fence_so(&self, instruction: InstructionType) -> bool {
        (instruction & MASK_I_FENCE_SO) != 0
    }

    fn extract_fence_si(&self, instruction: InstructionType) -> bool {
        (instruction & MASK_I_FENCE_SI) != 0
    }

    fn extract_fence_pw(&self, instruction: InstructionType) -> bool {
        (instruction & MASK_I_FENCE_PW) != 0
    }

    fn extract_fence_pr(&self, instruction: InstructionType) -> bool {
        (instruction & MASK_I_FENCE_PR) != 0
    }

    fn extract_fence_po(&self, instruction: InstructionType) -> bool {
        (instruction & MASK_I_FENCE_PO) != 0
    }

    fn extract_fence_pi(&self, instruction: InstructionType) -> bool {
        (instruction & MASK_I_FENCE_PI) != 0
    }

    fn extract_fence_fm(&self, instruction: InstructionType) -> FenceMode {
        FenceMode::from(
            (instruction & MASK_I_FENCE_FM) >> BIT_OFFSET_I_FENCE_FM,
        )
    }

    fn extract_csr_address(
        &self,
        instruction: InstructionType,
    ) -> UnsignedImmediate {
        instruction >> (BIT_OFFSET_IMM_11_0 - BIT_OFFSET_IMM_11_0_FINAL)
    }

    fn extract_imm_11_5(&self, instruction: InstructionType) -> SignedXLen {
        // right shift for signed values compiles to arithmetic shift right
        // (ASR) with sign extension
        (instruction as SignedInstructionType
            >> (BIT_OFFSET_IMM_11_5 - BIT_OFFSET_IMM_11_5_FINAL))
            as SignedXLen32 as SignedXLen
            & !((1 << BIT_OFFSET_IMM_11_5_FINAL) - 1)
    }

    fn extract_imm_4_0(&self, instruction: InstructionType) -> SignedXLen {
        ((instruction >> (BIT_OFFSET_IMM_4_0 - BIT_OFFSET_IMM_4_0_FINAL))
            & MASK_IMM_4_0) as SignedXLen32 as SignedXLen
    }

    fn extract_imm_31_12(&self, instruction: InstructionType) -> SignedXLen {
        // right shift for signed values compiles to arithmetic shift right
        // (ASR) with sign extension
        (instruction as SignedInstructionType
            >> (BIT_OFFSET_IMM_31_12 - BIT_OFFSET_IMM_31_12_FINAL))
            as SignedXLen32 as SignedXLen
            & !((1 << BIT_OFFSET_IMM_31_12_FINAL) - 1)
    }

    fn extract_imm_10_5(&self, instruction: InstructionType) -> SignedXLen {
        ((instruction >> (BIT_OFFSET_IMM_10_5 - BIT_OFFSET_IMM_10_5_FINAL))
            & MASK_IMM_10_5) as SignedXLen32 as SignedXLen
    }

    fn extract_imm_4_1(&self, instruction: InstructionType) -> SignedXLen {
        ((instruction >> (BIT_OFFSET_IMM_4_1 - BIT_OFFSET_IMM_4_1_FINAL))
            & MASK_IMM_4_1) as SignedXLen32 as SignedXLen
    }

    fn extract_imm_10_1(&self, instruction: InstructionType) -> SignedXLen {
        ((instruction >> (BIT_OFFSET_IMM_10_1 - BIT_OFFSET_IMM_10_1_FINAL))
            & MASK_IMM_10_1) as SignedXLen32 as SignedXLen
    }

    fn extract_imm_19_12(&self, instruction: InstructionType) -> SignedXLen {
        ((instruction >> (BIT_OFFSET_IMM_19_12 - BIT_OFFSET_IMM_19_12_FINAL))
            & MASK_IMM_19_12) as SignedXLen32 as SignedXLen
    }

    fn extract_b_11(&self, instruction: InstructionType) -> SignedXLen {
        ((instruction << (BIT_OFFSET_B_11_FINAL - BIT_OFFSET_B_11)) & MASK_B_11)
            as SignedXLen32 as SignedXLen
    }

    fn extract_b_12(&self, instruction: InstructionType) -> SignedXLen {
        // right shift for signed values compiles to arithmetic shift right
        // (ASR) with sign extension
        (instruction as SignedInstructionType
            >> (BIT_OFFSET_B_12 - BIT_OFFSET_B_12_FINAL))
            as SignedXLen32 as SignedXLen
            & !((1 << BIT_OFFSET_B_12_FINAL) - 1)
    }

    fn extract_j_11(&self, instruction: InstructionType) -> SignedXLen {
        ((instruction >> (BIT_OFFSET_J_11 - BIT_OFFSET_J_11_FINAL)) & MASK_J_11)
            as SignedXLen32 as SignedXLen
    }

    fn extract_j_20(&self, instruction: InstructionType) -> SignedXLen {
        // right shift for signed values compiles to arithmetic shift right
        // (ASR) with sign extension
        ((instruction as SignedInstructionType)
            >> (BIT_OFFSET_J_20 - BIT_OFFSET_J_20_FINAL))
            as SignedXLen32 as SignedXLen
            & !((1 << BIT_OFFSET_J_20_FINAL) - 1)
    }
}
