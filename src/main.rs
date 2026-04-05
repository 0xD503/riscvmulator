use log::{debug, error, info, trace, warn};
use std::mem::size_of;

/// architechtural defines
type XLen = u32;
// const IALIGN: i32 = 32;
// const ILEN: i32 = IALIGN * 1;
type InstructionType = u32;

// default constants
const PC_DEFAULT_VALUE: XLen = 0x00000000;
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
    fn reset(&mut self) {
        self.cpu.context.pc = PC_DEFAULT_VALUE;
        self.cpu.context.current_instruction = INITIAL_INSTRUCTION;
        self.memory.byte_array = [MEM_DEFAULT_VALUE; MEM_SIZE].to_vec();
    }

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

    fn step(&mut self) {
        trace!("Current PC: {}", self.cpu.context.pc);

        self.fetch();
        decode();
        execute();
        write_back();

        // step PC
        self.cpu.context.pc += PC_STEP;
    }
}

fn decode() {
    //
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

fn main() {
    init_logger();

    let mut soc: SoC = SoC {
        cpu: CPU {
            context: Context {
                pc: PC_DEFAULT_VALUE,
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
