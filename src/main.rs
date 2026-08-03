#![forbid(unsafe_code)]

use log::{error, info};
use riscvmulator_emulator::init::{emulator_create, emulator_prepare_loop};
use riscvmulator_emulator::{EmulatorEvent, ExitCode};

fn main() -> Result<(), ExitCode> {
    let custom_config = true;
    let initialize_logger = true;
    // take test path from config/envvar/CLI arg if was given
    let test_path = None;
    let test_cfg_file_name = None;

    let mut emulator = emulator_create(
        custom_config,
        initialize_logger,
        test_path,
        test_cfg_file_name,
    )?;

    // so far use `Ctrl+C` to send Stop events
    // TODO: this is being used until true power management is implemented
    let shut_down_event_sender = emulator.event_sender();
    ctrlc::set_handler(move || {
        info!("`Ctrl+C` was assereted, shutting down...");
        shut_down_event_sender
            .send(EmulatorEvent::Stop(ExitCode::Success))
            .unwrap_or_else(|err| {
                error!("Failed to send event on `Ctrl+C`: {err}");
            });
    })
    .unwrap_or_else(|err| {
        error!("Failed to set `Ctrl+C` handler: {err}");
    });

    emulator_prepare_loop(&mut emulator)?;

    emulator.run()
}
