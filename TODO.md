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
- [ ] comments should be 80-cut as well as code
- [ ] un'`pub` everything that shouldn't be `pub`
- [ ] `.editorconfig`???
- [ ] add unit tests

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

---

## Linux Readiness Roadmap

### Architectural decisions

These decisions affect every phase below; resolve them upfront.

- **Hot-plug model**: pause-point — simulation loop checks an `AtomicBool` between
  steps; device list is only modified at those pause points (no locking on bus
  read/write hot path)
- **Device scripting**: Wasm via `wasmtime`; design `Device` trait signatures to be
  WIT-expressible from the start (no closures, no borrowed data across boundary)
  so a `WasmDevice` adapter can be added later without changing the trait
- **C extension readiness**: redesign fetch stage early so all subsequent work
  builds on a 16/32-bit-aware foundation; compressed instruction size tracked in
  decoder context; `advance_pc()` uses that value
- **Extension enable/disable**: runtime flags from config, not Cargo feature flags;
  single configurable binary

---

### Phase 0 — Bug fixes (prerequisite, do first)

- [x] `FENCE_I`: decode path falls into `unreachable!()` — stub as no-op
- [x] `MStatus_MPP::From<XLen>`: only handles `0b00` (USER); add 1=SUPERVISOR,
      2=HYPERVISOR, 3=MACHINE
- [x] `SoC::reset()`: does not reset `decoder` or `execution` context fields
- [x] `elf_loader`: `.ok().unwrap().unwrap()` on symbol table panics on stripped
      ELF — convert to `Result`
- [x] `is_need_to_advance_pc()`: `if cond { true } else { false }` → just `cond`

---

### Phase 1 — Modularize lib.rs

Pure mechanical refactor; no behavior change. Run tests after each file move.

Target layout (module skeletons already exist, just empty):

- `lib.rs` — `SoC` struct + `step()` only
- `soc/cpu/core/decoder/mod.rs` — `Decoder`, `FENCE_Context`, `decode_*_format()`,
  `decode()`
- `soc/cpu/core/executor/mod.rs` — `ExecutionContext`, `execute_*()`, `execute()`
- `soc/cpu/core/mod.rs` — `Context` struct
- `soc/cpu/mod.rs` — `RISC_V_CPU`, `memory_operation()`, `write_back()`,
  `advance_pc()`
- `soc/cpu/priveleged/` — privilege types currently scattered in lib.rs

- [ ] Move decoder structs and methods; tests pass
- [ ] Move executor structs and methods; tests pass
- [ ] Move `Context` and `RISC_V_CPU`; tests pass
- [ ] Move privilege types; tests pass

---

### Phase 2 — Fetch/decode redesign (C extension readiness)

Compressed instructions are 16-bit (bits[1:0] ≠ `0b11`). The fetch stage must
detect this at the instruction boundary so all subsequent work is C-aware.

- [ ] Fetch reads 16 bits first; if bits[1:0] == `0b11`, reads remaining 16 bits
- [ ] Add `instruction_size: u8` (2 or 4) to decoder context
- [ ] `advance_pc()` uses `decoder.instruction_size` instead of hardcoded 4
- [ ] All existing RV64I tests still pass (they only produce 32-bit instructions)

---

### Phase 3 — Memory model cleanup

- [ ] `MemoryValue::to_le_bytes()`: change return from `Vec<Byte>` to a fixed-size
      array — eliminates heap allocation on every store instruction
- [ ] Remove unused phantom type from `Memory<AddrType>`
- [ ] Remove unused phantom type from `SystemBus<AddrType>`

---

### Phase 4 — Configuration system

- [ ] Fix `config.rs`: remove hardcoded relative path to `cfg/emulator.toml`
- [ ] Populate `EmulatorCfg` struct (memory size, extension flags, log level,
      device list)
- [ ] Config priority order: compiled defaults → `emulator.toml` → env vars → CLI
- [ ] Memory size from config actually applied (currently computed and dropped)
- [ ] Per-extension enable flags in config (M, A, F, D, C, Zifencei, etc.)

---

### Phase 5 — Device framework

Removes hardcoded HTIF from bus; introduces the peripheral plugin system.

- [ ] Define `Device` trait in `soc/peripherals/mod.rs` with WIT-compatible
      signatures (no closures, no borrows across boundary) — methods:
      `base_address()`, `size()`, `read(offset, width)`, `write(offset, width, value)`
- [ ] Device registry: `Vec<Box<dyn Device>>` in `SystemBus`, dispatched by
      address range
- [ ] `AtomicBool` pause-request in `SoC`; loop checks it between steps
- [ ] `SoC` exposes `request_pause()` / `attach_device()` / `detach_device()` /
      `resume()` API
- [ ] Move HTIF to `soc/peripherals/htif.rs` implementing `Device`; remove
      `htif_to_host` field from `SystemBus`
- [ ] Out-of-range bus access raises RISC-V access-fault exception instead of
      `panic!()`
- [ ] HTIF conditionally compiled for test builds only

---

### Phase 6 — Extension framework

- [ ] Move base RV64I decode/execute logic to `soc/cpu/extensions/base/`
- [ ] Each extension gets its own module under `soc/cpu/extensions/`
- [ ] Decoder queries extension modules in order when opcode unhandled by base ISA
- [ ] Runtime enable/disable from Phase 4 config; decoder skips disabled extensions
- [ ] Promote FENCE_I stub (Phase 0) into `soc/cpu/extensions/zifencei/` — no-op
      semantics are correct until instruction caches are added
