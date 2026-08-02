mod common;
mod peripherals;

use common::run_test;

const TEST_CONFIG_PATH: &str = "rv64im_single_hart.toml";

#[test]
fn div() {
    run_test("3rd_party/riscv-tests/isa/rv64um-p-div", TEST_CONFIG_PATH);
}

#[test]
fn divu() {
    run_test("3rd_party/riscv-tests/isa/rv64um-p-divu", TEST_CONFIG_PATH);
}

#[test]
fn divuw() {
    run_test("3rd_party/riscv-tests/isa/rv64um-p-divuw", TEST_CONFIG_PATH);
}

#[test]
fn divw() {
    run_test("3rd_party/riscv-tests/isa/rv64um-p-divw", TEST_CONFIG_PATH);
}

#[test]
fn mul() {
    run_test("3rd_party/riscv-tests/isa/rv64um-p-mul", TEST_CONFIG_PATH);
}

#[test]
fn mulh() {
    run_test("3rd_party/riscv-tests/isa/rv64um-p-mulh", TEST_CONFIG_PATH);
}

#[test]
fn mulhsu() {
    run_test(
        "3rd_party/riscv-tests/isa/rv64um-p-mulhsu",
        TEST_CONFIG_PATH,
    );
}

#[test]
fn mulhu() {
    run_test("3rd_party/riscv-tests/isa/rv64um-p-mulhu", TEST_CONFIG_PATH);
}

#[test]
fn mulw() {
    run_test("3rd_party/riscv-tests/isa/rv64um-p-mulw", TEST_CONFIG_PATH);
}

#[test]
fn rem() {
    run_test("3rd_party/riscv-tests/isa/rv64um-p-rem", TEST_CONFIG_PATH);
}

#[test]
fn remu() {
    run_test("3rd_party/riscv-tests/isa/rv64um-p-remu", TEST_CONFIG_PATH);
}

#[test]
fn remuw() {
    run_test("3rd_party/riscv-tests/isa/rv64um-p-remuw", TEST_CONFIG_PATH);
}

#[test]
fn remw() {
    run_test("3rd_party/riscv-tests/isa/rv64um-p-remw", TEST_CONFIG_PATH);
}
