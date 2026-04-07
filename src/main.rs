use log::{debug, error, info, trace, warn};
// use serde::Deserialize;
use std::convert::From;
use std::mem::size_of;
use std::{env, fs};

/// architechtural defines
type XLen = u32;
// const IALIGN: i32 = 32;
// const ILEN: i32 = IALIGN * 1;
type InstructionType = u32;
const RISCV_NUM_OF_REGISTERS: usize = 32;
const RISCV_X0_REGISTER_VALUE: XLen = 0x00000000;

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
const BIT_OFFSET_R_FUNCT7: usize = 25;
const MASK_R_FUNCT7: InstructionType = 0x7F;

const BIT_OFFSET_SHAMT: usize = 20;
const MASK_SHAMT: InstructionType = 0x1F;

const BIT_OFFSET_IMM_11_0: usize = 20;
const MASK_IMM_11_0: InstructionType = 0xFFF;
const BIT_OFFSET_IMM_11_5: usize = 25;
const MASK_IMM_11_5: InstructionType = 0x7F;
const BIT_OFFSET_IMM_4_0: usize = 7;
const MASK_IMM_4_0: InstructionType = 0x1F;
const BIT_OFFSET_IMM_31_12: usize = 12;
const MASK_IMM_31_12: InstructionType = 0xFFFFF;
const BIT_OFFSET_IMM_10_5: usize = 25;
const MASK_IMM_10_5: InstructionType = 0x3F;
const BIT_OFFSET_IMM_4_1: usize = 8;
const MASK_IMM_4_1: InstructionType = 0x0F;
const BIT_OFFSET_IMM_10_1: usize = 21;
const MASK_IMM_10_1: InstructionType = 0x3FF;
const BIT_OFFSET_IMM_19_12: usize = 12;
const MASK_IMM_19_12: InstructionType = 0xFF;

const BIT_OFFSET_B_11: usize = 7;
const MASK_B_11: InstructionType = 0x01;
const BIT_OFFSET_B_12: usize = 31;
const MASK_B_12: InstructionType = 0x01;
const BIT_OFFSET_J_11: usize = 20;
const MASK_J_11: InstructionType = 0x01;
const BIT_OFFSET_J_20: usize = 31;
const MASK_J_20: InstructionType = 0x01;

const BIT_OFFSET_I_IMM_INST_31: usize = 11;
const MASK_I_IMM_INST_31: InstructionType = 0x1FFFFF;
const BIT_OFFSET_I_IMM_INST_30_25: usize = 5;
const MASK_I_IMM_INST_30_25: InstructionType = 0x3F;
const BIT_OFFSET_I_IMM_INST_24_21: usize = 1;
const MASK_I_IMM_INST_24_21: InstructionType = 0x0F;
const BIT_OFFSET_I_IMM_20: usize = 0;
const MASK_I_IMM_20: InstructionType = 0x01;

const BIT_OFFSET_S_IMM_INST_31: usize = 11;
const MASK_S_IMM_INST_31: InstructionType = 0x1FFFFF;
const BIT_OFFSET_S_IMM_INST_30_25: usize = 5;
const MASK_S_IMM_INST_30_25: InstructionType = 0x3F;
const BIT_OFFSET_S_IMM_INST_11_8: usize = 1;
const MASK_S_IMM_INST_11_8: InstructionType = 0x0F;
const BIT_OFFSET_S_IMM_7: usize = 0;
const MASK_S_IMM_7: InstructionType = 0x01;

const BIT_OFFSET_B_IMM_INST_31: usize = 12;
const MASK_B_IMM_INST_31: InstructionType = 0xFFFFF;
const BIT_OFFSET_B_IMM_7: usize = 11;
const MASK_B_IMM_7: InstructionType = 0x01;
const BIT_OFFSET_B_IMM_INST_30_25: usize = 5;
const MASK_B_IMM_INST_30_25: InstructionType = 0x3F;
const BIT_OFFSET_B_IMM_INST_11_8: usize = 1;
const MASK_B_IMM_INST_11_8: InstructionType = 0x0F;
const BIT_OFFSET_B_IMM_0: usize = 0;
const MASK_B_IMM_0: InstructionType = 0x01;

const BIT_OFFSET_U_IMM_31: usize = 31;
const MASK_U_IMM_31: InstructionType = 0x01;
const BIT_OFFSET_U_IMM_INST_30_20: usize = 20;
const MASK_U_IMM_INST_30_20: InstructionType = 0x7FF;
const BIT_OFFSET_U_IMM_INST_19_12: usize = 12;
const MASK_U_IMM_INST_19_12: InstructionType = 0xFF;
const BIT_OFFSET_U_IMM_0: usize = 0;
const MASK_U_IMM_0: InstructionType = 0xFFF;

