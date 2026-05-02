// addr: {address:#018X?}

// std lib imports
use std::convert::From;
use std::fmt::{self, Binary, Debug};
use std::marker::PhantomData;
use std::mem::size_of;

// external libs imports
use log::{debug, error, info, trace, warn};

// current project imports
use config::configure;
use logger::init_logger;
use parse_args::parse_args;

// modules includes
mod config;
mod elf_loader;
mod logger;
mod parse_args;
mod soc;
mod utils;

/// architechtural defines
type XLen = u64;
type SignedXLen = i64;
type UnsignedXLen = u64;
type XLen32 = u32;
type SignedXLen32 = i32;

type Byte = u8;
type SignedByte = i8;
type HalfWord = u16;
type SignedHalfWord = i16;
type Word = u32;
type SignedWord = i32;
type DoubleWord = u64;
type SignedDoubleWord = i64;
// type QuadWord = u128;
// type SignedQuadWord = i128;

type SignedImmediate = i32;
type UnsignedImmediate = u32;

// const IALIGN: i32 = 32;
// const ILEN: i32 = IALIGN * 1;

type InstructionType = u32;
type SignedInstructionType = i32;

type BusAddrType = XLen;

const RISCV_NUM_OF_REGISTERS: usize = 32;
const RISCV_NUM_OF_CSR: usize = 1 << BITS_NUM_IMM_11_0;
const RISCV_X0_ZERO_REGISTER_VALUE: XLen = 0x00000000;
const RISCV_CSR_INITIAL_VALUE: XLen = 0x00000000;

// default constants
const PC_RESET_VALUE: XLen = RAM_BASE;
const PC_STEP: XLen = 0x00000004;

const INITIAL_INSTRUCTION: InstructionType = 0x00000000;

const MEM_DEFAULT_VALUE: Byte = 0x00;
const RAM_BASE: XLen = 0x80000000;
const RAM_SIZE: usize = 65536; // placeholder value, to be configured later
const RAM_END: XLen = RAM_BASE + RAM_SIZE as XLen;
// const MEM_SPACE_SIZE: usize = XLen::MAX as usize;

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
const BITS_NUM_IMM_11_0: usize = 12;
// const MASK_IMM_11_0: InstructionType =
//     ((1 << BITS_NUM_IMM_11_0) - 1) << BIT_OFFSET_IMM_11_0_FINAL;
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
const BIT_OFFSET_IMM_10_1_FINAL: usize = 1;
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

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
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

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
enum SYSTEM_Funct3 {
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
enum SYSTEM_Imm_transfer_control_return_from {
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
            0b101 => OP_32_Funct3::SRXW,

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

    ECALL,  // make service request to the execution environment
    EBREAK, // return control to debugging environment

    FENCE, // order I/O and memory accesses

    URET, // return from user exception
    SRET, // return from supervisor exception
    MRET, // return from machine exception

    CSRRW,  // CSR read/write
    CSRRS,  // CSR read/set
    CSRRC,  // CSR read/clear
    CSRRWI, // CSR read/write immediate
    CSRRSI, // CSR read/set immediate
    CSRRCI, // CSR read/clear immediate
}

trait RegisterInterface {
    fn read(&self) -> XLen;
    fn write(&mut self, value: XLen);
}

trait RegisterFileInterface {
    fn read(&self, offset: usize) -> XLen;
    fn write(&mut self, offset: usize, value: XLen);
}

#[derive(Copy, Clone)]
struct Register {
    value: XLen,
}

impl RegisterInterface for Register {
    fn read(&self) -> XLen {
        self.value
    }
    fn write(&mut self, value: XLen) {
        self.value = value;
    }
}

#[derive(Clone)]
struct RegisterFile<const REGS_NUM: usize> {
    registers: [Register; REGS_NUM],
}

impl<const REGS_NUM: usize> RegisterFileInterface for RegisterFile<REGS_NUM> {
    fn read(&self, offset: usize) -> XLen {
        trace!("Reading register x{offset:#?}...");

        self.registers[offset].read()
    }

    fn write(&mut self, offset: usize, value: XLen) {
        trace!("Writing register x{offset:#?}...");

        // register `x0_zero` is hardwired with all bits equal to 0, so do nothing with `x0_zero`
        if offset != (GPRegisterName::x0_zero as usize) {
            self.registers[offset].write(value);
        } else {
            trace!(
                "An attempt to write value {value} to register `x0_zero` was silently discarded"
            );
        }
    }
}

#[allow(non_camel_case_types, dead_code)]
enum GPRegisterName {
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

#[allow(non_camel_case_types, dead_code)]
enum CSRRegisterName {
    // user lever (unprivilleged) CSRs

    // unprivilleged floating-point CSRs
    fflags = 0x001, // floating-point accrued exceptions
    frm = 0x002,    // floating-point dynamic rounding mode
    fcsr = 0x003,   // floating-point control and status register (frm + fflags)

    // unprivilleged vector CSRs
    vstart = 0x008, // vector start position
    vxsat = 0x009,  // fixed point accrued saturation flag
    vxrm = 0x00A,   // fixed point rounding mode
    vcsr = 0x00F,   // vector control and status register
    vl = 0xC20,     // vector length
    vtype = 0xC21,  // vector data type register
    vlenb = 0xC22,  // vector register length in bytes

    // unprivilleged Zicfiss extension CSR
    ssp = 0x011, // shadow stack pointer

    // unprivilleged entropy source extension CSR
    seed = 0x015, // seed for cryptographic random bit generators

    // unprivilleged Zcmt extension CSR
    jvt = 0x017, // table jump base vector and control register (CR)

    // unprivilleged counter/timers
    cycle = 0xC00,        // cycle counter for RDCYCLE instruction
    time = 0xC01,         // timer for RDTIME instruction
    instret = 0xC02, // instructions-retired counter for RDINSTRET instruction
    hpmcounter3 = 0xC03, // performance-monitoring counter
    hpmcounter4 = 0xC04, // performance-monitoring counter
    hpmcounter5 = 0xC05, // performance-monitoring counter
    hpmcounter6 = 0xC06, // performance-monitoring counter
    hpmcounter7 = 0xC07, // performance-monitoring counter
    hpmcounter8 = 0xC08, // performance-monitoring counter
    hpmcounter9 = 0xC09, // performance-monitoring counter
    hpmcounter10 = 0xC0A, // performance-monitoring counter
    hpmcounter11 = 0xC0B, // performance-monitoring counter
    hpmcounter12 = 0xC0C, // performance-monitoring counter
    hpmcounter13 = 0xC0D, // performance-monitoring counter
    hpmcounter14 = 0xC0E, // performance-monitoring counter
    hpmcounter15 = 0xC0F, // performance-monitoring counter
    hpmcounter16 = 0xC10, // performance-monitoring counter
    hpmcounter17 = 0xC11, // performance-monitoring counter
    hpmcounter18 = 0xC12, // performance-monitoring counter
    hpmcounter19 = 0xC13, // performance-monitoring counter
    hpmcounter20 = 0xC14, // performance-monitoring counter
    hpmcounter21 = 0xC15, // performance-monitoring counter
    hpmcounter22 = 0xC16, // performance-monitoring counter
    hpmcounter23 = 0xC17, // performance-monitoring counter
    hpmcounter24 = 0xC18, // performance-monitoring counter
    hpmcounter25 = 0xC19, // performance-monitoring counter
    hpmcounter26 = 0xC1A, // performance-monitoring counter
    hpmcounter27 = 0xC1B, // performance-monitoring counter
    hpmcounter28 = 0xC1C, // performance-monitoring counter
    hpmcounter29 = 0xC1D, // performance-monitoring counter
    hpmcounter30 = 0xC1E, // performance-monitoring counter
    hpmcounter31 = 0xC1F, // performance-monitoring counter

