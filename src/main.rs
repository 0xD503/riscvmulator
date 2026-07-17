use riscvmulator::ExitCode;
use riscvmulator::init::{config_creator, create_emulator};

fn main() -> Result<(), ExitCode> {
    // take test path from config/envvar/CLI arg if was given
    let test_path = None;
    let mut emulator = create_emulator(config_creator, test_path)?;

    emulator.run()
}
