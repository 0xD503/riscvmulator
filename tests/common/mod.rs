use riscvmulator::{config::Config, emulator::Emulator};

fn test_setup() {
    //
}

fn test_teardown() {
    //
}

pub fn run_test(test_path: &'static str) {
    test_setup();

    let mut emu_config = Config::default();
    emu_config.set_input_elf_path(test_path.to_string());
    // emu_config.

    let mut emulator = Emulator::build_from_config(&emu_config)
        .expect("failed to init emulator");

    emulator
        .load_elf(emu_config.input_elf_path().unwrap())
        .expect("failed to load ELF file");

    emulator.ready_for_testing();

    emulator.run();

    test_teardown();
}
