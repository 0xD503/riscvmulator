use log::{error, info};

use riscvmulator::elf_loader::load_elf;
use riscvmulator::{init, soc::SoC};

fn main() {
    let config = init().unwrap_or_else(|errors| {
        error!("{errors}");
        std::process::exit(errors.exit_code());
    });

    let mut soc: SoC = SoC::new(&config.soc);
    soc.reset(&config.soc);
    info!("SoC was reset");

    if let Some(in_file) = config.emulator.input_file {
        load_elf(in_file.into(), &mut soc);
    } else {
        info!("running in interactive mode");
        todo!("interactive mode");
    }

    soc.ready_for_testing();

    // TODO: HALT condition
    loop {
        soc.step();
    }
}