- [ ] Extension-specific `Context` fields added as needed per extension (FP
      register file, LR reservation set, etc.)

---

### Phase 7 — M extension (mul/div)

Depends on: Phase 6

- [ ] `soc/cpu/extensions/m/mod.rs`
- [ ] MUL, MULH, MULHSU, MULHU, DIV, DIVU, REM, REMU
- [ ] MULW, DIVW, DIVUW, REMW, REMUW (32-bit word variants)
- [ ] Division-by-zero and signed-overflow corner cases per spec
- [ ] `rv64um-p-*` compliance tests pass

---

### Phase 8 — Privileged ISA expansion

Depends on: Phase 6

- [ ] Proper U/S/M mode state machine (currently only M-mode exists)
- [ ] All supervisor CSRs: `sstatus`, `sepc`, `scause`, `stvec`, `stval`, `satp`,
      `sip`, `sie`, `sscratch`
- [ ] Trap delegation: `medeleg`, `mideleg`
- [ ] `SRET` fully implemented
- [ ] `ECALL` routes to correct privilege mode, not always USER
- [ ] `rv64mi-p-*` and `rv64si-p-*` compliance tests pass

---

### Phase 9 — Virtual memory (Sv39 MMU)

Depends on: Phase 5 (clean bus), Phase 8 (`satp` CSR, S-mode)

- [ ] `Mmu` struct sits between CPU and `SystemBus` (CPU → MMU → Bus, not inside
      bus); MMU disabled in M-mode or when `satp.mode` == bare
- [ ] Sv39 page-table walker: 3-level, 4 KiB pages
- [ ] TLB: small cache of VA→PA translations
- [ ] `SFENCE.VMA` invalidates TLB (full or ASID/address-specific)
- [ ] Instruction-page-fault, load-page-fault, store-page-fault exceptions
- [ ] A/D bit updates in PTEs

---

### Phase 10 — Linux boot devices

Depends on: Phase 5 (device framework)

- [ ] **UART 16550A** (`soc/peripherals/uart.rs`) — console I/O
- [ ] **CLINT** (`soc/peripherals/clint.rs`) — `mtime`/`mtimecmp` registers,
      M-mode timer interrupt when `mtime >= mtimecmp`
- [ ] **PLIC** (`soc/peripherals/plic.rs`) — priority + claim/complete registers,
      routes external interrupts to S-mode
- [ ] **VirtIO block device** (`soc/peripherals/virtio_blk.rs`) — minimal
      virtio-mmio transport; required for mounting rootfs
- [ ] Device Tree Blob (DTB) describing memory map and devices; Linux reads at boot
- [ ] Boot a minimal Linux kernel to shell

---

### Phase 11 — A extension (atomics)

Depends on: Phase 9 (LR/SC uses physical addresses from MMU)

- [ ] `soc/cpu/extensions/a/mod.rs`
- [ ] LR.W / SC.W and LR.D / SC.D
- [ ] AMO instructions: AMOSWAP, AMOADD, AMOXOR, AMOAND, AMOOR, AMOMIN, AMOMAX,
      AMOMINU, AMOMAXU (.W and .D each)
- [ ] Reservation set: physical address in CPU context; SC fails if address was
      written externally
- [ ] `rv64ua-p-*` compliance tests pass

---

### Phase 12 — F/D extensions (floating point)

Depends on: Phase 6

- [ ] FP register file (32 × f64) in `Context`
- [ ] `FCSR` (`fflags` + `frm`) in CSR file
- [ ] `soc/cpu/extensions/f/mod.rs` — all F-extension instructions
- [ ] `soc/cpu/extensions/d/mod.rs` — all D-extension instructions
- [ ] NaN-boxing of F values in D registers (per spec)
- [ ] Rounding modes from `fcsr.frm` applied; five exception flags set correctly
- [ ] `rv64uf-p-*` and `rv64ud-p-*` compliance tests pass

---

### Phase 13 — C extension (compressed instructions)

Depends on: Phase 2 (fetch already 16/32-bit aware)

- [ ] `soc/cpu/extensions/c/mod.rs` — 16-bit → 32-bit expander; execute stage
      unchanged (works on expanded instructions)
- [ ] All compressed formats: CR, CI, CSS, CIW, CL, CS, CA, CB, CJ
- [ ] Illegal instruction exception on reserved encodings
- [ ] `rv64uc-*` compliance tests pass

---

### Phase 14 — Wasm device plugin system

Depends on: Phase 5 (`Device` trait already WIT-compatible)

- [ ] Add `wasmtime` dependency
- [ ] WIT interface mirroring `Device` trait
- [ ] `WasmDevice` adapter: loads `.wasm` module, wraps it as `Box<dyn Device>`
- [ ] Plugin loader: `.wasm` path + base address specified in `emulator.toml`
- [ ] Sandbox: each plugin in its own `wasmtime::Store`; host exports explicit
      and minimal
- [ ] Port HTIF to Wasm as reference implementation / smoke test

---

### Phase 15 — Q extension (optional, last)

- [ ] 128-bit floating point — needs soft-float library (`f128` not native in Rust)
- [ ] Deferred until F/D are complete and stable
