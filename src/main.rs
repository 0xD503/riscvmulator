use log::error;

use riscvmulator::emulator::Emulator;
use riscvmulator::init::init;

fn main() {
    let emu_config = init().unwrap_or_else(|err| {
        error!("{err}");
        std::process::exit(err.exit_code());
    });

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
    }

    emulator.ready_for_testing();

    emulator.run();
}
