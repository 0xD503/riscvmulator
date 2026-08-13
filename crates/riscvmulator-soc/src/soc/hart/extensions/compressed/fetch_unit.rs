use crate::soc::hart::{
    core::{
        decoder::constants::{OPCODE_QUADRANT_MASK, OpcodeQuadrant},
        fetch_unit::FetchUnit,
    },
    extensions::compressed::constants::InstructionTypeCompressed,
};

impl FetchUnit {
    pub(super) fn extract_quadrant_compressed(
        &self,
        instruction: InstructionTypeCompressed,
    ) -> OpcodeQuadrant {
        OpcodeQuadrant::from(
            instruction & OPCODE_QUADRANT_MASK as InstructionTypeCompressed,
        )
    }
}
