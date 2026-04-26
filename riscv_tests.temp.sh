git submodule update --init --recursive

export RISCV=$PWD/build
export RISCV_PREFIX=riscv64-unknown-elf-

cd 3rd-party/riscv-tests

autoconf
./configure --prefix=$RISCV/target --with-xlen=64
RISCV_GCC_OPTS="-isystem /usr/lib/picolibc/riscv64-unknown-elf/include -L/usr/lib/picolibc/riscv64-unknown-elf/lib" make isa

cd -
