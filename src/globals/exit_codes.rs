// pub const EXIT_SUCCESS: i32 = 0;
// pub const EXIT_ERROR: i32 = 1;
// pub const EXIT_ERROR: i32 = 2;
// pub const EXIT_ERROR: i32 = 3;

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
}
