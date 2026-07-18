use riscvmulator::{Bus, ExitCode, HookAction, init::emulator_create};

use crate::peripherals::htif::HTIF_SUCCESS;

fn test_setup() {
    //
}

fn test_teardown() {
    //
}

pub fn run_test(test_path: &'static str) {
    test_setup();

    let custom_config = false;
    let initialize_logger = false;
    let mut emulator =
        emulator_create(custom_config, initialize_logger, Some(test_path))
            .expect("failed to create emulator");

    let elf_data = emulator
        .load_elf(
            emulator
                .input_file()
                .expect("expected input ELF file as an integration test"),
        )
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
