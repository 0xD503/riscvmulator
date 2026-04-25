// addr: {address:#018X?}

use clap::Parser;
use log::{debug, error, info, trace, warn};
use serde::Deserialize;

use std::cmp::PartialOrd;
use std::convert::From;
use std::fmt::{self, Binary, Debug};
use std::marker::PhantomData;
use std::mem::size_of;
use std::{env, fs, u8};

/// architechtural defines
type XLen = u64;
type SignedXLen = i64;
type UnsignedXLen = u64;
type XLen32 = u32;
type SignedXLen32 = i32;
// type UnsignedXLen32 = u32;
// type XLen = u32;
// type SignedXLen = i32;
// type UnsignedXLen = u32;

type Byte = u8;
type SignedByte = i8;
type HalfWord = u16;
type SignedHalfWord = i16;
type Word = u32;
type SignedWord = i32;
type DoubleWord = u64;
type SignedDoubleWord = i8;
// type QuadWord = u128;
// type SignedQuadWord = i128;

// // type SignedImmediate = i64;
// // type UnsignedImmediate = u64;
// // type SignedImmediateCastToRegister = SignedXLen;
// // type UnsignedImmediateCastToRegister = UnsignedXLen;
// type SignedImmediate = i32;
type UnsignedImmediate = u32;

// const IALIGN: i32 = 32;
// const ILEN: i32 = IALIGN * 1;

type InstructionType = u32;
type SignedInstructionType = i32;

const RISCV_NUM_OF_REGISTERS: usize = 32;
const RISCV_X0_ZERO_REGISTER_VALUE: XLen = 0x00000000;

// default constants
const PC_RESET_VALUE: XLen = 0x00000000;
const PC_STEP: XLen = 0x00000004;

const INITIAL_INSTRUCTION: InstructionType = 0x00000000;

const MEM_DEFAULT_VALUE: u8 = 0x00;
const DRAM_BASE: XLen = 0x80000000;
const DRAM_SIZE: usize = 65536; // placeholder value, to be configured later
const DRAM_END: XLen = DRAM_BASE + DRAM_SIZE as XLen;

// decoder constants
const BIT_OFFSET_OPCODE: usize = 0;
const MASK_OPCODE: InstructionType = 0x7F;

const BIT_OFFSET_SOURCE_REGISTER_1: usize = 15;
const MASK_SOURCE_REGISTER_1: InstructionType = 0x1F;
const BIT_OFFSET_SOURCE_REGISTER_2: usize = 20;
const MASK_SOURCE_REGISTER_2: InstructionType = 0x1F;

const BIT_OFFSET_DESTINATION_REGISTER: usize = 7;
const MASK_DESTINATION_REGISTER: InstructionType = 0x1F;

const BIT_OFFSET_FUNCT3: usize = 12;
const MASK_FUNCT3: InstructionType = 0x07;
const BIT_OFFSET_I_FUNCT7: usize = 25;
const MASK_I_FUNCT7: InstructionType = 0x7F;
const BIT_OFFSET_R_FUNCT7: usize = 25;
const MASK_R_FUNCT7: InstructionType = 0x7F;

const BIT_OFFSET_SHAMT: usize = 20;
//const MASK_SHAMT: InstructionType = 0x1F;
const MASK_SHAMT: InstructionType = XLen::BITS - 1;

const BIT_OFFSET_IMM_11_0: usize = 20;
const BIT_OFFSET_IMM_11_0_FINAL: usize = 0;
const BIT_OFFSET_IMM_11_5: usize = 25;
const BIT_OFFSET_IMM_11_5_FINAL: usize = 5;
const BIT_OFFSET_IMM_4_0: usize = 7;
const BIT_OFFSET_IMM_4_0_FINAL: usize = 0;
const MASK_IMM_4_0: InstructionType = 0x1F << BIT_OFFSET_IMM_4_0_FINAL;
const BIT_OFFSET_IMM_31_12: usize = 12;
const BIT_OFFSET_IMM_31_12_FINAL: usize = 12;
const BIT_OFFSET_IMM_10_5: usize = 25;
const BIT_OFFSET_IMM_10_5_FINAL: usize = 5;
const MASK_IMM_10_5: InstructionType = 0x3F << BIT_OFFSET_IMM_10_5_FINAL;
const BIT_OFFSET_IMM_4_1: usize = 8;
const BIT_OFFSET_IMM_4_1_FINAL: usize = 1;
const MASK_IMM_4_1: InstructionType = 0x0F << BIT_OFFSET_IMM_4_1_FINAL;
const BIT_OFFSET_IMM_10_1: usize = 21;
const BIT_OFFSET_IMM_10_1_FINAL: usize = 21;
const MASK_IMM_10_1: InstructionType = 0x3FF << BIT_OFFSET_IMM_10_1_FINAL;
const BIT_OFFSET_IMM_19_12: usize = 12;
const BIT_OFFSET_IMM_19_12_FINAL: usize = 12;
const MASK_IMM_19_12: InstructionType = 0xFF << BIT_OFFSET_IMM_19_12_FINAL;

const BIT_OFFSET_B_11: usize = 7;
const BIT_OFFSET_B_11_FINAL: usize = 11;
const MASK_B_11: InstructionType = 0x01 << BIT_OFFSET_B_11_FINAL;
const BIT_OFFSET_B_12: usize = 31;
const BIT_OFFSET_B_12_FINAL: usize = 12;
const BIT_OFFSET_J_11: usize = 20;
const BIT_OFFSET_J_11_FINAL: usize = 11;
const MASK_J_11: InstructionType = 0x01 << BIT_OFFSET_B_11_FINAL;
const BIT_OFFSET_J_20: usize = 31;
const BIT_OFFSET_J_20_FINAL: usize = 20;

// const BIT_OFFSET_I_IMM_INST_31: usize = 11;
// const MASK_I_IMM_INST_31: InstructionType = 0x1FFFFF;
// const BIT_OFFSET_I_IMM_INST_30_25: usize = 5;
// const MASK_I_IMM_INST_30_25: InstructionType = 0x3F;
// const BIT_OFFSET_I_IMM_INST_24_21: usize = 1;
// const MASK_I_IMM_INST_24_21: InstructionType = 0x0F;
// const BIT_OFFSET_I_IMM_20: usize = 0;
// const MASK_I_IMM_20: InstructionType = 0x01;

// const BIT_OFFSET_S_IMM_INST_31: usize = 11;
// const MASK_S_IMM_INST_31: InstructionType = 0x1FFFFF;
// const BIT_OFFSET_S_IMM_INST_30_25: usize = 5;
// const MASK_S_IMM_INST_30_25: InstructionType = 0x3F;
// const BIT_OFFSET_S_IMM_INST_11_8: usize = 1;
// const MASK_S_IMM_INST_11_8: InstructionType = 0x0F;
// const BIT_OFFSET_S_IMM_7: usize = 0;
// const MASK_S_IMM_7: InstructionType = 0x01;

// const BIT_OFFSET_B_IMM_INST_31: usize = 12;
// const MASK_B_IMM_INST_31: InstructionType = 0xFFFFF;
// const BIT_OFFSET_B_IMM_7: usize = 11;
// const MASK_B_IMM_7: InstructionType = 0x01;
// const BIT_OFFSET_B_IMM_INST_30_25: usize = 5;
// const MASK_B_IMM_INST_30_25: InstructionType = 0x3F;
// const BIT_OFFSET_B_IMM_INST_11_8: usize = 1;
// const MASK_B_IMM_INST_11_8: InstructionType = 0x0F;
// const BIT_OFFSET_B_IMM_0: usize = 0;
// const MASK_B_IMM_0: InstructionType = 0x01;

// const BIT_OFFSET_U_IMM_31: usize = 31;
// const MASK_U_IMM_31: InstructionType = 0x01;
// const BIT_OFFSET_U_IMM_INST_30_20: usize = 20;
// const MASK_U_IMM_INST_30_20: InstructionType = 0x7FF;
// const BIT_OFFSET_U_IMM_INST_19_12: usize = 12;
// const MASK_U_IMM_INST_19_12: InstructionType = 0xFF;
// const BIT_OFFSET_U_IMM_0: usize = 0;
// const MASK_U_IMM_0: InstructionType = 0xFFF;

// const BIT_OFFSET_J_IMM_INST_31: usize = 20;
// const MASK_J_IMM_INST_31: InstructionType = 0xFFF;
// const BIT_OFFSET_J_IMM_INST_19_12: usize = 12;
// const BIT_OFFSET_J_IMM_INST_19_12_FINAL: usize = 12;
// const MASK_J_IMM_INST_19_12: InstructionType =
//     0xFF << BIT_OFFSET_J_IMM_INST_19_12_FINAL;
// const BIT_OFFSET_J_IMM_20: usize = 11;
// const BIT_OFFSET_J_IMM_20_FINAL: usize = 20;
// const MASK_J_IMM_20: InstructionType = 0x01 << BIT_OFFSET_J_IMM_20_FINAL;
// const BIT_OFFSET_J_IMM_INST_30_25: usize = 5;
// const MASK_J_IMM_INST_30_25: InstructionType = 0x3F;
// const BIT_OFFSET_J_IMM_INST_24_21: usize = 1;
// const MASK_J_IMM_INST_24_21: InstructionType = 0x0F;
// const BIT_OFFSET_J_IMM_0: usize = 0;
// const MASK_J_IMM_0: InstructionType = 0x01;

//const INSTRUCTION_NOP: InstructionType = Opcode::OP_IMM as InstructionType;

