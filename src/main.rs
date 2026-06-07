use riscvmulator::elf_loader::load_elf;
use riscvmulator::{init, soc::SoC};

fn main() {
    let config = init();

    let mut soc: SoC = SoC::new(&config.soc);

    soc.reset(&config.soc);

    // dbg!("{:?}", config.emulator.input_file.clone());
    load_elf(config.emulator.input_file.unwrap().into(), &mut soc);

    soc.ready_for_testing();

    // TODO: HALT condition
    loop {
        soc.step();
    }
}
