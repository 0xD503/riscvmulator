mod common;
mod peripherals;

use common::run_test;

const TEST_CONFIG_PATH: &str = "rv64i_single_hart.toml";

#[test]
fn xori() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-xori", TEST_CONFIG_PATH);
}

#[test]
fn xor() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-xor", TEST_CONFIG_PATH);
}

#[test]
fn subw() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-subw", TEST_CONFIG_PATH);
}

#[test]
fn sub() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-sub", TEST_CONFIG_PATH);
}

#[test]
fn srlw() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-srlw", TEST_CONFIG_PATH);
}

#[test]
fn srliw() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-srliw", TEST_CONFIG_PATH);
}

#[test]
fn srli() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-srli", TEST_CONFIG_PATH);
}

#[test]
fn srl() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-srl", TEST_CONFIG_PATH);
}

#[test]
fn sraw() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-sraw", TEST_CONFIG_PATH);
}

#[test]
fn sraiw() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-sraiw", TEST_CONFIG_PATH);
}

#[test]
fn srai() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-srai", TEST_CONFIG_PATH);
}

#[test]
fn sra() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-sra", TEST_CONFIG_PATH);
}

#[test]
fn sltu() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-sltu", TEST_CONFIG_PATH);
}

#[test]
fn sltiu() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-sltiu", TEST_CONFIG_PATH);
}

#[test]
fn slti() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-slti", TEST_CONFIG_PATH);
}

#[test]
fn slt() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-slt", TEST_CONFIG_PATH);
}

#[test]
fn sllw() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-sllw", TEST_CONFIG_PATH);
}

#[test]
fn slliw() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-slliw", TEST_CONFIG_PATH);
}

#[test]
fn slli() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-slli", TEST_CONFIG_PATH);
}

#[test]
fn sll() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-sll", TEST_CONFIG_PATH);
}

#[test]
fn st_ld() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-st_ld", TEST_CONFIG_PATH);
}

#[test]
fn sd() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-sd", TEST_CONFIG_PATH);
}

#[test]
fn sw() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-sw", TEST_CONFIG_PATH);
}

#[test]
fn sh() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-sh", TEST_CONFIG_PATH);
}

#[test]
fn sb() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-sb", TEST_CONFIG_PATH);
}

#[test]
fn ori() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-ori", TEST_CONFIG_PATH);
}

#[test]
fn or() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-or", TEST_CONFIG_PATH);
}

#[test]
fn ma_data() {
    run_test(
        "3rd_party/riscv-tests/isa/rv64ui-p-ma_data",
        TEST_CONFIG_PATH,
    );
}

#[test]
fn lui() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-lui", TEST_CONFIG_PATH);
}

#[test]
fn ld_st() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-ld_st", TEST_CONFIG_PATH);
}

#[test]
fn ld() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-ld", TEST_CONFIG_PATH);
}

#[test]
fn lwu() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-lwu", TEST_CONFIG_PATH);
}

#[test]
fn lw() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-lw", TEST_CONFIG_PATH);
}

#[test]
fn lhu() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-lhu", TEST_CONFIG_PATH);
}

#[test]
fn lh() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-lh", TEST_CONFIG_PATH);
}

#[test]
fn lbu() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-lbu", TEST_CONFIG_PATH);
}

#[test]
fn lb() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-lb", TEST_CONFIG_PATH);
}

#[test]
fn jalr() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-jalr", TEST_CONFIG_PATH);
}

#[test]
fn jal() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-jal", TEST_CONFIG_PATH);
}

#[test]
fn fence_i() {
    run_test(
        "3rd_party/riscv-tests/isa/rv64ui-p-fence_i",
        TEST_CONFIG_PATH,
    );
}

#[test]
fn simple() {
    run_test(
        "3rd_party/riscv-tests/isa/rv64ui-p-simple",
        TEST_CONFIG_PATH,
    );
}

#[test]
fn bne() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-bne", TEST_CONFIG_PATH);
}

#[test]
fn bltu() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-bltu", TEST_CONFIG_PATH);
}

#[test]
fn blt() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-blt", TEST_CONFIG_PATH);
}

#[test]
fn bgeu() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-bgeu", TEST_CONFIG_PATH);
}

#[test]
fn bge() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-bge", TEST_CONFIG_PATH);
}

#[test]
fn beq() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-beq", TEST_CONFIG_PATH);
}

#[test]
fn auipc() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-auipc", TEST_CONFIG_PATH);
}

#[test]
fn andi() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-andi", TEST_CONFIG_PATH);
}

#[test]
fn and() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-and", TEST_CONFIG_PATH);
}

#[test]
fn addw() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-addw", TEST_CONFIG_PATH);
}

#[test]
fn addiw() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-addiw", TEST_CONFIG_PATH);
}

#[test]
fn addi() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-addi", TEST_CONFIG_PATH);
}

#[test]
fn add() {
    run_test("3rd_party/riscv-tests/isa/rv64ui-p-add", TEST_CONFIG_PATH);
}