const OPCODE_LOAD: u8 = 0b0000011;
const OPCODE_STORE: u8 = 0b0100011;
const OPCODE_MADD: u8 = 0b1000011;
const OPCODE_BRANCH: u8 = 0b1100011;
const OPCODE_LOAD_FP: u8 = 0b0000111;
const OPCODE_STORE_FP: u8 = 0b0100111;
const OPCODE_MSUB: u8 = 0b1000111;
const OPCODE_JALR: u8 = 0b1100111;
const OPCODE_CUSTOM_0: u8 = 0b0001011;
const OPCODE_CUSTOM_1: u8 = 0b0101011;
const OPCODE_NMSUB: u8 = 0b1001011;
const OPCODE_RESERVED_1: u8 = 0b1101011;
const OPCODE_MISC_MEM: u8 = 0b0001111;
const OPCODE_AMO: u8 = 0b0101111;
const OPCODE_NMADD: u8 = 0b1001111;
const OPCODE_JAL: u8 = 0b1101111;
const OPCODE_OP_IMM: u8 = 0b0010011;
const OPCODE_OP: u8 = 0b0110011;
const OPCODE_OP_FP: u8 = 0b1010011;
const OPCODE_SYSTEM: u8 = 0b1110011;
const OPCODE_AUIPC: u8 = 0b0010111;
const OPCODE_LUI: u8 = 0b0110111;
const OPCODE_OP_V: u8 = 0b1010111;
const OPCODE_OP_VE: u8 = 0b1110111;
const OPCODE_OP_IMM_32: u8 = 0b0011011;
const OPCODE_OP_32: u8 = 0b0111011;
const OPCODE_CUSTOM_2: u8 = 0b1011011;
const OPCODE_CUSTOM_3: u8 = 0b1111011;
const OPCODE_RESERVED_2: u8 = 0b0011111;
const OPCODE_RESERVED_3: u8 = 0b0111111;
const OPCODE_RESERVED_4: u8 = 0b1011111;
const OPCODE_RESERVED_5: u8 = 0b1111111;

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Default, Clone)]
enum Opcode {
    LOAD = OPCODE_LOAD as isize,
    STORE = OPCODE_STORE as isize,
    MADD = OPCODE_MADD as isize,
    BRANCH = OPCODE_BRANCH as isize,

    LOAD_FP = OPCODE_LOAD_FP as isize,
    STORE_FP = OPCODE_STORE_FP as isize,
    MSUB = OPCODE_MSUB as isize,
    JALR = OPCODE_JALR as isize,

    custom_0 = OPCODE_CUSTOM_0 as isize,
    custom_1 = OPCODE_CUSTOM_1 as isize,
    NMSUB = OPCODE_NMSUB as isize,
    reserved_1 = OPCODE_RESERVED_1 as isize,

    MISC_MEM = OPCODE_MISC_MEM as isize,
    AMO = OPCODE_AMO as isize,
    NMADD = OPCODE_NMADD as isize,
    JAL = OPCODE_JAL as isize,

    #[default]
    OP_IMM = OPCODE_OP_IMM as isize,
    OP = OPCODE_OP as isize,
    OP_FP = OPCODE_OP_FP as isize,
    SYSTEM = OPCODE_SYSTEM as isize,

    AUIPC = OPCODE_AUIPC as isize,
    LUI = OPCODE_LUI as isize,
    OP_V = OPCODE_OP_V as isize,
    OP_VE = OPCODE_OP_VE as isize,

    OP_IMM_32 = OPCODE_OP_IMM_32 as isize,
    OP_32 = OPCODE_OP_32 as isize,
    custom_2 = OPCODE_CUSTOM_2 as isize,
    custom_3 = OPCODE_CUSTOM_3 as isize,

    reserved_2 = OPCODE_RESERVED_2 as isize,
    reserved_3 = OPCODE_RESERVED_3 as isize,
    reserved_4 = OPCODE_RESERVED_4 as isize,
    reserved_5 = OPCODE_RESERVED_5 as isize,
}

