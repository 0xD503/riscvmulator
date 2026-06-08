# TODO list

## Following action

- refactor device discovery, add HTIF as device, not statically hard code in bus
- phase 3

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

- [ ] use jump table for command execution
- [ ] add reset method to `Register`

## Features

- [ ] generate exception if address is misaligned (→ Phase 11)

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
- **Hardware description — single source of truth**: `emulator.toml` is the _only_
  human-authored description of the machine. The guest's device tree is a derived
  artifact, never hand-written:
  `config.toml → (generate) machine.dts → (dtc) machine.dtb → guest`. The emulator
  generates the DTS at startup, compiles it (shell out to `dtc` first; switch to a
  direct FDT writer later if the dependency is unwanted), places the DTB in guest
  RAM, and boots with `a1` = DTB address (`a0` = hartid). Rationale: config and DT
  describe overlapping hardware (addresses, IRQs, RAM size, hart count) for two
  different consumers (emulator vs. OpenSBI/Linux); hand-authoring both guarantees
  drift (move a base address in config, forget the DTS, kernel hangs). A static DTB
  is also incompatible with runtime-configurable devices + hot-plug + Wasm plugins.
  Generation is a _merge_: config supplies the chosen values (addresses, IRQs, RAM,
  harts); each `Device` impl supplies its own DT boilerplate (`compatible`, cell
  counts, phandles) — so the `Device` trait must be "describe-yourself-in-DT"-able,
  the same forward-design discipline as making it WIT-compatible. Host-only fields
  (disk-image paths, `.wasm` paths, log level) never enter the DTS. The generated
  `.dts` stays human-inspectable for debugging. An optional user DTS fragment may
  be merged for guest-only nodes the emulator doesn't model (`bootargs`, `/chosen`,
  initrd). Decided now because it shapes the `Device` trait (Phase 6) and config
  schema (Phase 4); implemented at Phase 15 (boot). Hot-plug caveat: a boot-time
  DTB covers only the boot set — runtime-attached devices need a DT overlay or only
  matter post-boot.
- **C extension readiness**: redesign fetch stage early so all subsequent work
  builds on a 16/32-bit-aware foundation; compressed instruction size tracked in
  decoder context; `advance_pc()` uses that value
- **Multicore model**: harts as `Vec<Hart>` over a shared `Send`/`Sync` bus
  (`ArchState` is the per-hart seam). Introduce the multi-hart _structure_ early
  (Phase 5) — same philosophy as the C-readiness fetch redesign — so every
  later device/extension/MMU phase is SMP-aware from the start. The SMP
  _semantics_ (cross-hart atomics, IPIs, per-hart interrupt contexts) are
  completed in the phases that own them, not all at once in Phase 5.
