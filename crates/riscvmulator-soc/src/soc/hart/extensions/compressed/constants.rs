use std::fmt::Display;

use log::error;

use crate::constants::arch::{InstructionType, RISCV_NUM_OF_REGISTERS};

pub type InstructionTypeCompressed = u16;
pub type SignedInstructionTypeCompressed = i16;

pub const COMPRESSED_INSTRUCTION_MASK: InstructionTypeCompressed =
    !0 as InstructionTypeCompressed;

pub const COMPRESSED_FUNCT3_OFFSET: usize = 13;
pub const COMPRESSED_FUNCT3_MASK: InstructionTypeCompressed =
    0b111 << COMPRESSED_FUNCT3_OFFSET;

pub const COMPRESSED_FUNCT4_OFFSET: usize = 12;
pub const COMPRESSED_FUNCT4_MASK: InstructionTypeCompressed =
    0b1111 << COMPRESSED_FUNCT4_OFFSET;

pub const COMPRESSED_BITS_12_OFFSET: usize = 12;
pub const COMPRESSED_BITS_12_MASK: InstructionTypeCompressed =
    0b1 << COMPRESSED_BITS_12_OFFSET;
pub const COMPRESSED_BITS_11_10_OFFSET: usize = 10;
pub const COMPRESSED_BITS_11_10_MASK: InstructionTypeCompressed =
    0b11 << COMPRESSED_BITS_11_10_OFFSET;

pub const COMPRESSED_CSS_UIMM_8_6_OFFSET: usize = 7;
pub const COMPRESSED_CSS_UIMM_8_6_MASK: InstructionTypeCompressed =
    0b111 << COMPRESSED_CSS_UIMM_8_6_OFFSET;
pub const COMPRESSED_CSS_UIMM_5_3_OFFSET: usize = 10;
pub const COMPRESSED_CSS_UIMM_5_3_MASK: InstructionTypeCompressed =
    0b111 << COMPRESSED_CSS_UIMM_5_3_OFFSET;
pub const COMPRESSED_CSS_UIMM_7_6_OFFSET: usize = 7;
pub const COMPRESSED_CSS_UIMM_7_6_MASK: InstructionTypeCompressed =
    0b11 << COMPRESSED_CSS_UIMM_7_6_OFFSET;
pub const COMPRESSED_CSS_UIMM_5_2_OFFSET: usize = 9;
pub const COMPRESSED_CSS_UIMM_5_2_MASK: InstructionTypeCompressed =
    0b1111 << COMPRESSED_CSS_UIMM_5_2_OFFSET;

pub const COMPRESSED_CIW_NZUIMM_OFFSET: usize = 5;
pub const COMPRESSED_CIW_NZUIMM_MASK: InstructionTypeCompressed =
    0b11111111 << COMPRESSED_CIW_NZUIMM_OFFSET;

pub const COMPRESSED_CIW_CL_RD_OFFSET_OFFSET: usize = 2;
pub const COMPRESSED_CIW_CL_RD_OFFSET_MASK: InstructionTypeCompressed =
    0b111 << COMPRESSED_CIW_CL_RD_OFFSET_OFFSET;

pub const COMPRESSED_CL_OFFSET_6_OFFSET: usize = 5;
pub const COMPRESSED_CL_OFFSET_6_MASK: InstructionTypeCompressed =
    0b1 << COMPRESSED_CL_OFFSET_6_OFFSET;
pub const COMPRESSED_CL_OFFSET_2_OFFSET: usize = 6;
pub const COMPRESSED_CL_OFFSET_2_MASK: InstructionTypeCompressed =
    0b1 << COMPRESSED_CL_OFFSET_2_OFFSET;
pub const COMPRESSED_CL_OFFSET_7_OFFSET: usize = 6;
pub const COMPRESSED_CL_OFFSET_7_MASK: InstructionTypeCompressed =
    0b1 << COMPRESSED_CL_OFFSET_7_OFFSET;
pub const COMPRESSED_CL_OFFSET_5_3_OFFSET: usize = 10;
pub const COMPRESSED_CL_OFFSET_5_3_MASK: InstructionTypeCompressed =
    0b111 << COMPRESSED_CL_OFFSET_5_3_OFFSET;

pub const COMPRESSED_CR_CI_RD_RS1_OFFSET: usize = 7;
pub const COMPRESSED_CR_CI_RD_RS1_MASK: InstructionTypeCompressed =
    ((RISCV_NUM_OF_REGISTERS - 1) as InstructionTypeCompressed)
        << COMPRESSED_CR_CI_RD_RS1_OFFSET;

