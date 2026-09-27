use log::{error, trace};

use crate::constants::arch::{
    BIT_OFFSET_DESTINATION_REGISTER, BIT_OFFSET_FUNCT3, BIT_OFFSET_FUNCT12,
    BIT_OFFSET_IMM_11_0, BIT_OFFSET_R_FUNCT7, BIT_OFFSET_SOURCE_REGISTER_1,
    BIT_OFFSET_SOURCE_REGISTER_2, InstructionType, RISCV_NUM_OF_REGISTERS,
    XLen,
};
use crate::soc::hart::core::decoder::constants::{
    OP_Funct7, OP_IMM_Funct3, Opcode, OpcodeQuadrant, SYSTEM_Funct3,
    SYSTEM_Imm_transfer_control_return_from,
};
use crate::soc::hart::core::gp_registers::RegisterFile;
use crate::soc::hart::core::gp_registers::constants::GPRegisterName;
use crate::soc::hart::core::registers::RegisterFileInterface;
use crate::soc::hart::core::{decoder::Decoder, fetch_unit::FetchUnit};
use crate::soc::hart::extensions::compressed;
use crate::soc::hart::extensions::compressed::constants::{
    Bits11_10_quadrant_01, COMPRESSED_BITS_11_10_MASK,
    COMPRESSED_BITS_11_10_OFFSET, COMPRESSED_CA_CB_RD_OR_RS1_SCALED_MASK,
    COMPRESSED_CA_CB_RD_OR_RS1_SCALED_OFFSET, COMPRESSED_CI_IMM_4_0_MASK,
    COMPRESSED_CI_IMM_4_0_OFFSET, COMPRESSED_CI_IMM_5_MASK,
    COMPRESSED_CI_IMM_5_OFFSET, COMPRESSED_CI_NZIMM_4_0_MASK,
    COMPRESSED_CI_NZIMM_4_0_OFFSET, COMPRESSED_CI_NZIMM_4_MASK,
    COMPRESSED_CI_NZIMM_4_OFFSET, COMPRESSED_CI_NZIMM_5_9_4_MASK,
    COMPRESSED_CI_NZIMM_5_9_4_OFFSET, COMPRESSED_CI_NZIMM_5_MASK,
    COMPRESSED_CI_NZIMM_5_OFFSET, COMPRESSED_CI_NZIMM_6_MASK,
    COMPRESSED_CI_NZIMM_6_OFFSET, COMPRESSED_CI_NZIMM_8_7_MASK,
    COMPRESSED_CI_NZIMM_8_7_OFFSET, COMPRESSED_CI_NZIMM_9_MASK,
    COMPRESSED_CI_NZIMM_9_OFFSET, COMPRESSED_CIW_CL_RD_SCALED_MASK,
    COMPRESSED_CIW_CL_RD_SCALED_OFFSET, COMPRESSED_CIW_IMM_MASK,
    COMPRESSED_CIW_IMM_OFFSET, COMPRESSED_CJ_JTA_IMM_MASK,
    COMPRESSED_CJ_JTA_IMM_OFFSET, COMPRESSED_CL_CS_RS1_SCALED_MASK,
    COMPRESSED_CL_CS_RS1_SCALED_OFFSET, COMPRESSED_CR_CI_RD_RS1_MASK,
    COMPRESSED_CR_CI_RD_RS1_OFFSET, COMPRESSED_CR_CSS_RS2_MASK,
    COMPRESSED_CR_CSS_RS2_OFFSET, COMPRESSED_CS_CA_RS2_SCALED_MASK,
    COMPRESSED_CS_CA_RS2_SCALED_OFFSET, COMPRESSED_FUNCT3_MASK,
    COMPRESSED_FUNCT4_MASK, COMPRESSED_FUNCT4_OFFSET,
    InstructionTypeCompressed,
};
use crate::soc::hart::extensions::integer;

