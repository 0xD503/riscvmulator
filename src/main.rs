use riscvmulator::ExitCode;
use riscvmulator::init::{emulator_create, emulator_prepare_loop};

fn main() -> Result<(), ExitCode> {
    let custom_config = true;
    let initialize_logger = true;
    // take test path from config/envvar/CLI arg if was given
    let test_path = None;

    let mut emulator =
        emulator_create(custom_config, initialize_logger, test_path)?;
    emulator_prepare_loop(&mut emulator)?;

    emulator.run()
}
