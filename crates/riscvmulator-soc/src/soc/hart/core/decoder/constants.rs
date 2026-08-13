use std::fmt::{self, Binary};

use log::error;

use crate::{
    constants::arch::*,
    soc::hart::extensions::compressed::constants::InstructionTypeCompressed,
};

pub const OPCODE_QUADRANT_MASK: InstructionType = 0b11;

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Default, Clone)]
pub enum Opcode {
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
    SYSTEM = OPCODE_SYSTEM as isize, // (Zicsr_extension)

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
            OPCODE_SYSTEM => Self::SYSTEM, // (Zicsr_extension)

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
pub enum OPCODE_LOAD_Funct3 {
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

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
pub enum OP_IMM_Funct3 {
    ADDI = 0b000,  // add immediate
    SLLI = 0b001,  // shift left logical immediate
    SLTI = 0b010,  // set if register less than immediate
    SLTIU = 0b011, // set if register less than immediate (unsigned)
    XORI = 0b100,  // XOR with immediate
    SRXI = 0b101,  // shift right X immediate
    ORI = 0b110,   // OR with immediate
    ANDI = 0b111,  // AND with immediate
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

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
pub enum MISC_MEM_Funct3 {
    FENCE = 0b000,
    FENCE_I = 0b001,
}

impl From<InstructionType> for MISC_MEM_Funct3 {
    fn from(value: InstructionType) -> Self {
        match value {
            0b000 => MISC_MEM_Funct3::FENCE,
            0b001 => MISC_MEM_Funct3::FENCE_I,

            _ => {
                error!("Unknown funct3: {value:#05b}");
                unreachable!("Unknown funct3: {value:#05b}");
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Default, Clone)]
pub enum OP_IMM_Funct7 {
    #[default]
    SXLI = 0b000_0000,
    SRAI = MASK_I_FUNCT_7_SHIFT_RAI as isize,
}

impl From<InstructionType> for OP_IMM_Funct7 {
    fn from(value: InstructionType) -> Self {
        match value {
            0b000_0000 => OP_IMM_Funct7::SXLI,
            MASK_I_FUNCT_7_SHIFT_RAI => OP_IMM_Funct7::SRAI,

            _ => {
                error!("Unknown funct7: {value:#09b}");
                unreachable!("Unknown funct7: {value:#09b}");
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Default, Clone)]
pub enum OP_IMM_ShiftType {
    #[default]
    SXLI = 0b000_0000,
    SRAI = MASK_SHIFT_RIGHT_TYPE as isize,
}

impl From<InstructionType> for OP_IMM_ShiftType {
    fn from(value: InstructionType) -> Self {
        match value {
            0b000_0000 => OP_IMM_ShiftType::SXLI,
            MASK_SHIFT_RIGHT_TYPE => OP_IMM_ShiftType::SRAI,

            _ => {
                error!("Unknown funct7: {value:#09b}");
                unreachable!("Unknown funct7: {value:#09b}");
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Default, Clone)]
pub enum Funct7 {
    #[default]
    F0 = 0b0000000,
    F1 = 0b0100000,
    F_MULDIV = 0b0000001,
}

impl From<InstructionType> for Funct7 {
    fn from(value: InstructionType) -> Self {
        match value {
            0b0000000 => Funct7::F0,
            0b0100000 => Funct7::F1,
            0b0000001 => Funct7::F_MULDIV,

            _ => {
                error!("Unknown funct7: {value:#09b}");
                unreachable!("Unknown funct7: {value:#09b}");
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Default, Clone, PartialEq)]
pub enum I_Funct12 {
    #[default]
    F0 = 0b0000_0000_0000,
}

impl From<InstructionType> for I_Funct12 {
    fn from(value: InstructionType) -> Self {
        match value {
            0b0000_0000_0000 => I_Funct12::F0,

            _ => {
                error!("Unknown funct12: {value:#014b}");
                unreachable!("Unknown funct12: {value:#014b}");
            }
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
pub enum JALR_Funct3 {
    JALR = 0b000, //
}

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
pub enum SYSTEM_Funct3 {
    TRANSFER_CONTROL_RETURN_FROM = 0b000,
    CSRRW = 0b001,
    CSRRS = 0b010,
    CSRRC = 0b011,
    CSRRWI = 0b101,
    CSRRSI = 0b110,
    CSRRCI = 0b111,
}

impl From<InstructionType> for SYSTEM_Funct3 {
    fn from(value: InstructionType) -> Self {
        match value {
            0b000 => SYSTEM_Funct3::TRANSFER_CONTROL_RETURN_FROM,
            0b001 => SYSTEM_Funct3::CSRRW,
            0b010 => SYSTEM_Funct3::CSRRS,
            0b011 => SYSTEM_Funct3::CSRRC,
            0b101 => SYSTEM_Funct3::CSRRWI,
            0b110 => SYSTEM_Funct3::CSRRSI,
            0b111 => SYSTEM_Funct3::CSRRCI,

            _ => {
                error!("Unknown funct3: {value:#05b}");
                unreachable!("Unknown funct3: {value:#05b}");
            }
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
pub enum SYSTEM_Imm_transfer_control_return_from {
    ECALL = 0,
    EBREAK = 1,
    URET = 2,
    SRET = 258,
    MRET = 770,
}

impl From<InstructionType> for SYSTEM_Imm_transfer_control_return_from {
    fn from(value: InstructionType) -> Self {
        match value {
            0x000 => SYSTEM_Imm_transfer_control_return_from::ECALL,
            0x001 => SYSTEM_Imm_transfer_control_return_from::EBREAK,
            0x002 => SYSTEM_Imm_transfer_control_return_from::URET,
            0x102 => SYSTEM_Imm_transfer_control_return_from::SRET,
            0x302 => SYSTEM_Imm_transfer_control_return_from::MRET,

            _ => {
                error!("Unknown immediate: {value:#05X}");
                unreachable!("Unknown immediate: {value:#05X}");
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
pub enum STORE_Funct3 {
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
pub enum BRANCH_Funct3 {
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

#[derive(Default, Clone)]
pub enum FenceMode {
    #[default]
    NORMAL = 0b0000,
    TSO = 0b1000,
}

impl From<InstructionType> for FenceMode {
    fn from(value: InstructionType) -> Self {
        match value {
            0b0000 => FenceMode::NORMAL,
            0b1000 => FenceMode::TSO,

            _ => {
                error!("Unknown FENCE mode: {value:#06b}");
                unreachable!("Unknown FENCE mode: {value:#06b}");
            }
        }
    }
}

#[allow(non_camel_case_types)]
pub enum OpcodeToInstructionFormat {
    I_3_LOAD = 0b0000011,
    I_7_LOAD_FP = 0b0000111,
    I_15_MISC_MEM = 0b0001111,
    I_19_OP_IMM = 0b0010011,
    U_23_AUIPC = 0b0010111,
    I_27_OP_IMM_32 = 0b0011011,
    S_35_STORE = 0b0100011,
    S_39_STORE_FP = 0b0100111,
    R_51_OP = 0b0110011,
    U_55_LUI = 0b0110111,
    R_59_OP_32 = 0b0111011,
    R4_67_MADD = 0b1000011,
    R4_71_MSUB = 0b1000111,
    R4_75_NMSUB = 0b1001011,
    R4_79_NMADD = 0b1001111,
    R_83_OP_FP = 0b1010011,
    B_99_BRANCH = 0b1100011,
    I_103_JALR = 0b1100111,
    J_111_JAL = 0b1101111,

    // // compressed instruction formats
    // CR_ = 0b0,

    // Zicsr_extension
    I_115_SYSTEM = 0b1110011,
}

impl From<Opcode> for OpcodeToInstructionFormat {
    fn from(opcode: Opcode) -> Self {
        match opcode {
            Opcode::LOAD => OpcodeToInstructionFormat::I_3_LOAD,
            Opcode::STORE => OpcodeToInstructionFormat::S_35_STORE,
            Opcode::MADD => OpcodeToInstructionFormat::R4_67_MADD,
            Opcode::BRANCH => OpcodeToInstructionFormat::B_99_BRANCH,

            Opcode::LOAD_FP => OpcodeToInstructionFormat::I_7_LOAD_FP,
            Opcode::STORE_FP => OpcodeToInstructionFormat::S_39_STORE_FP,
            Opcode::MSUB => OpcodeToInstructionFormat::R4_71_MSUB,
            Opcode::JALR => OpcodeToInstructionFormat::I_103_JALR,

            Opcode::NMSUB => OpcodeToInstructionFormat::R4_75_NMSUB,

            Opcode::NMADD => OpcodeToInstructionFormat::R4_79_NMADD,
            Opcode::JAL => OpcodeToInstructionFormat::J_111_JAL,

            Opcode::OP_IMM => OpcodeToInstructionFormat::I_19_OP_IMM,
            Opcode::OP => OpcodeToInstructionFormat::R_51_OP,
            Opcode::OP_FP => OpcodeToInstructionFormat::R_83_OP_FP,

            Opcode::AUIPC => OpcodeToInstructionFormat::U_23_AUIPC,
            Opcode::LUI => OpcodeToInstructionFormat::U_55_LUI,

            Opcode::OP_IMM_32 => OpcodeToInstructionFormat::I_27_OP_IMM_32,
            Opcode::OP_32 => OpcodeToInstructionFormat::R_59_OP_32,

            Opcode::SYSTEM => OpcodeToInstructionFormat::I_115_SYSTEM, // (Zicsr_extension)

            Opcode::MISC_MEM => OpcodeToInstructionFormat::I_15_MISC_MEM,

            Opcode::AMO | Opcode::OP_V | Opcode::OP_VE => {
                error!("Opcode not implemented yet: {:#09b}", opcode);
                todo!("Opcode not implemented yet: {:#09b}", opcode);
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
                unimplemented!(
                    "Opcode reserved for custom commands: {:#09b}",
                    opcode
                );
            }
        }
    }
}

#[derive(Default, Clone)]
pub enum OpcodeQuadrant {
    // 16-bit instruction size
    Quadrant00 = 0b00,
    Quadrant01 = 0b01,
    Quadrant10 = 0b10,
    #[default]
    // >= 32-bit instruction size
    Quadrant11 = 0b11,
}

impl From<InstructionType> for OpcodeQuadrant {
    fn from(quadrant: InstructionType) -> Self {
        match quadrant {
            // >= 32-bit instruction size
            0b11 => OpcodeQuadrant::Quadrant11,
            // 16-bit instruction size
            0b10 => OpcodeQuadrant::Quadrant10,
            0b01 => OpcodeQuadrant::Quadrant01,
            0b00 => OpcodeQuadrant::Quadrant00,

            _ => {
                error!("Opcode quadrant is unreachable: {:#04b}", quadrant);
                unreachable!(
                    "Opcode quadrant is unreachable: {:#04b}",
                    quadrant
                );
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum InstructionSize {
    I16 = size_of::<InstructionTypeCompressed>() as isize,
    I32 = size_of::<InstructionType>() as isize,
    I48 = 6,
    I64 = 8,
}

impl From<usize> for InstructionSize {
    fn from(value: usize) -> Self {
        match value {
            2 => InstructionSize::I16,
            4 => InstructionSize::I32,
            6 => InstructionSize::I48,
            8 => InstructionSize::I64,

            _ => {
                error!(
                    "Instruction size is either unimplemented either doesn't exist : {value:?}"
                );
                unreachable!(
                    "Instruction size is either unimplemented either doesn't exist : {value:?}"
                );
            }
        }
    }
}
