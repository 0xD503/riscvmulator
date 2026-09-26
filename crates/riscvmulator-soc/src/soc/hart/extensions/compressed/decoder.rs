use log::{error, trace};

use crate::soc::hart::core::decoder::constants::OpcodeQuadrant;
use crate::soc::hart::core::executor::constants::ExecutableCommand;
use crate::soc::hart::core::{
    ArchState, decoder::Decoder, fetch_unit::FetchUnit,
};
use crate::soc::hart::extensions::compressed::constants::{
    Bits11_10_quadrant_01, COMPRESSED_BITS_11_10_MASK,
    COMPRESSED_BITS_11_10_OFFSET, COMPRESSED_CIW_IMM_MASK,
    COMPRESSED_CIW_IMM_OFFSET, COMPRESSED_FUNCT3_MASK, COMPRESSED_FUNCT4_MASK,
    COMPRESSED_FUNCT4_OFFSET, InstructionTypeCompressed, OP_Funct3,
};

#[derive(Clone)]
pub struct DecoderCompressedISAContext {
    funct3: OP_Funct3,
}

impl Default for DecoderCompressedISAContext {
    fn default() -> Self {
        DecoderCompressedISAContext {
            funct3: OP_Funct3::F000,
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
                    self.decode_cl();
                }
                OP_Funct3::F101 | OP_Funct3::F110 | OP_Funct3::F111 => {
                    self.decode_cs();
                }

                OP_Funct3::F100 => {
                    error!("illegal instruction, reserved by specification");
                    todo!("illegal instruction, reserved by specification");
                }
            },
            OpcodeQuadrant::Quadrant01 => match self.compressedISA.funct3 {
                OP_Funct3::F000 | OP_Funct3::F010 | OP_Funct3::F011 => {
                    self.decode_ci();
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
                            self.decode_ca();
                        }
                        Bits11_10_quadrant_01::CB_00
                        | Bits11_10_quadrant_01::CB_01
                        | Bits11_10_quadrant_01::CB_10 => {
                            self.decode_cb();
                        }
                    }
                }
                OP_Funct3::F101 => {
                    self.decode_cj();
                }
                OP_Funct3::F110 | OP_Funct3::F111 => {
                    self.decode_cb();
                }
            },
            OpcodeQuadrant::Quadrant10 => match self.compressedISA.funct3 {
                OP_Funct3::F000
                | OP_Funct3::F001
                | OP_Funct3::F010
                | OP_Funct3::F011 => {
                    self.decode_ci();
                }
                OP_Funct3::F100 => {
                    self.decode_cr(fetch_unit);
                }
                OP_Funct3::F101 | OP_Funct3::F110 | OP_Funct3::F111 => {
                    self.decode_css();
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

        if self.extract_funct4_lsb_compressed(
            fetch_unit.current_instruction_compressed,
        ) {
            //
        } else {
            //
        }

        todo!("TODO: unimplemented");
    }

    fn decode_ci(&mut self) {
        trace!("Decoding CI-format instruction..");

        //

        todo!("TODO: unimplemented");
    }

    fn decode_css(&mut self) {
        trace!("Decoding CSS-format instruction..");

        //

        todo!("TODO: unimplemented");
    }

    fn decode_ciw(&mut self, fetch_unit: &FetchUnit) {
        trace!("Decoding CIW-format instruction..");

        // C.ADDI4SPN (RESERVED when uimm == 0)
        let uimm =
            self.extract_uimm_ciw(fetch_unit.current_instruction_compressed);
        if uimm != 0 {
            // C.ADDI4SPN
            self.command_to_execute = ExecutableCommand::C_ADDI4SPN;
        } else {
            // RESERVED, uimm == 0
            error!("illegal instruction trap, reserved by specification");
            todo!("illegal instruction trap, reserved by specification");
        }
    }

    fn decode_cl(&mut self) {
        trace!("Decoding CL-format instruction..");

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

    fn decode_cs(&mut self) {
        trace!("Decoding CS-format instruction..");

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

    fn decode_ca(&mut self) {
        trace!("Decoding CA-format instruction..");

        //

        todo!("TODO: unimplemented");
    }

    fn decode_cb(&mut self) {
        trace!("Decoding CB-format instruction..");

        //

        todo!("TODO: unimplemented");
    }

    fn decode_cj(&mut self) {
        trace!("Decoding CJ-format instruction..");

        //

        todo!("TODO: unimplemented");
    }

    fn extract_funct3_compressed(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> OP_Funct3 {
        OP_Funct3::from(instruction & COMPRESSED_FUNCT3_MASK)
    }

    fn extract_funct4_lsb_compressed(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> bool {
        ((instruction & COMPRESSED_FUNCT4_MASK) >> COMPRESSED_FUNCT4_OFFSET)
            & 0b1
            == 1
    }

    fn extract_uimm_ciw(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> InstructionTypeCompressed {
        (instruction & COMPRESSED_CIW_IMM_MASK) >> COMPRESSED_CIW_IMM_OFFSET
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