const BIT_OFFSET_J_IMM_INST_31: usize = 20;
const MASK_J_IMM_INST_31: InstructionType = 0xFFF;
const BIT_OFFSET_J_IMM_INST_19_12: usize = 12;
const MASK_J_IMM_INST_19_12: InstructionType = 0xFF;
const BIT_OFFSET_J_IMM_20: usize = 11;
const MASK_J_IMM_20: InstructionType = 0x01;
const BIT_OFFSET_J_IMM_INST_30_25: usize = 5;
const MASK_J_IMM_INST_30_25: InstructionType = 0x3F;
const BIT_OFFSET_J_IMM_INST_24_21: usize = 1;
const MASK_J_IMM_INST_24_21: InstructionType = 0x0F;
const BIT_OFFSET_J_IMM_0: usize = 0;
const MASK_J_IMM_0: InstructionType = 0x01;

//const INSTRUCTION_NOP: InstructionType = Opcode::OP_IMM as InstructionType;

#[derive(Default, Clone)]
#[allow(non_camel_case_types)]
enum Opcode {
    LOAD = 0b0000011,
    STORE = 0b0100011,
    MADD = 0b1000011,
    BRANCH = 0b1100011,

    LOAD_FP = 0b0000111,
    STORE_FP = 0b0100111,
    MSUB = 0b1000111,
    JALR = 0b1100111,

    custom_0 = 0b0001011,
    custom_1 = 0b0101011,
    NMSUB = 0b1001011,
    reserved_1 = 0b1101011,

    MISC_MEM = 0b0001111,
    AMO = 0b0101111,
    NMADD = 0b1001111,
    JAL = 0b1101111,

    #[default]
    OP_IMM = 0b0010011,
    OP = 0b0110011,
    OP_FP = 0b1010011,
    SYSTEM = 0b1110011,

    AUIPC = 0b0010111,
    LUI = 0b0110111,
    OP_V = 0b1010111,
    OP_VE = 0b1110111,

    OP_IMM_32 = 0b0011011,
    OP_32 = 0b0111011,
    custom_2 = 0b1011011,
    custom_3 = 0b1111011,

    reserved_2 = 0b0011111,
    reserved_3 = 0b0111111,
    reserved_4 = 0b1011111,
    reserved_5 = 0b1111111,
}

impl From<InstructionType> for Opcode {
    fn from(value: InstructionType) -> Self {
        match value {
            0b0000011 => Self::LOAD,
            0b0100011 => Self::STORE,
            0b1000011 => Self::MADD,
            0b1100011 => Self::BRANCH,

            0b0000111 => Self::LOAD_FP,
            0b0100111 => Self::STORE_FP,
            0b1000111 => Self::MSUB,
            0b1100111 => Self::JALR,

            0b0001011 => Self::custom_0,
            0b0101011 => Self::custom_1,
            0b1001011 => Self::NMSUB,
            0b1101011 => Self::reserved_1,

            0b0001111 => Self::MISC_MEM,
            0b0101111 => Self::AMO,
            0b1001111 => Self::NMADD,
            0b1101111 => Self::JAL,

            0b0010011 => Self::OP_IMM,
            0b0110011 => Self::OP,
            0b1010011 => Self::OP_FP,
            0b1110011 => Self::SYSTEM,

            0b0010111 => Self::AUIPC,
            0b0110111 => Self::LUI,
            0b1010111 => Self::OP_V,
            0b1110111 => Self::OP_VE,

            0b0011011 => Self::OP_IMM_32,
            0b0111011 => Self::OP_32,
            0b1011011 => Self::custom_2,
            0b1111011 => Self::custom_3,

            0b0011111 => Self::reserved_2,
            0b0111111 => Self::reserved_3,
            0b1011111 => Self::reserved_4,
            0b1111111 => Self::reserved_5,
            _ => {
                error!("Unknown opcode: {value:#018X}");
                panic!("Unknown opcode: {value:#018X}");
            }
        }
    }
}

#[allow(non_camel_case_types)]
enum OP_IMM_Funct3 {
    ADDI = 0b000,  // add immediate
    SLLI = 0b001,  // shift left logical immediate
    SLTI = 0b010,  //
    SLTIU = 0b011, //
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
                panic!("Unknown funct3: {value:#05b}");
            }
        }
    }
}

