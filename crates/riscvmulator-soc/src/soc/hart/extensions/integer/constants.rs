use log::error;

use crate::constants::arch::InstructionType;

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
pub enum OP_Funct3 {
    ARITH = 0b000, //
    SLL = 0b001,   //
    SLT = 0b010,   //
    SLTU = 0b011,  //
    XOR = 0b100,   //
    SRX = 0b101,   //
    OR = 0b110,    //
    AND = 0b111,   //
}

impl From<InstructionType> for OP_Funct3 {
    fn from(value: InstructionType) -> Self {
        match value {
            0b000 => OP_Funct3::ARITH,
            0b001 => OP_Funct3::SLL,
            0b010 => OP_Funct3::SLT,
            0b011 => OP_Funct3::SLTU,
            0b100 => OP_Funct3::XOR,
            0b101 => OP_Funct3::SRX,
            0b110 => OP_Funct3::OR,
            0b111 => OP_Funct3::AND,

            _ => {
                error!("Unknown funct3: {value:#05b}");
                unreachable!("Unknown funct3: {value:#05b}");
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
pub enum OP_32_Funct3 {
    ARITHW = 0b000, //
    SLLW = 0b001,   //
    SRXW = 0b101,   //
}

impl From<InstructionType> for OP_32_Funct3 {
    fn from(value: InstructionType) -> Self {
        match value {
            0b000 => OP_32_Funct3::ARITHW,
            0b001 => OP_32_Funct3::SLLW,
            0b101 => OP_32_Funct3::SRXW,

            _ => {
                error!("Unknown funct3: {value:#05b}");
                unreachable!("Unknown funct3: {value:#05b}");
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
pub enum OP_IMM_32_Funct3 {
    ADDIW = 0b000, //
    SLLIW = 0b001, //
    SRXIW = 0b101, //
}

impl From<InstructionType> for OP_IMM_32_Funct3 {
    fn from(value: InstructionType) -> Self {
        match value {
            0b000 => OP_IMM_32_Funct3::ADDIW,
            0b001 => OP_IMM_32_Funct3::SLLIW,
            0b101 => OP_IMM_32_Funct3::SRXIW,

            _ => {
                error!("Unknown funct3: {value:#05b}");
                unreachable!("Unknown funct3: {value:#05b}");
            }
        }
    }
}
