#![cfg(feature = "print-instructions")]

macro_rules! print_full_instruction {
    (op, $instruction_name:expr, $rd:expr, $rs1:expr, $rs2:expr, $rs1_value:expr, $rs2_value:expr) => {
        debug!(
            "Executing `{} rd{{{:#}}}, rs1{{{:#}}}, rs2{{{:#}}}` # x[rs1] = {:#}, x[rs2] = {:#}",
            $instruction_name,
            GPRegisterName::try_from($rd).expect("rd exceeded 5-bit width"),
            GPRegisterName::try_from($rs1).expect("rs1 exceeded 5-bit width"),
            GPRegisterName::try_from($rs2).expect("rs2 exceeded 5-bit width"),
            $rs1_value,
            $rs2_value
        );
    };
    (op_imm, $instruction_name:expr, $rd:expr, $rs1:expr, $imm:expr, $rs1_value:expr) => {
        debug!(
            "Executing `{} rd{{{:#}}}, rs1{{{:#}}}, imm{{{:#}}}` # x[rs1] = {:#}",
            $instruction_name,
            GPRegisterName::try_from($rd).expect("rd exceeded 5-bit width"),
            GPRegisterName::try_from($rs1).expect("rs1 exceeded 5-bit width"),
            $imm,
            $rs1_value
        );
    };
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
    (store, $instruction_name:expr, $rs1:expr, $rs2:expr, $imm:expr, $rs1_value:expr, $rs2_value:expr) => {
        debug!(
            "Executing `{} rs2{{{:#}}}, imm{{{:#}}}(rs1{{{:#}}})` # x[rs1] = {:#X}, x[rs2] = {:#X}",
            $instruction_name,
            GPRegisterName::try_from($rs2).expect("rd exceeded 5-bit width"),
            $imm,
            GPRegisterName::try_from($rs1).expect("rs1 exceeded 5-bit width"),
            $rs1_value,
            $rs2_value
        );
    };
    (op_32, $instruction_name:expr, $rd:expr, $rs1:expr, $rs2:expr, $rs1_value:expr, $rs2_value:expr) => {
        debug!(
            "Executing `{} rd{{{:#}}}, rs1{{{:#}}}, rs2{{{:#}}}` # x[rs1] = {:#}, x[rs2] = {:#}",
            $instruction_name,
            GPRegisterName::try_from($rd).expect("rd exceeded 5-bit width"),
            GPRegisterName::try_from($rs1).expect("rs1 exceeded 5-bit width"),
            GPRegisterName::try_from($rs2).expect("rs2 exceeded 5-bit width"),
            $rs1_value,
            $rs2_value
        );
    };
    (op_imm_32, $instruction_name:expr, $rd:expr, $rs1:expr, $imm:expr, $rs1_value:expr) => {
        debug!(
            "Executing `{} rd{{{:#}}}, rs1{{{:#}}}, imm{{{:#}}}` # x[rs1] = {:#}",
            $instruction_name,
            GPRegisterName::try_from($rd).expect("rd exceeded 5-bit width"),
            GPRegisterName::try_from($rs1).expect("rs1 exceeded 5-bit width"),
            $imm,
            $rs1_value
        );
    };
    (ecall) => {
        debug!(
            "Executing `ecall`",
        );
    };
    (ebreak) => {
        debug!(
            "Executing `ebreak`",
        );
    };
    (mret) => {
        debug!(
            "Executing `mret`",
        );
    };
    (sret) => {
        debug!(
            "Executing `sret`",
        );
    };
    (uret) => {
        debug!(
            "Executing `uret`",
        );
    };
    (csr, $instruction_name:expr, $rd:expr, $csr:expr, $rs1:expr, $csr_value:expr, $rs1_value:expr) => {
        debug!(
            "Executing `{} rd{{{:#}}}, csr{{{:#}}}, rs1{{{:#}}}` # csr = {:#X}, x[rs1] = {:#X}",
            $instruction_name,
            GPRegisterName::try_from($rd).expect("rd exceeded 5-bit width"),
            CSRRegisterName::try_from($csr).expect("invalid CSR"),
            GPRegisterName::try_from($rs1).expect("rs1 exceeded 5-bit width"),
            $csr_value,
            $rs1_value
        );
    };
    (csri, $instruction_name:expr, $rd:expr, $csr:expr, $imm:expr, $csr_value:expr) => {
        debug!(
            "Executing `{} rd{{{:#}}}, csr{{{:#}}}, imm{{{:#}}}` # csr = {:#X}",
            $instruction_name,
            GPRegisterName::try_from($rd).expect("rd exceeded 5-bit width"),
            CSRRegisterName::try_from($csr).expect("invalid CSR"),
            $imm,
            $csr_value
        );
    };
    (branch, $instruction_name:expr, $rs1:expr, $rs2:expr, $imm:expr, $rs1_value:expr, $rs2_value:expr) => {
        debug!(
            "Executing `{} rs1{{{:#}}}, rs2{{{:#}}}, imm{{{:#X}}}` # x[rs1] = {:#X}, x[rs2] = {:#X}",
            $instruction_name,
            GPRegisterName::try_from($rs1).expect("rs1 exceeded 5-bit width"),
            GPRegisterName::try_from($rs2).expect("rs2 exceeded 5-bit width"),
            $imm,
            $rs1_value,
            $rs2_value
        );
    };
    (luipc, $instruction_name:expr, $rd:expr, $imm:expr) => {
        debug!(
            "Executing `{} rd{{{:#}}}, imm{{{:#X}}}`",
            $instruction_name,
            GPRegisterName::try_from($rd).expect("rd exceeded 5-bit width"),
            $imm
        );
    };
    (jal, $instruction_name:expr, $rd:expr, $imm:expr) => {
        debug!(
            "Executing `{} rd{{{:#}}}, imm{{{:#X}}}`",
            $instruction_name,
            GPRegisterName::try_from($rd).expect("rd exceeded 5-bit width"),
            $imm
        );
    };
    (fence) => {
        debug!(
            "Executing `fence`",
        );
    };
    (fence_i) => {
        debug!(
            "Executing `fence.i`",
        );
    };
}
pub(crate) use print_full_instruction;
