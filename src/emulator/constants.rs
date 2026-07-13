use crate::globals::exit_codes::ExitCode;

pub const PERIODICAL_TASKS_PERIOD_MS: u64 = 10;

#[derive(Debug)]
pub(super) enum EmulatorEvent {
    // #[default]
    Start,
    Halt,
    Stop(ExitCode),
}