    cycleh = 0xC80,        // upper 32 bits of `cycle`, RV32 only
    timeh = 0xC81,         // upper 32 bits of `time`, RV32 only
    instreth = 0xC82,      // upper 32 bits of `instret`, RV32 only
    hpmcounter3h = 0xC83,  // upper 32 bits of `hpmcounter3`, RV32 only
    hpmcounter4h = 0xC84,  // upper 32 bits of `hpmcounter4`, RV32 only
    hpmcounter5h = 0xC85,  // upper 32 bits of `hpmcounter5`, RV32 only
    hpmcounter6h = 0xC86,  // upper 32 bits of `hpmcounter6`, RV32 only
    hpmcounter7h = 0xC87,  // upper 32 bits of `hpmcounter7`, RV32 only
    hpmcounter8h = 0xC88,  // upper 32 bits of `hpmcounter8`, RV32 only
    hpmcounter9h = 0xC89,  // upper 32 bits of `hpmcounter9`, RV32 only
    hpmcounter10h = 0xC8A, // upper 32 bits of `hpmcounter10`, RV32 only
    hpmcounter11h = 0xC8B, // upper 32 bits of `hpmcounter11`, RV32 only
    hpmcounter12h = 0xC8C, // upper 32 bits of `hpmcounter12`, RV32 only
    hpmcounter13h = 0xC8D, // upper 32 bits of `hpmcounter13`, RV32 only
    hpmcounter14h = 0xC8E, // upper 32 bits of `hpmcounter14`, RV32 only
    hpmcounter15h = 0xC8F, // upper 32 bits of `hpmcounter15`, RV32 only
    hpmcounter16h = 0xC90, // upper 32 bits of `hpmcounter16`, RV32 only
    hpmcounter17h = 0xC91, // upper 32 bits of `hpmcounter17`, RV32 only
    hpmcounter18h = 0xC92, // upper 32 bits of `hpmcounter18`, RV32 only
    hpmcounter19h = 0xC93, // upper 32 bits of `hpmcounter19`, RV32 only
    hpmcounter20h = 0xC94, // upper 32 bits of `hpmcounter20`, RV32 only
    hpmcounter21h = 0xC95, // upper 32 bits of `hpmcounter21`, RV32 only
    hpmcounter22h = 0xC96, // upper 32 bits of `hpmcounter22`, RV32 only
    hpmcounter23h = 0xC97, // upper 32 bits of `hpmcounter23`, RV32 only
    hpmcounter24h = 0xC98, // upper 32 bits of `hpmcounter24`, RV32 only
    hpmcounter25h = 0xC99, // upper 32 bits of `hpmcounter25`, RV32 only
    hpmcounter26h = 0xC9A, // upper 32 bits of `hpmcounter26`, RV32 only
    hpmcounter27h = 0xC9B, // upper 32 bits of `hpmcounter27`, RV32 only
    hpmcounter28h = 0xC9C, // upper 32 bits of `hpmcounter28`, RV32 only
    hpmcounter29h = 0xC9D, // upper 32 bits of `hpmcounter29`, RV32 only
    hpmcounter30h = 0xC9E, // upper 32 bits of `hpmcounter30`, RV32 only
    hpmcounter31h = 0xC9F, // upper 32 bits of `hpmcounter31`, RV32 only

    // supervisor-level CSRs

    // supervisor trap setup
    sstatus = 0x100,    // supervisor status register
    sie = 0x104,        // supervisor interrupt-enable register
    stvec = 0x105,      // supervisor trap handler base address
    scounteren = 0x106, // supervisor conter enable

    // supervisor configuration
    senvcfg = 0x10A, // supervisor environment configuration register

    // supervisor counter setup
    scountinhibit = 0x120, // supervisor counter-inhibit register

    // supervisor trap handling
    sscratch = 0x140,  // supervisor scratch register
    sepc = 0x141,      // supervisor exception program counter
    scause = 0x142,    // supervisor trap cause
    stval = 0x143,     // supervisor trap value
    sip = 0x144,       // supervisor interrupt pending
    scountovf = 0xDA0, // supervisor count overflow

    // supervisor indirect
    siselect = 0x150, // supervisor indirect register select
    sireg = 0x151,    // supervisor indirect register alias
    sireg2 = 0x152,   // supervisor indirect register alias 2
    sireg3 = 0x153,   // supervisor indirect register alias 3
    sireg4 = 0x155,   // supervisor indirect register alias 4
    sireg5 = 0x156,   // supervisor indirect register alias 5
    sireg6 = 0x157,   // supervisor indirect register alias 6

    // supervisor protection and translation
    satp = 0x180, // supervisor address translation and protection

    // supervisor timer compare
    stimecpm = 0x14D,  // supervisor timer compare
    stimecpmh = 0x15D, // upper 32 bits of `stimecpm`, RV32 only

    // supervisor debug/trace registers
    scontext = 0x5A8, // supervisor-mode context register

    // supervisor resource management configuration
    srmcfg = 0x181, // supervisor resource management configuration

    // supervisor state enable registers
    sstateen0 = 0x10C, // supervisor state enable 0 register
    sstateen1 = 0x10D, // supervisor state enable 1 register
    sstateen2 = 0x10E, // supervisor state enable 2 register
    sstateen3 = 0x10F, // supervisor state enable 3 register

    // supervisor control transfer records configuration
    sctrctl = 0x14E, // supervisor control transfer records control register
    sctrstatus = 0x14F, // supervisor control transfer records status register
    sctrdepth = 0x15F, // supervisor control transfer records depth register

    // hypervisor-level and virtual supervisor (VS) CSRs

    // hypervisor trap setup
    hstatus = 0x600,    // hypervisor status register
    hedeleg = 0x602,    // hypervisor exception delegation register
    hideleg = 0x603,    // hypervisor interrupt delegation register
    hie = 0x604,        // hypervisor interrupt-enable register
    hcounteren = 0x606, // hypervisor counter enable
    hgeie = 0x607,      // hypervisor guest external interrupt-enable register
    hedelegh = 0x612,   // 32 upper bits of `hedeleg`, RV32 only

    // hypervisor trap handling
    htval = 0x643,  // hypervisor trap value
    hip = 0x644,    // hypervisor interrupt pending
    hvip = 0x645,   // hypervisor interrupt pending
    htinst = 0x64A, // hypervisor count overflow
    hgeip = 0xE12,  // hypervisor interrupt pending

    // hypervisor configuration
    henvcfg = 0x60A, // hypervisor environment configuration register
    henvcfgh = 0x61A, // 32 upper bits of `henvcfg`, RV32 only

    // hypervisor protection and translation
    hgatp = 0x680, // hypervisor guest address translation and protection

    // hypervisor debug/trace registers
    hcontext = 0x6A8, // hypervisor-mode context register

    // hypervisor counter/timer virtualization registers
    htimedelta = 0x605,  // delta for VS/VU-mode timer
    htimedeltah = 0x615, // 32 upper bits of `htimedelta`, RV32 only

    // hypervisor state enable registers
    hstateen0 = 0x60C,  // hypervisor state enable 0 register
    hstateen1 = 0x60D,  // hypervisor state enable 1 register
    hstateen2 = 0x60E,  // hypervisor state enable 2 register
    hstateen3 = 0x60F,  // hypervisor state enable 3 register
    hstateen0h = 0x61C, // 32 upper bits of `hstateen0`, RV32 only
    hstateen1h = 0x61D, // 32 upper bits of `hstateen1`, RV32 only
    hstateen2h = 0x61E, // 32 upper bits of `hstateen2`, RV32 only
    hstateen3h = 0x61F, // 32 upper bits of `hstateen3`, RV32 only

    // virtual supervisor (VS) registers
    // virtual supervisor (VS) registers
    vstatus = 0x200,   // virtual supervisor status register
    vsie = 0x204,      // virtual supervisor interrupt-enable register
    vstvec = 0x205,    // virtual supervisor trap handler base address
    vsscratch = 0x240, // virtual supervisor scratch register
    vsepc = 0x241,     // virtual supervisor exception program counter
    vscause = 0x242,   // virtual supervisor trap cause
    vstval = 0x243,    // virtual supervisor trap value
    vsip = 0x244,      // virtual supervisor interrupt pending
    vsatp = 0x280,     // virtual supervisor address translation and protection

    // virtual supervisor indirect
    vsiselect = 0x250, // virtual supervisor indirect register select
    vsireg = 0x251,    // virtual supervisor indirect register alias
    vsireg2 = 0x252,   // virtual supervisor indirect register alias 2
    vsireg3 = 0x253,   // virtual supervisor indirect register alias 3
    vsireg4 = 0x255,   // virtual supervisor indirect register alias 4
    vsireg5 = 0x256,   // virtual supervisor indirect register alias 5
    vsireg6 = 0x257,   // virtual supervisor indirect register alias 6

    // virtual supervisor timer compare
    vstimecpm = 0x24D,  // virtual supervisor timer compare
    vstimecpmh = 0x25D, // virtual upper 32 bits of `vstimecpm`, RV32 only

    // virtual supervisor control transfer records configuration
    vsctrctl = 0x24E, // virtual supervisor control transfer records control register

    // machine-level CSRs

    // machine information registers
    mvendorid = 0xF11,  // vendor ID
    marchid = 0xF12,    // architecture ID
    mimpid = 0xF13,     // implementation ID
    mhartid = 0xF14,    // hardware thread ID
    mconfigptr = 0xF15, // pointer to configuration data structure

    // machine trap setup
    mstatus = 0x300,    // machine status register
    misa = 0x301,       // ISA and extensions
    medeleg = 0x302,    // machine exception delegation register
    mideleg = 0x303,    // machine interrupt delegation register
    mie = 0x304,        // machine interrupt-enable register
    mtvec = 0x305,      // machine trap-handler base address
    mcounteren = 0x306, // machine counter enable
    mstatush = 0x310,   // 32 upper bits of `mstatus`, RV32 only
    medelegh = 0x312,   // 32 upper bits of `medeleg`, RV32 only