pub const COMPRESSED_CR_CSS_RS2_OFFSET: usize = 2;
pub const COMPRESSED_CR_CSS_RS2_MASK: InstructionTypeCompressed =
    ((RISCV_NUM_OF_REGISTERS - 1) as InstructionTypeCompressed)
        << COMPRESSED_CR_CSS_RS2_OFFSET;

pub const COMPRESSED_CI_NZIMM_5_OFFSET: usize = 12;
pub const COMPRESSED_CI_NZIMM_5_OFFSET_FINAL: usize = 5;
pub const COMPRESSED_CI_NZIMM_5_MASK: InstructionTypeCompressed =
    0b1 << COMPRESSED_CI_NZIMM_5_OFFSET;
pub const COMPRESSED_CI_NZIMM_5_MASK_FINAL: InstructionTypeCompressed =
    0b1 << COMPRESSED_CI_NZIMM_5_OFFSET_FINAL;
pub const COMPRESSED_CI_NZIMM_4_0_OFFSET: usize = 2;
pub const COMPRESSED_CI_NZIMM_4_0_MASK: InstructionTypeCompressed =
    0b11111 << COMPRESSED_CI_NZIMM_4_0_OFFSET;
pub const COMPRESSED_CI_IMM_5_OFFSET: usize = 12;
pub const COMPRESSED_CI_IMM_5_OFFSET_FINAL: usize = 5;
pub const COMPRESSED_CI_IMM_5_MASK: InstructionTypeCompressed =
    0b1 << COMPRESSED_CI_IMM_5_OFFSET;
pub const COMPRESSED_CI_IMM_5_MASK_FINAL: InstructionTypeCompressed =
    0b1 << COMPRESSED_CI_IMM_5_OFFSET_FINAL;
pub const COMPRESSED_CI_CB_IMM_4_0_OFFSET: usize = 2;
pub const COMPRESSED_CI_CB_IMM_4_0_MASK: InstructionTypeCompressed =
    0b11111 << COMPRESSED_CI_CB_IMM_4_0_OFFSET;
pub const COMPRESSED_CI_NZIMM_9_OFFSET: usize = 12;
pub const COMPRESSED_CI_NZIMM_9_OFFSET_FINAL: usize = 9;
pub const COMPRESSED_CI_NZIMM_9_MASK: InstructionTypeCompressed =
    0b1 << COMPRESSED_CI_NZIMM_9_OFFSET;
pub const COMPRESSED_CI_NZIMM_9_MASK_FINAL: InstructionTypeCompressed =
    0b1 << COMPRESSED_CI_NZIMM_9_OFFSET_FINAL;
pub const COMPRESSED_CI_NZIMM_4_OFFSET: usize = 6;
pub const COMPRESSED_CI_NZIMM_4_MASK: InstructionTypeCompressed =
    0b1 << COMPRESSED_CI_NZIMM_4_OFFSET;
pub const COMPRESSED_CI_NZIMM_6_OFFSET: usize = 5;
pub const COMPRESSED_CI_NZIMM_6_MASK: InstructionTypeCompressed =
    0b1 << COMPRESSED_CI_NZIMM_6_OFFSET;
pub const COMPRESSED_CI_NZIMM_8_7_OFFSET: usize = 3;
pub const COMPRESSED_CI_NZIMM_8_7_MASK: InstructionTypeCompressed =
    0b11 << COMPRESSED_CI_NZIMM_8_7_OFFSET;
pub const COMPRESSED_CI_NZIMM_5_9_4_OFFSET: usize = 2;
pub const COMPRESSED_CI_NZIMM_5_9_4_MASK: InstructionTypeCompressed =
    0b1 << COMPRESSED_CI_NZIMM_5_9_4_OFFSET;

pub const COMPRESSED_CI_NZIMM_17_OFFSET: usize = 12;
pub const COMPRESSED_CI_NZIMM_17_OFFSET_FINAL: usize = 17;
pub const COMPRESSED_CI_NZIMM_17_MASK: InstructionTypeCompressed =
    0b1 << COMPRESSED_CI_NZIMM_17_OFFSET;
pub const COMPRESSED_CI_NZIMM_17_MASK_FINAL: InstructionType =
    0b1 << COMPRESSED_CI_NZIMM_17_OFFSET_FINAL;
