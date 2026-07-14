use crate::globals::exit_codes::ExitCode;

pub const PERIODICAL_TASKS_PERIOD_MS: u64 = 200;

#[derive(Debug)]
pub enum EmulatorEvent {
    Start,
    Halt,
    Stop(ExitCode),
    InteractiveMode,
}

// TODO: find a way to implement `exit_code()` for foreign type
// impl<Event: EmulatorEvent> SendError<Event> {
//     //
// }
