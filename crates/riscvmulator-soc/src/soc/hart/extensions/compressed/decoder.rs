use log::{error, trace};

use super::constants::{
    Bits11_10_quadrant_01, CA_Funct_2, COMPRESSED_BITS_11_10_MASK,
    COMPRESSED_BITS_11_10_OFFSET, COMPRESSED_BITS_12_MASK,
    COMPRESSED_BITS_12_OFFSET, COMPRESSED_CA_CB_RD_OR_RS1_OFFSET_MASK,
    COMPRESSED_CA_CB_RD_OR_RS1_OFFSET_OFFSET, COMPRESSED_CA_FUNCT_2_MASK,
    COMPRESSED_CA_FUNCT_2_OFFSET, COMPRESSED_CB_FUNCT_3_11X_IMM_2_1_MASK,
    COMPRESSED_CB_FUNCT_3_11X_IMM_2_1_OFFSET,
    COMPRESSED_CB_FUNCT_3_11X_IMM_5_MASK,
    COMPRESSED_CB_FUNCT_3_11X_IMM_7_6_MASK,
    COMPRESSED_CB_FUNCT_3_11X_IMM_7_6_OFFSET, COMPRESSED_CB_UIMM_4_0_MASK,
    COMPRESSED_CB_UIMM_4_0_OFFSET, COMPRESSED_CI_CB_IMM_4_0_MASK,
    COMPRESSED_CI_CB_IMM_4_0_OFFSET, COMPRESSED_CI_IMM_5_MASK,
    COMPRESSED_CI_IMM_5_OFFSET, COMPRESSED_CI_NZIMM_4_0_MASK,
    COMPRESSED_CI_NZIMM_4_0_OFFSET, COMPRESSED_CI_NZIMM_4_MASK,
    COMPRESSED_CI_NZIMM_4_OFFSET, COMPRESSED_CI_NZIMM_5_9_4_MASK,
    COMPRESSED_CI_NZIMM_5_9_4_OFFSET, COMPRESSED_CI_NZIMM_5_MASK,
    COMPRESSED_CI_NZIMM_5_OFFSET, COMPRESSED_CI_NZIMM_6_MASK,
    COMPRESSED_CI_NZIMM_6_OFFSET, COMPRESSED_CI_NZIMM_8_7_MASK,
    COMPRESSED_CI_NZIMM_8_7_OFFSET, COMPRESSED_CI_NZIMM_9_MASK,
    COMPRESSED_CI_NZIMM_9_OFFSET, COMPRESSED_CI_NZIMM_16_12_MASK,
    COMPRESSED_CI_NZIMM_16_12_OFFSET, COMPRESSED_CI_NZIMM_17_MASK,
    COMPRESSED_CI_NZIMM_17_OFFSET, COMPRESSED_CI_OFFSET_4_2_MASK,
    COMPRESSED_CI_OFFSET_4_2_OFFSET, COMPRESSED_CI_OFFSET_4_3_MASK,
    COMPRESSED_CI_OFFSET_4_3_OFFSET, COMPRESSED_CI_OFFSET_5_MASK,
    COMPRESSED_CI_OFFSET_5_OFFSET, COMPRESSED_CI_OFFSET_7_6_MASK,
    COMPRESSED_CI_OFFSET_7_6_OFFSET, COMPRESSED_CI_OFFSET_8_6_MASK,
    COMPRESSED_CI_OFFSET_8_6_OFFSET, COMPRESSED_CI_SHAMT_4_0_MASK,
    COMPRESSED_CI_SHAMT_4_0_OFFSET, COMPRESSED_CI_SHAMT_5_MASK,
    COMPRESSED_CI_SHAMT_5_OFFSET, COMPRESSED_CIW_CL_RD_OFFSET_MASK,
    COMPRESSED_CIW_CL_RD_OFFSET_OFFSET, COMPRESSED_CIW_NZUIMM_MASK,
    COMPRESSED_CIW_NZUIMM_OFFSET, COMPRESSED_CJ_JTA_IMM_3_1_MASK,
    COMPRESSED_CJ_JTA_IMM_3_1_OFFSET, COMPRESSED_CJ_JTA_IMM_4_MASK,
    COMPRESSED_CJ_JTA_IMM_4_OFFSET, COMPRESSED_CJ_JTA_IMM_5_MASK,
    COMPRESSED_CJ_JTA_IMM_5_OFFSET, COMPRESSED_CJ_JTA_IMM_6_MASK,
    COMPRESSED_CJ_JTA_IMM_6_OFFSET, COMPRESSED_CJ_JTA_IMM_7_MASK,
    COMPRESSED_CJ_JTA_IMM_7_OFFSET, COMPRESSED_CJ_JTA_IMM_9_8_MASK,
    COMPRESSED_CJ_JTA_IMM_9_8_OFFSET, COMPRESSED_CJ_JTA_IMM_10_MASK,
    COMPRESSED_CJ_JTA_IMM_10_OFFSET, COMPRESSED_CJ_JTA_IMM_11_MASK,
    COMPRESSED_CJ_JTA_IMM_11_OFFSET, COMPRESSED_CL_CS_RS1_OFFSET_MASK,
    COMPRESSED_CL_CS_RS1_OFFSET_OFFSET, COMPRESSED_CL_OFFSET_2_MASK,
    COMPRESSED_CL_OFFSET_2_OFFSET, COMPRESSED_CL_OFFSET_5_3_MASK,
    COMPRESSED_CL_OFFSET_5_3_OFFSET, COMPRESSED_CL_OFFSET_6_MASK,
    COMPRESSED_CL_OFFSET_6_OFFSET, COMPRESSED_CL_OFFSET_7_MASK,
    COMPRESSED_CL_OFFSET_7_OFFSET, COMPRESSED_CR_CI_RD_RS1_MASK,
    COMPRESSED_CR_CI_RD_RS1_OFFSET, COMPRESSED_CR_CSS_RS2_MASK,
    COMPRESSED_CR_CSS_RS2_OFFSET, COMPRESSED_CS_CA_RS2_OFFSET_MASK,
    COMPRESSED_CS_CA_RS2_OFFSET_OFFSET, COMPRESSED_CSS_UIMM_5_2_MASK,
    COMPRESSED_CSS_UIMM_5_2_OFFSET, COMPRESSED_CSS_UIMM_5_3_MASK,
    COMPRESSED_CSS_UIMM_5_3_OFFSET, COMPRESSED_CSS_UIMM_7_6_MASK,
    COMPRESSED_CSS_UIMM_7_6_OFFSET, COMPRESSED_CSS_UIMM_8_6_MASK,
    COMPRESSED_CSS_UIMM_8_6_OFFSET, COMPRESSED_FUNCT3_MASK,
    COMPRESSED_FUNCT4_MASK, COMPRESSED_FUNCT4_OFFSET,
    InstructionTypeCompressed,
};
use crate::constants::arch::{
    BIT_OFFSET_B_FORMAT_IMM_4_1, BIT_OFFSET_B_FORMAT_IMM_10_5,
    BIT_OFFSET_B_FORMAT_IMM_11, BIT_OFFSET_B_FORMAT_IMM_12,
    BIT_OFFSET_DESTINATION_REGISTER, BIT_OFFSET_FUNCT3, BIT_OFFSET_I_FUNCT7,
    BIT_OFFSET_IMM_4_0, BIT_OFFSET_IMM_11_0, BIT_OFFSET_IMM_11_5,
    BIT_OFFSET_IMM_31_12, BIT_OFFSET_R_FUNCT7, BIT_OFFSET_SHAMT,
    BIT_OFFSET_SHTYPE, BIT_OFFSET_SOURCE_REGISTER_1,
    BIT_OFFSET_SOURCE_REGISTER_2, InstructionType, MASK_B_FORMAT_IMM_4_1,
    MASK_B_FORMAT_IMM_10_5, MASK_B_FORMAT_IMM_11, MASK_B_FORMAT_IMM_12,
    MASK_IMM_4_0, MASK_IMM_11_5, SignedImmediate, UnsignedImmediate,
};
use crate::soc::hart::core::decoder::constants::{
    JALR_Funct3, OP_Funct7, OP_IMM_Funct3, OP_IMM_ShiftType,
    OPCODE_LOAD_Funct3, Opcode, OpcodeQuadrant, STORE_Funct3, SYSTEM_Funct3,
    SYSTEM_Imm_transfer_control_return_from,
};
use crate::soc::hart::core::gp_registers::constants::GPRegisterName;
use crate::soc::hart::core::{decoder::Decoder, fetch_unit::FetchUnit};
use crate::soc::hart::extensions;
use crate::soc::hart::extensions::integer::constants::OP_IMM_32_Funct3;

