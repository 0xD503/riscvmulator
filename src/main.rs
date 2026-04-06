use log::{debug, error, info, trace, warn};
use std::convert::From;
use std::env;
use std::mem::size_of;

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

// const BIT_OFFSET_: usize = ;
// const MASK_: InstructionType = 0x;

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

const INSTRUCTION_NOP: InstructionType = Opcode::OP_IMM as InstructionType;

impl From<u32> for Opcode {
    fn from(value: u32) -> Self {
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

// impl Opcode {
// fn from_u32(self, value: u32) -> Self {
//     match value {
//         0b0000011 => Self::LOAD,
//         _ => {
//             error!("Unknown opcode: {value:#018X}");
//             panic!("Unknown opcode: {value:#018X}");
//         }
//     }
// }
// }

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
        }
    }
}

struct Context {
    pc: XLen,
    register_file: RegisterFile,
    current_instruction: XLen,
}

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

        self.memory.byte_array = [MEM_DEFAULT_VALUE; MEM_SIZE].to_vec();
        // fill memory with NOP's
        for i in (0..MEM_SIZE).step_by(PC_STEP as usize) {
            self.memory.byte_array[i] = Opcode::OP_IMM as u8;
        }
    }

    fn step(&mut self) {
        trace!("Current PC: {:#018X}", self.cpu.context.pc);

        self.fetch();
        self.cpu.decode();
        execute();
        write_back();

        // step PC
        self.cpu.context.pc += PC_STEP;
    }
}

impl CPU {
    fn decode(&self) {
        let opcode = self.extract_opcode();

        match Opcode::from(opcode) {
            Opcode::OP => (),
            Opcode::OP_IMM => (),

            _ => {
                error!("Unknown opcode: {opcode}");
                panic!("Unknown opcode: {opcode}")
            }
        }

        let source_register_1 = self.extract_source_register_1();
        let source_register_2 = self.extract_source_register_2();
        let destination_register = self.extract_destination_register();

        let funct3 = self.extract_funct3();
        let r_funct7 = self.extract_r_funct7();

        let imm_11_0 = self.extract_imm_11_0();
        let imm_11_5 = self.extract_imm_11_5();
        let imm_4_0 = self.extract_imm_4_0();
        let imm_31_12 = self.extract_imm_31_12();
        let imm_10_5 = self.extract_imm_10_5();
        let imm_4_1 = self.extract_imm_4_1();
        let imm_10_1 = self.extract_imm_10_1();
        let imm_19_12 = self.extract_imm_19_12();

        let b_11 = self.extract_b_11();
        let b_12 = self.extract_b_12();
        let j_11 = self.extract_j_11();
        let j_20 = self.extract_j_20();

        let i_imm_inst_31 = self.extract_i_imm_inst_31();
        let i_imm_inst_30_25 = self.extract_i_imm_inst_30_25();
        let i_imm_inst_24_21 = self.extract_i_imm_inst_24_21();
        let i_imm_inst_20 = self.extract_i_imm_20();

        let s_imm_inst_31 = self.extract_s_imm_inst_31();
        let s_imm_inst_30_25 = self.extract_s_imm_inst_30_25();
        let s_imm_inst_11_8 = self.extract_s_imm_inst_11_8();
        let s_imm_inst_7 = self.extract_s_imm_7();

        let b_imm_inst_31 = self.extract_b_imm_inst_31();
        let b_imm_7 = self.extract_b_imm_7();
        let b_imm_inst_30_25 = self.extract_b_imm_inst_30_25();
        let b_imm_inst_11_8 = self.extract_b_imm_inst_11_8();
        let b_imm_0 = self.extract_b_imm_0();

        let u_imm_31 = self.extract_u_imm_31();
        let u_imm_inst_30_20 = self.extract_u_imm_inst_30_20();
        let u_imm_inst_19_12 = self.extract_u_imm_inst_19_12();
        let u_imm_0 = self.extract_u_imm_0();

        let j_imm_inst_31 = self.extract_j_imm_inst_31();
        let j_imm_inst_19_12 = self.extract_j_imm_inst_19_12();
        let j_imm_20 = self.extract_j_imm_20();
        let j_imm_inst_30_25 = self.extract_j_imm_inst_30_25();
        let j_imm_inst_24_21 = self.extract_j_imm_inst_24_21();
        let j_imm_0 = self.extract_j_imm_0();
    }

    fn extract_opcode(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_OPCODE) & MASK_OPCODE
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

    fn extract_imm_11_0(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_IMM_11_0)
            & MASK_IMM_11_0
    }

    fn extract_imm_11_5(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_IMM_11_5)
            & MASK_IMM_11_5
    }

    fn extract_imm_4_0(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_IMM_4_0) & MASK_IMM_4_0
    }

    fn extract_imm_31_12(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_IMM_31_12)
            & MASK_IMM_31_12
    }

    fn extract_imm_10_5(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_IMM_10_5)
            & MASK_IMM_10_5
    }

    fn extract_imm_4_1(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_IMM_4_1) & MASK_IMM_4_1
    }

    fn extract_imm_10_1(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_IMM_10_1)
            & MASK_IMM_10_1
    }

    fn extract_imm_19_12(&self) -> InstructionType {
        (self.context.current_instruction >> BIT_OFFSET_IMM_19_12)
            & MASK_IMM_19_12
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

fn execute() {
    //
}

fn write_back() {
    //
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

fn main() {
    init_logger();
    parse_args();

    let mut soc: SoC = SoC {
        cpu: CPU {
            context: Context {
                pc: PC_RESET_VALUE,
                register_file: RegisterFile {
                    registers: [RISCV_X0_REGISTER_VALUE;
                        RISCV_NUM_OF_REGISTERS],
                },
                current_instruction: INITIAL_INSTRUCTION,
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
