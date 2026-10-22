# Security posture

LaserCraft drives physical hardware from files and network input. Its security
work therefore starts at the trust boundaries. This document scopes them and
records the baseline controls.

## Trust boundaries

1. **Controller input.** A laser controller sends bytes back over USB, TCP, or
   UDP. These bytes are untrusted. The transport layer bounds every read and the
   protocol framers validate each frame. A malformed response resolves to a typed
   error. It never panics and never drives the hardware past a configured limit.
2. **Project and art files.** A `.lbrn` project is XML and imported art is image
   data. Both are untrusted. Parsers reject malformed input with an error and are
   fuzzed. The never-crash lints forbid `unwrap`, `expect`, and `panic` in every
   crate. A parse failure surfaces as a `Result`.
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

## Reporting

Report a suspected vulnerability privately to the maintainer. Do not open a public
issue for an unfixed security problem.
