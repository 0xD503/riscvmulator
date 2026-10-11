#![cfg(feature = "print-instructions")]

macro_rules! print_full_instruction {
    (load, $instruction_name:expr, $rd:expr, $rs1:expr, $imm:expr, $rs1_value:expr) => {
        debug!(
            "Executing `{} rd{{{:#}}}, imm{{{:#}}}(rs1{{{:#}}})` # x[rs1] = {:#X}",
            $instruction_name,
            GPRegisterName::try_from($rd).expect("rd exceeded 5-bit width"),
            $imm,
            GPRegisterName::try_from($rs1).expect("rs1 exceeded 5-bit width"),
            $rs1_value
        );
    };
}
pub(crate) use print_full_instruction;
