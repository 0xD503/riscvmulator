use serde::{Deserialize, Serialize};

//
use crate::soc::hart::core::decoder::constants::Opcode;
use crate::soc::hart::privileged::machine::{
    MXLen, SignedMXLen, UnsignedMXLen,
};

// XLEN is never greater than MXLEN, but XLEN might be smaller than MXLEN in
// less-privileged modes.
const _: () = assert!(size_of::<MXLen>() >= size_of::<XLen>());

pub type XLen = MXLen;
pub type SignedXLen = SignedMXLen;
pub type UnsignedXLen = UnsignedMXLen;
pub type XLen32 = u32;
pub type SignedXLen32 = i32;

pub type Byte = u8;
pub type SignedByte = i8;
pub type HalfWord = u16;
pub type SignedHalfWord = i16;
pub type Word = u32;
pub type SignedWord = i32;
pub type DoubleWord = u64;
pub type SignedDoubleWord = i64;
// pub type QuadWord = u128;
// pub type SignedQuadWord = i128;

pub type SignedImmediate = i32;
pub type UnsignedImmediate = u32;

// const IALIGN: i32 = 32;
// const ILEN: i32 = IALIGN * 1;

pub type InstructionType = u32;
pub type SignedInstructionType = i32;

pub type BusAddrType = XLen;

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub struct IRQ_Line(pub(crate) u32);

pub const RISCV_NUM_OF_REGISTERS: usize = 32;
pub const RISCV_NUM_OF_REGISTERS_EMBEDDED: usize = 16;
pub const RISCV_NUM_OF_CSR: usize = 1 << BITS_NUM_IMM_11_0;
pub const RISCV_X0_ZERO_REGISTER_VALUE: XLen = 0x0000_0000;
pub const RISCV_CSR_INITIAL_VALUE: XLen = 0x0000_0000;
// pub const RISCV_PC_STEP: XLen = size_of::<InstructionType>();

// decoder constants
pub const BIT_OFFSET_OPCODE: usize = 0;
pub const MASK_OPCODE: InstructionType = 0x7F;

pub const BIT_OFFSET_SOURCE_REGISTER_1: usize = 15;
pub const MASK_SOURCE_REGISTER_1: InstructionType = 0x1F;
pub const BIT_OFFSET_SOURCE_REGISTER_2: usize = 20;
pub const MASK_SOURCE_REGISTER_2: InstructionType = 0x1F;

pub const BIT_OFFSET_DESTINATION_REGISTER: usize = 7;
pub const MASK_DESTINATION_REGISTER: InstructionType = 0x1F;

pub const BIT_OFFSET_FUNCT3: usize = 12;
pub const MASK_FUNCT3: InstructionType = 0x07;
// pub const BIT_OFFSET_I_FUNCT7: usize = 25;
// pub const MASK_I_FUNCT7: InstructionType = 0x7F;
// pub const BIT_OFFSET_SHTYPE: usize = 25;
// pub const MASK_SHTYPE: InstructionType = 0x7F;
pub const BIT_OFFSET_SHTYPE: usize =
    if XLen::BITS == u64::BITS { 26 } else { 25 };
pub const MASK_SHTYPE: InstructionType =
    if XLen::BITS == u64::BITS { 0x3F } else { 0x7F };
pub const MASK_SHIFT_RIGHT_TYPE: InstructionType = if XLen::BITS == u64::BITS {
    0b1_0000
} else {
    0b10_0000
};

pub const MASK_I_FUNCT_7_SHIFT_RAI: InstructionType = if XLen::BITS == u64::BITS
{
    0b01_0000
} else {
    0b010_0000
};

pub const BIT_OFFSET_R_FUNCT7: usize = 25;
pub const MASK_R_FUNCT7: InstructionType = 0x7F;

pub const BIT_OFFSET_R_FUNCT7_RV64W: usize = 26;
pub const MASK_R_FUNCT7_RV64W: InstructionType = 0x3F;

pub const BIT_OFFSET_SHAMT: usize = 20;
//const MASK_SHAMT: InstructionType = 0x1F;
pub const MASK_SHAMT: InstructionType = XLen::BITS - 1;
pub const MASK_SHAMT_RV64W: InstructionType = XLen32::BITS - 1;

pub const BIT_OFFSET_IMM_11_0: usize = 20;
pub const BIT_OFFSET_IMM_11_0_FINAL: usize = 0;
pub const BITS_NUM_IMM_11_0: usize = 12;
// const MASK_IMM_11_0: InstructionType =
//     ((1 << BITS_NUM_IMM_11_0) - 1) << BIT_OFFSET_IMM_11_0_FINAL;
pub const BIT_OFFSET_IMM_11_5: usize = 25;
pub const BIT_OFFSET_IMM_11_5_FINAL: usize = 5;
pub const BIT_OFFSET_IMM_4_0: usize = 7;
pub const BIT_OFFSET_IMM_4_0_FINAL: usize = 0;
pub const MASK_IMM_4_0: InstructionType = 0x1F << BIT_OFFSET_IMM_4_0_FINAL;
pub const BIT_OFFSET_IMM_31_12: usize = 12;
pub const BIT_OFFSET_IMM_31_12_FINAL: usize = 12;
pub const BIT_OFFSET_IMM_10_5: usize = 25;
pub const BIT_OFFSET_IMM_10_5_FINAL: usize = 5;
pub const MASK_IMM_10_5: InstructionType = 0x3F << BIT_OFFSET_IMM_10_5_FINAL;
pub const BIT_OFFSET_IMM_4_1: usize = 8;
pub const BIT_OFFSET_IMM_4_1_FINAL: usize = 1;
pub const MASK_IMM_4_1: InstructionType = 0x0F << BIT_OFFSET_IMM_4_1_FINAL;
pub const BIT_OFFSET_IMM_10_1: usize = 21;
pub const BIT_OFFSET_IMM_10_1_FINAL: usize = 1;
pub const MASK_IMM_10_1: InstructionType = 0x3FF << BIT_OFFSET_IMM_10_1_FINAL;
pub const BIT_OFFSET_IMM_19_12: usize = 12;
pub const BIT_OFFSET_IMM_19_12_FINAL: usize = 12;
pub const MASK_IMM_19_12: InstructionType = 0xFF << BIT_OFFSET_IMM_19_12_FINAL;

