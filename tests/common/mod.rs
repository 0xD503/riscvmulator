use riscvmulator::{
    Bus, PeriodicalTaskArgs, config::Config, emulator::Emulator,
};

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

    let elf_data = emulator
        .load_elf(emu_config.input_elf_path().unwrap())
        .expect("failed to load ELF file");

    let tohost_addr = elf_data.tohost_addr;
    emulator.add_periodical_task(Box::new(move |args| {
        match args.bus.read_u32(tohost_addr) {
            Ok(0 | 1) => Some(0),
            Ok(code) => Some(code as i32),
            _ => None,
        }
    }));

    emulator.ready_for_testing();

    emulator.run();

    test_teardown();
}