#[derive(Clone)]
pub struct DecoderCompressedISAContext {
    funct3: extensions::compressed::constants::OP_Funct3,
    cr_funct4_lsb_compressed: bool,
    cr_ci_rd_or_rs1_offset: usize,
    cr_css_source_register_2_offset: usize,
    c_imm: SignedImmediate,
    c_uimm: UnsignedImmediate,
    ci_shamt: InstructionTypeCompressed,
    css_imm: SignedImmediate,
    ciw_cl_rd_offset: usize,
    cl_offset_6_2: InstructionTypeCompressed,
    cl_offset_7_3: InstructionTypeCompressed,
    cl_cs_rs1_offset: usize,
    cs_ca_rs2_offset: usize,
    ca_funct2: CA_Funct_2,
    ca_cb_rd_or_rs1_offset: usize,
    ca_cb_bit_12: InstructionTypeCompressed,
    ca_cb_bits_11_10_quad_01_funct3_100: Bits11_10_quadrant_01,
    cj_jta_imm: InstructionTypeCompressed,
}

impl Default for DecoderCompressedISAContext {
    fn default() -> Self {
        DecoderCompressedISAContext {
            funct3: extensions::compressed::constants::OP_Funct3::F000,
            cr_funct4_lsb_compressed: false,
            cr_ci_rd_or_rs1_offset: GPRegisterName::x0_zero as usize,
            cr_css_source_register_2_offset: GPRegisterName::x0_zero as usize,
            c_imm: 0,
            c_uimm: 0,
            ci_shamt: 0,
            css_imm: 0,
            ciw_cl_rd_offset: 0,
            cl_offset_6_2: 0,
            cl_offset_7_3: 0,
            cl_cs_rs1_offset: 0,
            cs_ca_rs2_offset: 0,
            ca_funct2: CA_Funct_2::F00,
            ca_cb_rd_or_rs1_offset: 0,
            ca_cb_bit_12: 0b0,
            ca_cb_bits_11_10_quad_01_funct3_100: Bits11_10_quadrant_01::CA_11,
            cj_jta_imm: 0,
        }
    }
}

impl Decoder {
    pub(crate) fn expand_compressed(
        &mut self,
        fetch_unit: &FetchUnit,
    ) -> InstructionType {
        self.compressedISA.funct3 = self.extract_funct3_compressed(
            fetch_unit.current_instruction_compressed,
        );

        match fetch_unit.current_quadrant {
            OpcodeQuadrant::Quadrant00 => match self.compressedISA.funct3 {
                extensions::compressed::constants::OP_Funct3::F000 => {
                    self.expand_ciw(fetch_unit.current_instruction_compressed)
                }
                extensions::compressed::constants::OP_Funct3::F001
                | extensions::compressed::constants::OP_Funct3::F010
                | extensions::compressed::constants::OP_Funct3::F011 => {
                    self.expand_cl(fetch_unit.current_instruction_compressed)
                }
                extensions::compressed::constants::OP_Funct3::F101
                | extensions::compressed::constants::OP_Funct3::F110
                | extensions::compressed::constants::OP_Funct3::F111 => {
                    self.expand_cs(fetch_unit.current_instruction_compressed)
                }

                extensions::compressed::constants::OP_Funct3::F100 => {
                    todo!("illegal instruction, reserved by specification");
                }
            },
            OpcodeQuadrant::Quadrant01 => match self.compressedISA.funct3 {
                extensions::compressed::constants::OP_Funct3::F000
                | extensions::compressed::constants::OP_Funct3::F010
                | extensions::compressed::constants::OP_Funct3::F011 => {
                    self.expand_ci(fetch_unit)
                }
                extensions::compressed::constants::OP_Funct3::F001 => {
                    todo!("distiguish between RV32/RV64");
                }
                extensions::compressed::constants::OP_Funct3::F100 => {
                    self.compressedISA.ca_cb_bit_12 = self
                        .extract_ca_cb_bit_12(
                            fetch_unit.current_instruction_compressed,
                        );
                    self.compressedISA.ca_cb_bits_11_10_quad_01_funct3_100 =
                        self.extract_bits_11_10_quadrant_01_funct3_100(
                            fetch_unit.current_instruction_compressed,
                        );
                    self.compressedISA.ca_cb_rd_or_rs1_offset = self
                        .extract_ca_cb_rd_or_rs1_offset(
                            fetch_unit.current_instruction_compressed,
                        );
                    match self.compressedISA.ca_cb_bits_11_10_quad_01_funct3_100
                    {
                        Bits11_10_quadrant_01::CA_11 => self.expand_ca(
                            fetch_unit.current_instruction_compressed,
                        ),
                        Bits11_10_quadrant_01::CB_00
                        | Bits11_10_quadrant_01::CB_01
                        | Bits11_10_quadrant_01::CB_10 => self.expand_cb(
                            fetch_unit.current_instruction_compressed,
                        ),
                    }
                }
                extensions::compressed::constants::OP_Funct3::F101 => {
                    self.expand_cj(fetch_unit.current_instruction_compressed)
                }
                extensions::compressed::constants::OP_Funct3::F110
                | extensions::compressed::constants::OP_Funct3::F111 => {
                    self.expand_cb(fetch_unit.current_instruction_compressed)
                }
            },
            OpcodeQuadrant::Quadrant10 => match self.compressedISA.funct3 {
                extensions::compressed::constants::OP_Funct3::F000
                | extensions::compressed::constants::OP_Funct3::F001
                | extensions::compressed::constants::OP_Funct3::F010
                | extensions::compressed::constants::OP_Funct3::F011 => {
                    self.expand_ci(fetch_unit)
                }
                extensions::compressed::constants::OP_Funct3::F100 => {
                    self.expand_cr(fetch_unit.current_instruction_compressed)
                }
                extensions::compressed::constants::OP_Funct3::F101
                | extensions::compressed::constants::OP_Funct3::F110
                | extensions::compressed::constants::OP_Funct3::F111 => {
                    self.expand_css(fetch_unit.current_instruction_compressed)
                }
            },

            OpcodeQuadrant::Quadrant11 => {
                error!(
                    "code flow reached compressed decoder but decodes non-compressed instruction"
                );
                unreachable!(
                    "code flow reached compressed decoder but decodes non-compressed instruction"
                );
            }
        }

        // match From::from(self.opcode) {
        //     OpcodeToInstructionFormat::
        // }

        // todo!("TODO: compressed ISA decoder");
    }