    // machine trap handling
    mscratch = 0x340, // machine scratch register
    mepc = 0x341,     // machine exception program counter
    mcause = 0x342,   // machine trap cause
    mtval = 0x343,    // machine trap value
    mip = 0x344,      // machine interrupt pending
    mtinst = 0x34A,   // machine
    mtval2 = 0x34B,   // machine second trap value

    // machine indirect
    miselect = 0x350, // machine indirect register select
    mireg = 0x351,    // machine indirect register alias
    mireg2 = 0x352,   // machine indirect register alias 2
    mireg3 = 0x353,   // machine indirect register alias 3
    mireg4 = 0x355,   // machine indirect register alias 4
    mireg5 = 0x356,   // machine indirect register alias 5
    mireg6 = 0x357,   // machine indirect register alias 6

    // machine configuration
    menvcfg = 0x30A, // machine environment configuration register
    menvcfgh = 0x31A, // 32 upper bits of `menvcfg`, RV32 only
    mseccfg = 0x747, // machine security configuration register
    mseccfgh = 0x757, // 32 upper bits of `mseccfg`, RV32 only

    // machine memory protection
    pmpcfg0 = 0x3A0,  // physical memory protection configuraiton
    pmpcfg1 = 0x3A1,  // physical memory protection configuraiton, RV32 only
    pmpcfg2 = 0x3A2,  // physical memory protection configuraiton
    pmpcfg3 = 0x3A3,  // physical memory protection configuraiton, RV32 only
    pmpcfg4 = 0x3A4,  // physical memory protection configuraiton
    pmpcfg5 = 0x3A5,  // physical memory protection configuraiton, RV32 only
    pmpcfg6 = 0x3A6,  // physical memory protection configuraiton
    pmpcfg7 = 0x3A7,  // physical memory protection configuraiton, RV32 only
    pmpcfg8 = 0x3A8,  // physical memory protection configuraiton
    pmpcfg9 = 0x3A9,  // physical memory protection configuraiton, RV32 only
    pmpcfg10 = 0x3AA, // physical memory protection configuraiton
    pmpcfg11 = 0x3AB, // physical memory protection configuraiton, RV32 only
    pmpcfg12 = 0x3AC, // physical memory protection configuraiton
    pmpcfg13 = 0x3AD, // physical memory protection configuraiton, RV32 only
    pmpcfg14 = 0x3AE, // physical memory protection configuraiton
    pmpcfg15 = 0x3AF, // physical memory protection configuraiton, RV32 only
    pmaddr0 = 0x3B0,  // physical memory protection address register
    pmaddr1 = 0x3B1,  // physical memory protection address register
    pmaddr2 = 0x3B2,  // physical memory protection address register
    pmaddr3 = 0x3B3,  // physical memory protection address register
    pmaddr4 = 0x3B4,  // physical memory protection address register
    pmaddr5 = 0x3B5,  // physical memory protection address register
    pmaddr6 = 0x3B6,  // physical memory protection address register
    pmaddr7 = 0x3B7,  // physical memory protection address register
    pmaddr8 = 0x3B8,  // physical memory protection address register
    pmaddr9 = 0x3B9,  // physical memory protection address register
    pmaddr10 = 0x3BA, // physical memory protection address register
    pmaddr11 = 0x3BB, // physical memory protection address register
    pmaddr12 = 0x3BC, // physical memory protection address register
    pmaddr13 = 0x3BD, // physical memory protection address register
    pmaddr14 = 0x3BE, // physical memory protection address register
    pmaddr15 = 0x3BF, // physical memory protection address register
    pmaddr16 = 0x3C0, // physical memory protection address register
    pmaddr17 = 0x3C1, // physical memory protection address register
    pmaddr18 = 0x3C2, // physical memory protection address register
    pmaddr19 = 0x3C3, // physical memory protection address register
    pmaddr20 = 0x3C4, // physical memory protection address register
    pmaddr21 = 0x3C5, // physical memory protection address register
    pmaddr22 = 0x3C6, // physical memory protection address register
    pmaddr23 = 0x3C7, // physical memory protection address register
    pmaddr24 = 0x3C8, // physical memory protection address register
    pmaddr25 = 0x3C9, // physical memory protection address register
    pmaddr26 = 0x3CA, // physical memory protection address register
    pmaddr27 = 0x3CB, // physical memory protection address register
    pmaddr28 = 0x3CC, // physical memory protection address register
    pmaddr29 = 0x3CD, // physical memory protection address register
    pmaddr30 = 0x3CE, // physical memory protection address register
    pmaddr31 = 0x3CF, // physical memory protection address register
    pmaddr32 = 0x3D0, // physical memory protection address register
    pmaddr33 = 0x3D1, // physical memory protection address register
    pmaddr34 = 0x3D2, // physical memory protection address register
    pmaddr35 = 0x3D3, // physical memory protection address register
    pmaddr36 = 0x3D4, // physical memory protection address register
    pmaddr37 = 0x3D5, // physical memory protection address register
    pmaddr38 = 0x3D6, // physical memory protection address register
    pmaddr39 = 0x3D7, // physical memory protection address register
    pmaddr40 = 0x3D8, // physical memory protection address register
    pmaddr41 = 0x3D9, // physical memory protection address register
    pmaddr42 = 0x3DA, // physical memory protection address register
    pmaddr43 = 0x3DB, // physical memory protection address register
    pmaddr44 = 0x3DC, // physical memory protection address register
    pmaddr45 = 0x3DD, // physical memory protection address register
    pmaddr46 = 0x3DE, // physical memory protection address register
    pmaddr47 = 0x3DF, // physical memory protection address register
    pmaddr48 = 0x3E0, // physical memory protection address register
    pmaddr49 = 0x3E1, // physical memory protection address register
    pmaddr50 = 0x3E2, // physical memory protection address register
    pmaddr51 = 0x3E3, // physical memory protection address register
    pmaddr52 = 0x3E4, // physical memory protection address register
    pmaddr53 = 0x3E5, // physical memory protection address register
    pmaddr54 = 0x3E6, // physical memory protection address register
    pmaddr55 = 0x3E7, // physical memory protection address register
    pmaddr56 = 0x3E8, // physical memory protection address register
    pmaddr57 = 0x3E9, // physical memory protection address register
    pmaddr58 = 0x3EA, // physical memory protection address register
    pmaddr59 = 0x3EB, // physical memory protection address register
    pmaddr60 = 0x3EC, // physical memory protection address register
    pmaddr61 = 0x3ED, // physical memory protection address register
    pmaddr62 = 0x3EE, // physical memory protection address register
    pmaddr63 = 0x3EF, // physical memory protection address register

    // machine state enable registers
    mstateen0 = 0x30C,  // machine state enable 0 register
    mstateen1 = 0x30D,  // machine state enable 1 register
    mstateen2 = 0x30E,  // machine state enable 2 register
    mstateen3 = 0x30F,  // machine state enable 3 register
    mstateen0h = 0x31C, // 32 upper bits of `mstateen0`, RV32 only
    mstateen1h = 0x31D, // 32 upper bits of `mstateen1`, RV32 only
    mstateen2h = 0x31E, // 32 upper bits of `mstateen2`, RV32 only
    mstateen3h = 0x31F, // 32 upper bits of `mstateen3`, RV32 only

    // machine non-maskable interrupt handling
    mnscratch = 0x740, // resumable NMI scratch register
    mnepc = 0x741,     // resumable NMI program counter
    mncause = 0x742,   // resumable NMI cause
    mnstatus = 0x744,  // resumable NMI status

