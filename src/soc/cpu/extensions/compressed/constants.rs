pub type InstructionTypeCompressed = u16;
pub type SignedInstructionTypeCompressed = i16;

pub const COMPRESSED_INSTRUCTION_MASK: InstructionTypeCompressed =
    !0 as InstructionTypeCompressed;