#[derive(Default, Clone, Debug)]
enum ExecutableInstruction {
    #[default]
    ADDI,
}

#[derive(Clone)]
struct RegisterFile {
    registers: [XLen; RISCV_NUM_OF_REGISTERS],
}

// enum RegisterName {
//     x0,
//     x1,
//     x2,
//     x3,
//     x4,
//     x5,
//     x6,
//     x7,
//     x8,
//     x9,
//     x10,
//     x11,
//     x12,
//     x13,
//     x14,
//     x15,
//     x16,
//     x17,
//     x18,
//     x19,
//     x20,
//     x21,
//     x22,
//     x23,
//     x24,
//     x25,
//     x26,
//     x27,
//     x28,
//     x29,
//     x30,
//     x31,
// }

impl RegisterFile {
    fn read(&self, offset: usize) -> XLen {
        self.registers[offset]
    }

    fn write(&mut self, offset: usize, value: XLen) {
        // register `x0` is hardwired with all bits equal to 0, so do nothing with `x0`
        if offset != 0 {
            self.registers[offset] = value;
        } else {
            trace!(
                "An attempt to write value {value} to register `x0` was silently discarded"
            );
        }
    }
}

#[derive(Default, Clone)]
struct Decoder {
    opcode: Opcode,
    source_register_1: InstructionType,
    source_register_2: InstructionType,
    destination_register: InstructionType,

    funct3: InstructionType,
    r_funct7: InstructionType,

    shamt: InstructionType,

    imm_11_0: InstructionType,
    imm_11_5: InstructionType,
    imm_4_0: InstructionType,
    imm_31_12: InstructionType,
    imm_10_5: InstructionType,
    imm_4_1: InstructionType,
    imm_10_1: InstructionType,
    imm_19_12: InstructionType,

    b_11: InstructionType,
    b_12: InstructionType,
    j_11: InstructionType,
    j_20: InstructionType,

    i_imm_inst_31: InstructionType,
    i_imm_inst_30_25: InstructionType,
    i_imm_inst_24_21: InstructionType,
    i_imm_inst_20: InstructionType,

    s_imm_inst_31: InstructionType,
    s_imm_inst_30_25: InstructionType,
    s_imm_inst_11_8: InstructionType,
    s_imm_inst_7: InstructionType,

    b_imm_inst_31: InstructionType,
    b_imm_7: InstructionType,
    b_imm_inst_30_25: InstructionType,
    b_imm_inst_11_8: InstructionType,
    b_imm_0: InstructionType,

    u_imm_31: InstructionType,
    u_imm_inst_30_20: InstructionType,
    u_imm_inst_19_12: InstructionType,
    u_imm_0: InstructionType,

    j_imm_inst_31: InstructionType,
    j_imm_inst_19_12: InstructionType,
    j_imm_20: InstructionType,
    j_imm_inst_30_25: InstructionType,
    j_imm_inst_24_21: InstructionType,
    j_imm_0: InstructionType,

    executable_instruction: ExecutableInstruction,
    need_to_write_back: bool,
}

#[derive(Clone)]
struct Context {
    pc: XLen,
    register_file: RegisterFile,
    current_instruction: XLen,
    decoder: Decoder,
    result: XLen,
}

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
        // register `x0` is hardwired with all bits equal to 0, so do nothing with `x0`
        for i in 1..RISCV_NUM_OF_REGISTERS {
            self.cpu
                .context
                .register_file
                .write(i, RISCV_X0_REGISTER_VALUE);
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
        self.cpu.write_back();

        // step PC
        self.cpu.context.pc += PC_STEP;
    }
}

fn extend_sign(value: XLen, value_mask: InstructionType) -> XLen {
    if value & ((value_mask + 1) >> 1) == 0 {
        value
    } else {
        let negative_sign_extension: XLen = XLen::MAX & !value_mask;
        value | negative_sign_extension
    }
}

