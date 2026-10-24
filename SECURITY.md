# Security posture

LaserCraft drives physical hardware from files and network input. Its security
work therefore starts at the trust boundaries. This document scopes them and
records the baseline controls.

## Trust boundaries

1. **Controller input.** A laser controller sends bytes back over USB, TCP, or
   UDP. These bytes are untrusted. The transport layer bounds every read and the
   protocol framers validate each frame. A malformed response resolves to a typed
   error. It never panics and never drives the hardware past a configured limit.
2. **Project and art files.** A `.lbrn` project is XML. Imported art is image
   data. Imported drawings are SVG and DXF. All of them are untrusted. The DXF and
   SVG paths run in the browser over a file the user drops in. Parsers reject
   malformed input with an error. Each is fuzzed. The DXF reader rejects non-finite
   coordinates. A crafted file cannot push an infinity toward the motion limits.
   The never-crash lints ban `unwrap` and `expect` in every crate. They ban an
   outright panic too. A parse failure surfaces as a `Result`.
3. **Network controllers.** The xTool path speaks HTTP and a WebSocket bus. TLS is
   required for any remote endpoint. Credentials are never compiled into the
   binary. Account linking and tokens flow through CloudCraft. That is the single
   place a credential lives, matching the house rule.
4. **The wasm build.** The browser build runs inside the page sandbox. It reaches
   the filesystem and the network only through host-provided capabilities, never
   directly.

## Baseline controls

- `unsafe` is forbidden across the workspace.
- Every crate denies the never-crash clippy lints.
- Motion output is clamped to the machine limits in `lasercraft-motion` before it
  reaches a dialect. A bad speed or power value cannot be emitted.
- Dependency advisories and licenses are checked by `cargo deny` using `deny.toml`.

## Audit status

Last reviewed 2026-10-23. `cargo deny check` reports advisories, bans, licenses
and sources all clean. The one weak-copyleft license in the tree, MPL-2.0 on
`option-ext`, enters only transitively through `craft-settings` and is allowed for
dependency use. The craft-libs git sources are the two first-party forks named in
`deny.toml`; no other git source is permitted. The DXF reader carries fuzz-style
tests that feed truncated, contradictory and pseudo-random input and assert the
reader returns without panicking and emits only finite coordinates.

## Reporting

Report a suspected vulnerability privately to the maintainer. Do not open a public
issue for an unfixed security problem.