impl Binary for Opcode {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.clone() as InstructionType)
    }
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            OPCODE_LOAD => Self::LOAD,
            OPCODE_STORE => Self::STORE,
            OPCODE_MADD => Self::MADD,
            OPCODE_BRANCH => Self::BRANCH,

            OPCODE_LOAD_FP => Self::LOAD_FP,
            OPCODE_STORE_FP => Self::STORE_FP,
            OPCODE_MSUB => Self::MSUB,
            OPCODE_JALR => Self::JALR,

            OPCODE_CUSTOM_0 => Self::custom_0,
            OPCODE_CUSTOM_1 => Self::custom_1,
            OPCODE_NMSUB => Self::NMSUB,
            OPCODE_RESERVED_1 => Self::reserved_1,

            OPCODE_MISC_MEM => Self::MISC_MEM,
            OPCODE_AMO => Self::AMO,
            OPCODE_NMADD => Self::NMADD,
            OPCODE_JAL => Self::JAL,

            OPCODE_OP_IMM => Self::OP_IMM,
            OPCODE_OP => Self::OP,
            OPCODE_OP_FP => Self::OP_FP,
            OPCODE_SYSTEM => Self::SYSTEM,

            OPCODE_AUIPC => Self::AUIPC,
            OPCODE_LUI => Self::LUI,
            OPCODE_OP_V => Self::OP_V,
            OPCODE_OP_VE => Self::OP_VE,

            OPCODE_OP_IMM_32 => Self::OP_IMM_32,
            OPCODE_OP_32 => Self::OP_32,
            OPCODE_CUSTOM_2 => Self::custom_2,
            OPCODE_CUSTOM_3 => Self::custom_3,

            OPCODE_RESERVED_2 => Self::reserved_2,
            OPCODE_RESERVED_3 => Self::reserved_3,
            OPCODE_RESERVED_4 => Self::reserved_4,
            OPCODE_RESERVED_5 => Self::reserved_5,
            _ => {
                error!("Unknown opcode: {value:#018X}");
                unreachable!("Unknown opcode: {value:#018X}");
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
enum OPCODE_LOAD_Funct3 {
    LB = 0b000,  // load byte
    LH = 0b001,  // load half word
    LW = 0b010,  // load word
    LD = 0b011,  // load double word
    LBU = 0b100, // load byte unsigned
    LHU = 0b101, // load half word unsigned
    LWU = 0b110, // load word unsigned
}

impl From<InstructionType> for OPCODE_LOAD_Funct3 {
    fn from(value: InstructionType) -> Self {
        match value {
            0b000 => OPCODE_LOAD_Funct3::LB,
            0b001 => OPCODE_LOAD_Funct3::LH,
            0b010 => OPCODE_LOAD_Funct3::LW,
            0b011 => OPCODE_LOAD_Funct3::LD,
            0b100 => OPCODE_LOAD_Funct3::LBU,
            0b101 => OPCODE_LOAD_Funct3::LHU,
            0b110 => OPCODE_LOAD_Funct3::LWU,

            _ => {
                error!("Unknown funct3: {value:#05b}");
                unreachable!("Unknown funct3: {value:#05b}");
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
enum OP_IMM_Funct3 {
    ADDI = 0b000,  // add immediate
    SLLI = 0b001,  // shift left logical immediate
    SLTI = 0b010,  // set if register less than immediate
    SLTIU = 0b011, // set if register less than immediate (unsigned)
    XORI = 0b100,  // XOR with immediate
    SRXI = 0b101,  // shift right X immediate
    ORI = 0b110,   // OR with immediate
    ANDI = 0b111,  // AND with immediate
}

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Default, Clone)]
enum OP_IMM_Funct7 {
    #[default]
    SXLI = 0b0000000,
    SRAI = 0b0100000,
}

impl From<InstructionType> for OP_IMM_Funct7 {
    fn from(value: InstructionType) -> Self {
        match value {
            0b0000000 => OP_IMM_Funct7::SXLI,
            0b0100000 => OP_IMM_Funct7::SRAI,

            _ => {
                error!("Unknown funct7: {value:#09b}");
                unreachable!("Unknown funct7: {value:#09b}");
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Default, Clone)]
enum Funct7 {
    #[default]
    F0 = 0b0000000,
    F1 = 0b0100000,
}

impl From<InstructionType> for Funct7 {
    fn from(value: InstructionType) -> Self {
        match value {
            0b0000000 => Funct7::F0,
            0b0100000 => Funct7::F1,

            _ => {
                error!("Unknown funct7: {value:#09b}");
                unreachable!("Unknown funct7: {value:#09b}");
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
enum OP_Funct3 {
    ARITH = 0b000, //
    SLL = 0b001,   //
    SLT = 0b010,   //
    SLTU = 0b011,  //
    XOR = 0b100,   //
    SRX = 0b101,   //
    OR = 0b110,    //
    AND = 0b111,   //
}

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
enum OP_32_Funct3 {
    ARITHW = 0b000, //
    SLLW = 0b001,   //
    SRXW = 0b101,   //
}

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
enum OP_IMM_32_Funct3 {
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

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
enum JALR_Funct3 {
    JALR = 0b000, //
}

impl From<InstructionType> for OP_IMM_Funct3 {
    fn from(value: InstructionType) -> Self {
        match value {
            0b000 => OP_IMM_Funct3::ADDI,
            0b001 => OP_IMM_Funct3::SLLI,
            0b010 => OP_IMM_Funct3::SLTI,
            0b011 => OP_IMM_Funct3::SLTIU,
            0b100 => OP_IMM_Funct3::XORI,
            0b101 => OP_IMM_Funct3::SRXI,
            0b110 => OP_IMM_Funct3::ORI,
            0b111 => OP_IMM_Funct3::ANDI,

            _ => {
                error!("Unknown funct3: {value:#05b}");
                unreachable!("Unknown funct3: {value:#05b}");
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
enum STORE_Funct3 {
    SB = 0b000, //
    SH = 0b001, //
    SW = 0b010, //
    SD = 0b011, //
}

impl From<InstructionType> for STORE_Funct3 {
    fn from(value: InstructionType) -> Self {
        match value {
            0b000 => STORE_Funct3::SB,
            0b001 => STORE_Funct3::SH,
            0b010 => STORE_Funct3::SW,
            0b011 => STORE_Funct3::SD,

            _ => {
                error!("Unknown funct3: {value:#05b}");
                unreachable!("Unknown funct3: {value:#05b}");
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
enum BRANCH_Funct3 {
    BEQ = 0b000,  //
    BNE = 0b001,  //
    BLT = 0b100,  //
    BGE = 0b101,  //
    BLTU = 0b110, //
    BGEU = 0b111, //
}

impl From<InstructionType> for BRANCH_Funct3 {
    fn from(value: InstructionType) -> Self {
        match value {
            0b000 => BRANCH_Funct3::BEQ,
            0b001 => BRANCH_Funct3::BNE,
            0b100 => BRANCH_Funct3::BLT,
            0b101 => BRANCH_Funct3::BGE,
            0b110 => BRANCH_Funct3::BLTU,
            0b111 => BRANCH_Funct3::BGEU,

            _ => {
                error!("Unknown funct3: {value:#05b}");
                unreachable!("Unknown funct3: {value:#05b}");
            }
        }
    }
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

impl From<InstructionType> for OP_32_Funct3 {
    fn from(value: InstructionType) -> Self {
        match value {
            0b000 => OP_32_Funct3::ARITHW,
            0b001 => OP_32_Funct3::SLLW,
            0b010 => OP_32_Funct3::SRXW,

            _ => {
                error!("Unknown funct3: {value:#05b}");
                unreachable!("Unknown funct3: {value:#05b}");
            }
        }
    }
}

impl From<InstructionType> for JALR_Funct3 {
    fn from(value: InstructionType) -> Self {
        match value {
            0b000 => JALR_Funct3::JALR,

            _ => {
                error!("Unknown funct3: {value:#05b}");
                unreachable!("Unknown funct3: {value:#05b}");
            }
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Default, Clone, Debug)]
enum ExecutableInstruction {
    #[default]
    LB, // load byte
    LH, // load halfword
    LW, // load word

    LD,  // load double word
    LBU, // load byte (unsigned)
    LHU, // load halfword (unsigned)
    LWU, // load word (unsigned)

    ADD,  // add registers
    SUB,  // subtract registers
    SLL,  // shift left logical registers
    SLT,  // set if register less then register
    SLTU, // set if register less then register (unsigned)
    XOR,  // XOR with register
    SRL,  // shift right logical register
    SRA,  // shift right arithmetic register
    OR,   // OR with register
    AND,  // AND with register

    ADDW, // add word (register)
    SUBW, // subtract word (register)
    SLLW, // shift left logical register word
    SRLW, // shift right logical register word
    SRAW, // shift right arithmetic register word

    ADDI,  // add immediate
    SLLI,  // shift left logical immediate
    SLTI,  // set if register less then immediate
    SLTIU, // set if register less then immediate (unsigned)
    XORI,  // XOR with immediate
    SRLI,  // shift right logical immediate
    SRAI,  // shift right arithmetic immediate
    ORI,   // OR with immediate
    ANDI,  // AND with immediate
    JALR,  // jump and link register

    ADDIW, // add immediate word
    SLLIW, // shift left logical immediate word
    SRLIW, // shift right logical immediate word
    SRAIW, // shift right arithmetic immediate word

    SB, // store byte
    SH, // store halfword
    SW, // store word

    SD, // store double word

    AUIPC, // add upper intermideate to PC
    LUI,   // load upper intermideate

    BEQ,  // branch if equal
    BNE,  // branch if not equal
    BLT,  // branch if less than
    BGE,  // branch if greater or equal than
    BLTU, // branch if less than (unsigned)
    BGEU, // branch if greater or equal than (unsigned)

    JAL, // jump and link
}

#[derive(Clone)]
struct RegisterFile {
    registers: [XLen; RISCV_NUM_OF_REGISTERS],
}

#[allow(non_camel_case_types, dead_code)]
enum RegisterName {
    x0_zero,  // always zero
    x1_ra,    // return address
    x2_sp,    // stack pointer
    x3_gp,    // global pointer
    x4_tp,    // thread pointer
    x5_t0,    // temporary register 0
    x6_t1,    // temporary register 1
    x7_t2,    // temporary register 2
    x8_s0_fp, // saved register 0 / frame pointer
    x9_s1,    // saved register 1
    x10_a0,   // function argument 0 / return value 0
    x11_a1,   // function argument 1 / return value 1
    x12_a2,   // function argument 2
    x13_a3,   // function argument 3
    x14_a4,   // function argument 4
    x15_a5,   // function argument 5
    x16_a6,   // function argument 6
    x17_a7,   // function argument 7
    x18_s2,   // saved register 2
    x19_s3,   // saved register 3
    x20_s4,   // saved register 4
    x21_s5,   // saved register 5
    x22_s6,   // saved register 6
    x23_s7,   // saved register 7
    x24_s8,   // saved register 8
    x25_s9,   // saved register 9
    x26_s10,  // saved register 10
    x27_s11,  // saved register 11
    x28_t3,   // temporary register 3
    x29_t4,   // temporary register 4
    x30_t5,   // temporary register 5
    x31_t6,   // temporary register 6
}

impl RegisterFile {
    fn read(&self, offset: usize) -> XLen {
        trace!("Reading register file...");

        self.registers[offset]
    }

    fn write(&mut self, offset: usize, value: XLen) {
        trace!("Writing register file...");

        // register `x0_zero` is hardwired with all bits equal to 0, so do nothing with `x0_zero`
        if offset != (RegisterName::x0_zero as usize) {
            self.registers[offset] = value;
        } else {
            trace!(
                "An attempt to write value {value} to register `x0_zero` was silently discarded"
            );
        }
    }
}

// #[derive(Default, Clone)]
// enum InstructionFormat {
//     R,
//     #[default]
//     I,
//     S,
//     B,
//     U,
//     J,
//     R4,
// }

#[derive(Default, Clone)]
struct Decoder {
    opcode: Opcode,
    source_register_1: XLen,
    source_register_2: XLen,
    destination_register_index: usize,

    funct3: InstructionType,
    i_funct7: OP_IMM_Funct7,
    r_funct7: Funct7,

    shamt: InstructionType,

    immediate: UnsignedImmediate,
    executable_instruction: ExecutableInstruction,
    need_to_read_memory: bool,
    need_to_write_memory: bool,
    need_to_write_back: bool,
    need_to_jump: bool,
    //
    // i_imm_inst_31: InstructionType,
    // i_imm_inst_30_25: InstructionType,
    // i_imm_inst_24_21: InstructionType,
    // i_imm_inst_20: InstructionType,

    // s_imm_inst_31: InstructionType,
    // s_imm_inst_30_25: InstructionType,
    // s_imm_inst_11_8: InstructionType,
    // s_imm_inst_7: InstructionType,

    // b_imm_inst_31: InstructionType,
    // b_imm_7: InstructionType,
    // b_imm_inst_30_25: InstructionType,
    // b_imm_inst_11_8: InstructionType,
    // b_imm_0: InstructionType,

    // u_imm_31: InstructionType,
    // u_imm_inst_30_20: InstructionType,
    // u_imm_inst_19_12: InstructionType,
    // u_imm_0: InstructionType,

    // j_imm_inst_31: InstructionType,
    // j_imm_inst_19_12: InstructionType,
    // j_imm_20: InstructionType,
    // j_imm_inst_30_25: InstructionType,
    // j_imm_inst_24_21: InstructionType,
    // j_imm_0: InstructionType,
}

#[derive(Default, Clone)]
struct ExecutionContext {
    result: XLen,
    branch_target_address: XLen,
    jump_target_address: XLen,
    memory_address: XLen,
    need_to_branch: bool,
}

#[allow(non_snake_case)]
#[derive(Clone)]
struct Context {
    register_file: RegisterFile,
    PC: XLen,
    current_instruction: InstructionType,
    decoder: Decoder,
    execution: ExecutionContext,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone)]
struct CPU<AddrType> {
    context: Context,

    _phantom_marker: PhantomData<AddrType>,
}

trait Bus<AddrType> {
    // fn read_byte(&self, address: XLen) -> Byte;
    // fn read_half_word(&self, address: XLen) -> HalfWord;
    // fn read_word(&self, address: XLen) -> Word;
    // fn read_double_word(&self, address: XLen) -> DoubleWord;
    // fn read_quad_word(&self, address: XLen) -> QuadWord;

    fn read<MemValType: MemoryValue>(&self, address: AddrType) -> MemValType;

    // fn read_data(&self, address: XLen, size: usize) -> Vec<Byte>;

    // fn write_byte(&self, address: XLen, value: Byte);
    // fn write_half_word(&self, address: XLen, value: HalfWord);
    // fn write_word(&self, address: XLen, value: Word);
    // fn write_double_word(&self, address: XLen, value: DoubleWord);
    // fn write_quad_word(&self, address: XLen, value: QuadWord);

    fn write<MemValType: MemoryValue>(
        &self,
        address: AddrType,
        value: MemValType,
    );

    // fn write_data(&mut self, address: XLen, data: Vec<Byte>);
}

type BusAddrType = XLen;

trait MemoryValue: Sized {
    fn from_le_bytes(bytes: &[u8]) -> Self;
}

impl MemoryValue for u8 {
    fn from_le_bytes(bytes: &[u8]) -> Self {
        Self::from_le_bytes(bytes.try_into().unwrap())
    }
}

impl MemoryValue for u16 {
    fn from_le_bytes(bytes: &[u8]) -> Self {
        u16::from_le_bytes(bytes.try_into().unwrap())
    }
}
impl MemoryValue for u32 {
    fn from_le_bytes(bytes: &[u8]) -> Self {
        u32::from_le_bytes(bytes.try_into().unwrap())
    }
}

impl MemoryValue for u64 {
    fn from_le_bytes(bytes: &[u8]) -> Self {
        u64::from_le_bytes(bytes.try_into().unwrap())
    }
}

impl MemoryValue for u128 {
    fn from_le_bytes(bytes: &[u8]) -> Self {
        u128::from_le_bytes(bytes.try_into().unwrap())
    }
}

struct Memory<AddrType> {
    byte_array: Vec<u8>,

    _phantom_marker: PhantomData<AddrType>,
}

impl<AddrType> Memory<AddrType> {
    fn read<MemValType: MemoryValue>(&self, address: AddrType) -> MemValType {
        let bytes = (self.byte_array)
            [address as usize..address as usize + size_of::<MemValType>()]
            .try_into()
            .unwrap();
        MemValType::from_le_bytes(bytes)
    }

    // fn write<MemValType: MemoryValue>(&self, )
}

struct SystemBus<AddrType>
where
    AddrType: PartialOrd<BusAddrType>,
    AddrType: Debug,
{
    memory: Memory<AddrType>,
}

impl<AddrType: PartialOrd<BusAddrType> + Debug> Bus<AddrType>
    for SystemBus<AddrType>
{
    fn read<MemValType: MemoryValue>(&self, address: AddrType) -> MemValType {
        if (address >= DRAM_BASE) && (address < DRAM_END) {
            trace!("Reading memory at addr: {address:#018X?}");
            self.memory.read(address)
        } else {
            error!("an attempt to read from unimplemented address space!");
            panic!("an attempt to read from unimplemented address space");
        }
    }
    fn write<MemValType: MemoryValue>(
        &self,
        address: AddrType,
        value: MemValType,
    ) {
        if (address >= DRAM_BASE) && (address < DRAM_END) {
            trace!("Writing memory at addr: {address:#018X?}");
            // self.memory
        } else {
            error!("an attempt to write to unimplemented address space!");
            panic!("an attempt to write to unimplemented address space");
        }
        todo!("TODO: unimplemented: segregated address space for memory/IO");
    }
}

struct SoC<AddrType: PartialOrd<BusAddrType> + Debug> {
    cpu: CPU<AddrType>,
    bus: SystemBus<AddrType>,
}

impl<AddrType> SoC<AddrType> {
    fn fetch(&mut self) {
        let bytes = self.bus.memory.byte_array[self.cpu.context.PC as usize
            ..self.cpu.context.PC as usize + size_of::<InstructionType>()]
            .try_into()
            .unwrap();
        // NOTICE: so far emulator only supports little-endian
        let instruction = InstructionType::from_le_bytes(bytes);
        self.cpu.context.current_instruction = instruction;
        self.cpu.context.current_instruction =
            self.bus.memory.read(self.cpu.context.PC);

        trace!(
            "Fetched instruction: {:#018X?}",
            self.cpu.context.current_instruction
        );
    }

    fn reset(&mut self) {
        self.cpu.context.PC = PC_RESET_VALUE;
        // register `x0_zero` is hardwired with all bits equal to 0, so do nothing with `x0_zero`
        for i in 1..RISCV_NUM_OF_REGISTERS {
            self.cpu
                .context
                .register_file
                .write(i, RISCV_X0_ZERO_REGISTER_VALUE);
        }
        self.cpu.context.current_instruction = INITIAL_INSTRUCTION;

        // fill memory with NOP's
        self.bus.memory.byte_array = [MEM_DEFAULT_VALUE; DRAM_SIZE].to_vec();
        for i in (0..DRAM_SIZE).step_by(PC_STEP as usize) {
            self.bus.memory.byte_array[i] = Opcode::OP_IMM as Byte;
        }
    }

    fn step(&mut self) {
        trace!("Current PC: {:#018X}", self.cpu.context.PC);

        self.fetch();
        self.cpu.decode();
        self.cpu.execute();
        self.cpu.memory_operation(&mut self.bus);
        self.cpu.write_back();

        // step PC
        self.cpu.context.PC += PC_STEP;
    }
}

#[allow(non_camel_case_types)]
enum OpcodeToInstructionFormat {
    I_3 = 0b0000011,
    I_7 = 0b0000111,
    I_19 = 0b0010011,
    U_23 = 0b0010111,
    I_27 = 0b0011011,
    S_35 = 0b0100011,
    S_39 = 0b0100111,
    R_51 = 0b0110011,
    U_55 = 0b0110111,
    R_59 = 0b0111011,
    R4_67 = 0b1000011,
    R4_71 = 0b1000111,
    R4_75 = 0b1001011,
    R4_79 = 0b1001111,
    R_83 = 0b1010011,
    B_99 = 0b1100011,
    I_103 = 0b1100111,
    J_111 = 0b1101111,
}

impl From<Opcode> for OpcodeToInstructionFormat {
    fn from(opcode: Opcode) -> Self {
        match opcode {
            Opcode::LOAD => OpcodeToInstructionFormat::I_3,
            Opcode::STORE => OpcodeToInstructionFormat::S_35,
            Opcode::MADD => OpcodeToInstructionFormat::R4_67,
            Opcode::BRANCH => OpcodeToInstructionFormat::B_99,

            Opcode::LOAD_FP => OpcodeToInstructionFormat::I_7,
            Opcode::STORE_FP => OpcodeToInstructionFormat::S_39,
            Opcode::MSUB => OpcodeToInstructionFormat::R4_71,
            Opcode::JALR => OpcodeToInstructionFormat::I_103,

            Opcode::NMSUB => OpcodeToInstructionFormat::R4_75,

            Opcode::NMADD => OpcodeToInstructionFormat::R4_79,
            Opcode::JAL => OpcodeToInstructionFormat::J_111,

            Opcode::OP_IMM => OpcodeToInstructionFormat::I_19,
            Opcode::OP => OpcodeToInstructionFormat::R_51,
            Opcode::OP_FP => OpcodeToInstructionFormat::R_83,

            Opcode::AUIPC => OpcodeToInstructionFormat::U_23,
            Opcode::LUI => OpcodeToInstructionFormat::U_55,

            Opcode::OP_IMM_32 => OpcodeToInstructionFormat::I_27,
            Opcode::OP_32 => OpcodeToInstructionFormat::R_59,

            Opcode::MISC_MEM
            | Opcode::AMO
            | Opcode::SYSTEM
            | Opcode::OP_V
            | Opcode::OP_VE => {
                error!("Opcode not implemented yet: {:#09b}", opcode);
                unreachable!("Opcode not implemented yet: {:#09b}", opcode);
            }

            Opcode::reserved_1
            | Opcode::reserved_2
            | Opcode::reserved_3
            | Opcode::reserved_4
            | Opcode::reserved_5 => {
                error!("Opcode reserved for future use: {:#09b}", opcode);
                unreachable!("Opcode reserved for future use: {:#09b}", opcode);
            }

            Opcode::custom_0
            | Opcode::custom_1
            | Opcode::custom_2
            | Opcode::custom_3 => {
                error!("Opcode reserved for custom commands: {:#09b}", opcode);
                unreachable!(
                    "Opcode reserved for custom commands: {:#09b}",
                    opcode
                );
            }
        }
    }
}

impl<AddrType: PartialOrd<BusAddrType> + Debug> CPU<AddrType> {
    fn decode_r_format(&mut self) {
        self.context.decoder.source_register_1 =
            self.extract_source_register_1();
        self.context.decoder.source_register_2 =
            self.extract_source_register_2();
        self.context.decoder.destination_register_index =
            self.extract_destination_register_index();
        self.context.decoder.need_to_read_memory = false;
        self.context.decoder.need_to_write_memory = false;
        self.context.decoder.need_to_write_back = true;
        self.context.decoder.need_to_jump = false;
        self.context.execution.need_to_branch = false;
        self.context.decoder.funct3 = self.extract_funct3();
        self.context.decoder.r_funct7 = self.extract_r_funct7();

        match self.context.decoder.opcode {
            Opcode::OP => match OP_Funct3::from(self.context.decoder.funct3) {
                OP_Funct3::ARITH => match self.context.decoder.r_funct7 {
                    Funct7::F0 => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::ADD;
                    }
                    Funct7::F1 => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::SUB;
                    }
                },
                OP_Funct3::SLL => match self.context.decoder.r_funct7 {
                    Funct7::F0 => {
                        self.context.decoder.shamt = self.extract_shamt();
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::SLL;
                    }

                    _ => {
                        error!("unreachable!");
                        unreachable!("unreachable!");
                    }
                },
                OP_Funct3::SLT => match self.context.decoder.r_funct7 {
                    Funct7::F0 => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::SLT;
                    }

                    _ => {
                        error!("unreachable!");
                        unreachable!("unreachable!");
                    }
                },
                OP_Funct3::SLTU => match self.context.decoder.r_funct7 {
                    Funct7::F0 => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::SLTU;
                    }

                    _ => {
                        error!("unreachable!");
                        unreachable!("unreachable!");
                    }
                },
                OP_Funct3::XOR => match self.context.decoder.r_funct7 {
                    Funct7::F0 => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::XOR;
                    }

                    _ => {
                        error!("unreachable!");
                        unreachable!("unreachable!");
                    }
                },
                OP_Funct3::SRX => {
                    self.context.decoder.shamt = self.extract_shamt();
                    match self.context.decoder.r_funct7 {
                        Funct7::F0 => {
                            self.context.decoder.executable_instruction =
                                ExecutableInstruction::SRL;
                        }
                        Funct7::F1 => {
                            self.context.decoder.executable_instruction =
                                ExecutableInstruction::SRA;
                        }
                    }
                }
                OP_Funct3::OR => match self.context.decoder.r_funct7 {
                    Funct7::F0 => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::OR;
                    }

                    _ => {
                        error!("unreachable!");
                        unreachable!("unreachable!");
                    }
                },
                OP_Funct3::AND => match self.context.decoder.r_funct7 {
                    Funct7::F0 => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::AND;
                    }

                    _ => {
                        error!("unreachable!");
                        unreachable!("unreachable!");
                    }
                },
            },
            Opcode::OP_32 => {
                match OP_32_Funct3::from(self.context.decoder.funct3) {
                    OP_32_Funct3::ARITHW => {
                        match self.context.decoder.r_funct7 {
                            Funct7::F0 => {
                                self.context.decoder.executable_instruction =
                                    ExecutableInstruction::ADDW;
                            }
                            Funct7::F1 => {
                                self.context.decoder.executable_instruction =
                                    ExecutableInstruction::SUBW;
                            }
                        }
                    }
                    OP_32_Funct3::SLLW => {
                        self.context.decoder.shamt = self.extract_shamt();
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::SLLW;
                    }
                    OP_32_Funct3::SRXW => {
                        self.context.decoder.shamt = self.extract_shamt();
                        match self.context.decoder.r_funct7 {
                            Funct7::F0 => {
                                self.context.decoder.executable_instruction =
                                    ExecutableInstruction::SRLW;
                            }
                            Funct7::F1 => {
                                self.context.decoder.executable_instruction =
                                    ExecutableInstruction::SRAW;
                            }
                        }
                    }
                }
            }

            _ => {
                error!("unreachable!");
                unreachable!("unreachable!");
            }
        }
    }

    fn decode_i_format(&mut self) {
        self.context.decoder.source_register_1 =
            self.extract_source_register_1();
        self.context.decoder.destination_register_index =
            self.extract_destination_register_index();
        self.context.decoder.immediate =
            self.extract_imm_11_0() as UnsignedImmediate;
        self.context.decoder.need_to_read_memory = false;
        self.context.decoder.need_to_write_memory = false;
        self.context.decoder.need_to_write_back = true;
        self.context.decoder.need_to_jump = false;
        self.context.execution.need_to_branch = false;
        self.context.decoder.funct3 = self.extract_funct3();

        match self.context.decoder.opcode {
            // I_3
            Opcode::LOAD => {
                match OPCODE_LOAD_Funct3::from(self.context.decoder.funct3) {
                    OPCODE_LOAD_Funct3::LB => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::LB
                    }
                    OPCODE_LOAD_Funct3::LH => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::LH
                    }
                    OPCODE_LOAD_Funct3::LW => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::LW
                    }
                    OPCODE_LOAD_Funct3::LD => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::LD
                    }
                    OPCODE_LOAD_Funct3::LBU => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::LBU
                    }
                    OPCODE_LOAD_Funct3::LHU => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::LHU
                    }
                    OPCODE_LOAD_Funct3::LWU => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::LWU
                    }
                }
            }
            // I_19
            Opcode::OP_IMM => {
                match OP_IMM_Funct3::from(self.context.decoder.funct3) {
                    OP_IMM_Funct3::ADDI => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::ADDI;
                    }
                    OP_IMM_Funct3::SLLI => {
                        self.context.decoder.shamt = self.extract_shamt();
                        self.context.decoder.i_funct7 = self.extract_i_funct7();
                        match self.context.decoder.i_funct7 {
                            OP_IMM_Funct7::SXLI => {
                                // funct3 equals SLLI
                                self.context.decoder.executable_instruction =
                                    ExecutableInstruction::SLLI
                            }

                            _ => {
                                error!("unreachable!");
                                unreachable!("unreachable!");
                            }
                        }
                    }
                    OP_IMM_Funct3::SLTI => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::SLTI;
                    }
                    OP_IMM_Funct3::SLTIU => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::SLTIU;
                    }
                    OP_IMM_Funct3::XORI => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::XORI;
                    }
                    OP_IMM_Funct3::SRXI => {
                        self.context.decoder.shamt = self.extract_shamt();
                        self.context.decoder.i_funct7 = self.extract_i_funct7();
                        match self.context.decoder.i_funct7 {
                            OP_IMM_Funct7::SXLI => {
                                // funct3 equals SRLI
                                self.context.decoder.executable_instruction =
                                    ExecutableInstruction::SRLI;
                            }
                            OP_IMM_Funct7::SRAI => {
                                self.context.decoder.executable_instruction =
                                    ExecutableInstruction::SRAI;
                            }
                        }
                    }
                    OP_IMM_Funct3::ORI => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::ORI;
                    }
                    OP_IMM_Funct3::ANDI => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::ANDI;
                    }
                }
            }
            // I_27
            Opcode::OP_IMM_32 => {
                // self.context.decoder.source_register_1 &=
                //     UnsignedImmediate::MAX as XLen;
                match OP_IMM_32_Funct3::from(self.context.decoder.funct3) {
                    OP_IMM_32_Funct3::ADDIW => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::ADDIW
                    }
                    OP_IMM_32_Funct3::SLLIW => {
                        self.context.decoder.shamt = self.extract_shamt();
                        match self.context.decoder.i_funct7 {
                            OP_IMM_Funct7::SXLI => {
                                // funct3 equals SLLIW
                                self.context.decoder.executable_instruction =
                                    ExecutableInstruction::SLLIW
                            }

                            _ => {
                                error!("unreachable!");
                                unreachable!("unreachable!");
                            }
                        }
                    }
                    OP_IMM_32_Funct3::SRXIW => {
                        self.context.decoder.shamt = self.extract_shamt();
                        match self.context.decoder.i_funct7 {
                            OP_IMM_Funct7::SXLI => {
                                // funct3 equals SRXIW
                                self.context.decoder.executable_instruction =
                                    ExecutableInstruction::SRLIW
                            }
                            OP_IMM_Funct7::SRAI => {
                                // funct3 equals SRXIW
                                self.context.decoder.executable_instruction =
                                    ExecutableInstruction::SRAIW
                            }
                        }
                    }
                }
            }
            // I_103
            Opcode::JALR => {
                match JALR_Funct3::from(self.context.decoder.funct3) {
                    JALR_Funct3::JALR => {
                        self.context.decoder.need_to_jump = true;
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::JALR
                    }
                }
            }

            _ => {
                error!("unreachable!");
                unreachable!("unreachable!");
            }
        }
    }

    fn decode_s_format(&mut self) {
        self.context.decoder.source_register_1 =
            self.extract_source_register_1();
        self.context.decoder.source_register_2 =
            self.extract_source_register_2();
        self.context.decoder.need_to_read_memory = false;
        self.context.decoder.need_to_write_memory = true;
        self.context.decoder.need_to_write_back = false;
        self.context.decoder.need_to_jump = false;
        self.context.execution.need_to_branch = false;

        // extract immediate
        self.context.decoder.immediate = self.extract_imm_4_0();
        self.context.decoder.immediate |=
            self.extract_imm_11_5() as UnsignedImmediate;

        self.context.decoder.funct3 = self.extract_funct3();

        match self.context.decoder.opcode {
            // S_35
            Opcode::STORE => {
                match STORE_Funct3::from(self.context.decoder.funct3) {
                    STORE_Funct3::SB => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::SB;
                    }
                    STORE_Funct3::SH => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::SH;
                    }
                    STORE_Funct3::SW => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::SW;
                    }
                    STORE_Funct3::SD => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::SD;
                    }
                }
            }

            _ => {
                error!("unreachable!");
                unreachable!("unreachable!");
            }
        };
    }

    fn decode_b_format(&mut self) {
        self.context.decoder.source_register_1 =
            self.extract_source_register_1();
        self.context.decoder.source_register_2 =
            self.extract_source_register_2();
        self.context.decoder.need_to_read_memory = false;
        self.context.decoder.need_to_write_memory = false;
        self.context.decoder.need_to_write_back = false;
        self.context.decoder.need_to_jump = false;
        // reset branch flag, will be updated during execution stage
        self.context.execution.need_to_branch = false;

        // extract immediate
        self.context.decoder.immediate = self.extract_imm_4_1();
        self.context.decoder.immediate |= self.extract_imm_10_5();
        self.context.decoder.immediate |= self.extract_b_11();
        self.context.decoder.immediate |=
            self.extract_b_12() as UnsignedImmediate;

        self.context.decoder.funct3 = self.extract_funct3();

        match self.context.decoder.opcode {
            Opcode::BRANCH => {
                match BRANCH_Funct3::from(self.context.decoder.funct3) {
                    BRANCH_Funct3::BEQ => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::BEQ
                    }
                    BRANCH_Funct3::BNE => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::BNE
                    }
                    BRANCH_Funct3::BLT => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::BLT
                    }
                    BRANCH_Funct3::BGE => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::BGE
                    }
                    BRANCH_Funct3::BLTU => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::BLTU
                    }
                    BRANCH_Funct3::BGEU => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::BGEU
                    }
                }
            }

            _ => {
                error!("unreachable!");
                unreachable!("unreachable!");
            }
        }
    }

    fn decode_u_format(&mut self) {
        self.context.decoder.destination_register_index =
            self.extract_destination_register_index();
        self.context.decoder.need_to_read_memory = false;
        self.context.decoder.need_to_write_memory = false;
        self.context.decoder.need_to_write_back = true;
        self.context.decoder.need_to_jump = false;
        self.context.execution.need_to_branch = false;

        // extract immediate
        self.context.decoder.immediate =
            self.extract_imm_31_12() as UnsignedImmediate;

        match self.context.decoder.opcode {
            Opcode::AUIPC => {
                self.context.decoder.executable_instruction =
                    ExecutableInstruction::AUIPC;
            }
            Opcode::LUI => {
                self.context.decoder.executable_instruction =
                    ExecutableInstruction::LUI;
            }

            _ => {
                error!("unreachable!");
                unreachable!("unreachable!");
            }
        };
    }

    fn decode_j_format(&mut self) {
        self.context.decoder.destination_register_index =
            self.extract_destination_register_index();
        self.context.decoder.need_to_read_memory = false;
        self.context.decoder.need_to_write_memory = false;
        self.context.decoder.need_to_write_back = true;
        self.context.decoder.need_to_jump = true;
        // reset branch flag, will be updated during execution stage
        self.context.execution.need_to_branch = false;

        // extract immediate
        self.context.decoder.immediate = self.extract_imm_10_1();
        self.context.decoder.immediate |= self.extract_j_11();
        self.context.decoder.immediate |= self.extract_imm_19_12();
        self.context.decoder.immediate |=
            self.extract_j_20() as UnsignedImmediate;

        match self.context.decoder.opcode {
            Opcode::JAL => {
                self.context.decoder.executable_instruction =
                    ExecutableInstruction::JAL;
            }

            _ => {
                error!("unreachable!");
                unreachable!("unreachable!");
            }
        };
    }

    fn decode_r4_format(&mut self) {
        //
        match self.context.decoder.opcode {
            //
            _ => match self.context.decoder.funct3 {
                _ => {
                    todo!("TODO: unimplemented");
                }
            },
        };
    }

    fn decode(&mut self) {
        self.context.decoder.opcode = self.extract_opcode();

        match From::from(self.context.decoder.opcode.clone()) {
            OpcodeToInstructionFormat::R_51
            | OpcodeToInstructionFormat::R_59
            | OpcodeToInstructionFormat::R_83 => self.decode_r_format(),
            OpcodeToInstructionFormat::I_3
            | OpcodeToInstructionFormat::I_7
            | OpcodeToInstructionFormat::I_19
            | OpcodeToInstructionFormat::I_27
            | OpcodeToInstructionFormat::I_103 => self.decode_i_format(),
            OpcodeToInstructionFormat::S_35
            | OpcodeToInstructionFormat::S_39 => self.decode_s_format(),
            OpcodeToInstructionFormat::B_99 => self.decode_b_format(),
            OpcodeToInstructionFormat::U_23
            | OpcodeToInstructionFormat::U_55 => self.decode_u_format(),
            OpcodeToInstructionFormat::J_111 => self.decode_j_format(),
            OpcodeToInstructionFormat::R4_67
            | OpcodeToInstructionFormat::R4_71
            | OpcodeToInstructionFormat::R4_75
            | OpcodeToInstructionFormat::R4_79 => self.decode_r4_format(),
        }

        // // TODO: remove unconditional decoding
        // self.context.decoder.i_imm_inst_31 = self.extract_i_imm_inst_31();
        // self.context.decoder.i_imm_inst_30_25 = self.extract_i_imm_inst_30_25();
        // self.context.decoder.i_imm_inst_24_21 = self.extract_i_imm_inst_24_21();
        // self.context.decoder.i_imm_inst_20 = self.extract_i_imm_20();

        // self.context.decoder.s_imm_inst_31 = self.extract_s_imm_inst_31();
        // self.context.decoder.s_imm_inst_30_25 = self.extract_s_imm_inst_30_25();
        // self.context.decoder.s_imm_inst_11_8 = self.extract_s_imm_inst_11_8();
        // self.context.decoder.s_imm_inst_7 = self.extract_s_imm_7();

        // self.context.decoder.b_imm_inst_31 = self.extract_b_imm_inst_31();
        // self.context.decoder.b_imm_7 = self.extract_b_imm_7();
        // self.context.decoder.b_imm_inst_30_25 = self.extract_b_imm_inst_30_25();
        // self.context.decoder.b_imm_inst_11_8 = self.extract_b_imm_inst_11_8();
        // self.context.decoder.b_imm_0 = self.extract_b_imm_0();

        // self.context.decoder.u_imm_31 = self.extract_u_imm_31();
        // self.context.decoder.u_imm_inst_30_20 = self.extract_u_imm_inst_30_20();
        // self.context.decoder.u_imm_inst_19_12 = self.extract_u_imm_inst_19_12();
        // self.context.decoder.u_imm_0 = self.extract_u_imm_0();

        // self.context.decoder.j_imm_inst_31 = self.extract_j_imm_inst_31();
        // self.context.decoder.j_imm_inst_19_12 = self.extract_j_imm_inst_19_12();
        // self.context.decoder.j_imm_20 = self.extract_j_imm_20();
        // self.context.decoder.j_imm_inst_30_25 = self.extract_j_imm_inst_30_25();
        // self.context.decoder.j_imm_inst_24_21 = self.extract_j_imm_inst_24_21();
        // self.context.decoder.j_imm_0 = self.extract_j_imm_0();
    }

    fn execute(&mut self) {
        match self.context.decoder.executable_instruction {
            // RV32I
            // I_3
            ExecutableInstruction::LB => self.instruction_load_byte(),
            ExecutableInstruction::LH => self.instruction_load_half(),
            ExecutableInstruction::LW => self.instruction_load_word(),
            ExecutableInstruction::LBU => self.instruction_load_byte_unsigned(),
            ExecutableInstruction::LHU => self.instruction_load_half_unsigned(),
            // I_19
            ExecutableInstruction::ADDI => self.instruction_addi(),
            ExecutableInstruction::SLLI => self.instruction_slli(),
            ExecutableInstruction::SLTI => self.instruction_slti(),
            ExecutableInstruction::SLTIU => self.instruction_sltiu(),
            ExecutableInstruction::XORI => self.instruction_xori(),
            ExecutableInstruction::SRLI => self.instruction_srli(),
            ExecutableInstruction::SRAI => self.instruction_srai(),
            ExecutableInstruction::ORI => self.instruction_ori(),
            ExecutableInstruction::ANDI => self.instruction_andi(),
            // U_23
            ExecutableInstruction::AUIPC => self.instruction_auipc(),
            // S_35
            ExecutableInstruction::SB => self.instruction_store_byte(),
            ExecutableInstruction::SH => self.instruction_store_half_word(),
            ExecutableInstruction::SW => self.instruction_store_word(),
            // R_51
            ExecutableInstruction::ADD => self.instruction_add(),
            ExecutableInstruction::SUB => self.instruction_sub(),
            ExecutableInstruction::SLL => self.instruction_sll(),
            ExecutableInstruction::SLT => self.instruction_slt(),
            ExecutableInstruction::SLTU => self.instruction_sltu(),
            ExecutableInstruction::XOR => self.instruction_xor(),
            ExecutableInstruction::SRL => self.instruction_srl(),
            ExecutableInstruction::SRA => self.instruction_sra(),
            ExecutableInstruction::OR => self.instruction_or(),
            ExecutableInstruction::AND => self.instruction_and(),
            // U_55
            ExecutableInstruction::LUI => self.instruction_lui(),
            // B_99
            ExecutableInstruction::BEQ => self.instruction_beq(),
            ExecutableInstruction::BNE => self.instruction_bne(),
            ExecutableInstruction::BLT => self.instruction_blt(),
            ExecutableInstruction::BGE => self.instruction_bge(),
            ExecutableInstruction::BLTU => self.instruction_bltu(),
            ExecutableInstruction::BGEU => self.instruction_bgeu(),
            // I_103
            ExecutableInstruction::JALR => self.instruction_jalr(),
            // J_111
            ExecutableInstruction::JAL => self.instruction_jal(),

            // RV64I
            // I_3
            ExecutableInstruction::LD => self.instruction_load_double_word(),
            ExecutableInstruction::LWU => self.instruction_load_word_unsigned(),
            // I_27
            ExecutableInstruction::ADDIW => {
                self.instruction_add_immediate_word();
            }
            ExecutableInstruction::SLLIW => {
                self.instruction_shift_left_logical_immediate_word();
            }
            ExecutableInstruction::SRLIW => {
                self.instruction_shift_right_logical_immediate_word();
            }
            ExecutableInstruction::SRAIW => {
                self.instruction_shift_right_arith_immediate_word();
            }
            // S_35
            ExecutableInstruction::SD => self.instruction_store_double_word(),
            // R_59
            ExecutableInstruction::ADDW => self.instruction_add_word(),
            ExecutableInstruction::SUBW => self.instruction_sub_word(),
            ExecutableInstruction::SLLW => {
                self.instruction_shift_left_logical_word();
            }
            ExecutableInstruction::SRLW => {
                self.instruction_shift_right_logical_word();
            }
            ExecutableInstruction::SRAW => {
                self.instruction_shift_right_arith_word();
            }
        }
    }

    fn instruction_load_byte(&mut self) {
        trace!("Executing LB...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedXLen as XLen);
        todo!("TODO: unimplemented");
    }

    fn instruction_load_half(&mut self) {
        trace!("Executing LH...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedXLen as XLen);
        todo!("TODO: unimplemented");
    }

    fn instruction_load_word(&mut self) {
        trace!("Executing LW...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedXLen as XLen);
        todo!("TODO: unimplemented");
    }

    fn instruction_load_byte_unsigned(&mut self) {
        trace!("Executing LBU...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedXLen as XLen);
        todo!("TODO: unimplemented");
    }

    fn instruction_load_half_unsigned(&mut self) {
        trace!("Executing LHU...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedXLen as XLen);
        todo!("TODO: unimplemented");
    }

    fn instruction_addi(&mut self) {
        trace!("Executing ADDI...");

        self.context.execution.result = self.context.decoder.source_register_1
            + self.context.decoder.immediate as SignedXLen as XLen;
    }

    fn instruction_slli(&mut self) {
        trace!("Executing SLLI...");

        self.context.execution.result = self.context.decoder.source_register_1
            << self.context.decoder.shamt;
    }

    fn instruction_slti(&mut self) {
        trace!("Executing SLTI...");

        self.context.execution.result =
            if (self.context.decoder.source_register_1 as SignedXLen)
                < (self.context.decoder.immediate as SignedXLen)
            {
                1
            } else {
                0
            };
    }

    fn instruction_sltiu(&mut self) {
        trace!("Executing SLTIU...");

        self.context.execution.result =
            if (self.context.decoder.source_register_1 as UnsignedXLen)
                < (self.context.decoder.immediate as UnsignedXLen)
            {
                1
            } else {
                0
            };
    }

    fn instruction_xori(&mut self) {
        trace!("Executing XORI...");

        self.context.execution.result = self.context.decoder.source_register_1
            ^ self.context.decoder.immediate as SignedXLen as XLen;
    }

    fn instruction_srli(&mut self) {
        trace!("Executing SRLI...");

        self.context.execution.result = self.context.decoder.source_register_1
            >> self.context.decoder.shamt;
    }

    fn instruction_srai(&mut self) {
        trace!("Executing SRAI...");

        self.context.execution.result =
            ((self.context.decoder.source_register_1 as SignedXLen)
                >> self.context.decoder.shamt) as XLen;
    }

    fn instruction_ori(&mut self) {
        trace!("Executing ORI...");

        self.context.execution.result = self.context.decoder.source_register_1
            | self.context.decoder.immediate as SignedXLen as XLen;
    }

    fn instruction_andi(&mut self) {
        trace!("Executing ANDI...");

        self.context.execution.result = self.context.decoder.source_register_1
            & self.context.decoder.immediate as SignedXLen as XLen;
    }

    fn instruction_auipc(&mut self) {
        trace!("Executing AUIPC...");

        self.context.execution.result = self.context.PC
            + self.context.decoder.immediate as SignedXLen as XLen;
    }

    fn instruction_store_byte(&mut self) {
        trace!("Executing SB...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedXLen as XLen);
        todo!("TODO: unimplemented");
    }

    fn instruction_store_half_word(&mut self) {
        trace!("Executing SH...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedXLen as XLen);
        todo!("TODO: unimplemented");
    }

    fn instruction_store_word(&mut self) {
        trace!("Executing SW...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedXLen as XLen);
        todo!("TODO: unimplemented");
    }

    fn instruction_add(&mut self) {
        trace!("Executing ADD...");

        self.context.execution.result = self.context.decoder.source_register_1
            + self.context.decoder.source_register_2;
    }

    fn instruction_sub(&mut self) {
        trace!("Executing SUB...");

        self.context.execution.result = self.context.decoder.source_register_1
            - self.context.decoder.source_register_2;
    }

    fn instruction_sll(&mut self) {
        trace!("Executing SLL...");

        self.context.execution.result = self.context.decoder.source_register_1
            << self.context.decoder.source_register_2;
    }

    fn instruction_slt(&mut self) {
        trace!("Executing SLT...");

        self.context.execution.result =
            if (self.context.decoder.source_register_1 as SignedXLen)
                < (self.context.decoder.source_register_2 as SignedXLen)
            {
                1
            } else {
                0
            }
    }

    fn instruction_sltu(&mut self) {
        trace!("Executing SLTU...");

        self.context.execution.result =
            if self.context.decoder.source_register_1
                < self.context.decoder.source_register_2
            {
                1
            } else {
                0
            }
    }

    fn instruction_xor(&mut self) {
        trace!("Executing XOR...");

        self.context.execution.result = self.context.decoder.source_register_1
            ^ self.context.decoder.source_register_2;
    }

    fn instruction_srl(&mut self) {
        trace!("Executing SRL...");

        self.context.execution.result = self.context.decoder.source_register_1
            >> self.context.decoder.source_register_2;
    }

    fn instruction_sra(&mut self) {
        trace!("Executing SRA...");

        self.context.execution.result =
            ((self.context.decoder.source_register_1 as SignedXLen)
                >> self.context.decoder.source_register_2) as XLen;
    }

    fn instruction_or(&mut self) {
        trace!("Executing OR...");

        self.context.execution.result = self.context.decoder.source_register_1
            | self.context.decoder.source_register_2;
    }

    fn instruction_and(&mut self) {
        trace!("Executing AND...");

        self.context.execution.result = self.context.decoder.source_register_1
            & self.context.decoder.source_register_2;
    }

    fn instruction_lui(&mut self) {
        trace!("Executing LUI...");

        self.context.execution.result =
            self.context.decoder.immediate as SignedXLen as XLen;
    }

    fn instruction_beq(&mut self) {
        trace!("Executing BEQ...");

        if self.context.decoder.source_register_1
            == self.context.decoder.source_register_2
        {
            self.context.execution.branch_target_address = self.context.PC
                + (self.context.decoder.immediate as SignedXLen as XLen);
            self.context.execution.need_to_branch = true;
        }
        panic!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_bne(&mut self) {
        trace!("Executing BNE...");

        if self.context.decoder.source_register_1
            != self.context.decoder.source_register_2
        {
            self.context.execution.branch_target_address = self.context.PC
                + (self.context.decoder.immediate as SignedXLen as XLen);
            self.context.execution.need_to_branch = true;
        }
        panic!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_blt(&mut self) {
        trace!("Executing BLT...");

        if (self.context.decoder.source_register_1 as SignedXLen)
            < (self.context.decoder.source_register_2 as SignedXLen)
        {
            self.context.execution.branch_target_address = self.context.PC
                + (self.context.decoder.immediate as SignedXLen as XLen);
            self.context.execution.need_to_branch = true;
        }
        panic!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_bge(&mut self) {
        trace!("Executing BGE...");

        if (self.context.decoder.source_register_1 as SignedXLen)
            >= (self.context.decoder.source_register_2 as SignedXLen)
        {
            self.context.execution.branch_target_address = self.context.PC
                + (self.context.decoder.immediate as SignedXLen as XLen);
            self.context.execution.need_to_branch = true;
        }
        panic!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_bltu(&mut self) {
        trace!("Executing BLTU...");

        if (self.context.decoder.source_register_1 as UnsignedXLen)
            < (self.context.decoder.source_register_2 as UnsignedXLen)
        {
            self.context.execution.branch_target_address = self.context.PC
                + (self.context.decoder.immediate as SignedXLen as XLen);
            self.context.execution.need_to_branch = true;
        }
        panic!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_bgeu(&mut self) {
        trace!("Executing BGEU...");

        if (self.context.decoder.source_register_1 as UnsignedXLen)
            >= (self.context.decoder.source_register_2 as UnsignedXLen)
        {
            self.context.execution.branch_target_address = self.context.PC
                + (self.context.decoder.immediate as SignedXLen as XLen);
            self.context.execution.need_to_branch = true;
        }
        panic!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_jalr(&mut self) {
        trace!("Executing JALR...");

        self.context.execution.jump_target_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedXLen as XLen);
        self.context.execution.result = self.context.PC + PC_STEP;
        panic!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_jal(&mut self) {
        trace!("Executing JAL...");

        self.context.execution.jump_target_address = self.context.PC
            + (self.context.decoder.immediate as SignedXLen as XLen);
        self.context.execution.result = self.context.PC + PC_STEP;
        panic!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_load_double_word(&mut self) {
        trace!("Executing LD...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedXLen as XLen)
    }

    fn instruction_load_word_unsigned(&mut self) {
        trace!("Executing LWU...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedXLen as XLen)
    }

    fn instruction_add_immediate_word(&mut self) {
        trace!("Executing ADDIW...");

        let result32 = (self.context.decoder.source_register_1 as XLen32)
            + self.context.decoder.immediate;
        self.context.execution.result = result32 as SignedXLen as XLen;
    }

    fn instruction_shift_left_logical_immediate_word(&mut self) {
        trace!("Executing SLLIW...");

        let result32 = (self.context.decoder.source_register_1 as XLen32)
            << self.context.decoder.immediate;
        self.context.execution.result = result32 as SignedXLen as XLen;
    }

    fn instruction_shift_right_logical_immediate_word(&mut self) {
        trace!("Executing SRLIW...");

        let result32 = (self.context.decoder.source_register_1 as XLen32)
            >> self.context.decoder.immediate;
        self.context.execution.result = result32 as SignedXLen as XLen;
    }

    fn instruction_shift_right_arith_immediate_word(&mut self) {
        trace!("Executing SRAIW...");

        let result32 = (self.context.decoder.source_register_1 as SignedXLen32)
            >> self.context.decoder.immediate;
        self.context.execution.result = result32 as SignedXLen as XLen;
    }

    fn instruction_store_double_word(&mut self) {
        trace!("Executing SD...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedXLen as XLen);
        todo!("TODO: unimplemented");
    }

    fn instruction_add_word(&mut self) {
        trace!("Executing ADDW...");

        let result32 = self.context.decoder.source_register_1 as SignedXLen32
            + self.context.decoder.source_register_2 as SignedXLen32;
        self.context.execution.result = result32 as SignedXLen as XLen;
    }

    fn instruction_sub_word(&mut self) {
        trace!("Executing SUBW...");

        let result32 = self.context.decoder.source_register_1 as SignedXLen32
            - self.context.decoder.source_register_2 as SignedXLen32;
        self.context.execution.result = result32 as SignedXLen as XLen;
    }

    fn instruction_shift_left_logical_word(&mut self) {
        trace!("Executing SLLW...");

        let result32 = (self.context.decoder.source_register_1 as XLen32)
            << self.context.decoder.shamt;
        self.context.execution.result = result32 as SignedXLen as XLen;
    }

    fn instruction_shift_right_logical_word(&mut self) {
        trace!("Executing SRLW...");

        let result32 = (self.context.decoder.source_register_1 as XLen32)
            >> self.context.decoder.shamt;
        self.context.execution.result = result32 as SignedXLen as XLen;
    }

    fn instruction_shift_right_arith_word(&mut self) {
        trace!("Executing SRAW...");

        let result32 = (self.context.decoder.source_register_1 as SignedXLen32)
            >> self.context.decoder.shamt;
        self.context.execution.result = result32 as SignedXLen as XLen;
    }

    fn memory_operation(&mut self, bus: &mut SystemBus<AddrType>) {
        if self.context.decoder.need_to_read_memory {
            trace!("Executing memory read...");

            self.context.execution.result =
                bus.read(self.context.execution.memory_address);
        } else if self.context.decoder.need_to_write_memory {
            trace!("Executing memory write...");

            // bus.write(address, value);
            todo!("TODO: unimplemented");
        }
    }

    fn write_back(&mut self) {
        if self.context.decoder.need_to_write_back {
            trace!("Writing back to register...");

            self.context.register_file.write(
                self.context.decoder.destination_register_index,
                self.context.execution.result,
            );
            // jumps always incur register write-back
            if self.context.decoder.need_to_jump {
                trace!("Updating PC to JTA...");

                error!(
                    "TODO: unimplemented: generate exception if BRANCH/JUMP (BTA/JTA) address is misaligned"
                );
                self.context.PC = self.context.execution.jump_target_address;
            }
        } else if self.context.execution.need_to_branch {
            trace!("Updating PC to BTA...");

            error!(
                "TODO: unimplemented: generate exception if BRANCH/JUMP (BTA/JTA) address is misaligned"
            );
            self.context.PC = self.context.execution.branch_target_address;
        }
    }

    fn extract_opcode(&self) -> Opcode {
        Opcode::from(
            ((self.context.current_instruction >> BIT_OFFSET_OPCODE)
                & MASK_OPCODE) as u8,
        )
    }

    fn extract_source_register_1(&self) -> XLen {
        self.context.register_file.read(
            ((self.context.current_instruction >> BIT_OFFSET_SOURCE_REGISTER_1)
                & MASK_SOURCE_REGISTER_1) as usize,
        )
    }

    fn extract_source_register_2(&self) -> XLen {
        self.context.register_file.read(
            ((self.context.current_instruction >> BIT_OFFSET_SOURCE_REGISTER_2)
                & MASK_SOURCE_REGISTER_2) as usize,
        )
    }

    fn extract_destination_register_index(&self) -> usize {
        ((self.context.current_instruction >> BIT_OFFSET_DESTINATION_REGISTER)
            & MASK_DESTINATION_REGISTER) as usize
    }

    fn extract_funct3(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_FUNCT3) & MASK_FUNCT3
    }

    fn extract_i_funct7(&self) -> OP_IMM_Funct7 {
        From::from(
            (self.context.current_instruction >> BIT_OFFSET_I_FUNCT7)
                & MASK_I_FUNCT7,
        )
    }

    fn extract_r_funct7(&self) -> Funct7 {
        From::from(
            (self.context.current_instruction >> BIT_OFFSET_R_FUNCT7)
                & MASK_R_FUNCT7,
        )
    }

    fn extract_shamt(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_SHAMT) & MASK_SHAMT
    }

    fn extract_imm_11_0(&self) -> SignedXLen {
        // right shift for signed values compiles to arithmetic shift right
        // (ASR) with sign extension
        (self.context.current_instruction as SignedInstructionType
            >> (BIT_OFFSET_IMM_11_0 - BIT_OFFSET_IMM_11_0_FINAL))
            as SignedXLen
    }

    fn extract_imm_11_5(&self) -> SignedXLen {
        // right shift for signed values compiles to arithmetic shift right
        // (ASR) with sign extension
        (self.context.current_instruction as SignedInstructionType
            >> (BIT_OFFSET_IMM_11_5 - BIT_OFFSET_IMM_11_5_FINAL))
            as SignedXLen
            & !((1 << BIT_OFFSET_IMM_11_5_FINAL) - 1)
    }

    fn extract_imm_4_0(&self) -> UnsignedImmediate {
        (self.context.current_instruction
            >> (BIT_OFFSET_IMM_4_0 - BIT_OFFSET_IMM_4_0_FINAL))
            & MASK_IMM_4_0
    }

    fn extract_imm_31_12(&self) -> SignedXLen {
        // right shift for signed values compiles to arithmetic shift right
        // (ASR) with sign extension
        (self.context.current_instruction as SignedInstructionType
            >> (BIT_OFFSET_IMM_31_12 - BIT_OFFSET_IMM_31_12_FINAL))
            as SignedXLen
            & !((1 << BIT_OFFSET_IMM_31_12_FINAL) - 1)
    }

    fn extract_imm_10_5(&self) -> UnsignedImmediate {
        (self.context.current_instruction
            >> (BIT_OFFSET_IMM_10_5 - BIT_OFFSET_IMM_10_5_FINAL))
            & MASK_IMM_10_5
    }

    fn extract_imm_4_1(&self) -> UnsignedImmediate {
        (self.context.current_instruction
            >> (BIT_OFFSET_IMM_4_1 - BIT_OFFSET_IMM_4_1_FINAL))
            & MASK_IMM_4_1
    }

    fn extract_imm_10_1(&self) -> UnsignedImmediate {
        (self.context.current_instruction
            >> (BIT_OFFSET_IMM_10_1 - BIT_OFFSET_IMM_10_1_FINAL))
            & MASK_IMM_10_1
    }

    fn extract_imm_19_12(&self) -> UnsignedImmediate {
        (self.context.current_instruction
            >> (BIT_OFFSET_IMM_19_12 - BIT_OFFSET_IMM_19_12_FINAL))
            & MASK_IMM_19_12
    }

    fn extract_b_11(&self) -> UnsignedImmediate {
        (self.context.current_instruction
            << (BIT_OFFSET_B_11_FINAL - BIT_OFFSET_B_11))
            & MASK_B_11
    }

    fn extract_b_12(&self) -> SignedXLen {
        // right shift for signed values compiles to arithmetic shift right
        // (ASR) with sign extension
        (self.context.current_instruction as SignedInstructionType
            >> (BIT_OFFSET_B_12 - BIT_OFFSET_B_12_FINAL)) as SignedXLen
            & !((1 << BIT_OFFSET_B_12_FINAL) - 1)
    }

    fn extract_j_11(&self) -> UnsignedImmediate {
        (self.context.current_instruction
            >> (BIT_OFFSET_J_11 - BIT_OFFSET_J_11_FINAL))
            & MASK_J_11
    }

    fn extract_j_20(&self) -> SignedXLen {
        // right shift for signed values compiles to arithmetic shift right
        // (ASR) with sign extension
        ((self.context.current_instruction as SignedInstructionType)
            >> (BIT_OFFSET_J_20 - BIT_OFFSET_J_20_FINAL)) as SignedXLen
            & !((1 << BIT_OFFSET_J_20_FINAL) - 1)
    }

    // fn extract_i_imm_inst_31(&self) -> UnsignedImmediate {
    //     (self.context.current_instruction >> BIT_OFFSET_I_IMM_INST_31)
    //         & MASK_I_IMM_INST_31
    // }

    // fn extract_i_imm_inst_30_25(&self) -> UnsignedImmediate {
    //     (self.context.current_instruction >> BIT_OFFSET_I_IMM_INST_30_25)
    //         & MASK_I_IMM_INST_30_25
    // }

    // fn extract_i_imm_inst_24_21(&self) -> UnsignedImmediate {
    //     (self.context.current_instruction >> BIT_OFFSET_I_IMM_INST_24_21)
    //         & MASK_I_IMM_INST_24_21
    // }

    // fn extract_i_imm_20(&self) -> UnsignedImmediate {
    //     (self.context.current_instruction >> BIT_OFFSET_I_IMM_20)
    //         & MASK_I_IMM_20
    // }

    // fn extract_s_imm_inst_31(&self) -> UnsignedImmediate {
    //     (self.context.current_instruction >> BIT_OFFSET_S_IMM_INST_31)
    //         & MASK_S_IMM_INST_31
    // }

    // fn extract_s_imm_inst_30_25(&self) -> UnsignedImmediate {
    //     (self.context.current_instruction >> BIT_OFFSET_S_IMM_INST_30_25)
    //         & MASK_S_IMM_INST_30_25
    // }

    // fn extract_s_imm_inst_11_8(&self) -> UnsignedImmediate {
    //     (self.context.current_instruction >> BIT_OFFSET_S_IMM_INST_11_8)
    //         & MASK_S_IMM_INST_11_8
    // }

    // fn extract_s_imm_7(&self) -> UnsignedImmediate {
    //     (self.context.current_instruction >> BIT_OFFSET_S_IMM_7) & MASK_S_IMM_7
    // }

    // fn extract_b_imm_inst_31(&self) -> UnsignedImmediate {
    //     (self.context.current_instruction >> BIT_OFFSET_B_IMM_INST_31)
    //         & MASK_B_IMM_INST_31
    // }

    // fn extract_b_imm_7(&self) -> UnsignedImmediate {
    //     (self.context.current_instruction >> BIT_OFFSET_B_IMM_7) & MASK_B_IMM_7
    // }

    // fn extract_b_imm_inst_30_25(&self) -> UnsignedImmediate {
    //     (self.context.current_instruction >> BIT_OFFSET_B_IMM_INST_30_25)
    //         & MASK_B_IMM_INST_30_25
    // }

    // fn extract_b_imm_inst_11_8(&self) -> UnsignedImmediate {
    //     (self.context.current_instruction >> BIT_OFFSET_B_IMM_INST_11_8)
    //         & MASK_B_IMM_INST_11_8
    // }

    // fn extract_b_imm_0(&self) -> UnsignedImmediate {
    //     (self.context.current_instruction >> BIT_OFFSET_B_IMM_0) & MASK_B_IMM_0
    // }

    // fn extract_u_imm_31(&self) -> UnsignedImmediate {
    //     (self.context.current_instruction >> BIT_OFFSET_U_IMM_31)
    //         & MASK_U_IMM_31
    // }

    // fn extract_u_imm_inst_30_20(&self) -> UnsignedImmediate {
    //     (self.context.current_instruction >> BIT_OFFSET_U_IMM_INST_30_20)
    //         & MASK_U_IMM_INST_30_20
    // }

    // fn extract_u_imm_inst_19_12(&self) -> UnsignedImmediate {
    //     (self.context.current_instruction >> BIT_OFFSET_U_IMM_INST_19_12)
    //         & MASK_U_IMM_INST_19_12
    // }

    // fn extract_u_imm_0(&self) -> UnsignedImmediate {
    //     (self.context.current_instruction >> BIT_OFFSET_U_IMM_0) & MASK_U_IMM_0
    // }

    // fn extract_j_imm_inst_31(&self) -> UnsignedImmediate {
    //     (self.context.current_instruction >> BIT_OFFSET_J_IMM_INST_31)
    //         & MASK_J_IMM_INST_31
    // }

    // fn extract_j_imm_inst_19_12(&self) -> UnsignedImmediate {
    //     (self.context.current_instruction
    //         >> (BIT_OFFSET_J_IMM_INST_19_12
    //             - BIT_OFFSET_J_IMM_INST_19_12_FINAL))
    //         & MASK_J_IMM_INST_19_12
    // }

    // fn extract_j_imm_20(&self) -> UnsignedImmediate {
    //     (self.context.current_instruction >> BIT_OFFSET_J_IMM_20)
    //         & MASK_J_IMM_20
    // }

    // fn extract_j_imm_inst_30_25(&self) -> UnsignedImmediate {
    //     (self.context.current_instruction >> BIT_OFFSET_J_IMM_INST_30_25)
    //         & MASK_J_IMM_INST_30_25
    // }

    // fn extract_j_imm_inst_24_21(&self) -> UnsignedImmediate {
    //     (self.context.current_instruction >> BIT_OFFSET_J_IMM_INST_24_21)
    //         & MASK_J_IMM_INST_24_21
    // }

    // fn extract_j_imm_0(&self) -> UnsignedImmediate {
    //     (self.context.current_instruction >> BIT_OFFSET_J_IMM_0) & MASK_J_IMM_0
    // }
}

fn init_logger() {
    let default_log_level = "info";
    // set default log level (unless specified with RUST_LOG environment variable)
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or(default_log_level),
    )
    .init();
}

fn print_license_text() {
    static LICENSE_TEXT: &str = "

    Copyright 2026 Ruslan Vostretsov

    Licensed under either of:

    * MIT License (LICENSE-MIT)
        https://opensource.org/licenses/MIT

    * Apache License, Version 2.0 (LICENSE-APACHE)
        https://www.apache.org/licenses/LICENSE-2.0

    at your option.

";
    print!("{LICENSE_TEXT}");
}

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(long, required = false, action = clap::ArgAction::SetTrue)]
    license: bool,
}

// #[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
// struct Args {
//     // /// Name of the person to greet
//     // #[arg(short, long)]
//     // name: String,
//     /// Number of times to greet
//     #[arg(short, long, default_value_t = 1)]
//     count: u8,
// }

fn parse_args() {
    // TODO: add `--license` argument to print license
    // let args: Vec<String> = env::args().collect();
    let args = Args::parse();

    if args.license {
        print_license_text();
        std::process::exit(0);
    }
}

#[derive(Deserialize)]
struct MemoryCfg {
    size_bytes: Option<usize>,
}

#[derive(Deserialize)]
struct SocConfig {
    soc: Option<MemoryCfg>,
}

fn configure() {
    const DEFAULT_CONFIG_PATH: &str = "conf/soc.toml";
    let config_path = DEFAULT_CONFIG_PATH;
    let binding = fs::read_to_string(config_path).unwrap();
    let file_content = binding.as_str();
    let config: SocConfig = toml::from_str(file_content).unwrap();
}

fn init() {
    init_logger();
    parse_args();
    configure();
}

fn main() {
    init();

    let mut soc: SoC = SoC {
        cpu: CPU {
            context: Context {
                PC: PC_RESET_VALUE,
                register_file: RegisterFile {
                    registers: [RISCV_X0_ZERO_REGISTER_VALUE;
                        RISCV_NUM_OF_REGISTERS],
                },
                current_instruction: INITIAL_INSTRUCTION,
                decoder: Decoder {
                    ..Default::default()
                },
                execution: ExecutionContext {
                    result: XLen::MIN,
                    branch_target_address: PC_RESET_VALUE,
                    jump_target_address: PC_RESET_VALUE,
                    memory_address: XLen::MIN,
                    need_to_branch: false,
                },
            },

            _phantom_marker: Default::default(),
        },
        bus: SystemBus {
            memory: Memory {
                byte_array: [MEM_DEFAULT_VALUE; DRAM_SIZE].to_vec(),
                _phantom_marker: Default::default(),
            },
        },
    };

    soc.reset();

    // TODO: HALT condition
    loop {
        soc.step();
    }
}
