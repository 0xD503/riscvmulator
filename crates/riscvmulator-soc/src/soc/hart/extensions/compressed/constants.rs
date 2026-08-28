use log::error;

pub type InstructionTypeCompressed = u16;
pub type SignedInstructionTypeCompressed = i16;

pub const COMPRESSED_INSTRUCTION_MASK: InstructionTypeCompressed =
    !0 as InstructionTypeCompressed;

pub const COMPRESSED_FUNCT3_OFFSET: usize = 13;
pub const COMPRESSED_FUNCT3_MASK: InstructionTypeCompressed =
    0b111 << COMPRESSED_FUNCT3_OFFSET;

pub const COMPRESSED_BITS_11_10_OFFSET: usize = 10;
pub const COMPRESSED_BITS_11_10_MASK: InstructionTypeCompressed =
    0b11 << COMPRESSED_BITS_11_10_OFFSET;

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
pub enum OP_Funct3 {
    F000 = 0b000 << COMPRESSED_FUNCT3_OFFSET, //
    F001 = 0b001 << COMPRESSED_FUNCT3_OFFSET, //
    F010 = 0b010 << COMPRESSED_FUNCT3_OFFSET, //
    F011 = 0b011 << COMPRESSED_FUNCT3_OFFSET, //
    F100 = 0b100 << COMPRESSED_FUNCT3_OFFSET, //
    F101 = 0b101 << COMPRESSED_FUNCT3_OFFSET, //
    F110 = 0b110 << COMPRESSED_FUNCT3_OFFSET, //
    F111 = 0b111 << COMPRESSED_FUNCT3_OFFSET, //
}

impl OP_Funct3 {
    const F000_VALUE: InstructionTypeCompressed =
        0b000 << COMPRESSED_FUNCT3_OFFSET;
    const F001_VALUE: InstructionTypeCompressed =
        0b001 << COMPRESSED_FUNCT3_OFFSET;
    const F010_VALUE: InstructionTypeCompressed =
        0b010 << COMPRESSED_FUNCT3_OFFSET;
    const F011_VALUE: InstructionTypeCompressed =
        0b011 << COMPRESSED_FUNCT3_OFFSET;
    const F100_VALUE: InstructionTypeCompressed =
        0b100 << COMPRESSED_FUNCT3_OFFSET;
    const F101_VALUE: InstructionTypeCompressed =
        0b101 << COMPRESSED_FUNCT3_OFFSET;
    const F110_VALUE: InstructionTypeCompressed =
        0b110 << COMPRESSED_FUNCT3_OFFSET;
    const F111_VALUE: InstructionTypeCompressed =
        0b111 << COMPRESSED_FUNCT3_OFFSET;
}

impl From<InstructionTypeCompressed> for OP_Funct3 {
    fn from(value: InstructionTypeCompressed) -> Self {
        match value {
            OP_Funct3::F000_VALUE => OP_Funct3::F000,
            OP_Funct3::F001_VALUE => OP_Funct3::F001,
            OP_Funct3::F010_VALUE => OP_Funct3::F010,
            OP_Funct3::F011_VALUE => OP_Funct3::F011,
            OP_Funct3::F100_VALUE => OP_Funct3::F100,
            OP_Funct3::F101_VALUE => OP_Funct3::F101,
            OP_Funct3::F110_VALUE => OP_Funct3::F110,
            OP_Funct3::F111_VALUE => OP_Funct3::F111,

            _ => {
                error!("Unknown funct3: {value:#05b}");
                unreachable!("Unknown funct3: {value:#05b}");
            }
        }
    }
}

#[allow(non_camel_case_types)]
pub enum Bits11_10_quadrant_01 {
    CB_00 = 0b00,
    CB_01 = 0b01,
    CB_10 = 0b10,
    CA_11 = 0b11,
}

impl From<InstructionTypeCompressed> for Bits11_10_quadrant_01 {
    fn from(value: InstructionTypeCompressed) -> Self {
        match value {
            0b00 => Bits11_10_quadrant_01::CB_00,
            0b01 => Bits11_10_quadrant_01::CB_01,
            0b10 => Bits11_10_quadrant_01::CB_10,
            0b11 => Bits11_10_quadrant_01::CA_11,

            _ => {
                error!("Unknown bits[11:10]: {value:#04b}");
                unreachable!("Unknown bits[11:10]: {value:#04b}");
            }
        }
    }
}
