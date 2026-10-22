# LaserCraft phased plan

LaserCraft is the thirteenth Craft app. It follows the house architecture: a vendor-free headless engine, an egui GUI, a CLI, and a WebAssembly build, all standing on the shared `craft-libs` workspace. The engine speaks the `engine.execute` control protocol used across the suite. The merge gate is `craftrules`: `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace`.

## Build status (2026-10-22)

Eleven crates build green on .57 with `fmt`, `clippy -D warnings`, and `test`: `device`, `laser-doc`, `gcode`, `motion`, `dsp`, `galvo`, `xtool`, `trace`, `toolpath`, `engine`, and `cli`. The `engine` crate composes the document, toolpath, motion, and gcode crates behind the `engine.execute` seam. The `cli` binary runs and emits valid GRBL GCode for a line-fill job. The interfaces are evidence-grounded scaffolds. The depth behind them (real art import, raster modes, byte-level DSP framing, galvo calibration) is the work still ahead.

## Subsystem map: LightBurn to LaserCraft crates

Each LightBurn subsystem becomes a new vendor-free `craft-*` crate or reuses an existing one. Laser-specific crates are new. Shell, cloud, collaboration, and fonts come from `craft-libs`.

| LightBurn subsystem | REA target | LaserCraft crate | Source |
|---|---|---|---|
| Job and layer model | data structures in `LightBurn.exe` | `craft-laser-doc` | new |
| Vector geometry and boolean ops | `LightBurn.exe`, shared with CAD work | reuse `craft-geom` from vectorcraft/cadcraft | existing |
| Bitmap to vector trace | `potrace.dll` | `craft-trace` | new, Rust potrace port |
| Toolpath generation, raster and vector | `LightBurn.exe` | `craft-toolpath` | new |
| Motion and acceleration planning | `acclplanner.dll` | `craft-motion` | new |
| GCode base and dialects | `LightBurn.exe` `Protocol_GCode_Base` and subclasses | `craft-gcode` with a dialect trait | new |
| DSP binary controllers | `LightBurn.exe` `Protocol_Ruida`, `Protocol_Trocen`, `Protocol_Topwisdom` | `craft-dsp` | new |
| xTool HTTP and WebSocket | `LightBurn.exe` `Protocol_Worker_XTool_V1`/`V2` | `craft-xtool` | new |
| Galvo and fiber control | `executor.dll`, `lcs2dll.dll`, `BslCal.dll` | `craft-galvo` | new, later phase |
| Device descriptor registry | `LightBurn.exe` `ProtocolDescriptor` and device JSON | `craft-device` registry | new |
| Device streaming | `executor.dll`, `transfer.dll` | `craft-device` | new |
| UI shell, panels, theming | not reverse engineered, reused | `craft-ui` | existing |
| Cloud open and save, accounts | not reverse engineered, reused | `craft-ui::cloud`, `craft-cloud-client` | existing |
| Collaborative editing | not reverse engineered, reused | `craft-collab` | existing |
| Localization | translation tables | craft i18n convention | existing |

## Provenance of the sibling DLLs

A pass over the DLL version metadata on 2026-10-22 informs which binaries are worth reverse engineering. `transfer.dll`, `executor.dll`, `acclplanner.dll`, `param.dll`, and `utils.dll` carry no version resource and are named like internal modules, consistent with LightBurn's own code. These are the primary RE targets. `potrace.dll` is Peter Selinger's open-source Potrace. `craft-trace` uses the public Potrace algorithm directly. No RE of that DLL is needed. `lcs2dll.dll` and `lcs2dllx64.dll` are a third-party galvo control SDK, version 1.0.0.6219 dated 2024, for JCZ-style LMC boards. Galvo support is a later phase and leans on the board's documented protocol.

## Controller protocols

The controller protocols are the keystone. LightBurn drives existing controller protocols. `craft-gcode`, `craft-ruida`, and `craft-grbl` are reimplementations of those public or semi-public wire formats, informed by REA observation of how LightBurn frames them.

## Phases

### Phase 0: toolchain and evidence (in progress)

- REA 6.7.0 running on Node 22. Done.
- Ghidra 12.1.4 and JDK 21 installed, `GHIDRA_INSTALL_DIR` set. In progress.
- First engine-free evidence captured in `rea-evidence/`. Done.
- Register REA for Claude Code, decide the MetaMCP onboarding. Pending.

### Phase 1: data model and file interop

- REA the job and layer data structures.
- Define `craft-laser-doc`, the native LaserCraft document.
- Write a `.lbrn` and `.lbrn2` importer so existing user files open. The format is XML. This phase is plain parsing.
- Round-trip test against a corpus of real `.lbrn` files.

### Phase 2: geometry and toolpaths

- Reuse `craft-geom` for curves, transforms, and boolean operations.
- Port bitmap tracing into `craft-trace`.
- Build `craft-toolpath`: line, fill, and image modes with ordering and optimization.
- Oracle: compare generated toolpaths against LightBurn output on identical jobs.

### Phase 3: motion and protocols

- `craft-motion`: acceleration planning informed by REA of `acclplanner.dll`.
- `craft-gcode`, `craft-grbl`, `craft-ruida`: controller framing.
- Oracle: capture LightBurn controller traffic with REA `inspect-pcap-capture` or a serial logic capture through `decode-signal-recording`, then diff LaserCraft output byte for byte.

### Phase 4: device streaming and a running GUI

- `craft-device`: connect, stream, pause, resume, stop.
- egui GUI on `craft-ui`, parity with the LightBurn layout operators expect.
- CLI `lasercraft-cli` for headless jobs.

### Phase 5: cloud, collaboration, WASM, i18n hardening

- Wire `craft-ui::cloud` and `craft-cloud-client` so jobs live in CloudCraft.
- `craft-collab` for shared job editing.
- WebAssembly build with precompressed assets per the Craft web convention.
- Localization tables filled out and a security review of the update channel and device access.

## Build and test placement

Rust builds run in Docker on .57. Windows binaries cross-compile on .57 with cargo-xwin. The WASM build follows the Craft precompressed convention. These placements come from existing lab rules and are not relitigated here.

## Crate location decision

`craft-libs/AGENTS.md` requires every crate there to be useful to more than one app. The laser crates (device, gcode, dsp, galvo, motion, toolpath, trace, laser-doc, xtool) are laser-specific. They do not qualify. They live in this LaserCraft repo as a Cargo workspace. LaserCraft consumes `craft-libs` for the shared parts (`craft-ui`, `craft-cloud`, `craft-cloud-client`, `craft-collab`, `craft-settings`, `craft-atomic`). If any laser crate later proves useful to another app, it folds into `craft-libs` the way `craft-nextcloud` did.

The workspace mirrors the craft-libs conventions: edition 2024, `unsafe_code = "forbid"`, and every `lib.rs` opens with the never-crash deny block (`unwrap_used`, `expect_used`, `panic`, `unimplemented`, `todo`, `unreachable`). The merge gate is the same three commands. Builds run in Docker on .57.

The `lasercraft-device` scaffold was built and gate-checked on .57 on 2026-10-22 with rustc 1.99.1. `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace` all pass.

## Open decisions

1. MetaMCP onboarding shape for REA, given REA must run where LightBurn lives. See `rea-workflow.md`.
2. Pinning strategy for `craft-libs` (git rev versus path) once the first LaserCraft crate depends on it.
3. Byte-level DSP protocol decompilation needs the Ghidra heap raised, since the full exe snapshot reopen is killed on load.