impl CPU {
    fn decode(&mut self) {
        self.context.decoder.opcode = self.extract_opcode();

        match self.context.decoder.opcode {
            // TODO: implement other opcodes beside OP_IMM
            Opcode::OP => (),
            Opcode::OP_IMM => {
                self.context.decoder.source_register_1 =
                    self.extract_source_register_1();
                self.context.decoder.destination_register =
                    self.extract_destination_register();
                self.context.decoder.funct3 = self.extract_funct3();
                match self.context.decoder.funct3 & 0b11 {
                    0b01 => {
                        // shift operations
                        self.context.decoder.r_funct7 = self.extract_r_funct7();
                        self.context.decoder.shamt = self.extract_shamt();
                    }
                    _ => {
                        self.context.decoder.imm_11_0 = self.extract_imm_11_0();
                    }
                }

                match OP_IMM_Funct3::from(self.context.decoder.funct3) {
                    OP_IMM_Funct3::ADDI => {
                        self.context.decoder.executable_instruction =
                            ExecutableInstruction::ADDI;
                        self.context.decoder.need_to_write_back = true;
                    }

                    _ => (),
                }
            }

            _ => {
                error!(
                    "Unknown opcode: {:#010b}",
                    self.context.decoder.opcode.clone() as InstructionType
                );
                panic!(
                    "Unknown opcode: {:#010b}",
                    self.context.decoder.opcode.clone() as InstructionType
                );
            }
        }

        // TODO: remove unconditional decoding
        self.context.decoder.source_register_1 =
            self.extract_source_register_1();
        self.context.decoder.source_register_2 =
            self.extract_source_register_2();
        self.context.decoder.destination_register =
            self.extract_destination_register();

        self.context.decoder.funct3 = self.extract_funct3();
        self.context.decoder.r_funct7 = self.extract_r_funct7();

        self.context.decoder.imm_11_0 = self.extract_imm_11_0();
        self.context.decoder.imm_11_5 = self.extract_imm_11_5();
        self.context.decoder.imm_4_0 = self.extract_imm_4_0();
        self.context.decoder.imm_31_12 = self.extract_imm_31_12();
        self.context.decoder.imm_10_5 = self.extract_imm_10_5();
        self.context.decoder.imm_4_1 = self.extract_imm_4_1();
        self.context.decoder.imm_10_1 = self.extract_imm_10_1();
        self.context.decoder.imm_19_12 = self.extract_imm_19_12();

        self.context.decoder.b_11 = self.extract_b_11();
        self.context.decoder.b_12 = self.extract_b_12();
        self.context.decoder.j_11 = self.extract_j_11();
        self.context.decoder.j_20 = self.extract_j_20();

        self.context.decoder.i_imm_inst_31 = self.extract_i_imm_inst_31();
        self.context.decoder.i_imm_inst_30_25 = self.extract_i_imm_inst_30_25();
        self.context.decoder.i_imm_inst_24_21 = self.extract_i_imm_inst_24_21();
        self.context.decoder.i_imm_inst_20 = self.extract_i_imm_20();

        self.context.decoder.s_imm_inst_31 = self.extract_s_imm_inst_31();
        self.context.decoder.s_imm_inst_30_25 = self.extract_s_imm_inst_30_25();
        self.context.decoder.s_imm_inst_11_8 = self.extract_s_imm_inst_11_8();
        self.context.decoder.s_imm_inst_7 = self.extract_s_imm_7();

        self.context.decoder.b_imm_inst_31 = self.extract_b_imm_inst_31();
        self.context.decoder.b_imm_7 = self.extract_b_imm_7();
        self.context.decoder.b_imm_inst_30_25 = self.extract_b_imm_inst_30_25();
        self.context.decoder.b_imm_inst_11_8 = self.extract_b_imm_inst_11_8();
        self.context.decoder.b_imm_0 = self.extract_b_imm_0();

        self.context.decoder.u_imm_31 = self.extract_u_imm_31();
        self.context.decoder.u_imm_inst_30_20 = self.extract_u_imm_inst_30_20();
        self.context.decoder.u_imm_inst_19_12 = self.extract_u_imm_inst_19_12();
        self.context.decoder.u_imm_0 = self.extract_u_imm_0();

        self.context.decoder.j_imm_inst_31 = self.extract_j_imm_inst_31();
        self.context.decoder.j_imm_inst_19_12 = self.extract_j_imm_inst_19_12();
        self.context.decoder.j_imm_20 = self.extract_j_imm_20();
        self.context.decoder.j_imm_inst_30_25 = self.extract_j_imm_inst_30_25();
        self.context.decoder.j_imm_inst_24_21 = self.extract_j_imm_inst_24_21();
        self.context.decoder.j_imm_0 = self.extract_j_imm_0();
    }

