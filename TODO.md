# TODO list

## Fixes
-[ ] get rid off `try_into()` if possible
-[ ] conditionally compile production release with no-op logger implementation\
(or without logger init) and without other libraries (minimize bin size)

## Features
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
