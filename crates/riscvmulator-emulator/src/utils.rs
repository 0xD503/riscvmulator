pub fn print_license_text() {
    static LICENSE_TEXT: &str = "

    Copyright 2026 Ruslan Vostretsov

    Licensed under either of:

    * MIT License (LICENSE-MIT)
        https://opensource.org/licenses/MIT

    * Apache License, Version 2.0 (LICENSE-APACHE)
        https://www.apache.org/licenses/LICENSE-2.0

    at your option.

";
    print!("{LICENSE_TEXT}");
}

// fn print_instruction_mnemonic(instruction_name: &str) {
//     let mnemonic = format!("Executing `{instruction_name}`");

//     debug!("{mnemonic}");
// }

#[macro_export]
macro_rules! print_instruction_mnemonic {
    ($instr_type:expr, $instruction_name:expr, $rd:expr, $rs1:expr, $imm:expr) => {
        let mnemonic =
            format!("Executing `{instruction_name} x{rd} {imm}(x{rs1})`");

        debug!("{mnemonic}");
    };
}
