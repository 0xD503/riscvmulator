# TODO list

## Following action

- make HW bringup
- make simple FW bringup

## Tech debt

- [ ] add UTs
- [ ] add benchmarks (compare with Spike)
- [ ] add docs
- [ ] add build profiles
- [ ] profile bottlenecks
- [ ] conditional compilation for test facilities/scaffolding
- [ ] get rid of warnings
- [ ] get rid of panics (`unwrap()`/`expect()`/`ok()`/etc)

## Fixes

- [ ] get rid off `try_into()` if possible
- [ ] conditionally compile production release with no-op logger implementation
      (or without logger init) and without other libraries (minimize bin size)
- [ ] comments should be 80-cut as well as code
- [ ] un'`pub` everything that shouldn't be `pub`
- [ ] `.editorconfig`???
- [ ] separate code into crates
- [ ] scripts/commands for building `riscv-tests` and `riscv-arch-test`
- [ ] fix project build profiles: `debug`, `release`, `release with debinfo`, `release size optimized`, etc
- [ ] make `just` commands usable: pass arguments, customize with arguments

## Refactor

- [ ] ~~use jump table for command execution~~ not needed, it compiles itself

## Features

- [ ] generate exception if address is misaligned
- [ ] HALT into interactive mode by `Ctrl+C`
- [ ] add emulator throtling
- [ ] implement `EBREAK`, add debug mode
- [ ] add fuzzer
- [ ] customize log string: add core num, PC value, cycle count
- [ ] interactive commands: help/usage, license, load-elf, start, peek/poke
- [ ] boot-state seeding: setting a0 = hartid, a1 = dtb, PC, and initial CSRs (mhartid, etc.)
- [ ] HW reset init:
  - [x] set privilege mode to `M`
  - [ ] reset `mstatus`'s fields `MIE` and `MPRV` to 0
  - [ ] `mstatus`/`mstatus`'s field `MBE` to 0 if little-endian mem accesses supported
  - [ ] `misa` register is reset to enable the maximal set of supported extensions
  - [x] `pc` is set to an implementation-defined reset vector
  - [ ] `mcause` register is set to a value indicating the cause of the reset
  - [ ] writable `PMP` registers’ `A` and `L` fields are set to 0, unless the platform mandates a different reset value for some PMP registers’ A and L fields
  - [ ] `MML`, `MMWP`, and `RLB` fields of the `mseccfg` register are set to 0, unless the platform mandates a different reset value.
  - [ ] `mcause` values after reset have implementation-specific interpretation, but the value 0 should be returned on implementations that do not distinguish different reset conditions. Implementations that distinguish different reset conditions should only use 0 to indicate the most complete reset.
  - [ ] `USEED` and `SSEED` fields of the `mseccfg` CSR must have defined reset values
  - [x] `mhartid` init (for multicore support)
- [ ] simple FW (in embedded Rust) reset init:
  - [ ] core parking
  - [ ] stack init
