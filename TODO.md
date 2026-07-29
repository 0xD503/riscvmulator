# TODO list

## Following action

- implement device framework
    - validate devices from config
- separate project into crates
    - ??
- implement extensions framework
    - ??
- implement M extension
- implement A extension
- implement F extension
- implement D extension
- implement C extension

## Fixes

- [ ] add UTs
- [ ] add benchmarks (compare with Spike)
- [ ] add docs
- [ ] use `sign_extend()` method instead of casting dupes
- [ ] add/fix project build profiles: `debug`, `release`, `release with debinfo`, `release size optimized`, etc
- [ ] profile bottlenecks
- [ ] get rid of warnings
- [ ] get rid of panics (`unwrap()`/`expect()`/`ok()`/etc)
- [ ] conditionally compile production release with no-op logger implementation
      (or without logger init) and without other libraries (minimize bin size)
- [ ] un'`pub` everything that shouldn't be `pub`
- [ ] ~~get rid off `try_into()` if possible~~ check what is better to use: `into` vs `try_into`
- [ ] comments should be 80-cut as well as code
- [ ] separate code into crates
- [ ] scripts/commands for building `riscv-tests` and `riscv-arch-test`
- [ ] make `just` commands usable: pass arguments, customize with arguments
- [ ] add cargo-deny list
- [ ] clean out of stale commented out code/comments
- [ ] get rid of heap allocations outside of initialization/deinitializations
- [ ] fix security vulnerabilities (if present)
- [ ] add CI/CD (forgejo/GitHub Actions compatible)

## Refactor

- [ ] ~~use jump table for command execution~~ check if code compiles to jump table first

## Features

- [ ] XTAL frequency
- [ ] throttle frequency
- [ ] generate exception if address is misaligned
- [ ] HALT/stop into interactive mode by `Ctrl+C`/double `Ctrl+C`
- [ ] implement `EBREAK`, add debug mode
- [ ] add fuzzer
- [ ] customize log string: add core num, PC value, cycle count
- [ ] interactive commands: help/usage, license, load-elf, start, peek/poke
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
- [ ] simple FW reset init:
    - [ ] core parking
    - [ ] stack init
    - [ ] boot-state seeding: setting a0 = hartid, a1 = dtb, PC, and initial CSRs (mhartid, etc.)
