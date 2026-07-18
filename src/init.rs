use log::error;

use crate::ExitCode;
use crate::emulator::Emulator;

use super::config::{Config, ConfigError};
use super::logger::init_logger;
use super::parse_args::parse_args;

fn config_creator(
    custom_config: bool,
    initialize_logger: bool,
    test_path: Option<&'static str>,
) -> Result<Config, ConfigError> {
    let mut config;
    if custom_config {
        let args = parse_args();
        config = Config::build(&args);
    } else {
        config = Ok(Config::default());
    }

    if initialize_logger {
        init_logger(&config);
    }

    dbg!("AHAHA1111");
    if let Some(path) = test_path {
        dbg!("AHAHA222");
        if let Ok(cfg) = config.as_mut() {
            dbg!("AHAHA3333");
            cfg.set_input_elf_path(path.to_string());
            dbg!("AHAHA4444");
        }
    }

    config
}

pub fn emulator_create(
    custom_config: bool,
    initialize_logger: bool,
    test_path: Option<&'static str>,
) -> Result<Emulator, ExitCode> {
    let emu_config =
        config_creator(custom_config, initialize_logger, test_path).map_err(
            |err| {
                error!("{err}");
                err.exit_code()
            },
        )?;

    let emulator = Emulator::build_from_config(&emu_config).map_err(|err| {
        error!("{err}");
        err.exit_code()
    })?;

    Ok(emulator)
}

pub fn emulator_prepare_loop(emulator: &mut Emulator) -> Result<(), ExitCode> {
    if let Some(elf_path) = emulator.input_file() {
        if let Err(err) = emulator.load_elf(elf_path) {
            match err {
                _ => {
                    error!("{err}");
                    return Err(From::from(err.exit_code()));
                }
            }
        }
        if let Err(_) = emulator.loop_prepare_non_interactive() {
            error!("failed to prepare emulator to non-interactive mode");
            // TODO: use `err.exit_code()` when impelment it
            return Err(ExitCode::EmulatorModePrepFailure);
        }
    } else {
        if let Err(_) = emulator.loop_prepare_interactive() {
            error!("failed to prepare emulator to interactive mode");
            // TODO: use `err.exit_code()` when impelment it
            return Err(ExitCode::EmulatorModePrepFailure);
        }
    }

    Ok(())
}