#[derive(Clone)]
pub struct DecoderCompressedISAContext {
    funct3: compressed::constants::OP_Funct3,
    cr_funct4_lsb_compressed: bool,
    cr_ci_destination_or_source_register_1_offset: usize,
    cr_css_source_register_2_offset: usize,
    ci_imm: InstructionTypeCompressed,
    ciw_nzuimm: InstructionTypeCompressed,
    ciw_cl_rd_scaled: InstructionTypeCompressed,
    cl_cs_rs1_scaled: InstructionTypeCompressed,
    cs_ca_rs2_scaled: InstructionTypeCompressed,
    ca_cb_rd_or_rs1_scaled: InstructionTypeCompressed,
    ca_cb_bits_11_10_quadrant_01_funct3_100: Bits11_10_quadrant_01,
    cj_jta_imm: InstructionTypeCompressed,
}

impl Default for DecoderCompressedISAContext {
    fn default() -> Self {
        DecoderCompressedISAContext {
            funct3: compressed::constants::OP_Funct3::F000,
            cr_funct4_lsb_compressed: false,
            cr_ci_destination_or_source_register_1_offset:
                GPRegisterName::x0_zero as usize,
            cr_css_source_register_2_offset: GPRegisterName::x0_zero as usize,
            ci_imm: 0,
            ciw_nzuimm: 0,
            ciw_cl_rd_scaled: 0,
            cl_cs_rs1_scaled: 0,
            cs_ca_rs2_scaled: 0,
            ca_cb_rd_or_rs1_scaled: 0,
            ca_cb_bits_11_10_quadrant_01_funct3_100:
                Bits11_10_quadrant_01::CA_11,
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
                compressed::constants::OP_Funct3::F000 => {
                    self.expand_ciw(fetch_unit.current_instruction_compressed)
                }
                compressed::constants::OP_Funct3::F001
                | compressed::constants::OP_Funct3::F010
                | compressed::constants::OP_Funct3::F011 => {
                    self.expand_cl(fetch_unit.current_instruction_compressed)
                }
                compressed::constants::OP_Funct3::F101
                | compressed::constants::OP_Funct3::F110
                | compressed::constants::OP_Funct3::F111 => {
                    self.expand_cs(fetch_unit.current_instruction_compressed)
                }

                compressed::constants::OP_Funct3::F100 => {
                    error!("illegal instruction, reserved by specification");
                    todo!("illegal instruction, reserved by specification");
                }
            },
            OpcodeQuadrant::Quadrant01 => {
                match self.compressedISA.funct3 {
                    compressed::constants::OP_Funct3::F000
                    | compressed::constants::OP_Funct3::F010
                    | compressed::constants::OP_Funct3::F011 => {
                        self.expand_ci(fetch_unit)
                    }
                    compressed::constants::OP_Funct3::F001 => {
                        todo!("distiguish between RV32/RV64");
                    }
                    compressed::constants::OP_Funct3::F100 => {
                        self.compressedISA
                            .ca_cb_bits_11_10_quadrant_01_funct3_100 = self
                            .extract_bits_11_10_quadrant_01_funct3_100(
                                fetch_unit.current_instruction_compressed,
                            );
                        match self
                            .compressedISA
                            .ca_cb_bits_11_10_quadrant_01_funct3_100
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
                    compressed::constants::OP_Funct3::F101 => self
                        .expand_cj(fetch_unit.current_instruction_compressed),
                    compressed::constants::OP_Funct3::F110
                    | compressed::constants::OP_Funct3::F111 => self
                        .expand_cb(fetch_unit.current_instruction_compressed),
                }
            }
            OpcodeQuadrant::Quadrant10 => {
                match self.compressedISA.funct3 {
                    compressed::constants::OP_Funct3::F000
                    | compressed::constants::OP_Funct3::F001
                    | compressed::constants::OP_Funct3::F010
                    | compressed::constants::OP_Funct3::F011 => {
                        self.expand_ci(fetch_unit)
                    }
                    compressed::constants::OP_Funct3::F100 => self
                        .expand_cr(fetch_unit.current_instruction_compressed),
                    compressed::constants::OP_Funct3::F101
                    | compressed::constants::OP_Funct3::F110
                    | compressed::constants::OP_Funct3::F111 => self
                        .expand_css(fetch_unit.current_instruction_compressed),
                }
            }

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
        trace!("Decoding CR-format instruction..");

        self.compressedISA.cr_funct4_lsb_compressed =
            self.extract_cr_funct4_lsb_compressed(instruction);
        self.compressedISA
            .cr_ci_destination_or_source_register_1_offset = self
            .extract_cr_ci_destination_or_source_register_1_offset(instruction);
        self.compressedISA.cr_css_source_register_2_offset =
            self.extract_cr_css_source_register_2_offset(instruction);

        if self.compressedISA.cr_funct4_lsb_compressed {
            if self.compressedISA.cr_css_source_register_2_offset == 0 {
                if self
                    .compressedISA
                    .cr_ci_destination_or_source_register_1_offset
                    != 0
                {
                    // C.JALR
                    self.expand_to_jalr()
                } else {
                    // C.EBREAK
                    self.expand_to_ebreak()
                }
            } else {
                // the code points with rd == x0 are HINTs
                if self
                    .compressedISA
                    .cr_ci_destination_or_source_register_1_offset
                    == 0
                {
                    trace!("hint was met");
                }
                // C.ADD
                self.expand_to_add()
            }
        } else {
            if self.compressedISA.cr_css_source_register_2_offset == 0 {
                // C.JR is valid only when rs1 != x0; the code point with rs1 = x0 is reserved.
                if self
                    .compressedISA
                    .cr_ci_destination_or_source_register_1_offset
                    != 0
                {
                    // C.JR
                    self.expand_to_jr()
                } else {
                    error!(
                        "illegal instruction trap, reserved by specification"
                    );
                    todo!(
                        "illegal instruction trap, reserved by specification"
                    );
                }
            } else {
                // C.MV is valid only when rs2 != x0; the code points with rd == x0 are HINTs
                self.destination_register_offset = self
                    .compressedISA
                    .cr_ci_destination_or_source_register_1_offset;
                if self.destination_register_offset == 0 {
                    trace!("hint was met");
                }
                // C.MV
                self.expand_to_mv()
            }
        }
    }

    fn expand_ci(&mut self, fetch_unit: &FetchUnit) -> InstructionType {
        trace!("Decoding CI-format instruction..");

        self.compressedISA
            .cr_ci_destination_or_source_register_1_offset = self
            .extract_cr_ci_destination_or_source_register_1_offset(
                fetch_unit.current_instruction_compressed,
            );

        match self.compressedISA.funct3 {
            compressed::constants::OP_Funct3::F000 => {
                match fetch_unit.current_quadrant {
                    OpcodeQuadrant::Quadrant01 => {
                        self.compressedISA.ci_imm = self.extract_ci_nzimm_5_0(
                            fetch_unit.current_instruction_compressed,
                        );
                        self.expand_to_addi()
                    }
                    OpcodeQuadrant::Quadrant10 => {
                        //
                        self.expand_to_slli()
                    }

                    _ => {
                        error!("unreachable opcode for CI instruction");
                        unreachable!("unreachable opcode for CI instruction");
                    }
                }
            }
            compressed::constants::OP_Funct3::F001 => {
                match fetch_unit.current_quadrant {
                    OpcodeQuadrant::Quadrant01 => {
                        self.compressedISA.ci_imm = self.extract_ci_imm_5_0(
                            fetch_unit.current_instruction_compressed,
                        );
                        self.expand_to_addiw()
                    }
                    OpcodeQuadrant::Quadrant10 => self.expand_to_fldsp(),

                    _ => {
                        error!("unreachable opcode for CI instruction");
                        unreachable!("unreachable opcode for CI instruction");
                    }
                }
            }
            compressed::constants::OP_Funct3::F010 => {
                match fetch_unit.current_quadrant {
                    OpcodeQuadrant::Quadrant01 => {
                        //
                        self.expand_to_li()
                    }
                    OpcodeQuadrant::Quadrant10 => {
                        //
                        self.expand_to_lwsp()
                    }

                    _ => {
                        error!("unreachable opcode for CI instruction");
                        unreachable!("unreachable opcode for CI instruction");
                    }
                }
            }
            compressed::constants::OP_Funct3::F011 => {
                match fetch_unit.current_quadrant {
                    OpcodeQuadrant::Quadrant01 => {
                        todo!("check unimplemented");
                        // if self.destination_register_offset
                        self.expand_to_lui()
                        // else {
                        // self.compressedISA.ci_imm = self.extract_ci_nzimm_9_4(fetch_unit.current_instruction_compressed);
                        // self.expand_to_addi16sp(instruction)
                        // }
                    }
                    OpcodeQuadrant::Quadrant10 => {
                        todo!("check unimplemented");
                        // if RV32FC
                        self.expand_to_flwsp()
                        // else if RV64C
                        // self.expand_to_ldsp(instruction)
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
        trace!("Decoding CSS-format instruction..");

        self.compressedISA.cr_css_source_register_2_offset =
            self.extract_cr_css_source_register_2_offset(instruction);

        todo!("TODO: unimplemented");
    }

    fn expand_ciw(
        &mut self,
        instruction: InstructionTypeCompressed,
    ) -> InstructionType {
        trace!("Decoding CIW-format instruction..");

        // C.ADDI4SPN (RESERVED when uimm == 0)
        self.compressedISA.ciw_nzuimm = self.extract_ciw_uimm(instruction);
        self.compressedISA.ciw_cl_rd_scaled =
            self.extract_ciw_cl_rd_scaled(instruction);

        if self.compressedISA.ciw_nzuimm != 0 {
            self.expand_to_addi4spn()
        } else {
            // RESERVED, uimm == 0
            error!("illegal instruction trap, reserved by specification");
            todo!("illegal instruction trap, reserved by specification");
        }
    }

    fn expand_cl(
        &mut self,
        instruction: InstructionTypeCompressed,
    ) -> InstructionType {
        trace!("Decoding CL-format instruction..");

        self.compressedISA.ciw_cl_rd_scaled =
            self.extract_ciw_cl_rd_scaled(instruction);
        self.compressedISA.cl_cs_rs1_scaled =
            self.extract_cl_cs_rs1_scaled(instruction);

        match self.compressedISA.funct3 {
            compressed::constants::OP_Funct3::F001 => {
                todo!("condition unimplemented yet");
                // if RV(32|64)DC {
                self.expand_to_fld()
                // }
            }
            compressed::constants::OP_Funct3::F010 => self.expand_to_lw(),
            compressed::constants::OP_Funct3::F011 => {
                todo!("condition unimplemented yet");
                // if RV32IFC {
                self.expand_to_flw()
                // } else { // RV64IC only
                // self.expand_to_ld()
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
        trace!("Decoding CS-format instruction..");

        self.compressedISA.cl_cs_rs1_scaled =
            self.extract_cl_cs_rs1_scaled(instruction);
        self.compressedISA.cs_ca_rs2_scaled =
            self.extract_cs_ca_rs2_scaled(instruction);

        match self.compressedISA.funct3 {
            compressed::constants::OP_Funct3::F101 => {
                todo!("condition unimplemented yet");
                // if RV(32|64)DC {
                self.expand_to_fsd()
                // } else {
                // todo!();
                // }
            }
            compressed::constants::OP_Funct3::F110 => self.expand_to_sw(),
            compressed::constants::OP_Funct3::F111 => {
                todo!("condition unimplemented yet");
                // if RV32IFC {
                self.expand_to_fsw()
                // } else { // RV64IC only
                // self.expand_to_sd()
                // }
            }

            _ => {
                error!(
                    "unreachable funct3 ({}) for CS instruction",
                    self.compressedISA.funct3
                );
                unreachable!(
                    "unreachable funct3 ({}) for CS instruction",
                    self.compressedISA.funct3
                );
            }
        }
    }

    fn expand_ca(
        &mut self,
        instruction: InstructionTypeCompressed,
    ) -> InstructionType {
        trace!("Decoding CA-format instruction..");

        self.compressedISA.cs_ca_rs2_scaled =
            self.extract_cs_ca_rs2_scaled(instruction);
        self.compressedISA.ca_cb_rd_or_rs1_scaled =
            self.extract_ca_cb_rd_or_rs1_scaled(instruction);

        todo!("TODO: unimplemented");
    }

    fn expand_cb(
        &mut self,
        instruction: InstructionTypeCompressed,
    ) -> InstructionType {
        trace!("Decoding CB-format instruction..");

        self.compressedISA.ca_cb_rd_or_rs1_scaled =
            self.extract_ca_cb_rd_or_rs1_scaled(instruction);

        todo!("TODO: unimplemented");
    }

    fn expand_cj(
        &mut self,
        instruction: InstructionTypeCompressed,
    ) -> InstructionType {
        trace!("Decoding CJ-format instruction..");

        self.compressedISA.cj_jta_imm = self.extract_cj_jta_imm(instruction);

        //

        todo!("TODO: unimplemented");
    }

    // extractors
    fn extract_funct3_compressed(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> compressed::constants::OP_Funct3 {
        compressed::constants::OP_Funct3::from(
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
    ) -> InstructionTypeCompressed {
        let mut imm = (instruction & COMPRESSED_CI_NZIMM_5_MASK)
            >> (COMPRESSED_CI_NZIMM_5_OFFSET - 5);
        imm |= (instruction & COMPRESSED_CI_NZIMM_4_0_MASK)
            >> (COMPRESSED_CI_NZIMM_4_0_OFFSET - 0);

        imm
    }

    fn extract_ci_imm_5_0(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> InstructionTypeCompressed {
        let mut imm = (instruction & COMPRESSED_CI_IMM_5_MASK)
            >> (COMPRESSED_CI_IMM_5_OFFSET - 5);
        imm |= (instruction & COMPRESSED_CI_IMM_4_0_MASK)
            >> (COMPRESSED_CI_IMM_4_0_OFFSET - 0);

        imm
    }

    fn extract_ci_nzimm_9_4(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> InstructionTypeCompressed {
        let mut imm = (instruction & COMPRESSED_CI_NZIMM_9_MASK)
            >> (COMPRESSED_CI_NZIMM_9_OFFSET - 9);
        imm |= (instruction & COMPRESSED_CI_NZIMM_4_MASK)
            >> (COMPRESSED_CI_NZIMM_4_OFFSET - 4);
        imm |= (instruction & COMPRESSED_CI_NZIMM_6_MASK)
            << (6 - COMPRESSED_CI_NZIMM_6_OFFSET);
        imm |= (instruction & COMPRESSED_CI_NZIMM_8_7_MASK)
            << (7 - COMPRESSED_CI_NZIMM_8_7_OFFSET);
        imm |= (instruction & COMPRESSED_CI_NZIMM_5_9_4_MASK)
            << (4 - COMPRESSED_CI_NZIMM_5_9_4_OFFSET);

        imm
    }

    fn extract_ciw_uimm(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> InstructionTypeCompressed {
        let mut nzuimm = (instruction & COMPRESSED_CIW_IMM_MASK)
            >> COMPRESSED_CIW_IMM_OFFSET;
        nzuimm = ((nzuimm >> 1) & 0b00000001)  // bit 2
               | ((nzuimm << 1) & 0b00000010)  // bit 3
               | ((nzuimm >> 4) & 0b00001100)  // bits 5-4
               | ((nzuimm << 2) & 0b11110000); // bits 9-6

        nzuimm
    }

    fn extract_ciw_cl_rd_scaled(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> InstructionTypeCompressed {
        (instruction & COMPRESSED_CIW_CL_RD_SCALED_MASK)
            >> (COMPRESSED_CIW_CL_RD_SCALED_OFFSET - 2)
    }

    fn extract_cl_cs_rs1_scaled(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> InstructionTypeCompressed {
        (instruction & COMPRESSED_CL_CS_RS1_SCALED_MASK)
            >> (COMPRESSED_CL_CS_RS1_SCALED_OFFSET - 2)
    }

    fn extract_cs_ca_rs2_scaled(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> InstructionTypeCompressed {
        (instruction & COMPRESSED_CS_CA_RS2_SCALED_MASK)
            >> (COMPRESSED_CS_CA_RS2_SCALED_OFFSET - 2)
    }

    fn extract_ca_cb_rd_or_rs1_scaled(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> InstructionTypeCompressed {
        (instruction & COMPRESSED_CA_CB_RD_OR_RS1_SCALED_MASK)
            >> (COMPRESSED_CA_CB_RD_OR_RS1_SCALED_OFFSET - 2)
    }

    fn extract_cj_jta_imm(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> InstructionTypeCompressed {
        let mut jta_imm = (instruction & COMPRESSED_CJ_JTA_IMM_MASK)
            >> COMPRESSED_CJ_JTA_IMM_OFFSET;
        todo!("move bits");
        // jta_imm =

        jta_imm
    }

    fn extract_bits_11_10_quadrant_01_funct3_100(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> Bits11_10_quadrant_01 {
        let bits_11_10 = (instruction & COMPRESSED_BITS_11_10_MASK)
            >> COMPRESSED_BITS_11_10_OFFSET;

        Bits11_10_quadrant_01::from(bits_11_10)
    }

    // instruction expanders/decompressors
    fn expand_to_jalr(&self) -> InstructionType {
        // C.JALR expands to `jalr x1, 0(rs1)`
        // set opcode
        let mut expanded_instruction: InstructionType =
            Opcode::JALR as InstructionType;
        // set rd
        expanded_instruction |= (GPRegisterName::x1_ra as InstructionType)
            << BIT_OFFSET_DESTINATION_REGISTER;
        // funct3 for JALR is zero
        // set rs1
        expanded_instruction |= self
            .compressedISA
            .cr_ci_destination_or_source_register_1_offset
            as InstructionType;
        // immediate for C.JALR is zero

        expanded_instruction
    }

    fn expand_to_ebreak(&self) -> InstructionType {
        // C.EBREAK expands to `ebreak`
        // set opcode
        let mut expanded_instruction: InstructionType =
            Opcode::SYSTEM as InstructionType;
        // set rd
        expanded_instruction |= (GPRegisterName::x0_zero as InstructionType)
            << BIT_OFFSET_DESTINATION_REGISTER;
        // set funct3
        expanded_instruction |= (SYSTEM_Funct3::TRANSFER_CONTROL_RETURN_FROM
            as InstructionType)
            << BIT_OFFSET_FUNCT3;
        // set rs1
        expanded_instruction |= (GPRegisterName::x0_zero as InstructionType)
            << BIT_OFFSET_SOURCE_REGISTER_1;
        // set funct12
        expanded_instruction |= (SYSTEM_Imm_transfer_control_return_from::EBREAK
            as InstructionType)
            << BIT_OFFSET_FUNCT12;

        expanded_instruction
    }

    fn expand_to_add(&self) -> InstructionType {
        // C.ADD expands to `add rd, rd, rs2`
        // set opcode
        let mut expanded_instruction: InstructionType =
            Opcode::OP as InstructionType;
        // set rd
        expanded_instruction |= (self
            .compressedISA
            .cr_ci_destination_or_source_register_1_offset
            as InstructionType)
            << BIT_OFFSET_DESTINATION_REGISTER;
        // set funct3
        expanded_instruction |= (integer::constants::OP_Funct3::ARITH
            as InstructionType)
            << BIT_OFFSET_FUNCT3;
        // set rs1
        expanded_instruction |= (self
            .compressedISA
            .cr_ci_destination_or_source_register_1_offset
            as InstructionType)
            << BIT_OFFSET_SOURCE_REGISTER_1;
        // set rs2
        expanded_instruction |=
            (self.compressedISA.cr_css_source_register_2_offset
                as InstructionType)
                << BIT_OFFSET_SOURCE_REGISTER_2;
        // set funct7
        expanded_instruction |=
            (OP_Funct7::F0 as InstructionType) << BIT_OFFSET_R_FUNCT7;

        expanded_instruction
    }

    fn expand_to_jr(&self) -> InstructionType {
        // C.JR expands to `jalr x0, 0(rs1)`
        // set opcode
        let mut expanded_instruction: InstructionType =
            Opcode::JALR as InstructionType;
        // set rd
        expanded_instruction |= (GPRegisterName::x0_zero as InstructionType)
            << BIT_OFFSET_DESTINATION_REGISTER;
        // funct3 for JALR is zero
        // set rs1
        expanded_instruction |= self
            .compressedISA
            .cr_ci_destination_or_source_register_1_offset
            as InstructionType;
        // immediate for C.JR is zero

        expanded_instruction
    }

    fn expand_to_mv(&self) -> InstructionType {
        // C.MV expands into `add rd, x0, rs2`
        // set opcode
        let mut expanded_instruction: InstructionType =
            Opcode::OP as InstructionType;
        // set rd
        expanded_instruction |= (self
            .compressedISA
            .cr_ci_destination_or_source_register_1_offset
            as InstructionType)
            << BIT_OFFSET_DESTINATION_REGISTER;
        // set funct3
        expanded_instruction |= (integer::constants::OP_Funct3::ARITH
            as InstructionType)
            << BIT_OFFSET_FUNCT3;
        // set rs1
        expanded_instruction |= (GPRegisterName::x0_zero as InstructionType)
            << BIT_OFFSET_SOURCE_REGISTER_1;
        // set rs2
        expanded_instruction |=
            (self.compressedISA.cr_css_source_register_2_offset
                as InstructionType)
                << BIT_OFFSET_SOURCE_REGISTER_2;
        // set funct7
        expanded_instruction |=
            (OP_Funct7::F0 as InstructionType) << BIT_OFFSET_R_FUNCT7;

        expanded_instruction
    }

    fn expand_to_addi(&self) -> InstructionType {
        // C.ADDI expands into `addi rd, rd, imm`
        // set opcode
        let mut expanded_instruction: InstructionType =
            Opcode::OP_IMM as InstructionType;
        // set rd
        expanded_instruction |= (self
            .compressedISA
            .cr_ci_destination_or_source_register_1_offset
            as InstructionType)
            << BIT_OFFSET_DESTINATION_REGISTER;
        // set funct3
        expanded_instruction |=
            (OP_IMM_Funct3::ADDI as InstructionType) << BIT_OFFSET_FUNCT3;
        // set rs1
        expanded_instruction |= (self
            .compressedISA
            .cr_ci_destination_or_source_register_1_offset
            as InstructionType)
            << BIT_OFFSET_SOURCE_REGISTER_1;
        // set imm_11_0
        expanded_instruction |= (self.compressedISA.ci_imm as InstructionType)
            << BIT_OFFSET_IMM_11_0;

        expanded_instruction
    }

    fn expand_to_slli(&self) -> InstructionType {
        todo!()
    }

    fn expand_to_addiw(&self) -> InstructionType {
        todo!()
    }

    fn expand_to_fldsp(&self) -> InstructionType {
        todo!()
    }

    fn expand_to_li(&self) -> InstructionType {
        todo!()
    }

    fn expand_to_lwsp(&self) -> InstructionType {
        todo!()
    }

    fn expand_to_lui(&self) -> InstructionType {
        todo!()
    }

    fn expand_to_addi16sp(&self) -> InstructionType {
        todo!()
    }

    fn expand_to_flwsp(&self) -> InstructionType {
        todo!()
    }

    fn expand_to_ldsp(&self) -> InstructionType {
        todo!()
    }

    fn expand_to_addi4spn(&self) -> InstructionType {
        todo!()
    }

    fn expand_to_fld(&self) -> InstructionType {
        todo!()
    }

    fn expand_to_lw(&self) -> InstructionType {
        todo!()
    }

    fn expand_to_flw(&self) -> InstructionType {
        todo!()
    }

    fn expand_to_ld(&self) -> InstructionType {
        todo!()
    }

    fn expand_to_fsd(&self) -> InstructionType {
        todo!()
    }

    fn expand_to_sw(&self) -> InstructionType {
        todo!()
    }

    fn expand_to_fsw(&self) -> InstructionType {
        todo!()
    }

    fn expand_to_sd(&self) -> InstructionType {
        todo!()
    }
}

// fn expand_compressed_instruction(
//     compressed_instruction: InstructionTypeCompressed,
// ) -> Option<InstructionType> {
//     None
// }
