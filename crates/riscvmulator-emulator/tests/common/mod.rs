use std::path::{Path, PathBuf};

use log::info;
use riscvmulator_emulator::{
    Addressable, ExitCode, HookAction, PeriodicTaskArgs, Word,
    init::{emulator_create, emulator_prepare_loop},
};

use crate::peripherals::htif::HTIF_SUCCESS;

fn test_setup() {
    //
}

fn test_teardown() {
    //
}

pub fn run_test(file_name: impl AsRef<Path>, cfg_file_name: impl AsRef<Path>) {
    test_setup();

    const REPO_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
    const CRATE_ROOT: &str = env!("CARGO_MANIFEST_DIR");

    let mut path_to_test = PathBuf::from(REPO_ROOT);
    path_to_test.push(file_name);

    let mut path_to_cfg = PathBuf::from(CRATE_ROOT);
    path_to_cfg.push("tests/cfg");
    path_to_cfg.push(cfg_file_name);

    // dbg!(format!("GREPME Current test path: {:?}", path_to_test));
    // dbg!(format!("GREPME Current cfg path: {:?}", path_to_cfg));

    let custom_config = false;
    let initialize_logger = false;
    let mut emulator = emulator_create(
        custom_config,
        initialize_logger,
        Some(path_to_test),
        Some(path_to_cfg),
    )
    .expect("failed to create emulator");

    let elf_data = emulator_prepare_loop(&mut emulator)
        .expect("expected successful ELF load/read")
        .expect("expected non-interactive loop");

    // add HTIF memory address spy/watcher
    let tohost_addr = elf_data.tohost_addr;
    info!("HTIF tohost address: {tohost_addr:#018X?}");
    let htif_spy = move |args: &PeriodicTaskArgs| {
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
