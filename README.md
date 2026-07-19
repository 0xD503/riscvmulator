# RISC-V emulator

## Development stage

RV64I is complete, passes `rv64ui-p` tests. The rest extensions are under development.

## Specifications

[Specification](https://riscv.org/specifications/ratified/)

## Project state

### Main features

- [ ] `RV32I`
- [x] `RV64I`
- [ ] `M` extension (mul/div)
- [ ] `A` extension (atomics)
- [ ] `F` extension (floating point)
- [ ] `D` extension
- [ ] `Zifencei` extension (`FENCE_I` instruction)
- [ ] `Zicsr` extension (control and status registers (CSRs))
- [ ] basic privileged ISA
- [ ] `G` extensions (general purpose `IMAFD` + privileged extensions)
- [ ] Linux-able
- [ ] `C` extension (compressed instructions)
- [ ] `RV64/32E` (embedded)
- [x] little-endian
- [ ] big-endian

Currently project implements base RV64I functionality. Project passes `rv64ui-p`
compliance tests.

### Compliance tests

- [x] rv64ui-p-\*
- [ ] rv64mi-p-\*
- [ ] rv64si-p-\*
- [ ] rv64um-p-\*
- [ ] rv64ua-p-\*
- [ ] rv64uf-p-\*
- [ ] rv64ud-p-\*
- [ ] rv64uq-\*
- [ ] rv64uc-\*

## Building and running

### Prerequisites

- `podman`/`podman-compose` or `docker` (with compose plugin)
- `just` task runner

Project setup has only being tested with `podman`, so `docker` users may meet
some issues (unsure).

### Clone, init, build, test, run

#### Clone and init

```Shell
git clone --recurse-submodules https://github.com/0xD503/riscvmulator.git
cd riscvmulator
git submodule update --init --recursive
```

If you use Docker instead of Podman, set `CONTAINER_ENGINE` variable:

```Shell
export CONTAINER_ENGINE=docker
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
target/release/riscvmulator --input-file=<path_to_elf> --config-file=cfg/rv64i_single_hart.toml --log-level=info
```

## Configuration

Emulator is configured in 4 layers (in ascending priority):

- default values
- values from `*.toml` configuration
- values from `RVMULATOR_*` environment variables
- values from command line arguments

So, if you run command like this:

```Shell
RVMULATOR_EMULATOR__LOGGING__LEVEL=error target/release/riscvmulator --input-file=<path_to_elf> --log-level=debug
```

, the log level will be set to `debug` level, because CLI argument takes
precedence over everything else.

To check all CLI flags, run:

```Shell
target/release/riscvmulator --help
```

## License

Project is dual-licensed under MIT or Apache-2.0.
Run the following command to check licenses:

```Shell
target/release/riscvmulator --license
```

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
