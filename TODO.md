# TODO list

## Following action

- phase5

## Tech debt

- [ ] add UTs
- [ ] add benchmarks
- [ ] add docs
- [ ] add build profiles
- [ ] profile bottlenecks
- [ ] conditional compilation for test facilities/scaffolding (→ Phase 6)
- [ ] get rid of warnings
- [ ] get rid of panics (`unwrap()`/`expect()`/`ok()`/etc)

## Fixes

- [ ] get rid off `try_into()` if possible
- [ ] conditionally compile production release with no-op logger implementation
      (or without logger init) and without other libraries (minimize bin size)
- [ ] comments should be 80-cut as well as code
- [ ] un'`pub` everything that shouldn't be `pub`
- [ ] `.editorconfig`???
- [ ] add unit tests
- [ ] add benchmarks (compare with Spike)
- [ ] separate code into crates (→ Phase 7 workspace move; finer multi-crate
      decomposition deferred to last)
- [ ] scripts/commands for building `riscv-tests` and `riscv-arch-test`

## Refactor

- [ ] use jump table for command execution (→ Phase 8 — see dispatch-table items)
- [ ] add reset method to `Register`

## Features

- [ ] generate exception if address is misaligned (→ Phase 11)
- [ ] HALT into interactive mode by `Ctrl+C`
- [ ] add emulator throtling
- [ ] implement `EBREAK`, add debug mode
- [ ] add fuzzer