pub const COMPRESSED_CI_NZIMM_16_12_OFFSET: usize = 2;
pub const COMPRESSED_CI_NZIMM_16_12_MASK: InstructionTypeCompressed =
    0b11111 << COMPRESSED_CI_NZIMM_16_12_OFFSET;

pub const COMPRESSED_CI_SHAMT_4_0_OFFSET: usize = 2;
pub const COMPRESSED_CI_SHAMT_4_0_MASK: InstructionTypeCompressed =
    0b11111 << COMPRESSED_CI_SHAMT_4_0_OFFSET;
pub const COMPRESSED_CI_SHAMT_5_OFFSET: usize = 12;
pub const COMPRESSED_CI_SHAMT_5_MASK: InstructionTypeCompressed =
    0b1 << COMPRESSED_CI_SHAMT_5_OFFSET;

pub const COMPRESSED_CI_OFFSET_5_OFFSET: usize = 12;
pub const COMPRESSED_CI_OFFSET_5_MASK: InstructionTypeCompressed =
    0b1 << COMPRESSED_CI_OFFSET_5_OFFSET;
pub const COMPRESSED_CI_OFFSET_4_2_OFFSET: usize = 4;
pub const COMPRESSED_CI_OFFSET_4_2_MASK: InstructionTypeCompressed =
    0b111 << COMPRESSED_CI_OFFSET_4_2_OFFSET;
pub const COMPRESSED_CI_OFFSET_7_6_OFFSET: usize = 2;
pub const COMPRESSED_CI_OFFSET_7_6_MASK: InstructionTypeCompressed =
    0b11 << COMPRESSED_CI_OFFSET_7_6_OFFSET;
pub const COMPRESSED_CI_OFFSET_4_3_OFFSET: usize = 5;
pub const COMPRESSED_CI_OFFSET_4_3_MASK: InstructionTypeCompressed =
    0b11 << COMPRESSED_CI_OFFSET_4_3_OFFSET;
pub const COMPRESSED_CI_OFFSET_8_6_OFFSET: usize = 2;
pub const COMPRESSED_CI_OFFSET_8_6_MASK: InstructionTypeCompressed =
    0b111 << COMPRESSED_CI_OFFSET_8_6_OFFSET;

pub const COMPRESSED_CL_CS_RS1_OFFSET_OFFSET: usize = 7;
pub const COMPRESSED_CL_CS_RS1_OFFSET_MASK: InstructionTypeCompressed =
    0b111 << COMPRESSED_CL_CS_RS1_OFFSET_OFFSET;

pub const COMPRESSED_CS_CA_RS2_OFFSET_OFFSET: usize = 2;
pub const COMPRESSED_CS_CA_RS2_OFFSET_MASK: InstructionTypeCompressed =
    0b111 << COMPRESSED_CS_CA_RS2_OFFSET_OFFSET;

pub const COMPRESSED_CS_IMM_4_0_OFFSET: usize = 0;
pub const COMPRESSED_CS_IMM_4_0_MASK: InstructionTypeCompressed =
    0b11111 << COMPRESSED_CS_IMM_4_0_OFFSET;
pub const COMPRESSED_CS_IMM_4_0_OFFSET_FINAL: usize = 7;
pub const COMPRESSED_CS_IMM_11_5_OFFSET: usize = 5;
pub const COMPRESSED_CS_IMM_11_5_MASK: InstructionTypeCompressed =
    0b111_1111 << COMPRESSED_CS_IMM_11_5_OFFSET;
pub const COMPRESSED_CS_IMM_11_5_OFFSET_FINAL: usize = 25;

pub const COMPRESSED_CB_IMM_11_OFFSET: usize = 11;
pub const COMPRESSED_CB_IMM_11_MASK: InstructionTypeCompressed =
    0b1 << COMPRESSED_CB_IMM_11_OFFSET;
pub const COMPRESSED_CB_IMM_11_OFFSET_FINAL: usize = 7;

pub const COMPRESSED_CB_IMM_4_1_OFFSET: usize = 1;
pub const COMPRESSED_CB_IMM_4_1_MASK: InstructionTypeCompressed =
    0b1111 << COMPRESSED_CB_IMM_4_1_OFFSET;
pub const COMPRESSED_CB_IMM_4_1_OFFSET_FINAL: usize = 8;

pub const COMPRESSED_CB_IMM_10_5_OFFSET: usize = 5;
pub const COMPRESSED_CB_IMM_10_5_MASK: InstructionTypeCompressed =
    0b11_1111 << COMPRESSED_CB_IMM_10_5_OFFSET;
