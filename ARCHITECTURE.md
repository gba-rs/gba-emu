# Architecture

This document tracks the current design of `gba-emu`, what changed in this
PR, and the larger structural work that's still open. It's meant to give
contributors (and reviewers) context without having to reconstruct it from
scratch.

## Current shape of the codebase

- `gba/` — top-level `GBA` struct that owns the CPU, GPU, memory bus,
  timers, DMA controller, and interrupt handler, and drives them one step
  at a time in `single_step`.
- `cpu/` — ARM7TDMI register file, mode/PSR handling, and instruction
  decode/dispatch (`decode_arm` / `decode_thumb`, table-driven by opcode).
- `arm_formats/` / `thumb_formats/` — one file per instruction format,
  implementing the shared `Instruction` trait (`operations/instruction.rs`).
- `memory/` — the flat address space (`MemoryMap`), the cycle-timing wrapper
  around it (`MemoryBus`), and per-peripheral I/O register definitions
  generated via the `memory-macros` proc-macro crate.
- `gpu/`, `dma/`, `timers/`, `interrupts/` — peripheral state machines,
  ticked once per `single_step` in a fixed order.
- `gamepak/` — ROM/BIOS/save-data handling and backup-type detection.

Two frontends live in sibling repos ([`minifb-frontend`](https://github.com/gba-rs/minifb-frontend),
[`web-frontend`](https://github.com/gba-rs/web-frontend)) and consume this
crate as a library; this crate itself also compiles to `wasm32-unknown-unknown`
via `wasm-bindgen` for the web frontend.

## Changes in this PR

### 1. `GamePack` no longer requires `std::fs` in the core API

`GamePack::new` used to call `std::fs::File::open` directly and `panic!` on
failure. That's a problem for two reasons:

- `std::fs` isn't available on `wasm32-unknown-unknown`, which this crate
  targets (see the `cdylib` crate type and `wasm-bindgen` dependency in
  `Cargo.toml`) — so the web frontend was never able to call this
  constructor at all, and presumably has its own separate, undocumented
  loading path.
- A bad file path is a completely ordinary, recoverable situation for a
  frontend to hit (wrong path, permissions, etc.), and shouldn't take down
  the whole process.

This PR splits the API:

- `GamePack::from_bytes(rom_bytes, bios_bytes) -> GamePack` — the portable
  core constructor. Pure parsing over in-memory bytes, no I/O, works
  identically on native and wasm targets. This is what the web frontend
  should call after fetching ROM bytes itself.
- `GamePack::load(bios_path, rom_path) -> Result<GamePack, GamePackError>`
  — native-only (`#[cfg(not(target_arch = "wasm32"))]`), does the file I/O,
  and returns a proper error instead of panicking.
- `GamePack::new` is kept as a `#[deprecated]` thin wrapper over `load` that
  preserves the old panic-on-failure behavior, so existing native callers
  (e.g. `minifb-frontend`) keep compiling while they migrate.
- `load_save_data` got the same treatment: a portable `set_save_data(Vec<u8>)`
  plus a native-only, `Result`-returning `load_save_data(path)`.
- Fixed a latent panic along the way: header parsing
  (`title`/`game_code`/`maker_code`) used to index the ROM slice directly
  (`&rom_bytes[0xA0..0xAC]`), which panics on any ROM shorter than the
  header. It now degrades to an empty string, with a regression test.

### 2. Faster aligned access to plain RAM regions

`MemoryMap::read_u32`/`write_u32` (and the `u16` equivalents) previously
always went through 2-4 separate calls into `read_u8`/`write_u8`, each of
which re-does the region-matching `match` and re-borrows the underlying
`RefCell`. That's the hottest path in the emulator — every instruction
fetch is a `read_u32`.

This PR adds a fast path (`MemoryMap::fast_region_index`) that, for accesses
which land entirely inside a single plain RAM-like region (WRAM, IWRAM,
palette RAM, VRAM, OAM) and don't straddle that region's mirror wraparound
boundary, does one `borrow()`/`borrow_mut()` and a direct slice read/write
via `u16::from_le_bytes` / `u32::from_le_bytes` instead of 2-4 byte-wise
dispatches.

It deliberately **excludes** the I/O region and the gamepak/backup region:
both have per-address side effects in `read_u8`/`write_u8` (halt-state
writes, IE/IF semantics, flash chip commands, SRAM/flash backup-type
branching) that a bulk copy would silently skip. Those keep going through
the exact same byte-wise code as before — this change touches nothing about
*what* gets stored, only *how many dispatches* it takes to store it for the
regions where that's safe. See the `fast_path_tests` module in
`memory_map.rs` for round-trip and mirror-boundary regression tests, plus a
guard test that I/O-region side effects (like `HALTCNT`) still fire.

### 3. Consolidated CI

The repo was running three separate CI configs (`.appveyor.yml`,
`.gitlab-ci.yml`, `.github/workflows/rust.yml`) doing essentially the same
`cargo build && cargo test`. Removed the AppVeyor and GitLab configs and
kept GitHub Actions as the single source of truth, and added `cargo fmt
--check` and `cargo clippy` steps (non-blocking for now — see below).

### 4. Dependency updates

Checked every direct dependency (this crate and `memory-macros`) against
crates.io's current latest versions and updated what was safe to update:

| Crate | Was | Now | Notes |
|---|---|---|---|
| `wasm-bindgen` | 0.2.100 | 0.2.127 | patch/minor bump within 0.2.x |
| `wasm-bindgen-test` | 0.3.50 | 0.3.77 | patch/minor bump within 0.3.x |
| `log` | 0.4.27 | 0.4.33 | patch/minor bump within 0.4.x |
| `serde_with` | 3.12.0 | 3.22.0 | minor bump within 3.x; usage is a single `serde_as` import in `gpu.rs`, unaffected |
| `num-derive` | 0.4.2 | *removed* | not imported anywhere in the crate (`grep` found zero usages) — dead dependency |
| `wee_alloc` | 0.4.5 (optional, not default) | *removed* | flagged unmaintained by [RUSTSEC-2022-0054](https://rustsec.org/advisories/RUSTSEC-2022-0054.html) (open memory-leak issues, last release 2019); upstream advice is to use the default allocator on `wasm32` targets, which this crate already does by default (`wee_alloc` was opt-in, off by default) |
| `console_error_panic_hook` | 0.1.7 | 0.1.7 | already latest |
| `serde` | 1.0 (unpinned) | 1.0 (unpinned) | already tracks latest patch automatically |
| `memory-macros`: `quote`, `proc-macro2` | "1" (unpinned) | "1" (unpinned) | already tracks latest automatically |
| `memory-macros`: `syn` | "1" | "1" (**not** bumped) | see below |

`syn` is the one dependency I deliberately left behind. Current crates.io
latest is 3.0.3, but `memory-macros/src/lib.rs` is ~400 lines written
directly against `syn`'s API (including the unstable
`Diagnostic`/`.error().emit()` surface, which is why this crate needs
`#![feature(proc_macro_diagnostic)]` and a nightly toolchain in the first
place). `syn`'s 1.x → 2.x → 3.x transitions include real breaking API
changes, and this crate's build already requires nightly, which wasn't
available in the environment this PR was prepared in (see the note on
`Cargo.lock` below) — so I couldn't compile-verify a `syn` bump myself.
Bumping it blind, in the same PR as unrelated changes, risked landing a
proc-macro crate that silently fails to build. Left as a clearly-scoped
follow-up: bump `syn`, run `cargo build` with the nightly toolchain, and
fix whatever the compiler flags.

Removing `num-derive` and `wee_alloc` also trims two dependencies (and
`wee_alloc`'s further transitive deps) from the build entirely, which is a
small win for build time and `cargo audit` cleanliness on its own.

### Cross-checked against GBATEK

Per request, I cross-checked the areas this PR touches against
[GBATEK](https://problemkaputt.de/gbatek.htm) (the de-facto hardware
reference for GBA emulator development) rather than relying on the
existing code being correct by assumption:

- The memory region base addresses and sizes used by `fast_region_index`
  (WRAM 256KB mirrored every 0x40000, IWRAM 32KB mirrored every 0x8000,
  Palette RAM 1KB mirrored every 0x400, OAM 1KB mirrored every 0x400) match
  GBATEK's memory map exactly, and match what the pre-existing
  `read_u8`/`write_u8` implementation already assumed — this PR's fast path
  is bit-for-bit consistent with the existing (correct) region layout, not
  a new interpretation of it.
- VRAM is handled as flat/literal-addressed with no mirroring in both the
  old code and this PR's fast path. GBATEK marks the space above VRAM's
  96KB (0x06018000-0x06FFFFFF) as simply "Not used" rather than specifying
  a mirror period the way it does for WRAM/Palette/OAM, so "no mirroring"
  is a reasonable reading of the spec here, not a known bug — flagged in
  the roadmap above as worth a closer look if display glitches ever trace
  back to VRAM aliasing, but not something this PR changes either way.
- `HALTCNT` (0x4000301) and the `IF` register's write-1-to-clear semantics
  (0x4000202/0x4000203) were re-checked against GBATEK's bit-level
  descriptions and match the existing special-casing in `write_u8` — the
  fast path explicitly excludes this region (see above) so this behavior
  is untouched either way, but it's now got a regression test
  (`io_region_writes_still_go_through_byte_wise_special_casing`) pinning it
  down.



These are the larger items from the architecture review that didn't make
it into this PR, either because the blast radius was too large to land
safely in one change, or because they need a decision from maintainers
first.

### Remove `Rc<RefCell<Vec<u8>>>` as the memory backing store

`MemoryMap` allocates a single 256MB `Vec<u8>` wrapped in
`Rc<RefCell<...>>` to back the entire 32-bit address space, even though the
GBA has ~288KB of actual RAM. This is the single biggest architectural
change worth making, but it's **not** contained to `memory_map.rs`: the
`io_register!` proc-macro in `memory-macros/src/lib.rs` generates every
peripheral register's `register()`/`get_register()`/`set_register()`
methods against `Rc<RefCell<GbaMem>>` / `Rc<RefCell<Vec<u8>>>` directly, and
every peripheral (`gpu`, `dma`, `timers`, `interrupts`, key input, sound,
system control, LCD I/O registers) is wired up through it. Doing this
properly means:

1. Redesigning the macro's generated code to work against owned,
   fixed-size region structs instead of a shared `Rc<RefCell<Vec<u8>>>`.
2. Updating every register definition site across `memory/*_registers.rs`.
3. Re-threading `GBA::register_memory` and friends.
4. Re-deriving `Serialize`/`Deserialize` for `MemoryMap` — the current
   hand-rolled impl (~150 lines) exists *because* of the `Rc<RefCell<>>`;
   removing it should let `#[derive(Serialize, Deserialize)]` work again.

This is worth doing as its own PR with its own review, since it touches
essentially every peripheral module. The fast-path change in this PR
(#2 above) gets some of the performance benefit without the redesign, but
the memory footprint and macro-generated `RefCell` overhead remain.

### Enum-based instruction dispatch instead of `Box<dyn Instruction>`

`CPU::decode` returns `Box<dyn Instruction>`, so every decoded instruction
is a heap allocation plus a vtable call on what's the hottest loop in the
program. The decode tables already give you the instruction format up
front; the natural follow-up is decoding into an enum
(`enum ArmInstr { DataProcessing(DataProcessing), Multiply(Multiply), ... }`)
and executing via `match` instead of dynamic dispatch. Deferred here
because it touches every file in `arm_formats/` and `thumb_formats/` (the
`From<u32>`/`From<u16>` impls would need to become enum variant
constructors) — a mechanical but wide-reaching change better done as its
own reviewable PR.

### Event-driven scheduler instead of a fixed per-step update order

`GBA::single_step` ticks CPU, then GPU, then timers, then DMA, then
interrupts, in that fixed order, every step. That's a reasonable starting
point, but GBA timing accuracy usually lives or dies on the exact
interaction between DMA, timers, and the PPU — a small scheduler where
components register "fire at cycle N" events (the approach mature
emulators like mGBA use) makes those relationships explicit and testable
instead of implicit in call order. This is a bigger design change that
should probably start as a proposal/discussion rather than a drive-by PR.

### Shared frontend trait boundary

`minifb-frontend` and `web-frontend` currently consume this crate directly
with no documented trait boundary (e.g. a `VideoSink`/`AudioSink` trait
exported from this crate). Without one, core API changes can silently break
both frontends with no compiler signal until someone tries to build them.
Worth scoping once there's appetite to also set up a Cargo workspace across
the three repos (or at least CI that builds all three together).

### Commit a `Cargo.lock`

`Cargo.lock` is currently gitignored, so every fresh `cargo build`
re-resolves the full dependency tree against whatever is newest on
crates.io at build time — for an application-shaped crate (it ships a
`cdylib` for wasm and is the root of the dependency graph, not a library
meant to be flexible for downstream semver resolution), that's a
reproducibility risk rather than a benefit: a `cargo build` today and one
six months from now can silently pull different transitive versions,
including major-version bumps in unrelated dependencies. Recommend
un-gitignoring it and committing the lockfile, generated with a current
Rust toolchain. Not done as part of this PR because generating a
trustworthy lockfile requires resolving against crates.io with a toolchain
new enough to handle current dependency editions, which wasn't available
in the environment this PR was prepared in — worth doing as a quick,
separate follow-up from a normal dev machine (`cargo update && git add
Cargo.lock`).
