use log::error;

use crate::ExitCode;

use super::config::{Config, ConfigError};
use super::logger::init_logger;
use super::parse_args::parse_args;

pub fn init() -> Result<Config, ConfigError> {
    let args = parse_args();
    let config = Config::build(&args);
    init_logger(&config);

    config
}

pub fn init_emulator() -> Result<(), ExitCode> {
    let emu_config = init().map_err(|err| {
        //
    };

    let mut emulator =
        Emulator::build_from_config(&emu_config).unwrap_or_else(|err| {
            error!("{err}");
            std::process::exit(err.exit_code());
        });

    if let Some(elf_path) = emu_config.input_elf_path() {
        if let Err(err) = emulator.load_elf(elf_path) {
            match err {
                _ => {
                    error!("{err}");
                    std::process::exit(err.exit_code());
                }
            }
        }
        if let Err(_) = emulator.loop_prepare_non_interactive() {
            error!("failed to prepare emulator to non-interactive mode");
            // TODO: use `err.exit_code()` when impelment it
            std::process::exit(ExitCode::EmulatorModePrepFailure as i32);
        }
    } else {
        if let Err(_) = emulator.loop_prepare_interactive() {
            error!("failed to prepare emulator to interactive mode");
            // TODO: use `err.exit_code()` when impelment it
            std::process::exit(ExitCode::EmulatorModePrepFailure as i32);
        }
    }
}
