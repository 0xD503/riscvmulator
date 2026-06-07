// modules includes
pub mod config;
mod constants;
pub mod elf_loader;
mod logger;
mod parse_args;
pub mod soc;
mod utils;

use config::{Config, get_config};
use logger::init_logger;
use parse_args::{CLI_Args, parse_args};

// addr: {address:#018X?}

// std lib imports

// external libs imports
// use log::{debug, error, info, trace, warn};

// current project imports

pub fn init() -> (CLI_Args, Config) {
    init_logger();
    let args = parse_args();
    let config = get_config(args.config_file.clone());

    (args, config)
}

pub fn init_test() {
    // configure();
}
