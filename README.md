# RISC-V emulator

## Specifications

[Specification](https://riscv.org/specifications/ratified/)

## Project state

Currently project implements base RV64I functionality. Project passes `rv64ui-p`
compliance tests.

## Building and running

### Prerequisites

- `podman`/`podman-compose`
- `just` task runner

### Clone, init, build, test, run

#### Clone and init

```Shell
git clone --recurse-submodules https://github.com/0xD503/riscvmulator.git
git submodule update --init --recursive
```

#### Build release version

```Shell
just build
```

#### Test

```Shell
just test
```

#### Run ELF binaries with emulator

```Shell
target/release/riscvmulator <ELF>
```
