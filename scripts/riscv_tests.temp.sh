#!/usr/bin/env sh


## exit on errors, undefined vars, pipeline fails
set -eu


REPO_FULL_PATH="$(git rev-parse --show-toplevel)"
RISCV_TESTS_ISA="${REPO_FULL_PATH}/3rd_party/riscv-tests/isa"

GCC_OPTS="-static -mcmodel=medany -fvisibility=hidden -nostdlib -nostartfiles"
GCC_OPTS="${GCC_OPTS} -isystem /usr/lib/picolibc/riscv64-unknown-elf/include"


make -C "${RISCV_TESTS_ISA}" -f Makefile -k -j"$(nproc)" \
    src_dir="${RISCV_TESTS_ISA}" \
    XLEN=64 \
    RISCV_GCC_OPTS="${GCC_OPTS}"


# #git submodule update --init --recursive

# export RISCV=${REPO_FULL_PATH}/build
# export RISCV_PREFIX=riscv64-unknown-elf-

# cd 3rd_party/riscv-tests || exit

# #autoconf
# ./configure --prefix=$RISCV/target --with-xlen=64
# RISCV_GCC_OPTS="-isystem /usr/lib/picolibc/riscv64-unknown-elf/include -L/usr/lib/picolibc/riscv64-unknown-elf/lib" make isa -k

# cd - || exit
