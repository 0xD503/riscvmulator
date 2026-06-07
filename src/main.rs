use riscvmulator::elf_loader::load_elf;
use riscvmulator::{init, soc::SoC};

fn main() {
    let (args, config) = init();

    let mut soc: SoC = SoC::new(config.soc.unwrap_or_default());

    soc.reset();

    load_elf(args.input_file.into(), &mut soc);

    soc.ready_for_testing();

    // TODO: HALT condition
    loop {
        soc.step();
    }
}
