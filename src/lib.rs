// modules includes
pub mod config;
mod constants;
pub mod elf_loader;
mod logger;
mod parse_args;
pub mod soc;
mod utils;

use config::Config;
use logger::init_logger;
use parse_args::{CLI_Args, parse_args};

// addr: {address:#018X?}

// std lib imports

// external libs imports
// use log::{debug, error, info, trace, warn};

// current project imports

pub fn init() -> Config {
    init_logger();
    let args = parse_args();
    let config = Config::build(&args.clone());

    config
}

pub fn init_test() {
    // configure();
}
