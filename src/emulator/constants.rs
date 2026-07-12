pub const PERIODICAL_HOOKS_PERIOD_MS: u64 = 10;

// #[derive(Default)]
pub(super) enum EmulatorEvent {
    // #[default]
    Start,
    Halt,
    Stop,
}