    fn expand_cr(
        &mut self,
        instruction: InstructionTypeCompressed,
    ) -> InstructionType {
        trace!("Expanding CR-format instruction..");

        self.compressedISA.cr_funct4_lsb_compressed =
            self.extract_cr_funct4_lsb_compressed(instruction);
        self.compressedISA.cr_ci_rd_or_rs1_offset = self
            .extract_cr_ci_destination_or_source_register_1_offset(instruction);
        self.compressedISA.cr_css_source_register_2_offset =
            self.extract_cr_css_source_register_2_offset(instruction);

        if self.compressedISA.cr_funct4_lsb_compressed {
            if self.compressedISA.cr_css_source_register_2_offset == 0 {
                if self.compressedISA.cr_ci_rd_or_rs1_offset != 0 {
                    // C.JALR expands into `jalr x1, 0(rs1)`
                    self.expand_to_i_format(
                        Opcode::JALR,
                        GPRegisterName::x1_ra as usize,
                        JALR_Funct3::JALR as InstructionType,
                        self.compressedISA.cr_ci_rd_or_rs1_offset,
                        // 0 immedaite is implied by `C.JALR`
                        0,
                    )
                } else {
                    // C.EBREAK expands into `ebreak`
                    self.expand_to_i_format(
                        Opcode::SYSTEM,
                        GPRegisterName::x0_zero as usize,
                        SYSTEM_Funct3::TRANSFER_CONTROL_RETURN_FROM
                            as InstructionType,
                        GPRegisterName::x0_zero as usize,
                        SYSTEM_Imm_transfer_control_return_from::EBREAK
                            as SignedImmediate,
                    )
                }
            } else {
                // the code points with rd == x0 are HINTs
                if self.compressedISA.cr_ci_rd_or_rs1_offset == 0 {
                    trace!("HINT was met");
                }
                // C.ADD expands into `add rd, rd, rs2`
                self.expand_to_r_format(
                    Opcode::OP,
                    self.compressedISA.cr_ci_rd_or_rs1_offset,
                    extensions::integer::constants::OP_Funct3::ARITH
                        as InstructionType,
                    self.compressedISA.cr_ci_rd_or_rs1_offset,
                    self.compressedISA.cr_css_source_register_2_offset,
                    OP_Funct7::F0 as InstructionType,
                )
            }
        } else {
            if self.compressedISA.cr_css_source_register_2_offset == 0 {
                // C.JR is valid only when rs1 != x0; the code point with rs1 = x0 is reserved.
                if self.compressedISA.cr_ci_rd_or_rs1_offset != 0 {
                    // C.JR expands into `jalr x0, 0(rs1)`
                    self.expand_to_i_format(
                        Opcode::JALR,
                        GPRegisterName::x0_zero as usize,
                        JALR_Funct3::JALR as InstructionType,
                        self.compressedISA.cr_ci_rd_or_rs1_offset as usize,
                        // 0 immediate is implied by `C.JR`
                        0,
                    )
                } else {
                    todo!(
                        "illegal instruction trap, reserved by specification"
                    );
                }
            } else {
                // C.MV is valid only when rs2 != x0; the code points with rd == x0 are HINTs
                self.destination_register_offset =
                    self.compressedISA.cr_ci_rd_or_rs1_offset;
                if self.destination_register_offset == 0 {
                    trace!("HINT was met");
                }
                // C.MV expands into `add rd, x0, rs2`
                self.expand_to_r_format(
                    Opcode::OP,
                    self.compressedISA.cr_ci_rd_or_rs1_offset,
                    extensions::integer::constants::OP_Funct3::ARITH
                        as InstructionType,
                    GPRegisterName::x0_zero as usize,
                    self.compressedISA.cr_css_source_register_2_offset,
                    OP_Funct7::F0 as InstructionType,
                )
            }
        }
    }

    fn expand_ci(&mut self, fetch_unit: &FetchUnit) -> InstructionType {
        trace!("Expanding CI-format instruction..");

        self.compressedISA.cr_ci_rd_or_rs1_offset = self
            .extract_cr_ci_destination_or_source_register_1_offset(
                fetch_unit.current_instruction_compressed,
            );

        match self.compressedISA.funct3 {
            extensions::compressed::constants::OP_Funct3::F000 => {
                match fetch_unit.current_quadrant {
                    OpcodeQuadrant::Quadrant01 => {
                        self.compressedISA.c_imm = self.extract_ci_nzimm_5_0(
                            fetch_unit.current_instruction_compressed,
                        );
                        // C.ADDI expands into `addi rd, rd, imm`
                        self.expand_to_i_format(
                            Opcode::OP_IMM,
                            self.compressedISA.cr_ci_rd_or_rs1_offset,
                            OP_IMM_Funct3::ADDI as InstructionType,
                            self.compressedISA.cr_ci_rd_or_rs1_offset,
                            self.compressedISA.c_imm,
                        )
                    }
                    OpcodeQuadrant::Quadrant10 => {
                        self.compressedISA.ci_shamt = self.extract_ci_shamt(
                            fetch_unit.current_instruction_compressed,
                        );
                        // C.SLLI expands into `slli rd, rd, shamt[5:0]`
                        self.expand_to_i_format(
                            Opcode::OP_IMM,
                            self.compressedISA.cr_ci_rd_or_rs1_offset as usize,
                            OP_IMM_Funct3::SLLI as InstructionType,
                            self.compressedISA.cr_ci_rd_or_rs1_offset,
                            ((self.compressedISA.ci_shamt)
                                | ((OP_IMM_ShiftType::SXLI
                                    as InstructionTypeCompressed)
                                    << (BIT_OFFSET_SHTYPE - BIT_OFFSET_SHAMT)))
                                as SignedImmediate,
                        )
                    }

                    _ => {
                        error!("unreachable opcode for CI instruction");
                        unreachable!("unreachable opcode for CI instruction");
                    }
                }
            }
            extensions::compressed::constants::OP_Funct3::F001 => {
                match fetch_unit.current_quadrant {
                    OpcodeQuadrant::Quadrant01 => {
                        self.compressedISA.c_imm = self.extract_ci_imm_5_0(
                            fetch_unit.current_instruction_compressed,
                        );
                        // C.ADDIW expands into `addiw rd, rd, imm`
                        self.expand_to_i_format(
                            Opcode::OP_IMM_32,
                            self.compressedISA.cr_ci_rd_or_rs1_offset,
                            OP_IMM_32_Funct3::ADDIW as InstructionType,
                            self.compressedISA.cr_ci_rd_or_rs1_offset,
                            self.compressedISA.c_imm,
                        )
                    }
                    OpcodeQuadrant::Quadrant10 => {
                        // C.FLDSP expands into `fld rd, offset(x2)`
                        todo!()
                    }

                    _ => {
                        error!("unreachable opcode for CI instruction");
                        unreachable!("unreachable opcode for CI instruction");
                    }
                }
            }
            extensions::compressed::constants::OP_Funct3::F010 => {
                match fetch_unit.current_quadrant {
                    OpcodeQuadrant::Quadrant01 => {
                        self.compressedISA.c_imm = self.extract_ci_imm_5_0(
                            fetch_unit.current_instruction_compressed,
                        );
                        // C.LI expands into `addi rd, x0, imm`
                        self.expand_to_i_format(
                            Opcode::OP_IMM,
                            self.compressedISA.cr_ci_rd_or_rs1_offset,
                            OP_IMM_Funct3::ADDI as InstructionType,
                            GPRegisterName::x0_zero as usize,
                            self.compressedISA.c_imm,
                        )
                    }
                    OpcodeQuadrant::Quadrant10 => {
                        self.compressedISA.c_imm = self
                            .extract_ci_load_word_sp_offset(
                                fetch_unit.current_instruction_compressed,
                            );
                        // C.LWSP expands into `lw rd, offset(x2)`
                        self.expand_to_i_format(
                            Opcode::LOAD,
                            self.compressedISA.cr_ci_rd_or_rs1_offset,
                            OPCODE_LOAD_Funct3::LW as InstructionType,
                            GPRegisterName::x2_sp as usize,
                            self.compressedISA.c_imm,
                        )
                    }

                    _ => {
                        error!("unreachable opcode for CI instruction");
                        unreachable!("unreachable opcode for CI instruction");
                    }
                }
            }
            extensions::compressed::constants::OP_Funct3::F011 => {
                match fetch_unit.current_quadrant {
                    OpcodeQuadrant::Quadrant01 => {
                        if self.compressedISA.cr_ci_rd_or_rs1_offset != 2 {
                            self.compressedISA.c_imm = self
                                .extract_ci_nzimm_17_12(
                                    fetch_unit.current_instruction_compressed,
                                );
                            if self.compressedISA.c_imm != 0 {
                                if self.compressedISA.cr_ci_rd_or_rs1_offset
                                    == 0
                                {
                                    trace!("HINT");
                                }
                                // C.LUI expands into `lui rd, imm`
                                self.expand_to_u_format(
                                    Opcode::LUI,
                                    self.compressedISA.cr_ci_rd_or_rs1_offset,
                                    self.compressedISA.c_imm,
                                )
                            } else {
                                // the code points with imm = 0 are reserved
                                todo!(
                                    "illegal instruction trap, reserved by specification"
                                );
                            }
                        } else {
                            self.compressedISA.c_imm = self
                                .extract_ci_nzimm_9_4(
                                    fetch_unit.current_instruction_compressed,
                                );
                            if self.compressedISA.c_imm != 0 {
                                if self.compressedISA.cr_ci_rd_or_rs1_offset
                                    == 0
                                {
                                    trace!("HINT");
                                }
                                // C.ADDI16SP expands into `addi x2, x2, nzimm[9:4]`
                                self.expand_to_i_format(
                                    Opcode::OP_IMM,
                                    self.compressedISA.cr_ci_rd_or_rs1_offset,
                                    OP_IMM_Funct3::ADDI as InstructionType,
                                    self.compressedISA.cr_ci_rd_or_rs1_offset,
                                    self.compressedISA.c_imm,
                                )
                            } else {
                                // the code point with nzimm = 0 is reserved
                                todo!(
                                    "illegal instruction trap, reserved by specification"
                                );
                            }
                        }
                    }
                    OpcodeQuadrant::Quadrant10 => {
                        todo!("check unimplemented");
                        // if if RV64C
                        if self.compressedISA.cr_ci_rd_or_rs1_offset != 0 {
                            self.compressedISA.c_imm = self
                                .extract_ci_load_double_word_sp_offset(
                                    fetch_unit.current_instruction_compressed,
                                );
                            // C.LDSP expands into `ld rd, offset(x2)`
                            self.expand_to_i_format(
                                Opcode::LOAD,
                                self.compressedISA.cr_ci_rd_or_rs1_offset,
                                OPCODE_LOAD_Funct3::LD as InstructionType,
                                GPRegisterName::x2_sp as usize,
                                self.compressedISA.c_imm,
                            )
                        } else {
                            // the code points with rd = x0 are reserved
                            todo!(
                                "illegal instruction trap, reserved by specification"
                            );
                        }
                        // else RV32FC
                        // // C.FLW expands into `flw rd', offset(rs1')`
                        // todo!();
                    }

                    _ => {
                        error!("unreachable opcode for CI instruction");
                        unreachable!("unreachable opcode for CI instruction");
                    }
                }
            }

            _ => {
                error!(
                    "unreachable funct3 ({}) for CI instruction",
                    self.compressedISA.funct3
                );
                unreachable!(
                    "unreachable funct3 ({}) for CI instruction",
                    self.compressedISA.funct3
                );
            }
        }
    }

