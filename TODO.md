# TODO list

## Tech debt
-[ ] add UTs
-[ ] add benchmarks
-[ ] add docs
-[ ] add build profiles
-[ ] profile bottlenecks

## Fixes
-[ ] get rid off `try_into()` if possible
-[ ] conditionally compile production release with no-op logger implementation\
(or without logger init) and without other libraries (minimize bin size)

## Refactor
-[ ] use jump table for command exection
-[ ] add reset method to `Register`

## Features
-[ ] generate exception if address is misaligned

## Main features
-[ ] `RV32I`
-[ ] `RV64I`
-[ ] `M` extension (mul/div)
-[ ] `A` extension (atomics)
-[ ] `F` extension (floating point)
-[ ] `D` extension ()
-[ ] `Zicsr` extension (control and status registers (CSRs))
-[ ] basic privilleged ISA
-[ ] `G` extensions (general purpose `IMAFD` + privilleged extensions)
-[ ] Linux-able
-[ ] `C` extension (compressed instructions)
-[ ] `RV64/32E` (embedded)
