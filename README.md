# RISC-V emulator

```text
 _ __(_)___  _____   ___ __ ___  _   _| | __ _| |_ ___  _ __
| '__| / __|/ __\ \ / / '_ ` _ \| | | | |/ _` | __/ _ \| '__|
| |  | \__ \ (__ \ V /| | | | | | |_| | | (_| | || (_) | |
|_|  |_|___/\___| \_/ |_| |_| |_|\__,_|_|\__,_|\__\___/|_|
```

## Specifications

[Specification](https://riscv.org/specifications/ratified/)

## Project state

Currently project implements base RV64IM functionality. Project passes
`rv64ui-p` and `rv64um-p` compliance tests.

### Main features

- [ ] `RV32I`
- [x] `RV64I`
- [x] `M` extension (mul/div)
- [ ] `A` extension (atomics)
- [ ] `F` extension (floating point)
- [ ] `D` extension
- [ ] `C` extension (compressed instructions)
- [ ] `Zicsr` extension (control and status registers (CSRs))
- [ ] `Zifencei` extension (`FENCE.I`)
- [ ] `G` extensions (general purpose `IMAFD_Zicsr_Zifencei`)
- [ ] basic privileged ISA
- [ ] Linux-able
- [ ] `RV64/32E` (embedded)
- [x] little-endian
- [ ] big-endian

### Compliance tests

- [x] rv64ui-p-\*
- [x] rv64um-p-\*
- [x] rv64uc-p\*
- [ ] rv64ua-p-\*
- [ ] rv64uf-p-\*
- [ ] rv64ud-p-\*
- [ ] rv64mi-p-\*
- [ ] rv64si-p-\*

## Building and running

### Prerequisites

- `git` for clonning
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

But, as I told above, Docker users may have some issues, because I didn't test
it with Docker.

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
