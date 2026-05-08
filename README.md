# RISC-V emulator

## Specifications

[Specification](https://riscv.org/specifications/ratified/)

## Building and running

### Prerequisites

- `podman`/`podman-compose` or `docker`/`docker-compose`
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
