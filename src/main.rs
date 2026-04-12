use clap::Parser;
use log::{debug, error, info, trace, warn};
use serde::Deserialize;
use std::convert::From;
use std::fmt::{self, Binary};
use std::mem::size_of;
use std::{env, fs};

/// architechtural defines
type XLen = u32;
type SignedXLen = i32;
type UnsignedXLen = u32;

type SignedImmediate = i32;
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

const INITIAL_INSTRUCTION: XLen = 0x00000000;

const MEM_DEFAULT_VALUE: u8 = 0x00;
const MEM_SIZE: usize = 65536; // placeholder value, to be configured later

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
const MASK_SHAMT: InstructionType = 0x1F;

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
    SRLI = 0b0000000,
    SRAI = 0b0100000,
}

impl From<InstructionType> for OP_IMM_Funct7 {
    fn from(value: InstructionType) -> Self {
        match value {
            0b0000000 => OP_IMM_Funct7::SRLI,
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
enum OP_Funct7 {
    #[default]
    F0 = 0b0000000,
    F1 = 0b0100000,
}

impl From<InstructionType> for OP_Funct7 {
    fn from(value: InstructionType) -> Self {
        match value {
            0b0000000 => OP_Funct7::F0,
            0b0100000 => OP_Funct7::F1,

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
        self.registers[offset]
    }

    fn write(&mut self, offset: usize, value: XLen) {
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
    r_funct7: OP_Funct7,

    shamt: InstructionType,

    immediate: UnsignedImmediate,
    executable_instruction: ExecutableInstruction,
    need_to_read_memory: bool,
    need_to_write_memory: bool,
    need_to_write_back: bool,
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

#[derive(Clone)]
struct Context {
    pc: XLen,
    register_file: RegisterFile,
    current_instruction: InstructionType,
    decoder: Decoder,
    result: XLen,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone)]
struct CPU {
    context: Context,
}

struct Memory {
    byte_array: Vec<u8>,
}

struct SoC {
    cpu: CPU,
    memory: Memory,
}

impl SoC {
    fn fetch(&mut self) {
        let bytes = self.memory.byte_array[self.cpu.context.pc as usize
            ..self.cpu.context.pc as usize + size_of::<InstructionType>()]
            .try_into()
            .unwrap();
        // NOTICE: so far emulator only supports little-endian
        let instruction = InstructionType::from_le_bytes(bytes);
        self.cpu.context.current_instruction = instruction;

        trace!(
            "Fetched instruction: {:#018X?}",
            self.cpu.context.current_instruction
        );
    }

    fn reset(&mut self) {
        self.cpu.context.pc = PC_RESET_VALUE;
        // register `x0_zero` is hardwired with all bits equal to 0, so do nothing with `x0_zero`
        for i in 1..RISCV_NUM_OF_REGISTERS {
            self.cpu
                .context
                .register_file
                .write(i, RISCV_X0_ZERO_REGISTER_VALUE);
        }
        self.cpu.context.current_instruction = INITIAL_INSTRUCTION;

        // fill memory with NOP's
        self.memory.byte_array = [MEM_DEFAULT_VALUE; MEM_SIZE].to_vec();
        for i in (0..MEM_SIZE).step_by(PC_STEP as usize) {
            self.memory.byte_array[i] = Opcode::OP_IMM as u8;
        }
    }

    fn step(&mut self) {
        trace!("Current PC: {:#018X}", self.cpu.context.pc);

        self.fetch();
        self.cpu.decode();
        self.cpu.execute();
        self.cpu.memory_operation();
        self.cpu.write_back();

        // step PC
        self.cpu.context.pc += PC_STEP;
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

impl CPU {
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
        self.context.decoder.funct3 = self.extract_funct3();
        self.context.decoder.r_funct7 = self.extract_r_funct7();

        match self.context.decoder.opcode {
            Opcode::OP => match OP_Funct3::from(self.context.decoder.funct3) {
                OP_Funct3::ARITH => match self.context.decoder.r_funct7 {
                    OP_Funct7::F0 => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::ADD;
                    }
                    OP_Funct7::F1 => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::SUB;
                    }
                },
                OP_Funct3::SLL => {
                    self.context.decoder.executable_instruction =
                        ExecutableInstruction::SLL;
                }
                OP_Funct3::SLT => {
                    self.context.decoder.executable_instruction =
                        ExecutableInstruction::SLT;
                }
                OP_Funct3::SLTU => {
                    self.context.decoder.executable_instruction =
                        ExecutableInstruction::SLTU;
                }
                OP_Funct3::XOR => {
                    self.context.decoder.executable_instruction =
                        ExecutableInstruction::XOR;
                }
                OP_Funct3::SRX => match self.context.decoder.r_funct7 {
                    OP_Funct7::F0 => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::SRL;
                    }
                    OP_Funct7::F1 => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::SRA;
                    }
                },
                OP_Funct3::OR => {
                    self.context.decoder.executable_instruction =
                        ExecutableInstruction::OR;
                }
                OP_Funct3::AND => {
                    self.context.decoder.executable_instruction =
                        ExecutableInstruction::AND;
                }
            },

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
        self.context.decoder.immediate = self.extract_imm_11_0();
        self.context.decoder.need_to_read_memory = false;
        self.context.decoder.need_to_write_memory = false;
        self.context.decoder.need_to_write_back = true;
        self.context.decoder.funct3 = self.extract_funct3();

        match self.context.decoder.opcode {
            Opcode::OP_IMM => {
                match OP_IMM_Funct3::from(self.context.decoder.funct3) {
                    OP_IMM_Funct3::ADDI => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::ADDI;
                    }
                    OP_IMM_Funct3::SLLI => {
                        self.context.decoder.shamt = self.extract_shamt();
                        // self.context.decoder.i_funct7 = self.extract_i_funct7();    // TODO: WHY THE FUCK???
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::SLLI;
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
                            OP_IMM_Funct7::SRLI => {
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

            _ => {
                panic!("Unimplemented");
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

        // extract immediate
        self.context.decoder.immediate = self.extract_imm_4_0();
        self.context.decoder.immediate |= self.extract_imm_11_5();

        self.context.decoder.funct3 = self.extract_funct3();

        match self.context.decoder.opcode {
            //
            _ => match self.context.decoder.funct3 {
                _ => {
                    panic!("Unimplemented");
                }
            },
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

        // extract immediate
        self.context.decoder.immediate = self.extract_imm_4_1();
        self.context.decoder.immediate |= self.extract_imm_10_5();
        self.context.decoder.immediate |= self.extract_b_11();
        self.context.decoder.immediate |= self.extract_b_12();

        self.context.decoder.funct3 = self.extract_funct3();
        panic!("Unimplemented");
    }

    fn decode_u_format(&mut self) {
        self.context.decoder.destination_register_index =
            self.extract_destination_register_index();
        self.context.decoder.need_to_read_memory = false;
        self.context.decoder.need_to_write_memory = false;
        self.context.decoder.need_to_write_back = true;

        // extract immediate
        self.context.decoder.immediate = self.extract_imm_31_12();

        panic!("Unimplemented");
    }

    fn decode_j_format(&mut self) {
        self.context.decoder.destination_register_index =
            self.extract_destination_register_index();
        self.context.decoder.need_to_read_memory = false;
        self.context.decoder.need_to_write_memory = false;
        self.context.decoder.need_to_write_back = true;

        // extract immediate
        self.context.decoder.immediate = self.extract_imm_10_1();
        self.context.decoder.immediate |= self.extract_j_11();
        self.context.decoder.immediate |= self.extract_imm_19_12();
        self.context.decoder.immediate |= self.extract_j_20();

        panic!("Unimplemented");
    }

    fn decode_r4_format(&mut self) {
        panic!("Unimplemented");
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
            ExecutableInstruction::ADDI => {
                trace!("Executing ADDI...");

                self.context.result = self.context.decoder.source_register_1
                    + self.context.decoder.immediate;
            }
            ExecutableInstruction::SLLI => {
                trace!("Executing SLLI...");

                self.context.result = self.context.decoder.source_register_1
                    << self.context.decoder.shamt;
            }
            ExecutableInstruction::SLTI => {
                trace!("Executing SLTI...");

                self.context.result = if (self.context.decoder.source_register_1
                    as SignedXLen)
                    < (self.context.decoder.immediate as SignedImmediate)
                {
                    1
                } else {
                    0
                };
            }
            ExecutableInstruction::SLTIU => {
                trace!("Executing SLTIU...");

                self.context.result = if (self.context.decoder.source_register_1
                    as UnsignedXLen)
                    < (self.context.decoder.immediate as UnsignedImmediate)
                {
                    1
                } else {
                    0
                };
            }
            ExecutableInstruction::XORI => {
                trace!("Executing XORI...");

                self.context.result = self.context.decoder.source_register_1
                    ^ self.context.decoder.immediate;
            }
            ExecutableInstruction::SRLI => {
                trace!("Executing SRLI...");

                self.context.result = self.context.decoder.source_register_1
                    >> self.context.decoder.shamt;
            }
            ExecutableInstruction::SRAI => {
                trace!("Executing SRAI...");

                self.context.result = self.context.decoder.source_register_1
                    >> self.context.decoder.shamt;
            }
            ExecutableInstruction::ORI => {
                trace!("Executing ORI...");

                panic!("Unimplemented");
            }
            ExecutableInstruction::ANDI => {
                trace!("Executing ANDI...");

                panic!("Unimplemented");
            }

            _ => {
                error!("unreachable");
                unreachable!("unreachable");
            }
        }
    }

    fn memory_operation(&mut self) {
        if self.context.decoder.need_to_read_memory {
            trace!("Executing memory read...");
            //
        } else if self.context.decoder.need_to_write_memory {
            trace!("Executing memory write...");
            //
        }
    }

    fn write_back(&mut self) {
        if self.context.decoder.need_to_write_back {
            trace!("Executing register write back...");
            self.context.register_file.write(
                self.context.decoder.destination_register_index,
                self.context.result,
            );
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

    fn extract_r_funct7(&self) -> OP_Funct7 {
        From::from(
            (self.context.current_instruction >> BIT_OFFSET_R_FUNCT7)
                & MASK_R_FUNCT7,
        )
    }

    fn extract_shamt(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_SHAMT) & MASK_SHAMT
    }

    fn extract_imm_11_0(&self) -> InstructionType {
        // right shift for signed values compiles to arithmetic shift right
        // (ASR) with sign extension
        (self.context.current_instruction as SignedInstructionType
            >> (BIT_OFFSET_IMM_11_0 - BIT_OFFSET_IMM_11_0_FINAL))
            as InstructionType
    }

    fn extract_imm_11_5(&self) -> InstructionType {
        // right shift for signed values compiles to arithmetic shift right
        // (ASR) with sign extension
        (self.context.current_instruction as SignedInstructionType
            >> (BIT_OFFSET_IMM_11_5 - BIT_OFFSET_IMM_11_5_FINAL))
            as InstructionType
            & !((1 << BIT_OFFSET_IMM_11_5_FINAL) - 1)
    }

    fn extract_imm_4_0(&self) -> InstructionType {
        (self.context.current_instruction
            >> (BIT_OFFSET_IMM_4_0 - BIT_OFFSET_IMM_4_0_FINAL))
            & MASK_IMM_4_0
    }

    fn extract_imm_31_12(&self) -> InstructionType {
        // right shift for signed values compiles to arithmetic shift right
        // (ASR) with sign extension
        (self.context.current_instruction as SignedInstructionType
            >> (BIT_OFFSET_IMM_31_12 - BIT_OFFSET_IMM_31_12_FINAL))
            as InstructionType
            & !((1 << BIT_OFFSET_IMM_31_12_FINAL) - 1)
    }

    fn extract_imm_10_5(&self) -> InstructionType {
        (self.context.current_instruction
            >> (BIT_OFFSET_IMM_10_5 - BIT_OFFSET_IMM_10_5_FINAL))
            & MASK_IMM_10_5
    }

    fn extract_imm_4_1(&self) -> InstructionType {
        (self.context.current_instruction
            >> (BIT_OFFSET_IMM_4_1 - BIT_OFFSET_IMM_4_1_FINAL))
            & MASK_IMM_4_1
    }

    fn extract_imm_10_1(&self) -> InstructionType {
        (self.context.current_instruction
            >> (BIT_OFFSET_IMM_10_1 - BIT_OFFSET_IMM_10_1_FINAL))
            & MASK_IMM_10_1
    }

    fn extract_imm_19_12(&self) -> InstructionType {
        (self.context.current_instruction
            >> (BIT_OFFSET_IMM_19_12 - BIT_OFFSET_IMM_19_12_FINAL))
            & MASK_IMM_19_12
    }

    fn extract_b_11(&self) -> InstructionType {
        (self.context.current_instruction
            << (BIT_OFFSET_B_11_FINAL - BIT_OFFSET_B_11))
            & MASK_B_11
    }

    fn extract_b_12(&self) -> InstructionType {
        // right shift for signed values compiles to arithmetic shift right
        // (ASR) with sign extension
        (self.context.current_instruction as SignedInstructionType
            >> (BIT_OFFSET_B_12 - BIT_OFFSET_B_12_FINAL))
            as InstructionType
            & !((1 << BIT_OFFSET_B_12_FINAL) - 1)
    }

    fn extract_j_11(&self) -> InstructionType {
        (self.context.current_instruction
            >> (BIT_OFFSET_J_11 - BIT_OFFSET_J_11_FINAL))
            & MASK_J_11
    }

    fn extract_j_20(&self) -> InstructionType {
        // right shift for signed values compiles to arithmetic shift right
        // (ASR) with sign extension
        ((self.context.current_instruction as SignedInstructionType)
            >> (BIT_OFFSET_J_20 - BIT_OFFSET_J_20_FINAL))
            as InstructionType
            & !((1 << BIT_OFFSET_J_20_FINAL) - 1)
    }

    // fn extract_i_imm_inst_31(&self) -> InstructionType {
    //     (self.context.current_instruction >> BIT_OFFSET_I_IMM_INST_31)
    //         & MASK_I_IMM_INST_31
    // }

    // fn extract_i_imm_inst_30_25(&self) -> InstructionType {
    //     (self.context.current_instruction >> BIT_OFFSET_I_IMM_INST_30_25)
    //         & MASK_I_IMM_INST_30_25
    // }

    // fn extract_i_imm_inst_24_21(&self) -> InstructionType {
    //     (self.context.current_instruction >> BIT_OFFSET_I_IMM_INST_24_21)
    //         & MASK_I_IMM_INST_24_21
    // }

    // fn extract_i_imm_20(&self) -> InstructionType {
    //     (self.context.current_instruction >> BIT_OFFSET_I_IMM_20)
    //         & MASK_I_IMM_20
    // }

    // fn extract_s_imm_inst_31(&self) -> InstructionType {
    //     (self.context.current_instruction >> BIT_OFFSET_S_IMM_INST_31)
    //         & MASK_S_IMM_INST_31
    // }

    // fn extract_s_imm_inst_30_25(&self) -> InstructionType {
    //     (self.context.current_instruction >> BIT_OFFSET_S_IMM_INST_30_25)
    //         & MASK_S_IMM_INST_30_25
    // }

    // fn extract_s_imm_inst_11_8(&self) -> InstructionType {
    //     (self.context.current_instruction >> BIT_OFFSET_S_IMM_INST_11_8)
    //         & MASK_S_IMM_INST_11_8
    // }

    // fn extract_s_imm_7(&self) -> InstructionType {
    //     (self.context.current_instruction >> BIT_OFFSET_S_IMM_7) & MASK_S_IMM_7
    // }

    // fn extract_b_imm_inst_31(&self) -> InstructionType {
    //     (self.context.current_instruction >> BIT_OFFSET_B_IMM_INST_31)
    //         & MASK_B_IMM_INST_31
    // }

    // fn extract_b_imm_7(&self) -> InstructionType {
    //     (self.context.current_instruction >> BIT_OFFSET_B_IMM_7) & MASK_B_IMM_7
    // }

    // fn extract_b_imm_inst_30_25(&self) -> InstructionType {
    //     (self.context.current_instruction >> BIT_OFFSET_B_IMM_INST_30_25)
    //         & MASK_B_IMM_INST_30_25
    // }

    // fn extract_b_imm_inst_11_8(&self) -> InstructionType {
    //     (self.context.current_instruction >> BIT_OFFSET_B_IMM_INST_11_8)
    //         & MASK_B_IMM_INST_11_8
    // }

    // fn extract_b_imm_0(&self) -> InstructionType {
    //     (self.context.current_instruction >> BIT_OFFSET_B_IMM_0) & MASK_B_IMM_0
    // }

    // fn extract_u_imm_31(&self) -> InstructionType {
    //     (self.context.current_instruction >> BIT_OFFSET_U_IMM_31)
    //         & MASK_U_IMM_31
    // }

    // fn extract_u_imm_inst_30_20(&self) -> InstructionType {
    //     (self.context.current_instruction >> BIT_OFFSET_U_IMM_INST_30_20)
    //         & MASK_U_IMM_INST_30_20
    // }

    // fn extract_u_imm_inst_19_12(&self) -> InstructionType {
    //     (self.context.current_instruction >> BIT_OFFSET_U_IMM_INST_19_12)
    //         & MASK_U_IMM_INST_19_12
    // }

    // fn extract_u_imm_0(&self) -> InstructionType {
    //     (self.context.current_instruction >> BIT_OFFSET_U_IMM_0) & MASK_U_IMM_0
    // }

    // fn extract_j_imm_inst_31(&self) -> InstructionType {
    //     (self.context.current_instruction >> BIT_OFFSET_J_IMM_INST_31)
    //         & MASK_J_IMM_INST_31
    // }

    // fn extract_j_imm_inst_19_12(&self) -> InstructionType {
    //     (self.context.current_instruction
    //         >> (BIT_OFFSET_J_IMM_INST_19_12
    //             - BIT_OFFSET_J_IMM_INST_19_12_FINAL))
    //         & MASK_J_IMM_INST_19_12
    // }

    // fn extract_j_imm_20(&self) -> InstructionType {
    //     (self.context.current_instruction >> BIT_OFFSET_J_IMM_20)
    //         & MASK_J_IMM_20
    // }

    // fn extract_j_imm_inst_30_25(&self) -> InstructionType {
    //     (self.context.current_instruction >> BIT_OFFSET_J_IMM_INST_30_25)
    //         & MASK_J_IMM_INST_30_25
    // }

    // fn extract_j_imm_inst_24_21(&self) -> InstructionType {
    //     (self.context.current_instruction >> BIT_OFFSET_J_IMM_INST_24_21)
    //         & MASK_J_IMM_INST_24_21
    // }

    // fn extract_j_imm_0(&self) -> InstructionType {
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
                pc: PC_RESET_VALUE,
                register_file: RegisterFile {
                    registers: [RISCV_X0_ZERO_REGISTER_VALUE;
                        RISCV_NUM_OF_REGISTERS],
                },
                current_instruction: INITIAL_INSTRUCTION,
                decoder: Decoder {
                    ..Default::default()
                },
                result: Default::default(),
            },
        },
        memory: Memory {
            byte_array: [MEM_DEFAULT_VALUE; MEM_SIZE].to_vec(),
        },
    };

    soc.reset();

    // TODO: HALT condition
    loop {
        soc.step();
    }
}
