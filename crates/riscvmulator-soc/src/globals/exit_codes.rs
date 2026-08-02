use std::process::Termination;

use log::error;

#[derive(Debug, PartialEq)]
pub enum ExitCode {
    Success = 0,
    Error = 1,

    // exit codes from application/protocol-specific exit-code space
    ConfigRead = 90,
    ConfigBuild = 91,
    ConfigValidation = 92,
    ConfigPathNotGiven = 93,

    ElfRead = 94,
    ElfParseHeader = 95,
    SegmentsLoad = 96,
    StringTable = 97,

    NoInputFileProvided = 98,
    ThreadPoisoned = 99,
    //
    // HookStopSignal = ,
    //
    EmulatorModePrepFailure = 110,
}

// impl PartialOrd for ExitCode {
//     //
// }

impl Termination for ExitCode {
    fn report(self) -> std::process::ExitCode {
        match self {
            ExitCode::Success => std::process::ExitCode::SUCCESS,
            ExitCode::Error => std::process::ExitCode::FAILURE,

            _ => std::process::ExitCode::FAILURE,
        }
    }
}

impl From<i32> for ExitCode {
    fn from(exit_code: i32) -> Self {
        match exit_code {
            0 => Self::Success,
            1 => Self::Error,

            90 => Self::ConfigRead,
            91 => Self::ConfigBuild,
            92 => Self::ConfigValidation,

            93 => Self::ElfRead,
            94 => Self::ElfParseHeader,
            95 => Self::SegmentsLoad,
            96 => Self::StringTable,

            97 => Self::NoInputFileProvided,

            110 => Self::EmulatorModePrepFailure,

            _ => {
                error!("Unknown exit code: {exit_code}");
                unreachable!("Unknown exit code: {exit_code}");
            }
        }
    }
}
