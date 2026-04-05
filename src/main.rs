use env_logger::Builder;
use log::{debug, error, info, trace, warn};
//use std::env::{set_var, var};

//#[derive(Debug)]
struct CPU {
    pc: u32,
}

struct Memory {
    byte_array: Vec<u8>,
}

struct SoC {
    cpu: CPU,
    memory: Memory,
}

fn fetch() {
    //
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
    // set RUST_LOG environment variable if it's not set
    // if var("RUST_LOG").is_err() {
    //     std::env::set_var("RUST_LOG", default_log_level);
    // }
    // set default log level (unless specified with RUST_LOG environment variable)
    Builder::from_env(env_logger::Env::default().default_filter_or(default_log_level)).init();
}

fn step(soc: &mut SoC) {
    fetch();
    decode();
    execute();
    write_back();
    soc.cpu.pc += 4;
    trace!("Current PC: {0}", soc.cpu.pc);
}

fn main() {
    const PC_DEFAULT_VALUE: u32 = 0;
    const MEM_DEFAULT_VALUE: u8 = 0;
    const MEM_SIZE: usize = 100;

    let mut soc: SoC = SoC {
        cpu: CPU {
            pc: PC_DEFAULT_VALUE,
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
