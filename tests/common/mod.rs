use riscvmulator::{
    Bus, ExitCode, HookAction,
    config::{Config, ConfigError},
    emulator::Emulator,
    init::create_emulator,
};

use crate::peripherals::htif::HTIF_SUCCESS;

fn test_setup() {
    //
}

fn test_teardown() {
    //
}

pub fn config_creator(
    test_path: Option<&'static str>,
) -> Result<Config, ConfigError> {
    let mut emu_config = Config::default();
    if let Some(path) = test_path {
        emu_config.set_input_elf_path(path.to_string());
    }

    Ok(emu_config)
}

pub fn run_test(test_path: &'static str) {
    test_setup();

    let mut emulator = create_emulator(config_creator, Some(test_path))
        .expect("failed to create emulator");

    // assert_eq!(emulator.run(), Ok(()));

    let emu_config = config_creator(Some(test_path)).unwrap();
    // emu_config.set_input_elf_path(test_path.to_string());

    // let mut emulator = Emulator::build_from_config(&emu_config)
    //     .expect("failed to init emulator");

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

    assert_eq!(emulator.run(), Ok(()));

    test_teardown();
}
