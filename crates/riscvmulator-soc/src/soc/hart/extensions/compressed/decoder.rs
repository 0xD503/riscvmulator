use log::{error, trace};

use crate::soc::hart::core::decoder::constants::OpcodeQuadrant;
use crate::soc::hart::core::executor::constants::ExecutableCommand;
use crate::soc::hart::core::gp_registers::constants::GPRegisterName;
use crate::soc::hart::core::{
    ArchState, decoder::Decoder, fetch_unit::FetchUnit,
};
use crate::soc::hart::extensions::compressed::constants::{
    Bits11_10_quadrant_01, COMPRESSED_BITS_11_10_MASK,
    COMPRESSED_BITS_11_10_OFFSET, COMPRESSED_CIW_CL_RD_SCALED_MASK,
    COMPRESSED_CIW_IMM_MASK, COMPRESSED_CIW_IMM_OFFSET,
    COMPRESSED_CR_CI_RD_RS1_MASK, COMPRESSED_CR_CI_RD_RS1_OFFSET,
    COMPRESSED_CR_CSS_RS2_MASK, COMPRESSED_CR_CSS_RS2_OFFSET,
    COMPRESSED_FUNCT3_MASK, COMPRESSED_FUNCT4_MASK, COMPRESSED_FUNCT4_OFFSET,
    InstructionTypeCompressed, OP_Funct3,
};

#[derive(Clone)]
pub struct DecoderCompressedISAContext {
    funct3: OP_Funct3,
    cr_funct4_lsb_compressed: bool,
    cr_ci_destination_or_source_register_1_offset: usize,
    cr_css_source_register_2_offset: usize,
    ciw_nzuimm: InstructionTypeCompressed,
    ciw_cl_rd_scaled: InstructionTypeCompressed,
}

impl Default for DecoderCompressedISAContext {
    fn default() -> Self {
        DecoderCompressedISAContext {
            funct3: OP_Funct3::F000,
            cr_funct4_lsb_compressed: false,
            cr_ci_destination_or_source_register_1_offset:
                GPRegisterName::x0_zero as usize,
            cr_css_source_register_2_offset: GPRegisterName::x0_zero as usize,
            ciw_nzuimm: 0,
            ciw_cl_rd_scaled: 0,
        }
    }
}

