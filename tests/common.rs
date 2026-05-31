use riscvmulator::soc::SoC;
use riscvmulator::{elf_loader::load_elf, init_test};

pub fn run_test(test_path: &'static str) {
    init_test();

    let mut soc: SoC = SoC::new();

    soc.reset();

    load_elf(test_path.into(), &mut soc);

    // TODO: conditional compilation for test facilities
    soc.ready_for_testing();

    // TODO: HALT condition
    loop {
        soc.step();
    }
}
