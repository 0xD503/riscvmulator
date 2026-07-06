// pub const EXIT_SUCCESS: i32 = 0;
// pub const EXIT_ERROR: i32 = 1;
// pub const EXIT_ERROR: i32 = 2;
// pub const EXIT_ERROR: i32 = 3;

pub enum ExitCode {
    Success = 0,
    Error = 1,
    ConfigRead = 2,
    ConfigBuild = 3,
    ConfigValidation = 4,

    ElfRead = 5,
    ElfParseHeader = 6,
    SegmentsLoad = 7,
    StringTable = 8,

    DeviceReadError = 9,
    DeviceWriteError = 10,
}