impl Decoder {
    pub(crate) fn decode_c(
        &mut self,
        arch_state: &ArchState,
        fetch_unit: &FetchUnit,
    ) {
        self.compressedISA.funct3 = self.extract_funct3_compressed(
            fetch_unit.current_instruction_compressed,
        );

        match fetch_unit.current_quadrant {
            OpcodeQuadrant::Quadrant00 => match self.compressedISA.funct3 {
                OP_Funct3::F000 => {
                    self.decode_ciw(fetch_unit);
                }
                OP_Funct3::F001 | OP_Funct3::F010 | OP_Funct3::F011 => {
                    self.decode_cl(fetch_unit);
                }
                OP_Funct3::F101 | OP_Funct3::F110 | OP_Funct3::F111 => {
                    self.decode_cs(fetch_unit);
                }

                OP_Funct3::F100 => {
                    error!("illegal instruction, reserved by specification");
                    todo!("illegal instruction, reserved by specification");
                }
            },
            OpcodeQuadrant::Quadrant01 => match self.compressedISA.funct3 {
                OP_Funct3::F000 | OP_Funct3::F010 | OP_Funct3::F011 => {
                    self.decode_ci(fetch_unit);
                }
                OP_Funct3::F001 => {
                    todo!("distiguish between RV32/RV64");
                }
                OP_Funct3::F100 => {
                    let bits_11_10 = self
                        .extract_bits_11_10_quadrant_01_funct3_100(
                            fetch_unit.current_instruction_compressed,
                        );
                    match bits_11_10 {
                        Bits11_10_quadrant_01::CA_11 => {
                            self.decode_ca(fetch_unit);
                        }
                        Bits11_10_quadrant_01::CB_00
                        | Bits11_10_quadrant_01::CB_01
                        | Bits11_10_quadrant_01::CB_10 => {
                            self.decode_cb(fetch_unit);
                        }
                    }
                }
                OP_Funct3::F101 => {
                    self.decode_cj(fetch_unit);
                }
                OP_Funct3::F110 | OP_Funct3::F111 => {
                    self.decode_cb(fetch_unit);
                }
            },
            OpcodeQuadrant::Quadrant10 => match self.compressedISA.funct3 {
                OP_Funct3::F000
                | OP_Funct3::F001
                | OP_Funct3::F010
                | OP_Funct3::F011 => {
                    self.decode_ci(fetch_unit);
                }
                OP_Funct3::F100 => {
                    self.decode_cr(fetch_unit);
                }
                OP_Funct3::F101 | OP_Funct3::F110 | OP_Funct3::F111 => {
                    self.decode_css(fetch_unit);
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

    fn decode_cr(&mut self, fetch_unit: &FetchUnit) {
        trace!("Decoding CR-format instruction..");

        self.compressedISA.cr_funct4_lsb_compressed = self
            .extract_cr_funct4_lsb_compressed(
                fetch_unit.current_instruction_compressed,
            );
        self.compressedISA
            .cr_ci_destination_or_source_register_1_offset = self
            .extract_cr_ci_destination_or_source_register_1_offset(
                fetch_unit.current_instruction_compressed,
            );
        self.compressedISA.cr_css_source_register_2_offset = self
            .extract_cr_css_source_register_2_offset(
                fetch_unit.current_instruction_compressed,
            );

        if self.compressedISA.cr_funct4_lsb_compressed {
            //
        } else {
            //
        }

        todo!("TODO: unimplemented");
    }

    fn decode_ci(&mut self, fetch_unit: &FetchUnit) {
        trace!("Decoding CI-format instruction..");

        self.compressedISA
            .cr_ci_destination_or_source_register_1_offset = self
            .extract_cr_ci_destination_or_source_register_1_offset(
                fetch_unit.current_instruction_compressed,
            );
        //

        match self.compressedISA.funct3 {
            OP_Funct3::F000 => match fetch_unit.current_quadrant {
                OpcodeQuadrant::Quadrant01 => {
                    self.command_to_execute = ExecutableCommand::C_ADDI;
                }
                OpcodeQuadrant::Quadrant10 => {
                    self.command_to_execute = ExecutableCommand::C_SLLI;
                }

                _ => {
                    error!("unreachable opcode for CI instruction");
                    unreachable!("unreachable opcode for CI instruction");
                }
            },
            OP_Funct3::F010 => match fetch_unit.current_quadrant {
                OpcodeQuadrant::Quadrant01 => {
                    self.command_to_execute = ExecutableCommand::C_LI;
                }
                OpcodeQuadrant::Quadrant10 => {
                    self.command_to_execute = ExecutableCommand::C_LWSP;
                }

                _ => {
                    error!("unreachable opcode for CI instruction");
                    unreachable!("unreachable opcode for CI instruction");
                }
            },
            OP_Funct3::F011 => match fetch_unit.current_quadrant {
                OpcodeQuadrant::Quadrant01 => {
                    todo!("check unimplemented");
                    // if self.destination_register_offset
                    self.command_to_execute = ExecutableCommand::C_LUI;
                    // else
                    self.command_to_execute = ExecutableCommand::C_ADDI16SP;
                }
                OpcodeQuadrant::Quadrant10 => {
                    todo!("check unimplemented");
                    // if RV32FC
                    self.command_to_execute = ExecutableCommand::C_FLWSP;
                    // else if RV64C
                    self.command_to_execute = ExecutableCommand::C_LDSP;
                }

                _ => {
                    error!("unreachable opcode for CI instruction");
                    unreachable!("unreachable opcode for CI instruction");
                }
            },

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

        todo!("TODO: unimplemented");
    }

    fn decode_css(&mut self, fetch_unit: &FetchUnit) {
        trace!("Decoding CSS-format instruction..");

        self.compressedISA.cr_css_source_register_2_offset = self
            .extract_cr_css_source_register_2_offset(
                fetch_unit.current_instruction_compressed,
            );

        todo!("TODO: unimplemented");
    }

    fn decode_ciw(&mut self, fetch_unit: &FetchUnit) {
        trace!("Decoding CIW-format instruction..");

        // C.ADDI4SPN (RESERVED when uimm == 0)
        self.compressedISA.ciw_nzuimm =
            self.extract_ciw_uimm(fetch_unit.current_instruction_compressed);
        self.compressedISA.ciw_cl_rd_scaled = self.extract_ciw_cl_rd_scaled(
            fetch_unit.current_instruction_compressed,
        );

        if self.compressedISA.ciw_nzuimm != 0 {
            // C.ADDI4SPN
            self.command_to_execute = ExecutableCommand::C_ADDI4SPN;
        } else {
            // RESERVED, uimm == 0
            error!("illegal instruction trap, reserved by specification");
            todo!("illegal instruction trap, reserved by specification");
        }
    }

    fn decode_cl(&mut self, fetch_unit: &FetchUnit) {
        trace!("Decoding CL-format instruction..");

        self.compressedISA.ciw_cl_rd_scaled = self.extract_ciw_cl_rd_scaled(
            fetch_unit.current_instruction_compressed,
        );

        match self.compressedISA.funct3 {
            OP_Funct3::F001 => {
                todo!("condition unimplemented yet");
                // if RV(32|64)DC {
                self.command_to_execute = ExecutableCommand::C_FLD
                // }
            }
            OP_Funct3::F010 => {
                self.command_to_execute = ExecutableCommand::C_LW
            }
            OP_Funct3::F011 => {
                todo!("condition unimplemented yet");
                // if RV32IFC {
                self.command_to_execute = ExecutableCommand::C_FLW;
                // } else { // RV64IC only
                self.command_to_execute = ExecutableCommand::C_LD;
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

        todo!("TODO: unimplemented");
    }

    fn decode_cs(&mut self, fetch_unit: &FetchUnit) {
        trace!("Decoding CS-format instruction..");

        //

        match self.compressedISA.funct3 {
            OP_Funct3::F101 => {
                todo!("condition unimplemented yet");
                // if RV(32|64)DC {
                self.command_to_execute = ExecutableCommand::C_FSD
                // }
            }
            OP_Funct3::F110 => {
                self.command_to_execute = ExecutableCommand::C_SW
            }
            OP_Funct3::F111 => {
                todo!("condition unimplemented yet");
                // if RV32IFC {
                self.command_to_execute = ExecutableCommand::C_FSW;
                // } else { // RV64IC only
                self.command_to_execute = ExecutableCommand::C_SD;
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

        todo!("TODO: unimplemented");
    }

    fn decode_ca(&mut self, fetch_unit: &FetchUnit) {
        trace!("Decoding CA-format instruction..");

        //

        todo!("TODO: unimplemented");
    }

    fn decode_cb(&mut self, fetch_unit: &FetchUnit) {
        trace!("Decoding CB-format instruction..");

        //

        todo!("TODO: unimplemented");
    }

    fn decode_cj(&mut self, fetch_unit: &FetchUnit) {
        trace!("Decoding CJ-format instruction..");

        //

        todo!("TODO: unimplemented");
    }

    // extractors
    fn extract_funct3_compressed(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> OP_Funct3 {
        OP_Funct3::from(instruction & COMPRESSED_FUNCT3_MASK)
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
        instruction & COMPRESSED_CIW_CL_RD_SCALED_MASK
    }

    fn extract_bits_11_10_quadrant_01_funct3_100(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> Bits11_10_quadrant_01 {
        let bits_11_10 = (instruction & COMPRESSED_BITS_11_10_MASK)
            >> COMPRESSED_BITS_11_10_OFFSET;

        Bits11_10_quadrant_01::from(bits_11_10)
    }
}
