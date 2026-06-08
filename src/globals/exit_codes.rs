// pub const EXIT_SUCCESS: i32 = 0;
// pub const EXIT_ERROR: i32 = 1;
// pub const EXIT_ERROR: i32 = 2;
// pub const EXIT_ERROR: i32 = 3;

pub enum ExitCode {
    Success = 0,
    // Error = 1,
    ConfigRead = 2,
    ConfigBuild = 78,
    ConfigValidation = 65,

    ElfRead = 3,
}