    fn expand_css(
        &mut self,
        instruction: InstructionTypeCompressed,
    ) -> InstructionType {
        trace!("Expanding CSS-format instruction..");

        self.compressedISA.cr_css_source_register_2_offset =
            self.extract_cr_css_source_register_2_offset(instruction);

        match self.compressedISA.funct3 {
            extensions::compressed::constants::OP_Funct3::F101 => {
                self.compressedISA.css_imm =
                    self.extract_css_uimm_8_3(instruction);
                // C.FSDSP expands into `fsd rs2, offset(x2)`

                todo!("TODO: unimplemented");
            }
            extensions::compressed::constants::OP_Funct3::F110 => {
                self.compressedISA.css_imm =
                    self.extract_css_uimm_7_2(instruction);
                // C.SWSP expands into `sw rs2, offset(x2)`

                todo!("TODO: unimplemented");
            }
            extensions::compressed::constants::OP_Funct3::F111 => {
                todo!("TODO: check unimplemented");

                // if RV64IC
                self.compressedISA.css_imm =
                    self.extract_css_uimm_8_3(instruction);
                // C.SDSP expands into `sd rs2, offset(x2)`
                //
                // else if RV32IFC
                self.compressedISA.css_imm =
                    self.extract_css_uimm_7_2(instruction);
                // C.FSWSP expands into `fsw rs2, offset(x2)`
                //

                todo!("TODO: unimplemented");
            }

            _ => {
                error!(
                    "unreachable funct3 ({}) for CI instruction",
                    self.compressedISA.funct3
                );
                unreachable!(
                    "unreachable funct3 ({}) for CI instruction",
                    self.compressedISA.funct3
                );
            }
        }
    }

    fn expand_ciw(
        &mut self,
        instruction: InstructionTypeCompressed,
    ) -> InstructionType {
        trace!("Expanding CIW-format instruction..");

        // C.ADDI4SPN (RESERVED when uimm == 0)
        self.compressedISA.c_uimm = self.extract_ciw_nzuimm_9_2(instruction);
        self.compressedISA.ciw_cl_rd_offset =
            self.extract_ciw_cl_rd_offset(instruction);

        if self.compressedISA.c_uimm != 0 {
            // C.ADDI4SPN expands into `addi rd', x2, nzuimm[9:2]`
            self.expand_to_i_format(
                Opcode::OP_IMM,
                self.compressedISA.ciw_cl_rd_offset,
                OP_IMM_Funct3::ADDI as InstructionType,
                GPRegisterName::x2_sp as usize,
                self.compressedISA.c_uimm as SignedImmediate,
            )
        } else {
            // RESERVED, uimm == 0
            todo!("illegal instruction trap, reserved by specification");
        }
    }

    fn expand_cl(
        &mut self,
        instruction: InstructionTypeCompressed,
    ) -> InstructionType {
        trace!("Expanding CL-format instruction..");

        self.compressedISA.ciw_cl_rd_offset =
            self.extract_ciw_cl_rd_offset(instruction);
        self.compressedISA.cl_cs_rs1_offset =
            self.extract_cl_cs_rs1_offset(instruction);

        match self.compressedISA.funct3 {
            extensions::compressed::constants::OP_Funct3::F001 => {
                todo!("condition unimplemented yet");
                // if RV(32|64)DC {
                self.compressedISA.cl_offset_7_3 =
                    self.extract_cl_offset_7_3(instruction);
                // C.FLD expands into `fld rd', offset(rs1')`
                //
                // }
                // else {
                //
                // }
            }
            extensions::compressed::constants::OP_Funct3::F010 => {
                self.compressedISA.cl_offset_6_2 =
                    self.extract_cl_offset_6_2(instruction);
                // C.LW expands into `lw rd', offset(rs1')`
                self.expand_to_i_format(
                    Opcode::LOAD,
                    self.compressedISA.ciw_cl_rd_offset,
                    OPCODE_LOAD_Funct3::LW as InstructionType,
                    self.compressedISA.cl_cs_rs1_offset,
                    self.compressedISA.cl_offset_6_2 as SignedImmediate,
                )
            }
            extensions::compressed::constants::OP_Funct3::F011 => {
                todo!("condition unimplemented yet");
                // if RV64IC only
                self.compressedISA.cl_offset_7_3 =
                    self.extract_cl_offset_7_3(instruction);
                // C.LD expands into `ld rd', offset(rs1')`
                self.expand_to_i_format(
                    Opcode::LOAD,
                    self.compressedISA.ciw_cl_rd_offset,
                    OPCODE_LOAD_Funct3::LD as InstructionType,
                    self.compressedISA.cl_cs_rs1_offset,
                    self.compressedISA.cl_offset_7_3 as SignedImmediate,
                )
                // } else if RV32IFC {
                // self.compressedISA.cl_offset_6_2 = self.extract_cl_offset_6_2(instruction);
                // // C.FLW expands into `flw rd', offset(rs1')`
                // todo!();
                // }
            }

            _ => {
                error!(
                    "unreachable funct3 ({}) for CL instruction",
                    self.compressedISA.funct3
                );
                unreachable!(
                    "unreachable funct3 ({}) for CL instruction",
                    self.compressedISA.funct3
                );
            }
        }
    }

