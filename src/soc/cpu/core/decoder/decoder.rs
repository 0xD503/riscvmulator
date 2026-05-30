use super::super::executor::constants::ExecutableCommand;
use super::constants::{
    BRANCH_Funct3, FenceMode, Funct7, I_Funct12, JALR_Funct3, MISC_MEM_Funct3,
    OP_32_Funct3, OP_Funct3, OP_IMM_32_Funct3, OP_IMM_Funct3, OP_IMM_Funct7,
    OP_IMM_ShiftType, OPCODE_LOAD_Funct3, Opcode, OpcodeToInstructionFormat,
    STORE_Funct3, SYSTEM_Funct3, SYSTEM_Imm_transfer_control_return_from,
};

use crate::constants::arch::*;
use crate::soc::cpu::core::gp_registers::constants::GPRegisterName;
use crate::soc::cpu::core::registers::RegisterFileInterface;
use crate::soc::cpu::cpu::RISC_V_CPU;
use crate::soc::cpu::extensions::zicsr::ZicsrContext;

use log::{debug, error, trace};

#[derive(Default, Clone)]
pub struct Decoder {
    pub(crate) opcode: Opcode,
    pub(crate) source_register_1: XLen,
    pub(crate) source_register_2: XLen,
    pub(crate) destination_register_offset: usize,

    pub(crate) funct3: InstructionType,
    pub(crate) i_funct7: OP_IMM_Funct7,
    pub(crate) i_funct12: I_Funct12,
    pub(crate) r_funct7: Funct7,

    pub(crate) shamt: InstructionType,
    pub(crate) shift_type: OP_IMM_ShiftType,

    pub(crate) immediate: UnsignedImmediate,
    pub(crate) command_to_execute: ExecutableCommand,
    pub(crate) need_to_read_memory: bool,
    pub(crate) need_to_write_memory: bool,
    pub(crate) need_to_write_back: bool,
    pub(crate) need_to_jump: bool,
    pub(crate) need_to_branch: bool,

    pub(crate) fence: FENCE_Context,

    pub(crate) zicsr: ZicsrContext,
}

impl Decoder {
    pub fn reset(&mut self) {
        *self = Default::default();
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

impl RISC_V_CPU {
    fn decode_r_format(&mut self) {
        trace!("Decoding R-type instruction");

        self.context.decoder.source_register_1 =
            self.extract_source_register_1();
        self.context.decoder.source_register_2 =
            self.extract_source_register_2();
        self.context.decoder.destination_register_offset =
            self.extract_destination_register_index();
        self.context.decoder.need_to_read_memory = false;
        self.context.decoder.need_to_write_memory = false;
        self.context.decoder.need_to_write_back = true;
        self.context.decoder.need_to_jump = false;
        self.context.decoder.need_to_branch = false;
        self.context.decoder.zicsr.need_to_read_csr = false;
        self.context.decoder.zicsr.need_to_write_csr = false;
        self.context.decoder.zicsr.need_to_return_from_trap = false;
        self.context.decoder.zicsr.need_to_call_trap = false;
        self.context.decoder.funct3 = self.extract_funct3();
        self.context.decoder.r_funct7 = self.extract_r_funct7();

        match self.context.decoder.opcode {
            Opcode::OP => match OP_Funct3::from(self.context.decoder.funct3) {
                OP_Funct3::ARITH => match self.context.decoder.r_funct7 {
                    Funct7::F0 => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::ADD;
                    }
                    Funct7::F1 => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::SUB;
                    }
                },
                OP_Funct3::SLL => match self.context.decoder.r_funct7 {
                    Funct7::F0 => {
                        self.context.decoder.shamt = self.extract_r_shamt();
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::SLL;
                    }

                    _ => {
                        error!("unreachable!");
                        unreachable!("unreachable!");
                    }
                },
                OP_Funct3::SLT => match self.context.decoder.r_funct7 {
                    Funct7::F0 => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::SLT;
                    }

                    _ => {
                        error!("unreachable!");
                        unreachable!("unreachable!");
                    }
                },
                OP_Funct3::SLTU => match self.context.decoder.r_funct7 {
                    Funct7::F0 => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::SLTU;
                    }

                    _ => {
                        error!("unreachable!");
                        unreachable!("unreachable!");
                    }
                },
                OP_Funct3::XOR => match self.context.decoder.r_funct7 {
                    Funct7::F0 => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::XOR;
                    }

                    _ => {
                        error!("unreachable!");
                        unreachable!("unreachable!");
                    }
                },
                OP_Funct3::SRX => {
                    self.context.decoder.shamt = self.extract_r_shamt();
                    match self.context.decoder.r_funct7 {
                        Funct7::F0 => {
                            self.context.decoder.command_to_execute =
                                ExecutableCommand::SRL;
                        }
                        Funct7::F1 => {
                            self.context.decoder.command_to_execute =
                                ExecutableCommand::SRA;
                        }
                    }
                }
                OP_Funct3::OR => match self.context.decoder.r_funct7 {
                    Funct7::F0 => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::OR;
                    }

