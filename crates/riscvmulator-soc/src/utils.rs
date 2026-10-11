macro_rules! print_full_instruction {
    (load, $instruction_name:expr, $rd:expr, $rs1:expr, $imm:expr) => {
        let mnemonic = format!(
            "Executing `{} x{:#} {:#}(x{:#})` // ",
            $instruction_name, $rd, $imm, $rs1
        );

        debug!("{mnemonic}");
    };
}
pub(crate) use print_full_instruction;