    fn expand_cs(
        &mut self,
        instruction: InstructionTypeCompressed,
    ) -> InstructionType {
        trace!("Expanding CS-format instruction..");

        self.compressedISA.cl_cs_rs1_offset =
            self.extract_cl_cs_rs1_offset(instruction);
        self.compressedISA.cs_ca_rs2_offset =
            self.extract_cs_ca_rs2_offset(instruction);

        match self.compressedISA.funct3 {
            extensions::compressed::constants::OP_Funct3::F101 => {
                todo!("condition unimplemented yet");
                // if RV(32|64)DC {
                // // C.FSD expands into `fsd rs2', offset(rs1')`
                // todo!();
                // } else {
                // todo!();
                // }
            }
            extensions::compressed::constants::OP_Funct3::F110 => {
                // // C.SW expands into `sw rs2', offset(rs1')`
                todo!();
            }
            extensions::compressed::constants::OP_Funct3::F111 => {
                todo!("condition unimplemented yet");
                // if { // RV64IC only
                // // C.SD expands into `sd rs2', offset(rs1')`
                // todo!();
                // } else if RV32IFC {
                // // C.FSW expands into `fsw rs2', offset(rs1')`
                // todo!();
                // }
            }

            _ => {
                todo!("illegal instruction trap, reserved by specification");
            }
        }
    }

    fn expand_ca(
        &mut self,
        instruction: InstructionTypeCompressed,
    ) -> InstructionType {
        trace!("Expanding CA-format instruction..");

        self.compressedISA.ca_funct2 = self.extract_ca_funct2(instruction);
        self.compressedISA.cs_ca_rs2_offset =
            self.extract_cs_ca_rs2_offset(instruction);

        if self.compressedISA.ca_cb_bit_12 != 0b0 {
            match self.compressedISA.ca_funct2 {
                CA_Funct_2::F00 => {
                    // C.SUB expands into `sub rd', rd', rs2'`
                    self.expand_to_r_format(
                        Opcode::OP,
                        self.compressedISA.ca_cb_rd_or_rs1_offset as usize,
                        extensions::integer::constants::OP_Funct3::ARITH
                            as InstructionType,
                        self.compressedISA.ca_cb_rd_or_rs1_offset as usize,
                        self.compressedISA.cs_ca_rs2_offset,
                        OP_Funct7::F1 as InstructionType,
                    )
                }
                CA_Funct_2::F01 => {
                    // C.XOR expands into `xor rd', rd', rs2'`
                    self.expand_to_r_format(
                        Opcode::OP,
                        self.compressedISA.ca_cb_rd_or_rs1_offset as usize,
                        extensions::integer::constants::OP_Funct3::XOR
                            as InstructionType,
                        self.compressedISA.ca_cb_rd_or_rs1_offset as usize,
                        self.compressedISA.cs_ca_rs2_offset,
                        OP_Funct7::F0 as InstructionType,
                    )
                }
                CA_Funct_2::F10 => {
                    // C.OR expands into or `rd', rd', rs2'`
                    self.expand_to_r_format(
                        Opcode::OP,
                        self.compressedISA.ca_cb_rd_or_rs1_offset as usize,
                        extensions::integer::constants::OP_Funct3::OR
                            as InstructionType,
                        self.compressedISA.ca_cb_rd_or_rs1_offset as usize,
                        self.compressedISA.cs_ca_rs2_offset,
                        OP_Funct7::F0 as InstructionType,
                    )
                }
                CA_Funct_2::F11 => {
                    // C.AND expands into `and rd', rd', rs2'`
                    self.expand_to_r_format(
                        Opcode::OP,
                        self.compressedISA.ca_cb_rd_or_rs1_offset as usize,
                        extensions::integer::constants::OP_Funct3::AND
                            as InstructionType,
                        self.compressedISA.ca_cb_rd_or_rs1_offset as usize,
                        self.compressedISA.cs_ca_rs2_offset,
                        OP_Funct7::F0 as InstructionType,
                    )
                }
            }
        } else {
            match self.compressedISA.ca_funct2 {
                CA_Funct_2::F00 => {
                    // C.SUBW expands into subw `rd', rd', rs2'`
                    todo!("add check");
                    //if RV64IC
                    self.expand_to_r_format(
                        Opcode::OP_32,
                        self.compressedISA.ca_cb_rd_or_rs1_offset as usize,
                        extensions::integer::constants::OP_32_Funct3::ARITHW
                            as InstructionType,
                        self.compressedISA.ca_cb_rd_or_rs1_offset as usize,
                        self.compressedISA.cs_ca_rs2_offset,
                        OP_Funct7::F1 as InstructionType,
                    )
                    // else if RV32I {
                    //     todo!(
                    //         "illegal instruction trap, reserved by specification"
                    //     );
                    // }
                }
                CA_Funct_2::F01 => {
                    // C.ADDW expands into addw `rd', rd', rs2'`
                    todo!("add check");
                    //if RV64IC
                    self.expand_to_r_format(
                        Opcode::OP_32,
                        self.compressedISA.ca_cb_rd_or_rs1_offset as usize,
                        extensions::integer::constants::OP_32_Funct3::ARITHW
                            as InstructionType,
                        self.compressedISA.ca_cb_rd_or_rs1_offset as usize,
                        self.compressedISA.cs_ca_rs2_offset,
                        OP_Funct7::F0 as InstructionType,
                    )
                    // else if RV32I {
                    //     todo!(
                    //         "illegal instruction trap, reserved by specification"
                    //     );
                    // }
                }

                _ => {
                    todo!(
                        "illegal instruction trap, reserved by specification"
                    );
                }
            }
        }
    }

    fn expand_cb(
        &mut self,
        instruction: InstructionTypeCompressed,
    ) -> InstructionType {
        trace!("Expanding CB-format instruction..");

        if self.compressedISA.funct3
            == extensions::compressed::constants::OP_Funct3::F100
        {
            if self.compressedISA.ca_cb_bits_11_10_quad_01_funct3_100
                == Bits11_10_quadrant_01::CB_10
            {
                self.compressedISA.c_imm = self.extract_cb_imm_5_0(instruction);
                // C.ANDI expands into `andi rd', rd', imm`
                return self.expand_to_i_format(
                    Opcode::OP_IMM,
                    self.compressedISA.ca_cb_rd_or_rs1_offset,
                    OP_IMM_Funct3::ANDI as InstructionType,
                    self.compressedISA.ca_cb_rd_or_rs1_offset,
                    self.compressedISA.c_imm,
                );
            } else {
                self.compressedISA.c_uimm =
                    self.extract_cb_uimm_5_0(instruction);

                match self.compressedISA.ca_cb_bits_11_10_quad_01_funct3_100 {
                    Bits11_10_quadrant_01::CB_00 => {
                        // SRLI and SRAI differ in the 6/5th bit of funct7 field
                        self.compressedISA.c_imm |= (OP_IMM_ShiftType::SXLI
                            as SignedImmediate)
                            << (BIT_OFFSET_I_FUNCT7 - BIT_OFFSET_IMM_11_0);
                        // C.SRLI expands into `srli rd', rd', shamt`
                        return self.expand_to_i_format(
                            Opcode::OP_IMM,
                            self.compressedISA.ca_cb_rd_or_rs1_offset,
                            OP_IMM_Funct3::SRXI as InstructionType,
                            self.compressedISA.ca_cb_rd_or_rs1_offset,
                            self.compressedISA.c_imm,
                        );
                    }
                    Bits11_10_quadrant_01::CB_01 => {
                        // SRLI and SRAI differ in the 6/5th bit of funct7 field
                        self.compressedISA.c_imm |= (OP_IMM_ShiftType::SRAI
                            as SignedImmediate)
                            << (BIT_OFFSET_I_FUNCT7 - BIT_OFFSET_IMM_11_0);
                        // C.SRAI expands into `srai rd', rd', shamt`
                        return self.expand_to_i_format(
                            Opcode::OP_IMM,
                            self.compressedISA.ca_cb_rd_or_rs1_offset,
                            OP_IMM_Funct3::SRXI as InstructionType,
                            self.compressedISA.ca_cb_rd_or_rs1_offset,
                            self.compressedISA.c_imm,
                        );
                    }

                    _ => {
                        unreachable!(
                            "unreachable bits 11_10 ({:#?}) for CB instruction",
                            self.compressedISA
                                .ca_cb_bits_11_10_quad_01_funct3_100
                        );
                    }
                }
            }
        } else {
            self.compressedISA.c_imm = self.extract_cb_imm_8_1(instruction);
            match self.compressedISA.funct3 {
                extensions::compressed::constants::OP_Funct3::F110 => {
                    // C.BEQZ expands into `beq rs1', x0, offset`
                    todo!("unimplemented yet");
                }
                extensions::compressed::constants::OP_Funct3::F111 => {
                    // C.BNEZ expands to `bne rs1', x0, offset`
                    todo!("unimplemented yet");
                }

                _ => {
                    unreachable!(
                        "unreachable func3 ({:#?}) for CB instruction",
                        self.compressedISA.funct3
                    );
                }
            }
        }
    }

