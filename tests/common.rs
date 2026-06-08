use riscvmulator::config::SocCfg;
use riscvmulator::elf_loader::load_elf;
use riscvmulator::soc::SoC;

fn init_test() {
    //
}

pub fn run_test(test_path: &'static str) {
    init_test();

    let cfg: SocCfg = Default::default();
    let mut soc: SoC = SoC::new(&cfg);

    soc.reset(&cfg);

    load_elf(test_path.into(), &mut soc);

    // TODO: conditional compilation for test facilities
    soc.ready_for_testing();

    // TODO: HALT condition
    loop {
        soc.step();
    }
}
