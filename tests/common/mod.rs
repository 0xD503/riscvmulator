use riscvmulator::{SoC, elf_loader::load_elf, init};

pub fn run_test(test_path: &'static str) {
    init();

    let mut soc: SoC = SoC::new();

    soc.reset();

    load_elf(test_path.into(), &mut soc);

    soc.ready_for_testing();

    // TODO: HALT condition
    loop {
        soc.step();
    }
}