    // machine counter/timers
    mcycle = 0xB00,        // machine cycle counter
    minstret = 0xB02,      // machine instructions-retired counter
    mhpmcounter3 = 0xB03,  // machine performance-monitoring counter
    mhpmcounter4 = 0xB04,  // machine performance-monitoring counter
    mhpmcounter5 = 0xB05,  // machine performance-monitoring counter
    mhpmcounter6 = 0xB06,  // machine performance-monitoring counter
    mhpmcounter7 = 0xB07,  // machine performance-monitoring counter
    mhpmcounter8 = 0xB08,  // machine performance-monitoring counter
    mhpmcounter9 = 0xB09,  // machine performance-monitoring counter
    mhpmcounter10 = 0xB0A, // machine performance-monitoring counter
    mhpmcounter11 = 0xB0B, // machine performance-monitoring counter
    mhpmcounter12 = 0xB0C, // machine performance-monitoring counter
    mhpmcounter13 = 0xB0D, // machine performance-monitoring counter
    mhpmcounter14 = 0xB0E, // machine performance-monitoring counter
    mhpmcounter15 = 0xB0F, // machine performance-monitoring counter
    mhpmcounter16 = 0xB10, // machine performance-monitoring counter
    mhpmcounter17 = 0xB11, // machine performance-monitoring counter
    mhpmcounter18 = 0xB12, // machine performance-monitoring counter
    mhpmcounter19 = 0xB13, // machine performance-monitoring counter
    mhpmcounter20 = 0xB14, // machine performance-monitoring counter
    mhpmcounter21 = 0xB15, // machine performance-monitoring counter
    mhpmcounter22 = 0xB16, // machine performance-monitoring counter
    mhpmcounter23 = 0xB17, // machine performance-monitoring counter
    mhpmcounter24 = 0xB18, // machine performance-monitoring counter
    mhpmcounter25 = 0xB19, // machine performance-monitoring counter
    mhpmcounter26 = 0xB1A, // machine performance-monitoring counter
    mhpmcounter27 = 0xB1B, // machine performance-monitoring counter
    mhpmcounter28 = 0xB1C, // machine performance-monitoring counter
    mhpmcounter29 = 0xB1D, // machine performance-monitoring counter
    mhpmcounter30 = 0xB1E, // machine performance-monitoring counter
    mhpmcounter31 = 0xB1F, // machine performance-monitoring counter

    mcycleh = 0xB80,       // upper 32 bits of `mcycle`, RV32 only
    minstreth = 0xB82,     // upper 32 bits of `minstret`, RV32 only
    mhpmcounter3h = 0xB83, // upper 32 bits of `mhpmcounter3`, RV32 only
    mhpmcounter4h = 0xB84, // upper 32 bits of `mhpmcounter4`, RV32 only
    mhpmcounter5h = 0xB85, // upper 32 bits of `mhpmcounter5`, RV32 only
    mhpmcounter6h = 0xB86, // upper 32 bits of `mhpmcounter6`, RV32 only
    mhpmcounter7h = 0xB87, // upper 32 bits of `mhpmcounter7`, RV32 only
    mhpmcounter8h = 0xB88, // upper 32 bits of `mhpmcounter8`, RV32 only
    mhpmcounter9h = 0xB89, // upper 32 bits of `mhpmcounter9`, RV32 only
    mhpmcounter10h = 0xB8A, // upper 32 bits of `mhpmcounter10`, RV32 only
    mhpmcounter11h = 0xB8B, // upper 32 bits of `mhpmcounter11`, RV32 only
    mhpmcounter12h = 0xB8C, // upper 32 bits of `mhpmcounter12`, RV32 only
    mhpmcounter13h = 0xB8D, // upper 32 bits of `mhpmcounter13`, RV32 only
    mhpmcounter14h = 0xB8E, // upper 32 bits of `mhpmcounter14`, RV32 only
    mhpmcounter15h = 0xB8F, // upper 32 bits of `mhpmcounter15`, RV32 only
    mhpmcounter16h = 0xB90, // upper 32 bits of `mhpmcounter16`, RV32 only
    mhpmcounter17h = 0xB91, // upper 32 bits of `mhpmcounter17`, RV32 only
    mhpmcounter18h = 0xB92, // upper 32 bits of `mhpmcounter18`, RV32 only
    mhpmcounter19h = 0xB93, // upper 32 bits of `mhpmcounter19`, RV32 only
    mhpmcounter20h = 0xB94, // upper 32 bits of `mhpmcounter20`, RV32 only
    mhpmcounter21h = 0xB95, // upper 32 bits of `mhpmcounter21`, RV32 only
    mhpmcounter22h = 0xB96, // upper 32 bits of `mhpmcounter22`, RV32 only
    mhpmcounter23h = 0xB97, // upper 32 bits of `mhpmcounter23`, RV32 only
    mhpmcounter24h = 0xB98, // upper 32 bits of `mhpmcounter24`, RV32 only
    mhpmcounter25h = 0xB99, // upper 32 bits of `mhpmcounter25`, RV32 only
    mhpmcounter26h = 0xB9A, // upper 32 bits of `mhpmcounter26`, RV32 only
    mhpmcounter27h = 0xB9B, // upper 32 bits of `mhpmcounter27`, RV32 only
    mhpmcounter28h = 0xB9C, // upper 32 bits of `mhpmcounter28`, RV32 only
    mhpmcounter29h = 0xB9D, // upper 32 bits of `mhpmcounter29`, RV32 only
    mhpmcounter30h = 0xB9E, // upper 32 bits of `mhpmcounter30`, RV32 only
    mhpmcounter31h = 0xB9F, // upper 32 bits of `mhpmcounter31`, RV32 only

    // machine counter setup
    mcountinhibit = 0x320, // machine counter-inhibit register
    mcyclecfg = 0x321,     // machine cycle-counter configuration register
    minstretcfg = 0x322,   // machine insert counter configuration register
    mhpmevent3 = 0x323,    // machine performance-monitoring event selector
    mhpmevent4 = 0x324,    // machine performance-monitoring event selector
    mhpmevent5 = 0x325,    // machine performance-monitoring event selector
    mhpmevent6 = 0x326,    // machine performance-monitoring event selector
    mhpmevent7 = 0x327,    // machine performance-monitoring event selector
    mhpmevent8 = 0x328,    // machine performance-monitoring event selector
    mhpmevent9 = 0x329,    // machine performance-monitoring event selector
    mhpmevent10 = 0x32A,   // machine performance-monitoring event selector
    mhpmevent11 = 0x32B,   // machine performance-monitoring event selector
    mhpmevent12 = 0x32C,   // machine performance-monitoring event selector
    mhpmevent13 = 0x32D,   // machine performance-monitoring event selector
    mhpmevent14 = 0x32E,   // machine performance-monitoring event selector
    mhpmevent15 = 0x32F,   // machine performance-monitoring event selector
    mhpmevent16 = 0x330,   // machine performance-monitoring event selector
    mhpmevent17 = 0x331,   // machine performance-monitoring event selector
    mhpmevent18 = 0x332,   // machine performance-monitoring event selector
    mhpmevent19 = 0x333,   // machine performance-monitoring event selector
    mhpmevent20 = 0x334,   // machine performance-monitoring event selector
    mhpmevent21 = 0x335,   // machine performance-monitoring event selector
    mhpmevent22 = 0x336,   // machine performance-monitoring event selector
    mhpmevent23 = 0x337,   // machine performance-monitoring event selector
    mhpmevent24 = 0x338,   // machine performance-monitoring event selector
    mhpmevent25 = 0x339,   // machine performance-monitoring event selector
    mhpmevent26 = 0x33A,   // machine performance-monitoring event selector
    mhpmevent27 = 0x33B,   // machine performance-monitoring event selector
    mhpmevent28 = 0x33C,   // machine performance-monitoring event selector
    mhpmevent29 = 0x33D,   // machine performance-monitoring event selector
    mhpmevent30 = 0x33E,   // machine performance-monitoring event selector
    mhpmevent31 = 0x33F,   // machine performance-monitoring event selector

    mcyclecfgh = 0x721, // upper 32 bits of `mcyclecfg`, RV32 only
    minstretcfgh = 0x722, // upper 32 bits of `minstretcfg`, RV32 only
    mhpmevent3h = 0x723, // upper 32 bits of `mhpmevent3`, RV32 only
    mhpmevent4h = 0x724, // upper 32 bits of `mhpmevent4`, RV32 only
    mhpmevent5h = 0x725, // upper 32 bits of `mhpmevent5`, RV32 only
    mhpmevent6h = 0x726, // upper 32 bits of `mhpmevent6`, RV32 only
    mhpmevent7h = 0x727, // upper 32 bits of `mhpmevent7`, RV32 only
    mhpmevent8h = 0x728, // upper 32 bits of `mhpmevent8`, RV32 only
    mhpmevent9h = 0x729, // upper 32 bits of `mhpmevent9`, RV32 only
    mhpmevent10h = 0x72A, // upper 32 bits of `mhpmevent10`, RV32 only
    mhpmevent11h = 0x72B, // upper 32 bits of `mhpmevent11`, RV32 only
    mhpmevent12h = 0x72C, // upper 32 bits of `mhpmevent12`, RV32 only
    mhpmevent13h = 0x72D, // upper 32 bits of `mhpmevent13`, RV32 only
    mhpmevent14h = 0x72E, // upper 32 bits of `mhpmevent14`, RV32 only
    mhpmevent15h = 0x72F, // upper 32 bits of `mhpmevent15`, RV32 only
    mhpmevent16h = 0x730, // upper 32 bits of `mhpmevent16`, RV32 only
    mhpmevent17h = 0x731, // upper 32 bits of `mhpmevent17`, RV32 only
    mhpmevent18h = 0x732, // upper 32 bits of `mhpmevent18`, RV32 only
    mhpmevent19h = 0x733, // upper 32 bits of `mhpmevent19`, RV32 only
    mhpmevent20h = 0x734, // upper 32 bits of `mhpmevent20`, RV32 only
    mhpmevent21h = 0x735, // upper 32 bits of `mhpmevent21`, RV32 only
    mhpmevent22h = 0x736, // upper 32 bits of `mhpmevent22`, RV32 only
    mhpmevent23h = 0x737, // upper 32 bits of `mhpmevent23`, RV32 only
    mhpmevent24h = 0x738, // upper 32 bits of `mhpmevent24`, RV32 only
    mhpmevent25h = 0x739, // upper 32 bits of `mhpmevent25`, RV32 only
    mhpmevent26h = 0x73A, // upper 32 bits of `mhpmevent26`, RV32 only
    mhpmevent27h = 0x73B, // upper 32 bits of `mhpmevent27`, RV32 only
    mhpmevent28h = 0x73C, // upper 32 bits of `mhpmevent28`, RV32 only
    mhpmevent29h = 0x73D, // upper 32 bits of `mhpmevent29`, RV32 only
    mhpmevent30h = 0x73E, // upper 32 bits of `mhpmevent30`, RV32 only
    mhpmevent31h = 0x73F, // upper 32 bits of `mhpmevent31`, RV32 only

