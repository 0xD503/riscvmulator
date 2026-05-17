# TODO list

## Following action

- refactor device discovery, add HTIF as device, not statically hard code in bus
- implement `M` extension

## Tech debt

- [ ] add UTs
- [ ] add benchmarks
- [ ] add docs
- [ ] add build profiles
- [ ] profile bottlenecks
- [ ] conditional compilation for test facilities/scaffolding
- [ ] get rid of warnings

## Fixes

- [ ] get rid off `try_into()` if possible
- [ ] conditionally compile production release with no-op logger implementation
      (or without logger init) and without other libraries (minimize bin size)

## Refactor

- [ ] use jump table for command execution
- [ ] add reset method to `Register`

## Features

- [ ] generate exception if address is misaligned

## Linux-ready compliance tests

- [x] rv64ui-p-\*
- [ ] rv64mi-p-\*
- [ ] rv64si-p-\*
- [ ] rv64um-p-\*
- [ ] rv64ua-p-\*
- [ ] rv64uf-p-\*
- [ ] rv64ud-p-\*
- [ ] rv64uq-\*
- [ ] rv64uc-\*

## Main features

- [x] `RV32I`
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
- [ ] `Q` extension (quad instructions)
- [ ] `RV64/32E` (embedded)
