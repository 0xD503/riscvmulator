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

---

## Linux Readiness Roadmap

### Architectural decisions

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

### Rust book reading order (just-in-time, by phase)

The Rust Book (2024 edition) chapters, mapped to the phase that first needs them
— read each when its phase arrives, not all up front:

- **Ch 15 (smart pointers) + Ch 16 (fearless concurrency)** → **Phase 5**. This
  is the whole toolkit: `Vec<Hart>`, `Arc` + interior mutability, threads /
  channels, and the `Send`/`Sync` section (the "shared bus is `Send`/`Sync`"
  deliverable — don't skip it). Ceiling for Phase 5 is ch 16; nothing past it is
  required.
- **Ch 17 (async/await)** → **not on this project's path.** Concurrency here is
  OS threads + an `AtomicBool` checked between steps (Phase 6 pause-point), not
  futures. Read for general knowledge whenever; blocks nothing.
- **Ch 18 (OOP) — trait-objects part (`dyn Trait`, `Box<dyn Device>`)** →
  **Phase 6** (device registry is `Vec<Box<dyn Device>>`). Encapsulation /
  state-pattern parts skippable.
- **Ch 20 (advanced features)** → "advanced functions and closures" (**fn
  pointers**) for the **Phase 8** dispatch table; "advanced traits" (associated
  types) helps design the **Phase 6** `Device` trait. The `unsafe` section only
  matters if the bus goes lock-free instead of `Arc<Mutex>` (deferred).
- **Ch 19 (patterns and matching)** → polish only; blocks nothing.
- **Ch 21 (final project: multithreaded web server / thread pool)** → optional
  but illustrative for a thread-per-hart Phase 5 run loop (channels +
  `Arc<Mutex<…>>`).

---

### Phase 1 — Modularize lib.rs

- [x] `FetchUnit`/`Decoder`/`Executor`, `ArchState`

---

### Phase 2 — Fetch/decode redesign

- [x] fetch is 16/32-bit aware via `instruction_size`

---

### Phase 3 — Memory model cleanup

- [x] `to_le_bytes()` refactored; memory access bounds check past `RAM_END`

---

### Phase 4 — Configuration system

- [x] configuraion with figment (defaults ← TOML ← `RVMULATOR_*` env vars ← CLI args), `RUST_LOG` changed to `RVMULATOR_EMULATOR__LOGGING__LEVEL`

---

### Phase 5 — Multicore foundation (SMP structure)

Depends on: Phase 1 (`ArchState` per-hart seam), Phase 4 (hart count from config)

Introduce the multi-hart _structure_ early — like the C-readiness fetch
redesign — so all subsequent device/extension/MMU work is built SMP-aware from
the start. This phase delivers plumbing and the run-loop model; the SMP
_semantics_ are completed in the phases that own them (marked "(later)").

- [ ] Introduce `Emulator` orchestrator (`src/emulator.rs`): owns `SoC` + the
      run loop + a halt flag. De-duplicates `main` and `tests/common` (both
      currently hand-roll `SoC::new → reset → load_elf → ready_for_testing →
loop { step }`). This is the seam the run/pause/step embedder API grows on
      — kept thin now, surface completed by Phase 6; extracted into the crate at
      Phase 7. The halt flag is also the first step toward replacing HTIF's
      `std::process::exit()` (retires `scripts/run_integration_tests.temp.py`).
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

Extract into a workspace once the embedder-facing API (construct, configure,
attach devices, run/pause/step) is stable. Doing it here also speeds incremental
compiles through the long extension grind that follows. Finer decomposition
(split decoder/isa/devices) stays deferred to last.

**Layered crates** — expose two entry points so consumers pick their layer:

```
crates/
  riscv-soc/        the machine: SoC, harts, bus, memory, cpu pipeline,
                    extensions, devices → SoC::new / step / attach_device
                    (raw-machine users — debugger, fuzzer, co-sim — stop here)
  riscv-emulator/   depends on riscv-soc; the host: run loop, halt/pause, ELF
                    load, lifecycle → Emulator::build_from_config / run
riscvmulator (bin)  depends on riscv-emulator; CLI + main
```

The `soc` / `emulator` sibling modules map directly onto `riscv-soc` /
`riscv-emulator`; this is why they are kept siblings (not nested) — see Phase 5.

- [ ] Create a cargo workspace
- [ ] Move the machine into `crates/riscv-soc/`; move the host (`Emulator`, run
      loop, ELF load, init) into `crates/riscv-emulator/` depending on it
- [ ] Thin binary crate (CLI / `main`) depends on `riscv-emulator`
- [ ] **One-way dependency:** `bin → emulator → soc`. SoC must never import the
      emulator — preserve this from now so the split stays mechanical
- [ ] **Config placement keeps the machine crate lean:** `SocCfg` lives in
      `riscv-soc`; `EmulatorCfg` + combined `Config` + figment `build()` + clap
      live in `riscv-emulator`/bin — so `riscv-soc` pulls in **no figment/clap**
- [ ] `riscv-emulator` optionally `pub use riscv_soc::SoC` so its users can reach
      the machine without a second dependency (or keep it opaque)
- [ ] Audit and lock the public API surface crossing each crate seam
- [ ] Workspace builds and tests green

---

### Phase 8 — Extension framework

Foundational refactor first: the execute-stage **dispatch table** (supersedes
Refactor → "use jump table"). Land it here, not earlier — it has **no perf
payoff** (a dense `match` already lowers to a jump table _and_ inlines the arms;
fn-pointers forbid inlining and add an indirect call), so its only value is the
per-extension decoupling this phase introduces — there's nothing to register
until extensions exist.

- [ ] Unify executor handler signatures behind an `ExecOperands<'a>` borrow
      bundle (`&Decoder`, `&CSRFile`, `&Register` pc, `&FetchUnit`) so every
      handler is one `fn(&mut Executor, &ExecOperands)` type. Required because an
      `Executor` method can't also take `&Context` (disjoint-borrow rule) — pass
      the disjoint siblings as a bundle, not the whole context; the bundle is
      assembled in `execute()`. (May be pulled earlier as a standalone seam so
      Phase 9+ handlers adopt the uniform signature from the start.)
- [ ] Replace the `match decoder.command_to_execute` in `Executor::dispatch`
      with a `[fn(&mut Executor, &ExecOperands); N]` table keyed by
      `ExecutableCommand` — `static`, so it's shared across harts
- [ ] Each extension module contributes its own handler slice, so enabling an
      extension = registering its entries (ties into the per-module split below)
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

### Phase 17 - Implement some device as extension

- [ ] Port HTIF to Wasm as reference implementation / smoke test