    fn expand_cj(
        &mut self,
        instruction: InstructionTypeCompressed,
    ) -> InstructionType {
        trace!("Expanding CJ-format instruction..");

        self.compressedISA.cj_jta_imm = self.extract_cj_jta_imm(instruction);

        match self.compressedISA.funct3 {
            extensions::compressed::constants::OP_Funct3::F001 => {
                // C.JAL expands into `jal x1, offset`
                todo!("condition unimplemented yet");
            }
            extensions::compressed::constants::OP_Funct3::F101 => {
                // C.J expands into `jal x0, offset`
                todo!("condition unimplemented yet");
            }

            _ => {
                todo!("illegal instruction trap, reserved by specification");
            }
        }

        todo!("TODO: unimplemented");
    }

    // extractors
    fn extract_funct3_compressed(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> extensions::compressed::constants::OP_Funct3 {
        extensions::compressed::constants::OP_Funct3::from(
            instruction & COMPRESSED_FUNCT3_MASK,
        )
    }

    fn extract_cr_funct4_lsb_compressed(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> bool {
        ((instruction & COMPRESSED_FUNCT4_MASK) >> COMPRESSED_FUNCT4_OFFSET)
            & 0b1
            == 1
    }

    fn extract_cr_ci_destination_or_source_register_1_offset(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> usize {
        ((instruction & COMPRESSED_CR_CI_RD_RS1_MASK)
            >> COMPRESSED_CR_CI_RD_RS1_OFFSET) as usize
    }

    fn extract_cr_css_source_register_2_offset(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> usize {
        ((instruction & COMPRESSED_CR_CSS_RS2_MASK)
            >> COMPRESSED_CR_CSS_RS2_OFFSET) as usize
    }

    fn extract_ci_nzimm_5_0(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> SignedImmediate {
        let mut imm = (instruction & COMPRESSED_CI_NZIMM_5_MASK)
            >> (COMPRESSED_CI_NZIMM_5_OFFSET - 5);
        imm |= (instruction & COMPRESSED_CI_NZIMM_4_0_MASK)
            >> (COMPRESSED_CI_NZIMM_4_0_OFFSET - 0);
        imm >>= 0;

        imm.into()
    }

    fn extract_ci_imm_5_0(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> SignedImmediate {
        let mut imm = (instruction & COMPRESSED_CI_IMM_5_MASK)
            >> (COMPRESSED_CI_IMM_5_OFFSET - 5);
        imm |= (instruction & COMPRESSED_CI_CB_IMM_4_0_MASK)
            >> (COMPRESSED_CI_CB_IMM_4_0_OFFSET - 0);
        imm >>= 0;

        imm.into()
    }

    fn extract_ci_nzimm_9_4(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> SignedImmediate {
        let mut imm = (instruction & COMPRESSED_CI_NZIMM_9_MASK)
            >> (COMPRESSED_CI_NZIMM_9_OFFSET - 9);
        imm |= (instruction & COMPRESSED_CI_NZIMM_4_MASK)
            >> (COMPRESSED_CI_NZIMM_4_OFFSET - 4);
        imm |= (instruction & COMPRESSED_CI_NZIMM_6_MASK)
            << (6 - COMPRESSED_CI_NZIMM_6_OFFSET);
        imm |= (instruction & COMPRESSED_CI_NZIMM_8_7_MASK)
            << (7 - COMPRESSED_CI_NZIMM_8_7_OFFSET);
        imm |= (instruction & COMPRESSED_CI_NZIMM_5_9_4_MASK)
            << (5 - COMPRESSED_CI_NZIMM_5_9_4_OFFSET);
        imm >>= 4;

        imm.into()
    }

    fn extract_ci_nzimm_17_12(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> SignedImmediate {
        let mut imm = ((instruction & COMPRESSED_CI_NZIMM_17_MASK)
            as SignedImmediate)
            << (17 - COMPRESSED_CI_NZIMM_17_OFFSET);
        imm |= ((instruction & COMPRESSED_CI_NZIMM_16_12_MASK)
            as SignedImmediate)
            << (12 - COMPRESSED_CI_NZIMM_16_12_OFFSET);
        imm >>= 12;

        imm
    }

    fn extract_ci_shamt(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> InstructionTypeCompressed {
        let mut shamt = (instruction & COMPRESSED_CI_SHAMT_5_MASK)
            >> (COMPRESSED_CI_SHAMT_5_OFFSET - 5);
        shamt |= (instruction & COMPRESSED_CI_SHAMT_4_0_MASK)
            >> (COMPRESSED_CI_SHAMT_4_0_OFFSET - 0);

        shamt
    }

    fn extract_ci_load_word_sp_offset(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> SignedImmediate {
        let mut imm = (instruction & COMPRESSED_CI_OFFSET_5_MASK)
            >> (COMPRESSED_CI_OFFSET_5_OFFSET - 5);
        imm |= (instruction & COMPRESSED_CI_OFFSET_4_2_MASK)
            >> (COMPRESSED_CI_OFFSET_4_2_OFFSET - 2);
        imm |= (instruction & COMPRESSED_CI_OFFSET_7_6_MASK)
            << (6 - COMPRESSED_CI_OFFSET_7_6_OFFSET);
        imm >>= 2;

        imm.into()
    }

    fn extract_ci_load_double_word_sp_offset(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> SignedImmediate {
        let mut imm = (instruction & COMPRESSED_CI_OFFSET_5_MASK)
            >> (COMPRESSED_CI_OFFSET_5_OFFSET - 5);
        imm |= (instruction & COMPRESSED_CI_OFFSET_4_3_MASK)
            >> (COMPRESSED_CI_OFFSET_4_3_OFFSET - 3);
        imm |= (instruction & COMPRESSED_CI_OFFSET_8_6_MASK)
            << (6 - COMPRESSED_CI_OFFSET_8_6_OFFSET);
        imm >>= 3;

        imm.into()
    }

    fn extract_css_uimm_8_3(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> SignedImmediate {
        let mut uimm = (instruction & COMPRESSED_CSS_UIMM_8_6_MASK)
            >> (COMPRESSED_CSS_UIMM_8_6_OFFSET - 6);
        uimm |= (instruction & COMPRESSED_CSS_UIMM_5_3_MASK)
            >> (COMPRESSED_CSS_UIMM_5_3_OFFSET - 3);

        uimm as SignedImmediate
    }

    fn extract_css_uimm_7_2(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> SignedImmediate {
        let mut uimm = (instruction & COMPRESSED_CSS_UIMM_7_6_MASK)
            >> (COMPRESSED_CSS_UIMM_7_6_OFFSET - 6);
        uimm |= (instruction & COMPRESSED_CSS_UIMM_5_2_MASK)
            >> (COMPRESSED_CSS_UIMM_5_2_OFFSET - 3);

        uimm as SignedImmediate
    }

    fn extract_ciw_nzuimm_9_2(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> UnsignedImmediate {
        let mut nzuimm = (instruction & COMPRESSED_CIW_NZUIMM_MASK)
            >> COMPRESSED_CIW_NZUIMM_OFFSET;
        nzuimm = ((nzuimm >> 1) & 0b00000001)  // bit 2
               | ((nzuimm << 1) & 0b00000010)  // bit 3
               | ((nzuimm >> 4) & 0b00001100)  // bits 5-4
               | ((nzuimm << 2) & 0b11110000); // bits 9-6
        nzuimm <<= 2;

        nzuimm.into()
    }

    fn extract_ciw_cl_rd_offset(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> usize {
        ((instruction & COMPRESSED_CIW_CL_RD_OFFSET_MASK)
            >> COMPRESSED_CIW_CL_RD_OFFSET_OFFSET) as usize
            + GPRegisterName::x8_s0_fp as usize
    }

    fn extract_cl_offset_6_2(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> InstructionTypeCompressed {
        let mut offset = (instruction & COMPRESSED_CL_OFFSET_6_MASK)
            << (6 - COMPRESSED_CL_OFFSET_6_OFFSET);
        offset |= (instruction & COMPRESSED_CL_OFFSET_2_MASK)
            >> (COMPRESSED_CL_OFFSET_2_OFFSET - 2);
        offset |= (instruction & COMPRESSED_CL_OFFSET_5_3_MASK)
            >> (COMPRESSED_CL_OFFSET_5_3_OFFSET - 3);

        offset
    }

    fn extract_cl_offset_7_3(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> InstructionTypeCompressed {
        let mut offset = (instruction & COMPRESSED_CL_OFFSET_6_MASK)
            << (6 - COMPRESSED_CL_OFFSET_6_OFFSET);
        offset |= (instruction & COMPRESSED_CL_OFFSET_7_MASK)
            << (7 - COMPRESSED_CL_OFFSET_7_OFFSET);
        offset |= (instruction & COMPRESSED_CL_OFFSET_5_3_MASK)
            >> (COMPRESSED_CL_OFFSET_5_3_OFFSET - 3);

        offset
    }

    fn extract_cl_cs_rs1_offset(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> usize {
        ((instruction & COMPRESSED_CL_CS_RS1_OFFSET_MASK)
            >> COMPRESSED_CL_CS_RS1_OFFSET_OFFSET) as usize
            + GPRegisterName::x8_s0_fp as usize
    }

    fn extract_cs_ca_rs2_offset(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> usize {
        ((instruction & COMPRESSED_CS_CA_RS2_OFFSET_MASK)
            >> COMPRESSED_CS_CA_RS2_OFFSET_OFFSET) as usize
            + GPRegisterName::x8_s0_fp as usize
    }

    fn extract_ca_funct2(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> CA_Funct_2 {
        CA_Funct_2::from(
            (instruction & COMPRESSED_CA_FUNCT_2_MASK)
                >> COMPRESSED_CA_FUNCT_2_OFFSET,
        )
    }

    fn extract_ca_cb_rd_or_rs1_offset(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> usize {
        (((instruction & COMPRESSED_CA_CB_RD_OR_RS1_OFFSET_MASK)
            >> COMPRESSED_CA_CB_RD_OR_RS1_OFFSET_OFFSET)
            + GPRegisterName::x8_s0_fp as InstructionTypeCompressed)
            as usize
    }

    fn extract_ca_cb_bit_12(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> InstructionTypeCompressed {
        let bit_12 = (instruction & COMPRESSED_BITS_12_MASK)
            >> COMPRESSED_BITS_12_OFFSET;

        bit_12
    }

    fn extract_bits_11_10_quadrant_01_funct3_100(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> Bits11_10_quadrant_01 {
        let bits_11_10 = (instruction & COMPRESSED_BITS_11_10_MASK)
            >> COMPRESSED_BITS_11_10_OFFSET;

        Bits11_10_quadrant_01::from(bits_11_10)
    }

    fn extract_cb_uimm_5_0(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> UnsignedImmediate {
        let mut uimm = instruction & COMPRESSED_CI_CB_IMM_4_0_MASK;
        uimm >>= COMPRESSED_CI_CB_IMM_4_0_OFFSET;
        // bit 12 is (u|)imm[5] in case of CB instruction, so shift left by 5 bits
        uimm |= self.compressedISA.ca_cb_bit_12 << 5;

        uimm as UnsignedImmediate
    }

    fn extract_cb_imm_5_0(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> SignedImmediate {
        let mut uimm = instruction & COMPRESSED_CB_UIMM_4_0_MASK;
        uimm >>= COMPRESSED_CB_UIMM_4_0_OFFSET;
        // bit 12 is (u|)imm[5] in case of CB instruction, so shift left by 5 bits
        uimm |= self.compressedISA.ca_cb_bit_12 << 5;

        uimm as SignedImmediate
    }

    fn extract_cb_imm_8_1(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> SignedImmediate {
        let mut uimm = (instruction & COMPRESSED_CB_FUNCT_3_11X_IMM_5_MASK)
            << (5 - COMPRESSED_CB_FUNCT_3_11X_IMM_5_MASK);
        uimm |= (instruction & COMPRESSED_CB_FUNCT_3_11X_IMM_2_1_MASK)
            >> (COMPRESSED_CB_FUNCT_3_11X_IMM_2_1_OFFSET - 1);
        uimm |= (instruction & COMPRESSED_CB_FUNCT_3_11X_IMM_7_6_MASK)
            << (6 - COMPRESSED_CB_FUNCT_3_11X_IMM_7_6_OFFSET);
        uimm |= self.compressedISA.ca_cb_bit_12 << 8; // imm[8]
        uimm |= (self.compressedISA.ca_cb_bits_11_10_quad_01_funct3_100
            as InstructionTypeCompressed)
            << 3; // imm[4:3]

        uimm as SignedImmediate
    }

    fn extract_cj_jta_imm(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> InstructionTypeCompressed {
        let mut jta_imm = (instruction & COMPRESSED_CJ_JTA_IMM_11_MASK)
            >> (COMPRESSED_CJ_JTA_IMM_11_OFFSET - 11);
        jta_imm |= (instruction & COMPRESSED_CJ_JTA_IMM_10_MASK)
            << (10 - COMPRESSED_CJ_JTA_IMM_10_OFFSET);
        jta_imm |= (instruction & COMPRESSED_CJ_JTA_IMM_9_8_MASK)
            >> (COMPRESSED_CJ_JTA_IMM_9_8_OFFSET - 8);
        jta_imm |= (instruction & COMPRESSED_CJ_JTA_IMM_7_MASK)
            << (7 - COMPRESSED_CJ_JTA_IMM_7_OFFSET);
        jta_imm |= (instruction & COMPRESSED_CJ_JTA_IMM_6_MASK)
            >> (COMPRESSED_CJ_JTA_IMM_6_OFFSET - 6);
        jta_imm |= (instruction & COMPRESSED_CJ_JTA_IMM_5_MASK)
            << (5 - COMPRESSED_CJ_JTA_IMM_5_OFFSET);
        jta_imm |= (instruction & COMPRESSED_CJ_JTA_IMM_4_MASK)
            >> (COMPRESSED_CJ_JTA_IMM_4_OFFSET - 4);
        jta_imm |= (instruction & COMPRESSED_CJ_JTA_IMM_3_1_MASK)
            >> (COMPRESSED_CJ_JTA_IMM_3_1_OFFSET - 1);

        jta_imm
    }

    // instruction expanders/decompressors
    fn expand_to_r_format(
        &self,
        opcode: Opcode,
        destination_register_offset: usize,
        funct3: InstructionType,
        source_register_1_offset: usize,
        source_register_2_offset: usize,
        funct7: InstructionType,
    ) -> InstructionType {
        // set opcode
        let mut expanded_instruction: InstructionType =
            opcode as InstructionType;
        // set rd
        expanded_instruction |= (destination_register_offset
            as InstructionType)
            << BIT_OFFSET_DESTINATION_REGISTER;
        // set funct3
        expanded_instruction |= funct3 << BIT_OFFSET_FUNCT3;
        // set rs1
        expanded_instruction |= (source_register_1_offset as InstructionType)
            << BIT_OFFSET_SOURCE_REGISTER_1;
        // set rs2
        expanded_instruction |= (source_register_2_offset as InstructionType)
            << BIT_OFFSET_SOURCE_REGISTER_2;
        // set funct7
        expanded_instruction |= funct7 << BIT_OFFSET_R_FUNCT7;

        expanded_instruction
    }

    fn expand_to_i_format(
        &self,
        opcode: Opcode,
        destination_register_offset: usize,
        funct3: InstructionType,
        source_register_1_offset: usize,
        imm_11_0: SignedImmediate,
    ) -> InstructionType {
        // set opcode
        let mut expanded_instruction: InstructionType =
            opcode as InstructionType;
        // set rd
        expanded_instruction |= (destination_register_offset
            as InstructionType)
            << BIT_OFFSET_DESTINATION_REGISTER;
        // set funct3
        expanded_instruction |= funct3 << BIT_OFFSET_FUNCT3;
        // set rs1
        expanded_instruction |= (source_register_1_offset as InstructionType)
            << BIT_OFFSET_SOURCE_REGISTER_1;
        // set imm_11_0
        expanded_instruction |=
            (imm_11_0 as InstructionType) << BIT_OFFSET_IMM_11_0;

        expanded_instruction
    }

    fn expand_to_s_format(
        &self,
        opcode: Opcode,
        source_register_1_offset: usize,
        source_register_2_offset: usize,
        imm_11_0: SignedImmediate,
        funct3: InstructionType,
    ) -> InstructionType {
        // set opcode
        let mut expanded_instruction: InstructionType =
            opcode as InstructionType;
        // set imm[4:0]
        expanded_instruction |= ((imm_11_0 as InstructionType) & MASK_IMM_4_0)
            << BIT_OFFSET_IMM_4_0;
        // set funct3
        expanded_instruction |= funct3 << BIT_OFFSET_FUNCT3;
        // set rs1
        expanded_instruction |= (source_register_1_offset as InstructionType)
            << BIT_OFFSET_SOURCE_REGISTER_1;
        // set rs2
        expanded_instruction |= (source_register_2_offset as InstructionType)
            << BIT_OFFSET_SOURCE_REGISTER_2;
        // set imm[11:5]
        expanded_instruction |= ((imm_11_0 as InstructionType) & MASK_IMM_11_5)
            << BIT_OFFSET_IMM_11_5;

        expanded_instruction
    }

    fn expand_to_b_format(
        &self,
        opcode: Opcode,
        source_register_1_offset: usize,
        source_register_2_offset: usize,
        imm_12_1: SignedImmediate,
        funct3: InstructionType,
    ) -> InstructionType {
        // set opcode
        let mut expanded_instruction: InstructionType =
            opcode as InstructionType;
        // set imm[11]
        expanded_instruction |= ((imm_12_1 as InstructionType)
            & MASK_B_FORMAT_IMM_11)
            << BIT_OFFSET_B_FORMAT_IMM_11;
        // set imm[4:1]
        expanded_instruction |= ((imm_12_1 as InstructionType)
            & MASK_B_FORMAT_IMM_4_1)
            << BIT_OFFSET_B_FORMAT_IMM_4_1;
        // set funct3
        expanded_instruction |= funct3 << BIT_OFFSET_FUNCT3;
        // set rs1
        expanded_instruction |= (source_register_1_offset as InstructionType)
            << BIT_OFFSET_SOURCE_REGISTER_1;
        // set rs2
        expanded_instruction |= (source_register_2_offset as InstructionType)
            << BIT_OFFSET_SOURCE_REGISTER_2;
        // set imm[10:5]
        expanded_instruction |= ((imm_12_1 as InstructionType)
            & MASK_B_FORMAT_IMM_10_5)
            << BIT_OFFSET_B_FORMAT_IMM_10_5;
        // set imm[11]
        expanded_instruction |= ((imm_12_1 as InstructionType)
            & MASK_B_FORMAT_IMM_12)
            << BIT_OFFSET_B_FORMAT_IMM_12;

        expanded_instruction
    }

    fn expand_to_u_format(
        &self,
        opcode: Opcode,
        destination_register_offset: usize,
        imm_31_12: SignedImmediate,
    ) -> InstructionType {
        // set opcode
        let mut expanded_instruction: InstructionType =
            opcode as InstructionType;
        // set rd
        expanded_instruction |= (destination_register_offset
            as InstructionType)
            << BIT_OFFSET_DESTINATION_REGISTER;
        // set imm_31_12
        expanded_instruction |=
            (imm_31_12 as InstructionType) << BIT_OFFSET_IMM_31_12;

        expanded_instruction
    }

    // fn expand_to_j_format(
    //     &self,
    //     opcode: Opcode,
    //     destination_register_offset: usize,
    //     funct3: OP_IMM_Funct3,
    //     source_register_1_offset: usize,
    //     imm_11_0: SignedImmediate,
    // ) -> InstructionType {
    //     //
    // }

    fn expand_to_fldsp(&self) -> InstructionType {
        // C.FLDSP expands into `fld rd, offset(x2)`
        // set opcode
        // let mut expanded_instruction: InstructionType =
        //     Opcode:: as InstructionType;

        // expanded_instruction
        todo!()
    }

    fn expand_to_flwsp(&self) -> InstructionType {
        // // C.FLWSP expands into `flw rd, offset(x2)`
        // // set opcode
        // let mut expanded_instruction: InstructionType =
        //     Opcode:: as InstructionType;

        // expanded_instruction
        todo!()
    }

    fn expand_to_fld(&self) -> InstructionType {
        // // C.FLD expands into `fld rd', offset(rs1')`
        // // set opcode
        // let mut expanded_instruction: InstructionType =
        //     Opcode:: as InstructionType;

        // expanded_instruction
        todo!()
    }

    fn expand_to_flw(&self) -> InstructionType {
        // // C.FLW expands into `flw rd', offset(rs1')`
        // // set opcode
        // let mut expanded_instruction: InstructionType =
        //     Opcode:: as InstructionType;

        // expanded_instruction
        todo!()
    }

    fn expand_to_fsd(&self) -> InstructionType {
        // // C.FSD expands into `fsd rs2', offset(rs1')`
        // // set opcode
        // let mut expanded_instruction: InstructionType =
        //     Opcode:: as InstructionType;

        // expanded_instruction
        todo!()
    }

    fn expand_to_sw(&self) -> InstructionType {
        // // C.SW expands into `sw rs2', offset(rs1')`
        // // set opcode
        // let mut expanded_instruction: InstructionType =
        //     Opcode:: as InstructionType;

        // expanded_instruction
        todo!()
    }

    fn expand_to_sd(&self) -> InstructionType {
        // // C.SD expands into `sd rs2', offset(rs1')`
        // // set opcode
        // let mut expanded_instruction: InstructionType =
        //     Opcode:: as InstructionType;

        // expanded_instruction
        todo!()
    }

    fn expand_to_fsw(&self) -> InstructionType {
        // // C.FSW expands into `fsw rs2', offset(rs1')`
        // // set opcode
        // let mut expanded_instruction: InstructionType =
        //     Opcode:: as InstructionType;

        // expanded_instruction
        todo!()
    }
}
