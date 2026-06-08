// modules includes
pub mod config;
mod constants;
pub mod elf_loader;
mod logger;
mod parse_args;
pub mod soc;
mod utils;

// external libs imports
// use log::{debug, error, info, trace, warn};

// current project imports
use config::Config;
use logger::init_logger;
use parse_args::{CLI_Args, parse_args};

pub fn init() -> Config {
    init_logger();
    let args = parse_args();
    let config = Config::build(&args.clone());

    config
}