pub const COMPRESSED_CB_IMM_10_5_OFFSET_FINAL: usize = 25;

pub const COMPRESSED_CB_IMM_12_OFFSET: usize = 12;
pub const COMPRESSED_CB_IMM_12_MASK: InstructionTypeCompressed =
    0b1 << COMPRESSED_CB_IMM_12_OFFSET;
pub const COMPRESSED_CB_IMM_12_OFFSET_FINAL: usize = 31;

pub const COMPRESSED_CJ_IMM_19_12_OFFSET: usize = 12;
pub const COMPRESSED_CJ_IMM_19_12_MASK: InstructionType =
    0b1111_1111 << COMPRESSED_CJ_IMM_19_12_OFFSET;
pub const COMPRESSED_CJ_IMM_19_12_OFFSET_FINAL: usize = 12;

pub const COMPRESSED_CJ_IMM_11_OFFSET: usize = 11;
pub const COMPRESSED_CJ_IMM_11_MASK: InstructionTypeCompressed =
    0b1 << COMPRESSED_CJ_IMM_11_OFFSET;
pub const COMPRESSED_CJ_IMM_11_OFFSET_FINAL: usize = 20;

pub const COMPRESSED_CJ_IMM_10_1_OFFSET: usize = 1;
pub const COMPRESSED_CJ_IMM_10_1_MASK: InstructionTypeCompressed =
    0b11_1111_1111 << COMPRESSED_CJ_IMM_10_1_OFFSET;
pub const COMPRESSED_CJ_IMM_10_1_OFFSET_FINAL: usize = 21;

pub const COMPRESSED_CJ_IMM_20_OFFSET: usize = 20;
pub const COMPRESSED_CJ_IMM_20_MASK: InstructionType =
    0b1 << COMPRESSED_CJ_IMM_20_OFFSET;
pub const COMPRESSED_CJ_IMM_20_OFFSET_FINAL: usize = 31;

pub const COMPRESSED_CA_FUNCT_2_OFFSET: usize = 5;
pub const COMPRESSED_CA_FUNCT_2_MASK: InstructionTypeCompressed =
    0b11 << COMPRESSED_CA_FUNCT_2_OFFSET;

pub const COMPRESSED_CA_CB_RD_OR_RS1_OFFSET_OFFSET: usize = 7;
pub const COMPRESSED_CA_CB_RD_OR_RS1_OFFSET_MASK: InstructionTypeCompressed =
    0b111 << COMPRESSED_CA_CB_RD_OR_RS1_OFFSET_OFFSET;

pub const COMPRESSED_CB_UIMM_4_0_OFFSET: usize = 2;
pub const COMPRESSED_CB_UIMM_4_0_MASK: InstructionTypeCompressed =
    0b1_1111 << COMPRESSED_CB_UIMM_4_0_OFFSET;

pub const COMPRESSED_CB_IMM_5_OFFSET_FINAL: usize = 5;
pub const COMPRESSED_CB_IMM_5_MASK_FINAL: InstructionTypeCompressed =
    0b1 << COMPRESSED_CB_IMM_5_OFFSET_FINAL;
pub const COMPRESSED_CB_IMM_4_0_OFFSET: usize = 2;
pub const COMPRESSED_CB_IMM_4_0_MASK: InstructionTypeCompressed =
    0b1_1111 << COMPRESSED_CB_IMM_4_0_OFFSET;

pub const COMPRESSED_CB_IMM_8_OFFSET_FINAL: usize = 8;
pub const COMPRESSED_CB_IMM_8_MASK_FINAL: InstructionTypeCompressed =
    0b1 << COMPRESSED_CB_IMM_8_OFFSET_FINAL;
pub const COMPRESSED_CB_FUNCT_3_11X_IMM_5_OFFSET: usize = 2;
pub const COMPRESSED_CB_FUNCT_3_11X_IMM_5_MASK: InstructionTypeCompressed =
    0b1 << COMPRESSED_CB_FUNCT_3_11X_IMM_5_OFFSET;
pub const COMPRESSED_CB_FUNCT_3_11X_IMM_2_1_OFFSET: usize = 3;
pub const COMPRESSED_CB_FUNCT_3_11X_IMM_2_1_MASK: InstructionTypeCompressed =
    0b11 << COMPRESSED_CB_FUNCT_3_11X_IMM_2_1_OFFSET;