    // machine control transfer records configuration
    mctrctl = 0x34E, // machine control transfer records control register

    // machine/debug level CSRs

    // debug/trace registers (shared with debug mode)
    tselect = 0x7A0,  // debug/trace trigger register select
    tdata1 = 0x7A1,   // first debug/trace trigger data register
    tdata2 = 0x7A2,   // second debug/trace trigger data register
    tdata3 = 0x7A3,   // third debug/trace trigger data register
    mcontext = 0x7A8, // machine-mode context register

    // debug mode registers

    // debug mode registers
    dcsr = 0x7B0,      // debug control and status register
    dpc = 0x7B1,       // debug program counter
    dscratch0 = 0x7B2, // debug scratch register 0
    dscratch1 = 0x7B3, // debug scratch register 1
}

#[derive(Default, Clone)]
struct ZicsrContext {
    need_to_read_csr: bool,
    need_to_write_csr: bool,
    need_to_return_from_trap: bool,
    csr_value: XLen,
    uimm: XLen,
}

#[derive(Default, Clone)]
struct Decoder {
    opcode: Opcode,
    source_register_1: XLen,
    source_register_2: XLen,
    destination_register_offset: usize,

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
    need_to_branch: bool,

    zicsr: ZicsrContext,
}

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(Default, Clone)]
enum MemoryValueType {
    BYTE,
    HALF_WORD,
    WORD,
    #[default]
    DOUBLE_WORD,
    // QUAD_WORD,
    UNSIGNED_BYTE,
    UNSIGNED_HALF_WORD,
    UNSIGNED_WORD,
    // UNSIGNED_DOUBLE_WORD,
}

#[derive(Default, Clone)]
struct ExecutionContext {
    result: XLen,
    branch_target_address: XLen,
    jump_target_address: XLen,
    return_target_address: XLen,
    memory_address: XLen,
    value_to_store_u8: Byte,
    value_to_store_u16: HalfWord,
    value_to_store_u32: Word,
    value_to_store_u64: DoubleWord,
    // value_to_store_u128: u128,
    memory_value_type: MemoryValueType,
}

#[allow(non_snake_case)]
#[derive(Clone)]
struct Context {
    register_file: RegisterFile<RISCV_NUM_OF_REGISTERS>,
    PC: XLen,
    control_status_registers: RegisterFile<RISCV_NUM_OF_CSR>,
    current_instruction: InstructionType,
    decoder: Decoder,
    execution: ExecutionContext,
}

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(Clone)]
struct RISC_V_CPU {
    context: Context,
}

trait Bus<AddrType> {
    fn read<MemValType: MemoryValue>(&self, address: AddrType) -> MemValType;
    // fn read_data(&self, address: XLen, size: usize) -> Vec<Byte>;

    fn write<MemValType: MemoryValue>(
        &mut self,
        address: AddrType,
        value: MemValType,
    );
    // fn write_data(&mut self, address: XLen, data: Vec<Byte>);
}

trait MemoryValue: Sized {
    fn from_le_bytes(bytes: &[Byte]) -> Self;
    fn to_le_bytes(&self) -> Vec<Byte>;

    fn sign_extend(&self) -> XLen;
    fn zero_extend(&self) -> XLen;
}

impl MemoryValue for Byte {
    fn from_le_bytes(bytes: &[Byte]) -> Self {
        Self::from_le_bytes(bytes.try_into().unwrap())
    }

    fn to_le_bytes(&self) -> Vec<Byte> {
        Self::to_le_bytes(*self).to_vec()
    }

    fn sign_extend(&self) -> XLen {
        *self as SignedByte as SignedXLen as XLen
    }

    fn zero_extend(&self) -> XLen {
        *self as XLen
    }
}

impl MemoryValue for HalfWord {
    fn from_le_bytes(bytes: &[Byte]) -> Self {
        Self::from_le_bytes(bytes.try_into().unwrap())
    }

    fn to_le_bytes(&self) -> Vec<Byte> {
        Self::to_le_bytes(*self).to_vec()
    }

    fn sign_extend(&self) -> XLen {
        *self as SignedHalfWord as SignedXLen as XLen
    }

    fn zero_extend(&self) -> XLen {
        *self as XLen
    }
}
impl MemoryValue for Word {
    fn from_le_bytes(bytes: &[Byte]) -> Self {
        Self::from_le_bytes(bytes.try_into().unwrap())
    }

    fn to_le_bytes(&self) -> Vec<Byte> {
        Self::to_le_bytes(*self).to_vec()
    }

    fn sign_extend(&self) -> XLen {
        *self as SignedWord as SignedXLen as XLen
    }

    fn zero_extend(&self) -> XLen {
        *self as XLen
    }
}

impl MemoryValue for DoubleWord {
    fn from_le_bytes(bytes: &[Byte]) -> Self {
        Self::from_le_bytes(bytes.try_into().unwrap())
    }

    fn to_le_bytes(&self) -> Vec<Byte> {
        Self::to_le_bytes(*self).to_vec()
    }

    fn sign_extend(&self) -> XLen {
        *self as SignedDoubleWord as SignedXLen as XLen
    }

    fn zero_extend(&self) -> XLen {
        *self as XLen
    }
}

// impl MemoryValue for QuadWord {
//     fn from_le_bytes(bytes: &[Byte]) -> Self {
//         Self::from_le_bytes(bytes.try_into().unwrap())
//     }

//     fn to_le_bytes(&self) -> Vec<Byte> {
//         Self::to_le_bytes(*self).to_vec()
//     }

//     // NOTICE: truncates value
//     fn sign_extend(&self) -> XLen {
//         *self as SignedQuadWord as SignedXLen as XLen
//     }

//     // NOTICE: truncates value
//     fn zero_extend(&self) -> XLen {
//         *self as XLen
//     }
// }

struct Memory<AddrType> {
    byte_array: Vec<Byte>,

    _phantom_marker: PhantomData<AddrType>,
}

impl<AddrType> Memory<AddrType> {
    fn read<MemValType: MemoryValue>(&self, address: XLen) -> MemValType {
        trace!("Reading memory at addr: {address:#018X?}");

        let address = address - RAM_BASE;
        let bytes = (self.byte_array)
            [address as usize..address as usize + size_of::<MemValType>()]
            .try_into()
            .unwrap();
        // TODO: notice, so far emulator only supports little-endian
        MemValType::from_le_bytes(bytes)
    }

    fn write<MemValType: MemoryValue>(
        &mut self,
        address: XLen,
        value: MemValType,
    ) {
        trace!("Writing memory at addr: {address:#018X?}");

        let address = address - RAM_BASE;
        // TODO: notice, so far emulator only supports little-endian
        let bytes = value.to_le_bytes();
        self.byte_array
            [address as usize..address as usize + size_of::<MemValType>()]
            .copy_from_slice(&bytes);
    }
}

struct SystemBus<AddrType> {
    memory: Memory<AddrType>,
}

impl<AddrType> Bus<XLen> for SystemBus<AddrType> {
    fn read<MemValType: MemoryValue>(&self, address: XLen) -> MemValType {
        if (RAM_BASE..RAM_END).contains(&address) {
            self.memory.read(address)
        } else {
            error!("an attempt to read from unimplemented address space!");
            panic!("an attempt to read from unimplemented address space");
        }
    }
    fn write<MemValType: MemoryValue>(
        &mut self,
        address: XLen,
        value: MemValType,
    ) {
        if (RAM_BASE..RAM_END).contains(&address) {
            self.memory.write(address, value);
        } else {
            error!("an attempt to write to unimplemented address space!");
            panic!("an attempt to write to unimplemented address space");
        }
    }
}

