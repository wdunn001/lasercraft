# LaserCraft phased plan

LaserCraft is the thirteenth Craft app. It follows the house architecture: a vendor-free headless engine, an egui GUI, a CLI, and a WebAssembly build, all standing on the shared `craft-libs` workspace. The engine speaks the `engine.execute` control protocol used across the suite. The merge gate is `craftrules`: `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace`.

## Subsystem map: LightBurn to LaserCraft crates

Each LightBurn subsystem becomes a new vendor-free `craft-*` crate or reuses an existing one. Laser-specific crates are new. Shell, cloud, collaboration, and fonts come from `craft-libs`.

| LightBurn subsystem | REA target | LaserCraft crate | Source |
|---|---|---|---|
| Job and layer model | data structures in `LightBurn.exe` | `craft-laser-doc` | new |
| Vector geometry and boolean ops | `LightBurn.exe`, shared with CAD work | reuse `craft-geom` from vectorcraft/cadcraft | existing |
| Bitmap to vector trace | `potrace.dll` | `craft-trace` | new, Rust potrace port |
| Toolpath generation, raster and vector | `LightBurn.exe` | `craft-toolpath` | new |
| Motion and acceleration planning | `acclplanner.dll` | `craft-motion` | new |
| GCode framing | `LightBurn.exe` | `craft-gcode` | new |
| Ruida framing | `transfer.dll`, `executor.dll` | `craft-ruida` | new |
| Marlin and GRBL dialects | `LightBurn.exe` | `craft-grbl` | new |
| Galvo and fiber control | `lcs2dll.dll`, `BslCal.dll` | `craft-galvo` | new, later phase |
| Device streaming | `executor.dll`, `transfer.dll` | `craft-device` | new |
| UI shell, panels, theming | not reverse engineered, reused | `craft-ui` | existing |
| Cloud open and save, accounts | not reverse engineered, reused | `craft-ui::cloud`, `craft-cloud-client` | existing |
| Collaborative editing | not reverse engineered, reused | `craft-collab` | existing |
| Localization | translation tables | craft i18n convention | existing |

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

## Open decisions

1. Final project name. `lasercraft` is provisional.
2. Whether LaserCraft lives in its own repo like the other apps, with the laser crates folded into `craft-libs`, matching how `craft-nextcloud` was added.
3. MetaMCP onboarding shape for REA, given REA must run where LightBurn lives. See `rea-workflow.md`.
