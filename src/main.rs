use log::{debug, error, info, trace, warn};

/// architechtural defines
type XLen = u32;
// const IALIGN: i32 = 32;
// const ILEN: i32 = IALIGN * 1;
type InstructionType = u32;

// default constants
const PC_DEFAULT_VALUE: XLen = 0;
const PC_STEP: XLen = 4;
const MEM_DEFAULT_VALUE: u8 = 0;
const MEM_SIZE: usize = 100;
const UNDEFINED_INSTRUCTION: XLen = 0;

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

fn fetch(soc: &mut SoC) {
    let bytes = soc.memory.byte_array[soc.cpu.context.pc as usize
        ..soc.cpu.context.pc as usize + PC_STEP as usize]
        .try_into()
        .unwrap();
    // NOTICE: so far emulator only supports low endian
    let instruction = InstructionType::from_le_bytes(bytes);
    soc.cpu.context.current_instruction = instruction;
}

fn decode(soc: &SoC) {
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

fn step(soc: &mut SoC) {
    fetch(soc);
    decode(soc);
    execute();
    write_back();
    soc.cpu.context.pc += PC_STEP;

    trace!("Current PC: {0}", soc.cpu.context.pc);
}

fn main() {
    let mut soc: SoC = SoC {
        cpu: CPU {
            context: Context {
                pc: PC_DEFAULT_VALUE,
                current_instruction: UNDEFINED_INSTRUCTION,
            },
        },
        memory: Memory {
            byte_array: [MEM_DEFAULT_VALUE; MEM_SIZE].to_vec(),
        },
    };

    init_logger();

    loop {
        step(&mut soc);
    }
}