struct SoC {
    cpu: RISC_V_CPU,
    bus: SystemBus<BusAddrType>,
}

impl SoC {
    fn fetch(&mut self) {
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
        self.bus.memory.byte_array = [MEM_DEFAULT_VALUE; RAM_SIZE].to_vec();
        for addr in (RAM_BASE..RAM_END).step_by(PC_STEP as usize) {
            self.bus.memory.write(addr, Opcode::OP_IMM as Byte);
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
    I_3_LOAD = 0b0000011,
    I_7_LOAD_FP = 0b0000111,
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

    I_115_SYSTEM = 0b1110011, // (Zicsr_extension)
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

            Opcode::MISC_MEM | Opcode::AMO | Opcode::OP_V | Opcode::OP_VE => {
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

impl RISC_V_CPU {
    fn decode_r_format(&mut self) {
        self.context.decoder.source_register_1 =
            self.extract_source_register_1();
        self.context.decoder.source_register_2 =
            self.extract_source_register_2();
        self.context.decoder.destination_register_offset =
            self.extract_destination_register_index();
        self.context.decoder.need_to_read_memory = false;
        self.context.decoder.need_to_write_memory = false;
        self.context.decoder.need_to_write_back = true;
        self.context.decoder.need_to_jump = false;
        self.context.decoder.need_to_branch = false;
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
        self.context.decoder.destination_register_offset =
            self.extract_destination_register_index();
        self.context.decoder.immediate =
            self.extract_imm_11_0() as UnsignedImmediate;

        self.context.decoder.funct3 = self.extract_funct3();

        self.context.decoder.need_to_read_memory = false;
        self.context.decoder.need_to_write_memory = false;
        self.context.decoder.need_to_write_back = true;
        self.context.decoder.need_to_jump = false;
        self.context.decoder.need_to_branch = false;
        self.context.decoder.zicsr.need_to_read_csr = false;
        self.context.decoder.zicsr.need_to_write_csr = false;
        self.context.decoder.zicsr.need_to_return_from_trap = false;

        match self.context.decoder.opcode {
            // I_3
            Opcode::LOAD => {
                self.context.decoder.need_to_read_memory = true;
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

            // I_115 (Zicsr_extension)
            Opcode::SYSTEM => {
                match SYSTEM_Funct3::from(self.context.decoder.funct3) {
                    SYSTEM_Funct3::TRANSFER_CONTROL_RETURN_FROM => {
                        self.context.decoder.need_to_write_back = false;
                        let are_rd_and_rs1_zero = || -> bool {
                            (self.context.decoder.source_register_1 == 0)
                                && (self.context.register_file.read(
                                    self.context
                                        .decoder
                                        .destination_register_offset,
                                ) == 0)
                        };
                        let assert_rd_and_rs1_are_zero = || {
                            if !are_rd_and_rs1_zero() {
                                error!("rs1 and rd should be equal 0");
                                unreachable!("rs1 and rd should be equal 0");
                            }
                        };

                        assert_rd_and_rs1_are_zero();
                        match From::from(self.context.decoder.immediate) {
                            SYSTEM_Imm_transfer_control_return_from::ECALL => {
                                //
                                self.context.decoder.executable_instruction =
                                    ExecutableInstruction::ECALL;
                                todo!();
                            }
                            SYSTEM_Imm_transfer_control_return_from::EBREAK => {
                                //
                                self.context.decoder.executable_instruction =
                                    ExecutableInstruction::EBREAK;
                                todo!();
                            }
                            SYSTEM_Imm_transfer_control_return_from::URET => {
                                self.context
                                    .decoder
                                    .zicsr
                                    .need_to_return_from_trap = true;
                                self.context.decoder.executable_instruction =
                                    ExecutableInstruction::URET;
                                todo!();
                            }
                            SYSTEM_Imm_transfer_control_return_from::SRET => {
                                self.context
                                    .decoder
                                    .zicsr
                                    .need_to_return_from_trap = true;
                                self.context.decoder.executable_instruction =
                                    ExecutableInstruction::SRET;
                                todo!();
                            }
                            SYSTEM_Imm_transfer_control_return_from::MRET => {
                                self.context
                                    .decoder
                                    .zicsr
                                    .need_to_return_from_trap = true;
                                self.context.decoder.executable_instruction =
                                    ExecutableInstruction::MRET;
                            }
                        }
                        todo!();
                    }
                    // CSR instructions are atomic
                    SYSTEM_Funct3::CSRRW => {
                        self.context.decoder.zicsr.need_to_write_csr = true;
                        if self.context.decoder.destination_register_offset
                            != GPRegisterName::x0_zero as usize
                        {
                            self.context.decoder.zicsr.need_to_read_csr = true;
                        }
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::CSRRW;
                    }
                    SYSTEM_Funct3::CSRRS => {
                        self.context.decoder.zicsr.need_to_read_csr = true;
                        if self.extract_source_register_1_index() != 0 {
                            self.context.decoder.zicsr.need_to_write_csr = true;
                        }
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::CSRRS;
                    }
                    SYSTEM_Funct3::CSRRC => {
                        self.context.decoder.zicsr.need_to_read_csr = true;
                        if self.extract_source_register_1_index() != 0 {
                            self.context.decoder.zicsr.need_to_write_csr = true;
                        }
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::CSRRC;
                    }
                    SYSTEM_Funct3::CSRRWI => {
                        self.context.decoder.zicsr.need_to_write_csr = true;
                        self.context.decoder.zicsr.uimm =
                            self.extract_source_register_1_index() as XLen;
                        if self.context.decoder.destination_register_offset
                            != GPRegisterName::x0_zero as usize
                        {
                            self.context.decoder.zicsr.need_to_read_csr = true;
                        }
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::CSRRWI;
                    }
                    SYSTEM_Funct3::CSRRSI => {
                        self.context.decoder.zicsr.need_to_read_csr = true;
                        if self.extract_source_register_1_index() != 0 {
                            self.context.decoder.zicsr.need_to_write_csr = true;
                            self.context.decoder.zicsr.uimm =
                                self.extract_source_register_1_index() as XLen;
                        }
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::CSRRSI;
                    }
                    SYSTEM_Funct3::CSRRCI => {
                        self.context.decoder.zicsr.need_to_read_csr = true;
                        if self.extract_source_register_1_index() != 0 {
                            self.context.decoder.zicsr.need_to_write_csr = true;
                            self.context.decoder.zicsr.uimm =
                                self.extract_source_register_1_index() as XLen;
                        }
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::CSRRCI;
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
        self.context.decoder.need_to_branch = false;

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
        self.context.decoder.need_to_branch = true;

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
        self.context.decoder.destination_register_offset =
            self.extract_destination_register_index();
        self.context.decoder.need_to_read_memory = false;
        self.context.decoder.need_to_write_memory = false;
        self.context.decoder.need_to_write_back = true;
        self.context.decoder.need_to_jump = false;
        self.context.decoder.need_to_branch = false;

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
        self.context.decoder.destination_register_offset =
            self.extract_destination_register_index();
        self.context.decoder.need_to_read_memory = false;
        self.context.decoder.need_to_write_memory = false;
        self.context.decoder.need_to_write_back = true;
        self.context.decoder.need_to_jump = true;
        self.context.decoder.need_to_branch = false;

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
                //
                _ => {
                    todo!("TODO: unimplemented");
                }
            },
        };
    }

    fn decode(&mut self) {
        self.context.decoder.opcode = self.extract_opcode();

        match From::from(self.context.decoder.opcode.clone()) {
            OpcodeToInstructionFormat::R_51_OP
            | OpcodeToInstructionFormat::R_59_OP_32
            | OpcodeToInstructionFormat::R_83_OP_FP => self.decode_r_format(),
            OpcodeToInstructionFormat::I_3_LOAD
            | OpcodeToInstructionFormat::I_7_LOAD_FP
            | OpcodeToInstructionFormat::I_19_OP_IMM
            | OpcodeToInstructionFormat::I_27_OP_IMM_32
            | OpcodeToInstructionFormat::I_103_JALR
            | OpcodeToInstructionFormat::I_115_SYSTEM // (Zicsr_extension)
            => self.decode_i_format(),
            OpcodeToInstructionFormat::S_35_STORE
            | OpcodeToInstructionFormat::S_39_STORE_FP => {
                self.decode_s_format()
            }
            OpcodeToInstructionFormat::B_99_BRANCH => self.decode_b_format(),
            OpcodeToInstructionFormat::U_23_AUIPC
            | OpcodeToInstructionFormat::U_55_LUI => self.decode_u_format(),
            OpcodeToInstructionFormat::J_111_JAL => self.decode_j_format(),
            OpcodeToInstructionFormat::R4_67_MADD
            | OpcodeToInstructionFormat::R4_71_MSUB
            | OpcodeToInstructionFormat::R4_75_NMSUB
            | OpcodeToInstructionFormat::R4_79_NMADD => self.decode_r4_format(),
        }
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

            ExecutableInstruction::FENCE => {
                self.instruction_fence();
            }

            ExecutableInstruction::ECALL => {
                self.instruction_execution_call();
            }
            ExecutableInstruction::EBREAK => {
                self.instruction_execution_break();
            }

            ExecutableInstruction::URET => {
                self.instruction_user_return();
            }
            ExecutableInstruction::SRET => {
                self.instruction_supervisor_return();
            }
            ExecutableInstruction::MRET => {
                self.instruction_machine_return();
            }

            // Zicsr extension
            ExecutableInstruction::CSRRW => {
                self.instruction_csr_read_write();
            }
            ExecutableInstruction::CSRRS => {
                self.instruction_csr_read_set();
            }
            ExecutableInstruction::CSRRC => {
                self.instruction_csr_read_clear();
            }
            ExecutableInstruction::CSRRWI => {
                self.instruction_csr_read_write_immediate();
            }
            ExecutableInstruction::CSRRSI => {
                self.instruction_csr_read_set_immediate();
            }
            ExecutableInstruction::CSRRCI => {
                self.instruction_csr_read_clear_immediate();
            }
        }
    }

    fn instruction_load_byte(&mut self) {
        trace!("Executing LB...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.execution.memory_value_type = MemoryValueType::BYTE;
    }

    fn instruction_load_half(&mut self) {
        trace!("Executing LH...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.execution.memory_value_type = MemoryValueType::HALF_WORD;
    }

    fn instruction_load_word(&mut self) {
        trace!("Executing LW...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.execution.memory_value_type = MemoryValueType::WORD;
    }

    fn instruction_load_byte_unsigned(&mut self) {
        trace!("Executing LBU...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.execution.memory_value_type =
            MemoryValueType::UNSIGNED_BYTE;
    }

    fn instruction_load_half_unsigned(&mut self) {
        trace!("Executing LHU...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.execution.memory_value_type =
            MemoryValueType::UNSIGNED_HALF_WORD;
    }

    fn instruction_addi(&mut self) {
        trace!("Executing ADDI...");

        self.context.execution.result = self.context.decoder.source_register_1
            + self.context.decoder.immediate as SignedImmediate as SignedXLen
                as XLen;
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
                < (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen)
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
                < (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as UnsignedXLen)
            {
                1
            } else {
                0
            };
    }

    fn instruction_xori(&mut self) {
        trace!("Executing XORI...");

        self.context.execution.result = self.context.decoder.source_register_1
            ^ self.context.decoder.immediate as SignedImmediate as SignedXLen
                as XLen;
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
            | self.context.decoder.immediate as SignedImmediate as SignedXLen
                as XLen;
    }

    fn instruction_andi(&mut self) {
        trace!("Executing ANDI...");

        self.context.execution.result = self.context.decoder.source_register_1
            & self.context.decoder.immediate as SignedImmediate as SignedXLen
                as XLen;
    }

    fn instruction_auipc(&mut self) {
        trace!("Executing AUIPC...");

        self.context.execution.result = self.context.PC
            + self.context.decoder.immediate as SignedImmediate as SignedXLen
                as XLen;
    }

    fn instruction_store_byte(&mut self) {
        trace!("Executing SB...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.execution.value_to_store_u8 =
            self.context.decoder.source_register_2 as Byte;
        self.context.execution.memory_value_type = MemoryValueType::BYTE;
    }

    fn instruction_store_half_word(&mut self) {
        trace!("Executing SH...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.execution.value_to_store_u16 =
            self.context.decoder.source_register_2 as HalfWord;
        self.context.execution.memory_value_type = MemoryValueType::HALF_WORD;
    }

    fn instruction_store_word(&mut self) {
        trace!("Executing SW...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.execution.value_to_store_u32 =
            self.context.decoder.source_register_2 as Word;
        self.context.execution.memory_value_type = MemoryValueType::WORD;
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

        self.context.execution.result = self.context.decoder.immediate
            as SignedImmediate
            as SignedXLen as XLen;
    }

    fn instruction_beq(&mut self) {
        trace!("Executing BEQ...");

        if self.context.decoder.source_register_1
            == self.context.decoder.source_register_2
        {
            self.context.execution.branch_target_address = self.context.PC
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        }
        error!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_bne(&mut self) {
        trace!("Executing BNE...");

        if self.context.decoder.source_register_1
            != self.context.decoder.source_register_2
        {
            self.context.execution.branch_target_address = self.context.PC
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        }
        error!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_blt(&mut self) {
        trace!("Executing BLT...");

        if (self.context.decoder.source_register_1 as SignedXLen)
            < (self.context.decoder.source_register_2 as SignedXLen)
        {
            self.context.execution.branch_target_address = self.context.PC
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        }
        error!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_bge(&mut self) {
        trace!("Executing BGE...");

        if (self.context.decoder.source_register_1 as SignedXLen)
            >= (self.context.decoder.source_register_2 as SignedXLen)
        {
            self.context.execution.branch_target_address = self.context.PC
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        }
        error!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_bltu(&mut self) {
        trace!("Executing BLTU...");

        if (self.context.decoder.source_register_1 as UnsignedXLen)
            < (self.context.decoder.source_register_2 as UnsignedXLen)
        {
            self.context.execution.branch_target_address = self.context.PC
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        }
        error!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_bgeu(&mut self) {
        trace!("Executing BGEU...");

        if (self.context.decoder.source_register_1 as UnsignedXLen)
            >= (self.context.decoder.source_register_2 as UnsignedXLen)
        {
            self.context.execution.branch_target_address = self.context.PC
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        }
        error!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_jalr(&mut self) {
        trace!("Executing JALR...");

        self.context.execution.jump_target_address =
            (self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen))
                & !(XLen::MAX & 0x01);
        self.context.execution.result = self.context.PC + PC_STEP;
        error!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_jal(&mut self) {
        trace!("Executing JAL...");

        self.context.execution.jump_target_address = self.context.PC
            + (self.context.decoder.immediate as SignedImmediate as SignedXLen
                as XLen);
        self.context.execution.result = self.context.PC + PC_STEP;
        error!(
            "TODO: unimplemented: generate exception if address is misaligned"
        );
    }

    fn instruction_load_double_word(&mut self) {
        trace!("Executing LD...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.execution.memory_value_type = MemoryValueType::DOUBLE_WORD;
    }

    fn instruction_load_word_unsigned(&mut self) {
        trace!("Executing LWU...");

        self.context.execution.memory_address =
            self.context.decoder.source_register_1
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.execution.memory_value_type =
            MemoryValueType::UNSIGNED_WORD;
    }

    fn instruction_add_immediate_word(&mut self) {
        trace!("Executing ADDIW...");

        let result32 = (self.context.decoder.source_register_1 as XLen32)
            + self.context.decoder.immediate;
        self.context.execution.result =
            result32 as SignedXLen32 as SignedXLen as XLen;
    }

    fn instruction_shift_left_logical_immediate_word(&mut self) {
        trace!("Executing SLLIW...");

        let result32 = (self.context.decoder.source_register_1 as XLen32)
            << self.context.decoder.immediate;
        self.context.execution.result =
            result32 as SignedXLen32 as SignedXLen as XLen;
    }

    fn instruction_shift_right_logical_immediate_word(&mut self) {
        trace!("Executing SRLIW...");

        let result32 = (self.context.decoder.source_register_1 as XLen32)
            >> self.context.decoder.immediate;
        self.context.execution.result =
            result32 as SignedXLen32 as SignedXLen as XLen;
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
                + (self.context.decoder.immediate as SignedImmediate
                    as SignedXLen as XLen);
        self.context.execution.value_to_store_u64 =
            self.context.decoder.source_register_2;
        self.context.execution.memory_value_type = MemoryValueType::DOUBLE_WORD;
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

    fn instruction_fence(&mut self) {
        //
        todo!();
    }

    fn instruction_execution_call(&mut self) {
        //
        todo!();
    }

    fn instruction_execution_break(&mut self) {
        //
        todo!();
    }

    fn instruction_user_return(&mut self) {
        //
        todo!();
    }

    fn instruction_supervisor_return(&mut self) {
        //
        todo!();
    }

    fn instruction_machine_return(&mut self) {
        self.context.execution.return_target_address = self
            .context
            .control_status_registers
            .read(CSRRegisterName::mepc as usize);
        todo!();
    }

    fn read_csr_to_result(&mut self) {
        self.context.execution.result = self
            .context
            .control_status_registers
            .read(self.context.decoder.immediate as usize);
    }

    fn prepare_value_for_csr(&mut self, value: XLen) {
        self.context.decoder.zicsr.csr_value = value;
    }

    fn prepare_set_bits_for_csr(&mut self, value: XLen) {
        self.context.decoder.zicsr.csr_value |= value;
    }

    fn prepare_clear_bits_for_csr(&mut self, value: XLen) {
        self.context.decoder.zicsr.csr_value &= !value;
    }

    fn instruction_csr_read_write(&mut self) {
        if self.context.decoder.zicsr.need_to_read_csr {
            self.read_csr_to_result();
        }
        self.prepare_value_for_csr(self.context.decoder.source_register_1);
    }

    fn instruction_csr_read_set(&mut self) {
        self.read_csr_to_result();
        if self.context.decoder.zicsr.need_to_write_csr {
            self.prepare_set_bits_for_csr(
                self.context.decoder.source_register_1,
            );
        }
    }

    fn instruction_csr_read_clear(&mut self) {
        self.read_csr_to_result();
        if self.context.decoder.zicsr.need_to_write_csr {
            self.prepare_clear_bits_for_csr(
                self.context.decoder.source_register_1,
            );
        }
    }

    fn instruction_csr_read_write_immediate(&mut self) {
        if self.context.decoder.zicsr.need_to_read_csr {
            self.read_csr_to_result();
        }
        self.prepare_value_for_csr(self.context.decoder.zicsr.uimm);
    }

    fn instruction_csr_read_set_immediate(&mut self) {
        self.read_csr_to_result();
        if self.context.decoder.zicsr.need_to_write_csr {
            self.prepare_set_bits_for_csr(self.context.decoder.zicsr.uimm);
        }
    }

    fn instruction_csr_read_clear_immediate(&mut self) {
        self.read_csr_to_result();
        if self.context.decoder.zicsr.need_to_write_csr {
            self.prepare_clear_bits_for_csr(self.context.decoder.zicsr.uimm);
        }
    }

    fn read_memory_signed<ValType: MemoryValue>(
        &mut self,
        bus: &mut SystemBus<BusAddrType>,
    ) {
        self.context.execution.result = bus
            .read::<ValType>(self.context.execution.memory_address)
            .sign_extend();
    }

    fn read_memory_unsigned<ValType: MemoryValue>(
        &mut self,
        bus: &mut SystemBus<BusAddrType>,
    ) {
        self.context.execution.result = bus
            .read::<ValType>(self.context.execution.memory_address)
            .zero_extend();
    }

    fn memory_operation(&mut self, bus: &mut SystemBus<BusAddrType>) {
        if self.context.decoder.need_to_read_memory {
            trace!("Executing memory read...");

            match self.context.execution.memory_value_type {
                MemoryValueType::DOUBLE_WORD => {
                    self.read_memory_signed::<DoubleWord>(bus);
                }
                MemoryValueType::WORD => {
                    self.read_memory_signed::<Word>(bus);
                }
                MemoryValueType::HALF_WORD => {
                    self.read_memory_signed::<HalfWord>(bus);
                }
                MemoryValueType::BYTE => {
                    self.read_memory_signed::<Byte>(bus);
                }
                MemoryValueType::UNSIGNED_WORD => {
                    self.read_memory_unsigned::<Word>(bus);
                }
                MemoryValueType::UNSIGNED_HALF_WORD => {
                    self.read_memory_unsigned::<HalfWord>(bus);
                }
                MemoryValueType::UNSIGNED_BYTE => {
                    self.read_memory_unsigned::<Byte>(bus);
                }
            }
        } else if self.context.decoder.need_to_write_memory {
            trace!("Executing memory write...");

            match self.context.execution.memory_value_type {
                MemoryValueType::DOUBLE_WORD => {
                    bus.write(
                        self.context.execution.memory_address,
                        self.context.execution.value_to_store_u64,
                    );
                }
                MemoryValueType::WORD => {
                    bus.write(
                        self.context.execution.memory_address,
                        self.context.execution.value_to_store_u32,
                    );
                }
                MemoryValueType::HALF_WORD => {
                    bus.write(
                        self.context.execution.memory_address,
                        self.context.execution.value_to_store_u16,
                    );
                }
                MemoryValueType::BYTE => {
                    bus.write(
                        self.context.execution.memory_address,
                        self.context.execution.value_to_store_u8,
                    );
                }

                MemoryValueType::UNSIGNED_WORD
                | MemoryValueType::UNSIGNED_HALF_WORD
                | MemoryValueType::UNSIGNED_BYTE => {
                    error!("unreachable!");
                    unreachable!("unreachable!");
                }
            }
        }
    }

    fn write_back_to_reg(&mut self) {
        let register_offset = self.context.decoder.destination_register_offset;
        trace!("Writing back to register x{register_offset:#?}...");

        self.context
            .register_file
            .write(register_offset, self.context.execution.result);
    }

    fn set_pc_to_target_address(&mut self, target_addr: XLen) {
        error!(
            "TODO: unimplemented: generate exception if BRANCH/JUMP (BTA/JTA) address is misaligned"
        );
        self.context.PC = target_addr;
    }

    fn return_to_target_address(&mut self) {
        trace!("Returning to target address from trap...");

        self.set_pc_to_target_address(
            self.context.execution.return_target_address,
        );
    }

    fn branch_to_target_address(&mut self) {
        trace!("Updating PC to BTA...");

        self.set_pc_to_target_address(
            self.context.execution.branch_target_address,
        );
    }

    fn jump_to_target_address(&mut self) {
        trace!("Updating PC to JTA...");

        self.set_pc_to_target_address(
            self.context.execution.jump_target_address,
        );
    }

    fn write_to_csr(&mut self) {
        let register_offset = self.context.decoder.immediate as usize;
        trace!("Writing to CSR #{register_offset:#?}...");

        self.context
            .control_status_registers
            .write(register_offset, self.context.decoder.zicsr.csr_value);
    }

    fn write_back(&mut self) {
        // check if write to general purpose register is necessary
        if self.context.decoder.need_to_write_back {
            self.write_back_to_reg();
            // jumps always incur register write-back
            if self.context.decoder.need_to_jump {
                self.jump_to_target_address();
                return;
            }
        } else if self.context.decoder.need_to_branch {
            self.branch_to_target_address();
            return;
        }
        // check if write to CSR is necessary
        if self.context.decoder.zicsr.need_to_write_csr {
            self.write_to_csr();
        }
        // check if return from trap is needed
        else if self.context.decoder.zicsr.need_to_return_from_trap {
            self.return_to_target_address();
        }
    }

    fn extract_opcode(&self) -> Opcode {
        Opcode::from(
            ((self.context.current_instruction >> BIT_OFFSET_OPCODE)
                & MASK_OPCODE) as u8,
        )
    }

    fn extract_source_register_1_index(&self) -> usize {
        ((self.context.current_instruction >> BIT_OFFSET_SOURCE_REGISTER_1)
            & MASK_SOURCE_REGISTER_1) as usize
    }

    fn extract_source_register_1(&self) -> XLen {
        self.context
            .register_file
            .read(self.extract_source_register_1_index())
    }

    fn extract_source_register_2_index(&self) -> usize {
        ((self.context.current_instruction >> BIT_OFFSET_SOURCE_REGISTER_2)
            & MASK_SOURCE_REGISTER_2) as usize
    }

    fn extract_source_register_2(&self) -> XLen {
        self.context
            .register_file
            .read(self.extract_source_register_2_index())
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
}

fn init() {
    init_logger();
    parse_args();
    configure();
}

fn main() {
    init();

    let mut soc: SoC = SoC {
        cpu: RISC_V_CPU {
            context: Context {
                PC: PC_RESET_VALUE,
                register_file: RegisterFile {
                    registers: [Register {
                        value: RISCV_X0_ZERO_REGISTER_VALUE,
                    }; RISCV_NUM_OF_REGISTERS],
                },
                control_status_registers: RegisterFile {
                    registers: [Register {
                        value: RISCV_CSR_INITIAL_VALUE,
                    }; RISCV_NUM_OF_CSR],
                },
                current_instruction: INITIAL_INSTRUCTION,
                decoder: Decoder {
                    ..Default::default()
                },
                execution: ExecutionContext {
                    result: XLen::MIN,
                    branch_target_address: PC_RESET_VALUE,
                    jump_target_address: PC_RESET_VALUE,
                    return_target_address: PC_RESET_VALUE,
                    memory_address: XLen::MIN,
                    value_to_store_u8: Default::default(),
                    value_to_store_u16: Default::default(),
                    value_to_store_u32: Default::default(),
                    value_to_store_u64: Default::default(),
                    memory_value_type: Default::default(),
                },
            },
        },
        bus: SystemBus {
            memory: Memory {
                byte_array: [MEM_DEFAULT_VALUE; RAM_SIZE].to_vec(),

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