pub const COMPRESSED_CB_FUNCT_3_11X_IMM_7_6_OFFSET: usize = 5;
pub const COMPRESSED_CB_FUNCT_3_11X_IMM_7_6_MASK: InstructionTypeCompressed =
    0b11 << COMPRESSED_CB_FUNCT_3_11X_IMM_7_6_OFFSET;

pub const COMPRESSED_CJ_JTA_IMM_11_OFFSET: usize = 12;
pub const COMPRESSED_CJ_JTA_IMM_11_OFFSET_FINAL: usize = 11;
pub const COMPRESSED_CJ_JTA_IMM_11_MASK: InstructionTypeCompressed =
    0b1 << COMPRESSED_CJ_JTA_IMM_11_OFFSET;
pub const COMPRESSED_CJ_JTA_IMM_11_MASK_FINAL: InstructionTypeCompressed =
    0b1 << COMPRESSED_CJ_JTA_IMM_11_OFFSET_FINAL;
pub const COMPRESSED_CJ_JTA_IMM_10_OFFSET: usize = 8;
pub const COMPRESSED_CJ_JTA_IMM_10_MASK: InstructionTypeCompressed =
    0b1 << COMPRESSED_CJ_JTA_IMM_10_OFFSET;
pub const COMPRESSED_CJ_JTA_IMM_9_8_OFFSET: usize = 9;
pub const COMPRESSED_CJ_JTA_IMM_9_8_MASK: InstructionTypeCompressed =
    0b11 << COMPRESSED_CJ_JTA_IMM_9_8_OFFSET;
pub const COMPRESSED_CJ_JTA_IMM_7_OFFSET: usize = 6;
pub const COMPRESSED_CJ_JTA_IMM_7_MASK: InstructionTypeCompressed =
    0b1 << COMPRESSED_CJ_JTA_IMM_7_OFFSET;
pub const COMPRESSED_CJ_JTA_IMM_6_OFFSET: usize = 7;
pub const COMPRESSED_CJ_JTA_IMM_6_MASK: InstructionTypeCompressed =
    0b1 << COMPRESSED_CJ_JTA_IMM_6_OFFSET;
pub const COMPRESSED_CJ_JTA_IMM_5_OFFSET: usize = 2;
pub const COMPRESSED_CJ_JTA_IMM_5_MASK: InstructionTypeCompressed =
    0b1 << COMPRESSED_CJ_JTA_IMM_5_OFFSET;
pub const COMPRESSED_CJ_JTA_IMM_4_OFFSET: usize = 11;
pub const COMPRESSED_CJ_JTA_IMM_4_MASK: InstructionTypeCompressed =
    0b1 << COMPRESSED_CJ_JTA_IMM_4_OFFSET;
pub const COMPRESSED_CJ_JTA_IMM_3_1_OFFSET: usize = 3;
pub const COMPRESSED_CJ_JTA_IMM_3_1_MASK: InstructionTypeCompressed =
    0b111 << COMPRESSED_CJ_JTA_IMM_3_1_OFFSET;

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, PartialEq, Debug)]
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

impl Display for OP_Funct3 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OP_Funct3::F000 => {
                writeln!(formatter, "0b000")
            }
            OP_Funct3::F001 => {
                writeln!(formatter, "0b001")
            }
            OP_Funct3::F010 => {
                writeln!(formatter, "0b010")
            }
            OP_Funct3::F011 => {
                writeln!(formatter, "0b011")
            }
            OP_Funct3::F100 => {
                writeln!(formatter, "0b100")
            }
            OP_Funct3::F101 => {
                writeln!(formatter, "0b101")
            }
            OP_Funct3::F110 => {
                writeln!(formatter, "0b110")
            }
            OP_Funct3::F111 => {
                writeln!(formatter, "0b111")
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq, Debug)]
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

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub enum CA_Funct_2 {
    F00 = 0b00,
    F01 = 0b01,
    F10 = 0b10,
    F11 = 0b11,
}

impl From<InstructionTypeCompressed> for CA_Funct_2 {
    fn from(value: InstructionTypeCompressed) -> Self {
        match value {
            0b00 => CA_Funct_2::F00,
            0b01 => CA_Funct_2::F01,
            0b10 => CA_Funct_2::F10,
            0b11 => CA_Funct_2::F11,

            _ => {
                error!("Unknown bits[11:10]: {value:#04b}");
                unreachable!("Unknown bits[11:10]: {value:#04b}");
            }
        }
    }
}
