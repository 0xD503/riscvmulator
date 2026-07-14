// use std::process::Termination;

#[derive(Debug, PartialEq)]
pub enum ExitCode {
    Success = 0,
    Error = 1,

    // exit codes from application/protocol-specific exit-code space
    ConfigRead = 90,
    ConfigBuild = 91,
    ConfigValidation = 92,

    ElfRead = 93,
    ElfParseHeader = 94,
    SegmentsLoad = 95,
    StringTable = 96,
    //
    // DeviceReadError = 97,
    // DeviceWriteError = 98,
    //
    // HookStopSignal = 99,
    //
    EmulatorModePrepFailure = 110,
}

// impl PartialOrd for ExitCode {
//     //
// }

// impl Termination for ExitCode {
//     fn report(self) -> std::process::ExitCode {
//         match self {
//             ExitCode::Success => std::process::ExitCode::SUCCESS,
//             ExitCode::Error => std::process::ExitCode::FAILURE,

//             _ => std::process::ExitCode::FAILURE,
//         }
//     }
// }