- **Crate layout**: extract the SoC into a workspace crate under `crates/`
  (Phase 7) once the outward/embedder API — construct, configure, attach
  devices, run/pause/step — has settled, i.e. after the device framework. Not
  now (module tree + `pub` surface still churn through fetch redesign, config,
  devices) and not at the very end (you'd lose the compile-time win during the
  long extension grind). Finer multi-crate decomposition (split
  decoder/isa/devices) stays deferred to last (see Fixes → "separate code into
  crates").
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
- `soc/cpu/mod.rs` — `Hart`, `memory_operation()`, `write_back()`,
  `advance_pc()`
- `soc/cpu/priveleged/` — privilege types currently scattered in lib.rs

- [x] Move decoder structs and methods; tests pass
- [x] Move executor structs and methods; tests pass
- [x] Move `Context` and `Hart`; tests pass
- [x] Move privilege types; tests pass

---

### Phase 2 — Fetch/decode redesign (C extension readiness)

Compressed instructions are 16-bit (bits[1:0] ≠ `0b11`). The fetch stage must
detect this at the instruction boundary so all subsequent work is C-aware.

- [x] Fetch reads 16 bits first; if bits[1:0] == `0b11`, reads remaining 16 bits
- [x] Add `instruction_size: u8` (2 or 4) to decoder context
- [x] `advance_pc()` uses `decoder.instruction_size` instead of hardcoded 4
- [x] All existing RV64I tests still pass (they only produce 32-bit instructions)

---

### Phase 3 — Memory model cleanup

- [x] `MemoryValue::to_le_bytes()`: change return from `Vec<Byte>` to a fixed-size
      array — eliminates heap allocation on every store instruction
- [ ] ~~Remove unused phantom type from `Memory<AddrType>`~~
- [ ] ~~Remove unused phantom type from `SystemBus<AddrType>`~~
- [x] Bounds-check the _full_ access, not just the start address: a multi-byte
      load/store straddling `RAM_END` currently slices past the `Vec` and panics
      unintentionally (this is a real bug, not an ISA placeholder)

---

### Phase 4 — Configuration system

- [x] Fix `config.rs`: remove hardcoded relative path to `cfg/emulator.toml`
- [ ] Populate `EmulatorCfg` struct (memory size, extension flags, log level,
      device list, hart count)
- [x] Config priority order: compiled defaults → `emulator.toml` → env vars → CLI
- [x] Memory size from config actually applied (currently computed and dropped)
- [x] Per-extension enable flags in config (M, A, F, D, C, Zifencei, etc.)
- [ ] Per-device fields needed for DTB generation: base address, size, IRQ line
      (so the device tree can be derived from config — see "Hardware description"
      decision; consumed at Phase 15)

---

### Phase 5 — Multicore foundation (SMP structure)

Depends on: Phase 1 (`ArchState` per-hart seam), Phase 4 (hart count from config)

Introduce the multi-hart _structure_ early — like the C-readiness fetch
redesign — so all subsequent device/extension/MMU work is built SMP-aware from
the start. This phase delivers plumbing and the run-loop model; the SMP
_semantics_ are completed in the phases that own them (marked "(later)").

- [ ] `SoC` owns `harts: Vec<Hart>` instead of a single hart; each hart has a
      `hartid`
- [ ] Shared bus is `Send`/`Sync` (Arc + interior mutability, or lock-free),
      reachable from every hart
- [ ] Run-loop / scheduling model decided and documented (thread-per-hart vs.
      cooperative round-robin)
- [ ] Per-hart reset; boot convention `a0 = hartid`, `a1 = dtb`
- [ ] Stop signal works across harts (replaces `std::process::exit` — see Phase 6)
- [ ] Hart count comes from config (Phase 4)
- (later) cross-hart LR/SC reservation invalidation — Phase 10
- (later) software interrupts / IPI via CLINT `msip` — Phase 11
- (later) per-hart PLIC contexts — Phase 15

---

### Phase 6 — Device framework

Removes hardcoded HTIF from bus; introduces the peripheral plugin system.

- [ ] Define `Device` trait in `soc/peripherals/mod.rs` with WIT-compatible
      signatures (no closures, no borrows across boundary) — methods:
      `base_address()`, `size()`, `read(offset, width)`, `write(offset, width, value)`
- [ ] `Device` trait carries a "describe-yourself-in-DT" method (emits its DT node:
      `compatible`, `reg`, interrupt info) so the DTB can be generated from the
      device model — see "Hardware description" decision; consumed at Phase 15
- [ ] Device registry: `Vec<Box<dyn Device>>` in `SystemBus`, dispatched by
      address range
- [ ] `AtomicBool` pause-request in `SoC`; loop checks it between steps
- [ ] `SoC` exposes `request_pause()` / `attach_device()` / `detach_device()` /
      `resume()` API
- [ ] Move HTIF to `soc/peripherals/htif.rs` implementing `Device`; remove
      `htif_to_host` field from `SystemBus`
- [ ] Replace HTIF's `std::process::exit()` with a graceful run-loop stop signal —
      required for multicore (one hart must not kill the whole process)
- [ ] Out-of-range bus access raises RISC-V access-fault exception instead of
      `panic!()`
- [ ] HTIF conditionally compiled for test builds only

---

### Phase 7 — Crate extraction (workspace)

Depends on: Phases 4, 5, 6 (outward/embedder API has settled)

Extract the SoC into a workspace crate once the embedder-facing API (construct,
configure, attach devices, run/pause/step) is stable. Doing it here also speeds
incremental compiles through the long extension grind that follows. This is the
single-crate-move step; finer multi-crate decomposition stays deferred to last.

- [ ] Create a cargo workspace; move the SoC lib into `crates/riscv-soc/`
- [ ] Thin binary crate (CLI / `main`) depends on `riscv-soc`
- [ ] Audit and lock the public API surface that crosses the crate seam
- [ ] Workspace builds and tests green

---

### Phase 8 — Extension framework

- [ ] Move base RV64I decode/execute logic to `soc/cpu/extensions/base/`
- [ ] Each extension gets its own module under `soc/cpu/extensions/`
- [ ] Decoder queries extension modules in order when opcode unhandled by base ISA
- [ ] Runtime enable/disable from Phase 4 config; decoder skips disabled extensions
- [ ] Promote FENCE_I stub (Phase 0) into `soc/cpu/extensions/zifencei/` — no-op
      semantics are correct until instruction caches are added
- [ ] Extension-specific `Context` fields added as needed per extension (FP
      register file, LR reservation set, etc.)

---

### Phase 9 — M extension (mul/div)

Depends on: Phase 8

- [ ] `soc/cpu/extensions/m/mod.rs`
- [ ] MUL, MULH, MULHSU, MULHU, DIV, DIVU, REM, REMU
- [ ] MULW, DIVW, DIVUW, REMW, REMUW (32-bit word variants)
- [ ] Division-by-zero and signed-overflow corner cases per spec
- [ ] `rv64um-p-*` compliance tests pass

---

### Phase 10 — A extension (atomics)

Depends on: Phase 8 (extension framework). **Not** the MMU — `rv64ua-p-*` is
physical-mode, so the reservation set only needs a physical address (identity in
bare mode). Atomics are a hard prerequisite for booting Linux, so they come
early, right after M.

- [ ] `soc/cpu/extensions/a/mod.rs`
- [ ] LR.W / SC.W and LR.D / SC.D
- [ ] AMO instructions: AMOSWAP, AMOADD, AMOXOR, AMOAND, AMOOR, AMOMIN, AMOMAX,
      AMOMINU, AMOMAXU (.W and .D each)
- [ ] Reservation set: physical address in hart context; SC fails if address was
      written externally
- [ ] Cross-hart reservation invalidation (completes the Phase 5 SMP item)
- [ ] `rv64ua-p-*` compliance tests pass

---

### Phase 11 — Privileged ISA expansion + interrupts

Depends on: Phase 6 (device framework — hosts the CLINT)

- [ ] Proper U/S/M mode state machine (currently only M-mode exists)
- [ ] All supervisor CSRs: `sstatus`, `sepc`, `scause`, `stvec`, `stval`, `satp`,
      `sip`, `sie`, `sscratch`
- [ ] Trap delegation: `medeleg`, `mideleg`
- [ ] `SRET` fully implemented
- [ ] `ECALL` routes to correct privilege mode, not always USER
- [ ] **Asynchronous interrupt delivery**: between steps, check `mip & mie` (and
      `mideleg` → S-mode) with correct priority; take the interrupt as a trap
- [ ] **`WFI`** instruction (idle the hart until an interrupt) — needed for the
      Linux idle loop
- [ ] **CLINT** (`soc/peripherals/clint.rs`): `mtime`/`mtimecmp` timer interrupt,
      `msip` software interrupt / IPI (completes the Phase 5 IPI item)
- [ ] Misaligned load/store/fetch address exceptions (from the Features list)
- [ ] `rv64mi-p-*` and `rv64si-p-*` compliance tests pass

---

### Phase 12 — Virtual memory (Sv39 MMU)

Depends on: Phase 6 (clean bus), Phase 11 (`satp` CSR, S-mode)

Sv39 is the **mandatory** S-mode paging mode for RV64 (the spec's floor: any
hart with S-mode paging must support it, and Sv48/Sv57 are a strict superset
that must also support Sv39). It's the entry point, not the ceiling — Linux
boots on it by default (39-bit VA = 512 GiB, plenty for our goal). Sv48/Sv57 are
the same 8-byte PTEs / 4 KiB pages / fault types / `SFENCE.VMA` with one or two
extra walk levels, so they fall out of a level-parameterized walker later.

- [ ] `Mmu` struct sits between CPU and `SystemBus` (CPU → MMU → Bus, not inside
      bus); MMU disabled in M-mode or when `satp.mode` == bare
- [ ] Sv39 page-table walker: 3-level, 4 KiB pages
- [ ] TLB: small cache of VA→PA translations
- [ ] `SFENCE.VMA` invalidates TLB (full or ASID/address-specific)
- [ ] Instruction-page-fault, load-page-fault, store-page-fault exceptions
- [ ] A/D bit updates in PTEs
- [ ] (follow-up, after boot) parameterize the walker by level count + switch on
      `satp.MODE` to add Sv48/Sv57 — cheap on top of a correct Sv39

---

### Phase 13 — F/D extensions (floating point)

Depends on: Phase 8

- [ ] FP register file (32 × f64) in `Context`
- [ ] `FCSR` (`fflags` + `frm`) in CSR file
- [ ] `soc/cpu/extensions/f/mod.rs` — all F-extension instructions
- [ ] `soc/cpu/extensions/d/mod.rs` — all D-extension instructions
- [ ] NaN-boxing of F values in D registers (per spec)
- [ ] Rounding modes from `fcsr.frm` applied; five exception flags set correctly
- [ ] `rv64uf-p-*` and `rv64ud-p-*` compliance tests pass

---

### Phase 14 — C extension (compressed instructions)

Depends on: Phase 2 (fetch already 16/32-bit aware), Phase 8

- [ ] `soc/cpu/extensions/c/mod.rs` — 16-bit → 32-bit expander; execute stage
      unchanged (works on expanded instructions)
- [ ] All compressed formats: CR, CI, CSS, CIW, CL, CS, CA, CB, CJ
- [ ] Illegal instruction exception on reserved encodings
- [ ] `rv64uc-*` compliance tests pass

---

### Phase 15 — Linux boot

Depends on: Phases 10 (A), 11 (priv + interrupts + CLINT), 12 (MMU), 13 (F/D),
14 (C), and 5 (SMP) for multicore boot. Comes after the full ISA because the
boot target is a usable userspace.

**Boot target** (per project owner): the kernel boots, init starts, and a user
can **log in and run coreutils** (`ls`, `cd`, …) at an interactive shell — i.e.
full userspace, which is why F/D and the rest of the ISA must already be in place.

- [ ] **Firmware / SBI**: run OpenSBI as the M-mode payload (Linux runs in
      S-mode and calls SBI for console/timer/IPI/HSM), or implement the SBI calls
      Linux needs directly
- [ ] **UART 16550A** (`soc/peripherals/uart.rs`) — console I/O
- [ ] **PLIC** (`soc/peripherals/plic.rs`) — priority + claim/complete registers,
      per-hart contexts (completes the Phase 5 item), routes external interrupts
      to S-mode
- [ ] **VirtIO block device** (`soc/peripherals/virtio_blk.rs`) — minimal
      virtio-mmio transport; required for mounting rootfs
- [ ] Device Tree Blob (DTB) describing memory map, harts, and devices; Linux
      reads it at boot
- [ ] Root filesystem with `init` + busybox/coreutils
- [ ] Boot to an interactive shell; log in; run `ls`/`cd`/coreutils
- [ ] (stretch) SMP boot: bring secondary harts online via SBI HSM / IPIs

---

### Phase 16 — Wasm device plugin system

Depends on: Phase 6 (`Device` trait already WIT-compatible)

- [ ] Add `wasmtime` dependency
- [ ] WIT interface mirroring `Device` trait
- [ ] `WasmDevice` adapter: loads `.wasm` module, wraps it as `Box<dyn Device>`
- [ ] Plugin loader: `.wasm` path + base address specified in `emulator.toml`
- [ ] Sandbox: each plugin in its own `wasmtime::Store`; host exports explicit
      and minimal
- [ ] Port HTIF to Wasm as reference implementation / smoke test

### Phase 17 - Implement some device as extension

- [ ] implemented