    fn execute(&mut self) {
        match self.context.decoder.executable_instruction {
            ExecutableInstruction::ADDI => {
                trace!("Executing ADDI...");

                self.context.result = self
                    .context
                    .register_file
                    .read(self.context.decoder.source_register_1 as usize)
                    + self.context.decoder.imm_11_0;
            }

            _ => {
                error!(
                    "Unknown instruction: {:?}",
                    self.context.decoder.executable_instruction
                );
                panic!(
                    "Unknown instruction: {:?}",
                    self.context.decoder.executable_instruction
                );
            }
        }
    }

    fn write_back(&mut self) {
        if self.context.decoder.need_to_write_back {
            self.context.register_file.write(
                self.context.decoder.destination_register as usize,
                self.context.result,
            );
        }
    }

    fn extract_opcode(&self) -> Opcode {
        Opcode::from(
            (self.context.current_instruction >> BIT_OFFSET_OPCODE)
                & MASK_OPCODE,
        )
    }

    fn extract_source_register_1(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_SOURCE_REGISTER_1)
            & MASK_SOURCE_REGISTER_1
    }

    fn extract_source_register_2(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_SOURCE_REGISTER_2)
            & MASK_SOURCE_REGISTER_2
    }

    fn extract_destination_register(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_DESTINATION_REGISTER)
            & MASK_DESTINATION_REGISTER
    }

    fn extract_funct3(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_FUNCT3) & MASK_FUNCT3
    }

    fn extract_r_funct7(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_R_FUNCT7)
            & MASK_R_FUNCT7
    }

    fn extract_shamt(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_SHAMT) & MASK_SHAMT
    }

    // fn extend_sign_improved(
    //     immediate: XLen,
    //     immediate_bit_width: usize,
    //     immediate_bit_offset: usize,
    //     immediate_final_bit_offset: usize,
    // ) -> XLen {
    //     // immediate <<
    //     const XLEN_BIT_WIDTH: usize = size_of::<XLen>() * 8;
    //     let shift_left_amount =
    //         XLEN_BIT_WIDTH - immediate_bit_offset - immediate_bit_width;
    //     let left_shifted_imm = immediate << shift_left_amount;
    //     (left_shifted_imm as i32
    //         >> (XLEN_BIT_WIDTH - immediate_bit_width
    //             + immediate_final_bit_offset)) as XLen
    // }

    fn extract_imm_11_0(&self) -> InstructionType {
        let immediate = (self.context.current_instruction
            >> BIT_OFFSET_IMM_11_0)
            & MASK_IMM_11_0;
        // immediate [11;0] is sign extended
        extend_sign(immediate, MASK_IMM_11_0)
    }

    fn extract_imm_11_5(&self) -> InstructionType {
        let immediate = (self.context.current_instruction
            >> BIT_OFFSET_IMM_11_5)
            & MASK_IMM_11_5;
        // immediate [11;5] is sign extended
        extend_sign(immediate, MASK_IMM_11_5)
    }

    fn extract_imm_4_0(&self) -> InstructionType {
        let immediate = (self.context.current_instruction
            >> BIT_OFFSET_IMM_4_0)
            & MASK_IMM_4_0;
        // immediate [4;0] is sign extended
        extend_sign(immediate, MASK_IMM_4_0)
    }

    fn extract_imm_31_12(&self) -> InstructionType {
        let immediate = (self.context.current_instruction
            >> BIT_OFFSET_IMM_31_12)
            & MASK_IMM_31_12;
        // immediate [31;12] is sign extended
        extend_sign(immediate, MASK_IMM_31_12)
    }

    fn extract_imm_10_5(&self) -> InstructionType {
        let immediate = (self.context.current_instruction
            >> BIT_OFFSET_IMM_10_5)
            & MASK_IMM_10_5;
        // immediate [10;5] is sign extended
        extend_sign(immediate, MASK_IMM_10_5)
    }

    fn extract_imm_4_1(&self) -> InstructionType {
        let immediate = (self.context.current_instruction
            >> BIT_OFFSET_IMM_4_1)
            & MASK_IMM_4_1;
        // immediate [4;1] is sign extended
        extend_sign(immediate, MASK_IMM_4_1)
    }

    fn extract_imm_10_1(&self) -> InstructionType {
        let immediate = (self.context.current_instruction
            >> BIT_OFFSET_IMM_10_1)
            & MASK_IMM_10_1;
        // immediate [10;1] is sign extended
        extend_sign(immediate, MASK_IMM_10_1)
    }

    fn extract_imm_19_12(&self) -> InstructionType {
        let immediate = (self.context.current_instruction
            >> BIT_OFFSET_IMM_19_12)
            & MASK_IMM_19_12;
        // immediate [19;12] is sign extended
        extend_sign(immediate, MASK_IMM_19_12)
    }

    fn extract_b_11(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_B_11) & MASK_B_11
    }

    fn extract_b_12(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_B_12) & MASK_B_12
    }

    fn extract_j_11(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_J_11) & MASK_J_11
    }

    fn extract_j_20(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_J_20) & MASK_J_20
    }

    fn extract_i_imm_inst_31(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_I_IMM_INST_31)
            & MASK_I_IMM_INST_31
    }

    fn extract_i_imm_inst_30_25(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_I_IMM_INST_30_25)
            & MASK_I_IMM_INST_30_25
    }

    fn extract_i_imm_inst_24_21(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_I_IMM_INST_24_21)
            & MASK_I_IMM_INST_24_21
    }

    fn extract_i_imm_20(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_I_IMM_20)
            & MASK_I_IMM_20
    }

    fn extract_s_imm_inst_31(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_S_IMM_INST_31)
            & MASK_S_IMM_INST_31
    }

    fn extract_s_imm_inst_30_25(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_S_IMM_INST_30_25)
            & MASK_S_IMM_INST_30_25
    }

    fn extract_s_imm_inst_11_8(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_S_IMM_INST_11_8)
            & MASK_S_IMM_INST_11_8
    }

    fn extract_s_imm_7(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_S_IMM_7) & MASK_S_IMM_7
    }

    fn extract_b_imm_inst_31(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_B_IMM_INST_31)
            & MASK_B_IMM_INST_31
    }

    fn extract_b_imm_7(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_B_IMM_7) & MASK_B_IMM_7
    }

    fn extract_b_imm_inst_30_25(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_B_IMM_INST_30_25)
            & MASK_B_IMM_INST_30_25
    }

    fn extract_b_imm_inst_11_8(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_B_IMM_INST_11_8)
            & MASK_B_IMM_INST_11_8
    }

    fn extract_b_imm_0(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_B_IMM_0) & MASK_B_IMM_0
    }

    fn extract_u_imm_31(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_U_IMM_31)
            & MASK_U_IMM_31
    }

    fn extract_u_imm_inst_30_20(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_U_IMM_INST_30_20)
            & MASK_U_IMM_INST_30_20
    }

    fn extract_u_imm_inst_19_12(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_U_IMM_INST_19_12)
            & MASK_U_IMM_INST_19_12
    }

    fn extract_u_imm_0(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_U_IMM_0) & MASK_U_IMM_0
    }

    fn extract_j_imm_inst_31(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_J_IMM_INST_31)
            & MASK_J_IMM_INST_31
    }

    fn extract_j_imm_inst_19_12(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_J_IMM_INST_19_12)
            & MASK_J_IMM_INST_19_12
    }

    fn extract_j_imm_20(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_J_IMM_20)
            & MASK_J_IMM_20
    }

    fn extract_j_imm_inst_30_25(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_J_IMM_INST_30_25)
            & MASK_J_IMM_INST_30_25
    }

    fn extract_j_imm_inst_24_21(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_J_IMM_INST_24_21)
            & MASK_J_IMM_INST_24_21
    }

    fn extract_j_imm_0(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_J_IMM_0) & MASK_J_IMM_0
    }
}

fn init_logger() {
    let default_log_level = "info";
    // set default log level (unless specified with RUST_LOG environment variable)
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or(default_log_level),
    )
    .init();
}

static LICENSE_TEXT: &str = "
Copyright 2026 Ruslan Vostretsov

Licensed under either of:

  * MIT License (LICENSE-MIT)
    https://opensource.org/licenses/MIT

  * Apache License, Version 2.0 (LICENSE-APACHE)
    https://www.apache.org/licenses/LICENSE-2.0

at your option.
";

fn parse_args() {
    // TODO: add `--license` argument to print license
    let args: Vec<String> = env::args().collect();
    //
}

// #[derive(Deserialize)]
// struct MemoryCfg {
//     size_bytes: Option<usize>,
// }

// #[derive(Deserialize)]
// struct SocConfig {
//     soc: Option<MemoryCfg>,
// }

fn configure() {
    // const DEFAULT_CONFIG_PATH: &str = "conf/soc.toml";
    // let config_path = DEFAULT_CONFIG_PATH;
    // let file_content = fs::read_to_string(config_path).unwrap().as_str();
    // let config: SocConfig = toml::from_str(file_content).unwrap();
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
                    registers: [RISCV_X0_REGISTER_VALUE;
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
