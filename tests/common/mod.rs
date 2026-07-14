use riscvmulator::{
    Bus, ExitCode, HookAction, config::Config, emulator::Emulator,
};

use crate::peripherals::htif::HTIF_SUCCESS;

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

    let mut emulator = Emulator::build_from_config(&emu_config)
        .expect("failed to init emulator");

    let elf_data = emulator
        .load_elf(emu_config.input_elf_path().unwrap())
        .expect("failed to load ELF file");
    emulator
        .loop_prepare_non_interactive()
        .expect("failed to prepare emulator to non-interactive mode");

    let tohost_addr = elf_data.tohost_addr;
    emulator.add_periodical_task(Box::new(move |args| {
        match args.bus.read_u32(tohost_addr) {
            Ok(HTIF_SUCCESS) => HookAction::Stop(ExitCode::Success),
            // riscv-tests HTIF returns number of failed test case on failure
            Ok(code) if code != 0 => HookAction::Stop(ExitCode::Error),

            _ => HookAction::None,
        }
    }));

    assert_eq!(emulator.run(), ExitCode::Success);

    test_teardown();
}