                    _ => {
                        error!("unreachable!");
                        unreachable!("unreachable!");
                    }
                },
                OP_Funct3::AND => match self.context.decoder.r_funct7 {
                    Funct7::F0 => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::AND;
                    }

                    _ => {
                        error!("unreachable!");
                        unreachable!("unreachable!");
                    }
                },
            },
            Opcode::OP_32 => {
                match OP_32_Funct3::from(self.context.decoder.funct3) {
                    OP_32_Funct3::ARITHW => {
                        match self.context.decoder.r_funct7 {
                            Funct7::F0 => {
                                self.context.decoder.command_to_execute =
                                    ExecutableCommand::ADDW;
                            }
                            Funct7::F1 => {
                                self.context.decoder.command_to_execute =
                                    ExecutableCommand::SUBW;
                            }
                        }
                    }
                    OP_32_Funct3::SLLW => {
                        self.context.decoder.shamt =
                            self.extract_r_shamt_rv64w();
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::SLLW;
                    }
                    OP_32_Funct3::SRXW => {
                        self.context.decoder.shamt =
                            self.extract_r_shamt_rv64w();
                        debug!("New shamt: {}", self.context.decoder.shamt);
                        match self.context.decoder.r_funct7 {
                            Funct7::F0 => {
                                self.context.decoder.command_to_execute =
                                    ExecutableCommand::SRLW;
                            }
                            Funct7::F1 => {
                                self.context.decoder.command_to_execute =
                                    ExecutableCommand::SRAW;
                            }
                        }
                    }
                }
            }

            _ => {
                error!("unreachable!");
                unreachable!("unreachable!");
            }
        }
    }

    fn decode_i_format(&mut self) {
        trace!("Decoding I-type instruction");

        self.context.decoder.source_register_1 =
            self.extract_source_register_1();
        self.context.decoder.destination_register_offset =
            self.extract_destination_register_index();

        self.context.decoder.funct3 = self.extract_funct3();

        self.context.decoder.need_to_read_memory = false;
        self.context.decoder.need_to_write_memory = false;
        self.context.decoder.need_to_write_back = true;
        self.context.decoder.need_to_jump = false;
        self.context.decoder.need_to_branch = false;
        self.context.decoder.zicsr.need_to_read_csr = false;
        self.context.decoder.zicsr.need_to_write_csr = false;
        self.context.decoder.zicsr.need_to_return_from_trap = false;
        self.context.decoder.zicsr.need_to_call_trap = false;

        match self.context.decoder.opcode {
            // I_3
            Opcode::LOAD => {
                self.context.decoder.immediate =
                    self.extract_imm_11_0() as UnsignedImmediate;
                self.context.decoder.need_to_read_memory = true;
                match OPCODE_LOAD_Funct3::from(self.context.decoder.funct3) {
                    OPCODE_LOAD_Funct3::LB => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::LB
                    }
                    OPCODE_LOAD_Funct3::LH => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::LH
                    }
                    OPCODE_LOAD_Funct3::LW => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::LW
                    }
                    OPCODE_LOAD_Funct3::LD => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::LD
                    }
                    OPCODE_LOAD_Funct3::LBU => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::LBU
                    }
                    OPCODE_LOAD_Funct3::LHU => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::LHU
                    }
                    OPCODE_LOAD_Funct3::LWU => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::LWU
                    }
                }
            }
            // I_15
            Opcode::MISC_MEM => {
                match MISC_MEM_Funct3::from(self.context.decoder.funct3) {
                    MISC_MEM_Funct3::FENCE => {
                        self.context.decoder.need_to_write_back = false;

                        self.context.decoder.fence.successor_mem_write =
                            self.extract_fence_sw();
                        self.context.decoder.fence.successor_mem_read =
                            self.extract_fence_sr();
                        self.context.decoder.fence.successor_device_output =
                            self.extract_fence_so();
                        self.context.decoder.fence.successor_device_input =
                            self.extract_fence_si();

                        self.context.decoder.fence.predecessor_mem_write =
                            self.extract_fence_pw();
                        self.context.decoder.fence.predecessor_mem_read =
                            self.extract_fence_pr();
                        self.context.decoder.fence.predecessor_device_output =
                            self.extract_fence_po();
                        self.context.decoder.fence.predecessor_device_input =
                            self.extract_fence_pi();

                        self.context.decoder.fence.mode =
                            self.extract_fence_fm();

                        match self.context.decoder.fence.mode {
                            FenceMode::NORMAL => {
                                //
                                error!("TODO: FENCE.NORMAL is unimplemented");
                            }
                            FenceMode::TSO => {
                                //
                                error!("TODO: FENCE.TSO is unimplemented");
                            }
                        }

                        self.context.decoder.command_to_execute =
                            ExecutableCommand::FENCE;
                    }

                    MISC_MEM_Funct3::FENCE_I => {
                        self.context.decoder.need_to_write_back = false;

                        let check_if_fence_i = || -> bool {
                            self.context.decoder.source_register_1 == 0
                                && self.extract_destination_register() == 0
                                && self.context.decoder.i_funct12
                                    == I_Funct12::F0
                        };

                        if check_if_fence_i() {
                            self.context.decoder.command_to_execute =
                                ExecutableCommand::FENCE_I;
                        } else {
                            unreachable!();
                        }
                    }
                }
            }
            // I_19
            Opcode::OP_IMM => {
                self.context.decoder.immediate =
                    self.extract_imm_11_0() as UnsignedImmediate;
                match OP_IMM_Funct3::from(self.context.decoder.funct3) {
                    OP_IMM_Funct3::ADDI => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::ADDI;
                    }
                    OP_IMM_Funct3::SLLI => {
                        self.context.decoder.shamt = self.extract_i_shamt();
                        self.context.decoder.shift_type =
                            self.extract_shift_type();
                        match self.context.decoder.shift_type {
                            OP_IMM_ShiftType::SXLI => {
                                // funct3 equals SLLI
                                self.context.decoder.command_to_execute =
                                    ExecutableCommand::SLLI
                            }

                            _ => {
                                error!("unreachable!");
                                unreachable!("unreachable!");
                            }
                        }
                    }
                    OP_IMM_Funct3::SLTI => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::SLTI;
                    }
                    OP_IMM_Funct3::SLTIU => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::SLTIU;
                    }
                    OP_IMM_Funct3::XORI => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::XORI;
                    }
                    OP_IMM_Funct3::SRXI => {
                        self.context.decoder.shamt = self.extract_i_shamt();
                        self.context.decoder.shift_type =
                            self.extract_shift_type();
                        match self.context.decoder.shift_type {
                            OP_IMM_ShiftType::SXLI => {
                                // funct3 equals SRLI
                                self.context.decoder.command_to_execute =
                                    ExecutableCommand::SRLI;
                            }
                            OP_IMM_ShiftType::SRAI => {
                                self.context.decoder.command_to_execute =
                                    ExecutableCommand::SRAI;
                            }
                        }
                    }
                    OP_IMM_Funct3::ORI => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::ORI;
                    }
                    OP_IMM_Funct3::ANDI => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::ANDI;
                    }
                }
            }
            // I_27
            Opcode::OP_IMM_32 => {
                self.context.decoder.immediate =
                    self.extract_imm_11_0() as UnsignedImmediate;
                match OP_IMM_32_Funct3::from(self.context.decoder.funct3) {
                    OP_IMM_32_Funct3::ADDIW => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::ADDIW
                    }
                    OP_IMM_32_Funct3::SLLIW => {
                        self.context.decoder.shamt =
                            self.extract_i_shamt_rv64w();
                        self.context.decoder.i_funct7 = self.extract_i_funct7();
                        match self.context.decoder.i_funct7 {
                            OP_IMM_Funct7::SXLI => {
                                // funct3 equals SLLIW
                                self.context.decoder.command_to_execute =
                                    ExecutableCommand::SLLIW
                            }

                            _ => {
                                error!("unreachable!");
                                unreachable!("unreachable!");
                            }
                        }
                    }
                    OP_IMM_32_Funct3::SRXIW => {
                        self.context.decoder.shamt =
                            self.extract_i_shamt_rv64w();
                        self.context.decoder.i_funct7 = self.extract_i_funct7();
                        match self.context.decoder.i_funct7 {
                            OP_IMM_Funct7::SXLI => {
                                // funct3 equals SRXIW
                                self.context.decoder.command_to_execute =
                                    ExecutableCommand::SRLIW
                            }
                            OP_IMM_Funct7::SRAI => {
                                // funct3 equals SRXIW
                                self.context.decoder.command_to_execute =
                                    ExecutableCommand::SRAIW
                            }
                        }
                    }
                }
            }
            // I_103
            Opcode::JALR => {
                self.context.decoder.immediate =
                    self.extract_imm_11_0() as UnsignedImmediate;
                match JALR_Funct3::from(self.context.decoder.funct3) {
                    JALR_Funct3::JALR => {
                        self.context.decoder.need_to_jump = true;
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::JALR
                    }
                }
            }

            // I_115 (Zicsr_extension)
            Opcode::SYSTEM => {
                self.context.decoder.immediate = self.extract_csr_address();
                match SYSTEM_Funct3::from(self.context.decoder.funct3) {
                    SYSTEM_Funct3::TRANSFER_CONTROL_RETURN_FROM => {
                        self.context.decoder.need_to_write_back = false;
                        let are_rd_and_rs1_zero = || -> bool {
                            (self.context.decoder.source_register_1 == 0)
                                && (self.context.gp_register_file.read(
                                    self.context
                                        .decoder
                                        .destination_register_offset,
                                ) == 0)
                        };
                        let assert_rd_and_rs1_are_zero = || {
                            if !are_rd_and_rs1_zero() {
                                error!("rs1 and rd should be equal 0");
                                unreachable!("rs1 and rd should be equal 0");
                            }
                        };

                        assert_rd_and_rs1_are_zero();
                        match From::from(self.context.decoder.immediate) {
                            SYSTEM_Imm_transfer_control_return_from::ECALL => {
                                self.context.decoder.zicsr.need_to_call_trap =
                                    true;
                                self.context.decoder.command_to_execute =
                                    ExecutableCommand::ECALL;
                            }
                            SYSTEM_Imm_transfer_control_return_from::EBREAK => {
                                //
                                self.context.decoder.command_to_execute =
                                    ExecutableCommand::EBREAK;
                                todo!();
                            }
                            SYSTEM_Imm_transfer_control_return_from::URET => {
                                self.context
                                    .decoder
                                    .zicsr
                                    .need_to_return_from_trap = true;
                                self.context.decoder.command_to_execute =
                                    ExecutableCommand::URET;
                                todo!();
                            }
                            SYSTEM_Imm_transfer_control_return_from::SRET => {
                                self.context
                                    .decoder
                                    .zicsr
                                    .need_to_return_from_trap = true;
                                self.context.decoder.command_to_execute =
                                    ExecutableCommand::SRET;
                                todo!();
                            }
                            SYSTEM_Imm_transfer_control_return_from::MRET => {
                                self.context
                                    .decoder
                                    .zicsr
                                    .need_to_return_from_trap = true;
                                self.context.decoder.command_to_execute =
                                    ExecutableCommand::MRET;
                            }
                        }
                    }
                    // CSR instructions are atomic
                    SYSTEM_Funct3::CSRRW => {
                        self.context.decoder.zicsr.need_to_write_csr = true;
                        if self.context.decoder.destination_register_offset
                            != GPRegisterName::x0_zero as usize
                        {
                            self.context.decoder.zicsr.need_to_read_csr = true;
                        }
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::CSRRW;
                    }
                    SYSTEM_Funct3::CSRRS => {
                        self.context.decoder.zicsr.need_to_read_csr = true;
                        if self.extract_source_register_1_index() != 0 {
                            self.context.decoder.zicsr.need_to_write_csr = true;
                        }
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::CSRRS;
                    }
                    SYSTEM_Funct3::CSRRC => {
                        self.context.decoder.zicsr.need_to_read_csr = true;
                        if self.extract_source_register_1_index() != 0 {
                            self.context.decoder.zicsr.need_to_write_csr = true;
                        }
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::CSRRC;
                    }
                    SYSTEM_Funct3::CSRRWI => {
                        self.context.decoder.zicsr.need_to_write_csr = true;
                        self.context.decoder.zicsr.uimm =
                            self.extract_source_register_1_index() as XLen;
                        if self.context.decoder.destination_register_offset
                            != GPRegisterName::x0_zero as usize
                        {
                            self.context.decoder.zicsr.need_to_read_csr = true;
                        }
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::CSRRWI;
                    }
                    SYSTEM_Funct3::CSRRSI => {
                        self.context.decoder.zicsr.need_to_read_csr = true;
                        if self.extract_source_register_1_index() != 0 {
                            self.context.decoder.zicsr.need_to_write_csr = true;
                            self.context.decoder.zicsr.uimm =
                                self.extract_source_register_1_index() as XLen;
                        }
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::CSRRSI;
                    }
                    SYSTEM_Funct3::CSRRCI => {
                        self.context.decoder.zicsr.need_to_read_csr = true;
                        if self.extract_source_register_1_index() != 0 {
                            self.context.decoder.zicsr.need_to_write_csr = true;
                            self.context.decoder.zicsr.uimm =
                                self.extract_source_register_1_index() as XLen;
                        }
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::CSRRCI;
                    }
                }
            }

            _ => {
                error!("unreachable!");
                unreachable!("unreachable!");
            }
        }
    }

    fn decode_s_format(&mut self) {
        trace!("Decoding S-type instruction");

        self.context.decoder.source_register_1 =
            self.extract_source_register_1();
        self.context.decoder.source_register_2 =
            self.extract_source_register_2();
        self.context.decoder.need_to_read_memory = false;
        self.context.decoder.need_to_write_memory = true;
        self.context.decoder.need_to_write_back = false;
        self.context.decoder.need_to_jump = false;
        self.context.decoder.need_to_branch = false;
        self.context.decoder.zicsr.need_to_read_csr = false;
        self.context.decoder.zicsr.need_to_write_csr = false;
        self.context.decoder.zicsr.need_to_return_from_trap = false;
        self.context.decoder.zicsr.need_to_call_trap = false;

        // extract immediate
        self.context.decoder.immediate = self.extract_imm_4_0();
        self.context.decoder.immediate |=
            self.extract_imm_11_5() as UnsignedImmediate;

        self.context.decoder.funct3 = self.extract_funct3();

        match self.context.decoder.opcode {
            // S_35
            Opcode::STORE => {
                match STORE_Funct3::from(self.context.decoder.funct3) {
                    STORE_Funct3::SB => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::SB;
                    }
                    STORE_Funct3::SH => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::SH;
                    }
                    STORE_Funct3::SW => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::SW;
                    }
                    STORE_Funct3::SD => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::SD;
                    }
                }
            }

            _ => {
                error!("unreachable!");
                unreachable!("unreachable!");
            }
        };
    }

    fn decode_b_format(&mut self) {
        trace!("Decoding B-type instruction");

        self.context.decoder.source_register_1 =
            self.extract_source_register_1();
        self.context.decoder.source_register_2 =
            self.extract_source_register_2();
        self.context.decoder.need_to_read_memory = false;
        self.context.decoder.need_to_write_memory = false;
        self.context.decoder.need_to_write_back = false;
        self.context.decoder.need_to_jump = false;
        self.context.decoder.need_to_branch = false; // set to true only branch is needed (during exectuion stage)
        self.context.decoder.zicsr.need_to_read_csr = false;
        self.context.decoder.zicsr.need_to_write_csr = false;
        self.context.decoder.zicsr.need_to_return_from_trap = false;
        self.context.decoder.zicsr.need_to_call_trap = false;

        // extract immediate
        self.context.decoder.immediate = self.extract_imm_4_1();
        self.context.decoder.immediate |= self.extract_imm_10_5();
        self.context.decoder.immediate |= self.extract_b_11();
        self.context.decoder.immediate |=
            self.extract_b_12() as UnsignedImmediate;

        self.context.decoder.funct3 = self.extract_funct3();

        match self.context.decoder.opcode {
            Opcode::BRANCH => {
                match BRANCH_Funct3::from(self.context.decoder.funct3) {
                    BRANCH_Funct3::BEQ => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::BEQ
                    }
                    BRANCH_Funct3::BNE => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::BNE
                    }
                    BRANCH_Funct3::BLT => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::BLT
                    }
                    BRANCH_Funct3::BGE => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::BGE
                    }
                    BRANCH_Funct3::BLTU => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::BLTU
                    }
                    BRANCH_Funct3::BGEU => {
                        self.context.decoder.command_to_execute =
                            ExecutableCommand::BGEU
                    }
                }
            }

            _ => {
                error!("unreachable!");
                unreachable!("unreachable!");
            }
        }
    }

    fn decode_u_format(&mut self) {
        trace!("Decoding U-type instruction");

        self.context.decoder.destination_register_offset =
            self.extract_destination_register_index();
        self.context.decoder.immediate =
            self.extract_imm_31_12() as UnsignedImmediate;

        self.context.decoder.need_to_read_memory = false;
        self.context.decoder.need_to_write_memory = false;
        self.context.decoder.need_to_write_back = true;
        self.context.decoder.need_to_jump = false;
        self.context.decoder.need_to_branch = false;
        self.context.decoder.zicsr.need_to_read_csr = false;
        self.context.decoder.zicsr.need_to_write_csr = false;
        self.context.decoder.zicsr.need_to_return_from_trap = false;
        self.context.decoder.zicsr.need_to_call_trap = false;

        match self.context.decoder.opcode {
            Opcode::AUIPC => {
                self.context.decoder.command_to_execute =
                    ExecutableCommand::AUIPC;
            }
            Opcode::LUI => {
                self.context.decoder.command_to_execute =
                    ExecutableCommand::LUI;
            }

            _ => {
                error!("unreachable!");
                unreachable!("unreachable!");
            }
        };
    }

    fn decode_j_format(&mut self) {
        trace!("Decoding J-type instruction");

        self.context.decoder.destination_register_offset =
            self.extract_destination_register_index();
        self.context.decoder.need_to_read_memory = false;
        self.context.decoder.need_to_write_memory = false;
        self.context.decoder.need_to_write_back = true;
        self.context.decoder.need_to_jump = true;
        self.context.decoder.need_to_branch = false;
        self.context.decoder.zicsr.need_to_read_csr = false;
        self.context.decoder.zicsr.need_to_write_csr = false;
        self.context.decoder.zicsr.need_to_return_from_trap = false;
        self.context.decoder.zicsr.need_to_call_trap = false;

        // extract immediate
        self.context.decoder.immediate = self.extract_imm_10_1();
        self.context.decoder.immediate |= self.extract_j_11();
        self.context.decoder.immediate |= self.extract_imm_19_12();
        self.context.decoder.immediate |=
            self.extract_j_20() as UnsignedImmediate;

        match self.context.decoder.opcode {
            Opcode::JAL => {
                self.context.decoder.command_to_execute =
                    ExecutableCommand::JAL;
            }

            _ => {
                error!("unreachable!");
                unreachable!("unreachable!");
            }
        };
    }

    fn decode_r4_format(&mut self) {
        trace!("Decoding R4-type instruction");

        //
        match self.context.decoder.opcode {
            //
            _ => match self.context.decoder.funct3 {
                //
                _ => {
                    todo!("TODO: unimplemented");
                }
            },
        };
    }

    pub fn decode(&mut self) {
        self.context.decoder.opcode = self.extract_opcode();

        match From::from(self.context.decoder.opcode.clone()) {
            OpcodeToInstructionFormat::R_51_OP
            | OpcodeToInstructionFormat::R_59_OP_32
            | OpcodeToInstructionFormat::R_83_OP_FP => self.decode_r_format(),
            OpcodeToInstructionFormat::I_3_LOAD
            | OpcodeToInstructionFormat::I_7_LOAD_FP
            | OpcodeToInstructionFormat::I_19_OP_IMM
            | OpcodeToInstructionFormat::I_27_OP_IMM_32
            | OpcodeToInstructionFormat::I_103_JALR
            | OpcodeToInstructionFormat::I_115_SYSTEM // (Zicsr_extension)
            | OpcodeToInstructionFormat::I_15_MISC_MEM // FENCE, Zifencei
            => self.decode_i_format(),
            OpcodeToInstructionFormat::S_35_STORE
            | OpcodeToInstructionFormat::S_39_STORE_FP => {
                self.decode_s_format()
            }
            OpcodeToInstructionFormat::B_99_BRANCH => self.decode_b_format(),
            OpcodeToInstructionFormat::U_23_AUIPC
            | OpcodeToInstructionFormat::U_55_LUI => self.decode_u_format(),
            OpcodeToInstructionFormat::J_111_JAL => self.decode_j_format(),
            OpcodeToInstructionFormat::R4_67_MADD
            | OpcodeToInstructionFormat::R4_71_MSUB
            | OpcodeToInstructionFormat::R4_75_NMSUB
            | OpcodeToInstructionFormat::R4_79_NMADD => self.decode_r4_format(),
        }
    }

    fn extract_opcode(&self) -> Opcode {
        Opcode::from(
            ((self.context.current_instruction >> BIT_OFFSET_OPCODE)
                & MASK_OPCODE) as u8,
        )
    }

    fn extract_source_register_1_index(&self) -> usize {
        ((self.context.current_instruction >> BIT_OFFSET_SOURCE_REGISTER_1)
            & MASK_SOURCE_REGISTER_1) as usize
    }

    fn extract_source_register_1(&self) -> XLen {
        self.context
            .gp_register_file
            .read(self.extract_source_register_1_index())
    }

    fn extract_source_register_2_index(&self) -> usize {
        ((self.context.current_instruction >> BIT_OFFSET_SOURCE_REGISTER_2)
            & MASK_SOURCE_REGISTER_2) as usize
    }

    fn extract_source_register_2(&self) -> XLen {
        self.context
            .gp_register_file
            .read(self.extract_source_register_2_index())
    }

    fn extract_destination_register_index(&self) -> usize {
        ((self.context.current_instruction >> BIT_OFFSET_DESTINATION_REGISTER)
            & MASK_DESTINATION_REGISTER) as usize
    }

    fn extract_destination_register(&self) -> XLen {
        self.context
            .gp_register_file
            .read(self.extract_destination_register_index())
    }

    fn extract_funct3(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_FUNCT3) & MASK_FUNCT3
    }

    fn extract_i_funct7(&self) -> OP_IMM_Funct7 {
        From::from(
            (self.context.current_instruction >> BIT_OFFSET_R_FUNCT7_RV64W)
                & MASK_R_FUNCT7_RV64W,
        )
    }

    fn extract_shift_type(&self) -> OP_IMM_ShiftType {
        From::from(
            (self.context.current_instruction >> BIT_OFFSET_SHTYPE)
                & MASK_SHTYPE,
        )
    }

    fn extract_r_funct7(&self) -> Funct7 {
        From::from(
            (self.context.current_instruction >> BIT_OFFSET_R_FUNCT7)
                & MASK_R_FUNCT7,
        )
    }

    fn extract_r_shamt(&self) -> InstructionType {
        (self.context.decoder.source_register_2 & (MASK_SHAMT as XLen))
            as InstructionType
    }

    fn extract_r_shamt_rv64w(&self) -> InstructionType {
        (self.context.decoder.source_register_2 & (MASK_SHAMT_RV64W as XLen))
            as InstructionType
    }

    fn extract_i_shamt(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_SHAMT) & MASK_SHAMT
    }

    fn extract_i_shamt_rv64w(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_SHAMT)
            & MASK_SHAMT_RV64W
    }

    fn extract_imm_11_0(&self) -> SignedXLen {
        // right shift for signed values compiles to arithmetic shift right
        // (ASR) with sign extension
        (self.context.current_instruction as SignedInstructionType
            >> (BIT_OFFSET_IMM_11_0 - BIT_OFFSET_IMM_11_0_FINAL))
            as SignedXLen
    }

    fn extract_fence_sw(&self) -> bool {
        (self.context.current_instruction & MASK_I_FENCE_SW) != 0
    }

    fn extract_fence_sr(&self) -> bool {
        (self.context.current_instruction & MASK_I_FENCE_SR) != 0
    }

    fn extract_fence_so(&self) -> bool {
        (self.context.current_instruction & MASK_I_FENCE_SO) != 0
    }

    fn extract_fence_si(&self) -> bool {
        (self.context.current_instruction & MASK_I_FENCE_SI) != 0
    }

    fn extract_fence_pw(&self) -> bool {
        (self.context.current_instruction & MASK_I_FENCE_PW) != 0
    }

    fn extract_fence_pr(&self) -> bool {
        (self.context.current_instruction & MASK_I_FENCE_PR) != 0
    }

    fn extract_fence_po(&self) -> bool {
        (self.context.current_instruction & MASK_I_FENCE_PO) != 0
    }

    fn extract_fence_pi(&self) -> bool {
        (self.context.current_instruction & MASK_I_FENCE_PI) != 0
    }

    fn extract_fence_fm(&self) -> FenceMode {
        FenceMode::from(
            (self.context.current_instruction & MASK_I_FENCE_FM)
                >> BIT_OFFSET_I_FENCE_FM,
        )
    }

    fn extract_csr_address(&self) -> UnsignedImmediate {
        self.context.current_instruction
            >> (BIT_OFFSET_IMM_11_0 - BIT_OFFSET_IMM_11_0_FINAL)
    }

    fn extract_imm_11_5(&self) -> SignedXLen {
        // right shift for signed values compiles to arithmetic shift right
        // (ASR) with sign extension
        (self.context.current_instruction as SignedInstructionType
            >> (BIT_OFFSET_IMM_11_5 - BIT_OFFSET_IMM_11_5_FINAL))
            as SignedXLen
            & !((1 << BIT_OFFSET_IMM_11_5_FINAL) - 1)
    }

    fn extract_imm_4_0(&self) -> UnsignedImmediate {
        (self.context.current_instruction
            >> (BIT_OFFSET_IMM_4_0 - BIT_OFFSET_IMM_4_0_FINAL))
            & MASK_IMM_4_0
    }

    fn extract_imm_31_12(&self) -> SignedXLen {
        // right shift for signed values compiles to arithmetic shift right
        // (ASR) with sign extension
        (self.context.current_instruction as SignedInstructionType
            >> (BIT_OFFSET_IMM_31_12 - BIT_OFFSET_IMM_31_12_FINAL))
            as SignedXLen
            & !((1 << BIT_OFFSET_IMM_31_12_FINAL) - 1)
    }

    fn extract_imm_10_5(&self) -> UnsignedImmediate {
        (self.context.current_instruction
            >> (BIT_OFFSET_IMM_10_5 - BIT_OFFSET_IMM_10_5_FINAL))
            & MASK_IMM_10_5
    }

    fn extract_imm_4_1(&self) -> UnsignedImmediate {
        (self.context.current_instruction
            >> (BIT_OFFSET_IMM_4_1 - BIT_OFFSET_IMM_4_1_FINAL))
            & MASK_IMM_4_1
    }

    fn extract_imm_10_1(&self) -> UnsignedImmediate {
        (self.context.current_instruction
            >> (BIT_OFFSET_IMM_10_1 - BIT_OFFSET_IMM_10_1_FINAL))
            & MASK_IMM_10_1
    }

    fn extract_imm_19_12(&self) -> UnsignedImmediate {
        (self.context.current_instruction
            >> (BIT_OFFSET_IMM_19_12 - BIT_OFFSET_IMM_19_12_FINAL))
            & MASK_IMM_19_12
    }

    fn extract_b_11(&self) -> UnsignedImmediate {
        (self.context.current_instruction
            << (BIT_OFFSET_B_11_FINAL - BIT_OFFSET_B_11))
            & MASK_B_11
    }

    fn extract_b_12(&self) -> SignedXLen {
        // right shift for signed values compiles to arithmetic shift right
        // (ASR) with sign extension
        (self.context.current_instruction as SignedInstructionType
            >> (BIT_OFFSET_B_12 - BIT_OFFSET_B_12_FINAL)) as SignedXLen
            & !((1 << BIT_OFFSET_B_12_FINAL) - 1)
    }

    fn extract_j_11(&self) -> UnsignedImmediate {
        (self.context.current_instruction
            >> (BIT_OFFSET_J_11 - BIT_OFFSET_J_11_FINAL))
            & MASK_J_11
    }

    fn extract_j_20(&self) -> SignedXLen {
        // right shift for signed values compiles to arithmetic shift right
        // (ASR) with sign extension
        ((self.context.current_instruction as SignedInstructionType)
            >> (BIT_OFFSET_J_20 - BIT_OFFSET_J_20_FINAL)) as SignedXLen
            & !((1 << BIT_OFFSET_J_20_FINAL) - 1)
    }
}
