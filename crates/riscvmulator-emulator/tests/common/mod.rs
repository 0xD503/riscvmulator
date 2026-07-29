use std::path::{Path, PathBuf};

use riscvmulator_emulator::{
    Addressable, ExitCode, HookAction, PeriodicalTaskArgs, Word,
    init::{emulator_create, emulator_prepare_loop},
};

use crate::peripherals::htif::HTIF_SUCCESS;

fn test_setup() {
    //
}

fn test_teardown() {
    //
}

pub fn run_test(test_path: impl AsRef<Path>) {
    test_setup();

    let mut path =
        PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../../"));
    path.push(test_path);

    let custom_config = false;
    let initialize_logger = false;
    let mut emulator =
        emulator_create(custom_config, initialize_logger, Some(path))
            .expect("failed to create emulator");

    let elf_data = emulator_prepare_loop(&mut emulator)
        .expect("expected successful ELF load/read")
        .expect("expected non-interactive loop");

    // add HTIF memory address spy/watcher
    let tohost_addr = elf_data.tohost_addr;
    let htif_spy = move |args: &PeriodicalTaskArgs| {
        let mut dest = [0u8; size_of::<Word>()];

        let mem_read_result = args.bus.read(tohost_addr, &mut dest);

        if mem_read_result.is_ok() {
            let dest = Word::from_le_bytes(dest);
            match dest {
                HTIF_SUCCESS => HookAction::Stop(ExitCode::Success),
                code if code != 0 => HookAction::Stop(ExitCode::Error),

                _ => HookAction::None,
            }
        } else {
            HookAction::None
        }
    };
    emulator.add_periodic_task(Box::new(htif_spy));

    assert_eq!(emulator.run(), Ok(()));

    test_teardown();
}