pub const BIT_OFFSET_I_FENCE_SW: usize = 20;
pub const MASK_I_FENCE_SW: InstructionType = 1 << BIT_OFFSET_I_FENCE_SW;
pub const BIT_OFFSET_I_FENCE_SR: usize = 21;
pub const MASK_I_FENCE_SR: InstructionType = 1 << BIT_OFFSET_I_FENCE_SR;
pub const BIT_OFFSET_I_FENCE_SO: usize = 22;
pub const MASK_I_FENCE_SO: InstructionType = 1 << BIT_OFFSET_I_FENCE_SO;
pub const BIT_OFFSET_I_FENCE_SI: usize = 23;
pub const MASK_I_FENCE_SI: InstructionType = 1 << BIT_OFFSET_I_FENCE_SI;
pub const BIT_OFFSET_I_FENCE_PW: usize = 24;
pub const MASK_I_FENCE_PW: InstructionType = 1 << BIT_OFFSET_I_FENCE_PW;
pub const BIT_OFFSET_I_FENCE_PR: usize = 25;
pub const MASK_I_FENCE_PR: InstructionType = 1 << BIT_OFFSET_I_FENCE_PR;
pub const BIT_OFFSET_I_FENCE_PO: usize = 26;
pub const MASK_I_FENCE_PO: InstructionType = 1 << BIT_OFFSET_I_FENCE_PO;
pub const BIT_OFFSET_I_FENCE_PI: usize = 27;
pub const MASK_I_FENCE_PI: InstructionType = 1 << BIT_OFFSET_I_FENCE_PI;
pub const BIT_OFFSET_I_FENCE_FM: usize = 28;
pub const MASK_I_FENCE_FM: InstructionType = 0x0F << BIT_OFFSET_I_FENCE_FM;

pub const BIT_OFFSET_B_11: usize = 7;
pub const BIT_OFFSET_B_11_FINAL: usize = 11;
pub const MASK_B_11: InstructionType = 0x01 << BIT_OFFSET_B_11_FINAL;
pub const BIT_OFFSET_B_12: usize = 31;
pub const BIT_OFFSET_B_12_FINAL: usize = 12;
pub const BIT_OFFSET_J_11: usize = 20;
pub const BIT_OFFSET_J_11_FINAL: usize = 11;
pub const MASK_J_11: InstructionType = 0x01 << BIT_OFFSET_B_11_FINAL;
pub const BIT_OFFSET_J_20: usize = 31;
pub const BIT_OFFSET_J_20_FINAL: usize = 20;

pub const OPCODE_LOAD: u8 = 0b0000011;
pub const OPCODE_STORE: u8 = 0b0100011;
pub const OPCODE_MADD: u8 = 0b1000011;
pub const OPCODE_BRANCH: u8 = 0b1100011;
pub const OPCODE_LOAD_FP: u8 = 0b0000111;
pub const OPCODE_STORE_FP: u8 = 0b0100111;
pub const OPCODE_MSUB: u8 = 0b1000111;
pub const OPCODE_JALR: u8 = 0b1100111;
pub const OPCODE_CUSTOM_0: u8 = 0b0001011;
pub const OPCODE_CUSTOM_1: u8 = 0b0101011;
pub const OPCODE_NMSUB: u8 = 0b1001011;
pub const OPCODE_RESERVED_1: u8 = 0b1101011;
pub const OPCODE_MISC_MEM: u8 = 0b0001111;
pub const OPCODE_AMO: u8 = 0b0101111;
pub const OPCODE_NMADD: u8 = 0b1001111;
pub const OPCODE_JAL: u8 = 0b1101111;
pub const OPCODE_OP_IMM: u8 = 0b0010011;
pub const OPCODE_OP: u8 = 0b0110011;
pub const OPCODE_OP_FP: u8 = 0b1010011;
pub const OPCODE_SYSTEM: u8 = 0b1110011;
pub const OPCODE_AUIPC: u8 = 0b0010111;
pub const OPCODE_LUI: u8 = 0b0110111;
pub const OPCODE_OP_V: u8 = 0b1010111;
pub const OPCODE_OP_VE: u8 = 0b1110111;
pub const OPCODE_OP_IMM_32: u8 = 0b0011011;
pub const OPCODE_OP_32: u8 = 0b0111011;
pub const OPCODE_CUSTOM_2: u8 = 0b1011011;
pub const OPCODE_CUSTOM_3: u8 = 0b1111011;
pub const OPCODE_RESERVED_2: u8 = 0b0011111;
pub const OPCODE_RESERVED_3: u8 = 0b0111111;
pub const OPCODE_RESERVED_4: u8 = 0b1011111;
pub const OPCODE_RESERVED_5: u8 = 0b1111111;

// pub const RISCV_MAX_INSTRUCTION_SIZE: usize = 24;
// pub const RISCV_MIN_INSTRUCTION_SIZE: usize = 2;

pub const INSTRUCTION_NOP: InstructionType = Opcode::OP_IMM as InstructionType;
