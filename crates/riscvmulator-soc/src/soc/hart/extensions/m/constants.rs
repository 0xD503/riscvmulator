use log::error;

use crate::constants::arch::InstructionType;

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
pub enum OP_Funct3 {
    MUL = 0b000,
    MULH = 0b001,
    MULHSU = 0b010,
    MULHU = 0b011,
    DIV = 0b100,
    DIVU = 0b101,
    REM = 0b110,
    REMU = 0b111,
}

impl From<InstructionType> for OP_Funct3 {
    fn from(value: InstructionType) -> Self {
        match value {
            0b000 => OP_Funct3::MUL,
            0b001 => OP_Funct3::MULH,
            0b010 => OP_Funct3::MULHSU,
            0b011 => OP_Funct3::MULHU,
            0b100 => OP_Funct3::DIV,
            0b101 => OP_Funct3::DIVU,
            0b110 => OP_Funct3::REM,
            0b111 => OP_Funct3::REMU,

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
    MULW = 0b000,
    DIVW = 0b100,
    DIVUW = 0b101,
    REMW = 0b110,
    REMUW = 0b111,
}

impl From<InstructionType> for OP_32_Funct3 {
    fn from(value: InstructionType) -> Self {
        match value {
            0b000 => OP_32_Funct3::MULW,
            0b100 => OP_32_Funct3::DIVW,
            0b101 => OP_32_Funct3::DIVUW,
            0b110 => OP_32_Funct3::REMW,
            0b111 => OP_32_Funct3::REMUW,

            _ => {
                error!("Unknown funct3: {value:#05b}");
                unreachable!("Unknown funct3: {value:#05b}");
            }
        }
    }
}
