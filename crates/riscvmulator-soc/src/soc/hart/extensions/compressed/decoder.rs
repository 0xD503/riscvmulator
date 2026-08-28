use log::error;

use crate::constants::arch::RISCV_NUM_OF_REGISTERS;
use crate::soc::hart::core::decoder::constants::OpcodeQuadrant;
use crate::soc::hart::core::{
    ArchState, decoder::Decoder, fetch_unit::FetchUnit,
    gp_registers::RegisterFile,
};
use crate::soc::hart::extensions::compressed::constants::{
    COMPRESSED_FUNCT3_MASK, InstructionTypeCompressed, OP_Funct3,
};

impl Decoder {
    pub(crate) fn decode_c(
        &mut self,
        arch_state: &ArchState,
        fetch_unit: &FetchUnit,
        register_file: &RegisterFile<RISCV_NUM_OF_REGISTERS>,
    ) {
        let quadrant = fetch_unit.extract_quadrant_compressed(
            fetch_unit.current_instruction_compressed,
        );
        let funct3 = self.extract_funct3_compressed(
            fetch_unit.current_instruction_compressed,
        );

        match quadrant {
            OpcodeQuadrant::Quadrant00 => {
                match funct3 {
                    OP_Funct3::F000 => {
                        let uimm = self.extract_uimm_ciw(
                            fetch_unit.current_instruction_compressed,
                        );
                        if uimm != 0 {
                            self.decode_ciw();
                        } else {
                            error!("Illegal instruction trap");
                            todo!("Illegal instruction trap");
                        }
                    }

                    _ => {
                        error!("todo");
                        todo!("todo");
                    }
                }
                error!("TODO: unimplemented");
                todo!("TODO: unimplemented");
            }
            OpcodeQuadrant::Quadrant01 => {
                error!("TODO: unimplemented");
                todo!("TODO: unimplemented");
            }
            OpcodeQuadrant::Quadrant10 => {
                error!("TODO: unimplemented");
                todo!("TODO: unimplemented");
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

    fn extract_funct3_compressed(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> OP_Funct3 {
        OP_Funct3::from(instruction & COMPRESSED_FUNCT3_MASK)
    }

    fn extract_uimm_ciw(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> InstructionTypeCompressed {
        instruction
    }

    fn decode_cr(&mut self) {
        //

        todo!("TODO: unimplemented");
    }

    fn decode_ci(&mut self) {
        //

        todo!("TODO: unimplemented");
    }

    fn decode_css(&mut self) {
        //

        todo!("TODO: unimplemented");
    }

    fn decode_ciw(&mut self) {
        //

        todo!("TODO: unimplemented");
    }

    fn decode_cl(&mut self) {
        //

        todo!("TODO: unimplemented");
    }

    fn decode_cs(&mut self) {
        //

        todo!("TODO: unimplemented");
    }

    fn decode_ca(&mut self) {
        //

        todo!("TODO: unimplemented");
    }

    fn decode_cb(&mut self) {
        //

        todo!("TODO: unimplemented");
    }

    fn decode_cj(&mut self) {
        //

        todo!("TODO: unimplemented");
    }
}
