use log::{debug, error, info, trace, warn};
use std::env;
use std::mem::size_of;

/// architechtural defines
type XLen = u32;
// const IALIGN: i32 = 32;
// const ILEN: i32 = IALIGN * 1;
type InstructionType = u32;

// default constants
const PC_RESET_VALUE: XLen = 0x00000000;
const PC_STEP: XLen = 0x00000004;

const INITIAL_INSTRUCTION: XLen = 0x00000000;

const MEM_DEFAULT_VALUE: u8 = 0x00;
const MEM_SIZE: usize = 65536; // placeholder value, to be configured later

struct Context {
    pc: XLen,
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
            "Fetched instruction: {:#010X?}",
            self.cpu.context.current_instruction
        );
    }

    fn reset(&mut self) {
        self.cpu.context.pc = PC_RESET_VALUE;
        self.cpu.context.current_instruction = INITIAL_INSTRUCTION;
        self.memory.byte_array = [MEM_DEFAULT_VALUE; MEM_SIZE].to_vec();
    }

    fn step(&mut self) {
        trace!("Current PC: {}", self.cpu.context.pc);

        self.fetch();
        self.cpu.decode();
        execute();
        write_back();

        // step PC
        self.cpu.context.pc += PC_STEP;
    }
}

fn extract_opcode(instruction: InstructionType) -> u32 {
    instruction & 0x7F
}

impl CPU {
    fn decode(&self) {
        let opcode = extract_opcode(self.context.current_instruction);
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
