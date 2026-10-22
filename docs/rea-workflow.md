# REA reverse-engineering toolchain

REA (Reverse Engineer Anything, `rea-agents` on npm, mirrored to Forgejo at `morluto-ref/rea`) drives the clean-room analysis of LightBurn. This file records how it is set up on the laptop and how it is used.

## Install state

- REA version 6.7.0, installed globally with `npm install --global rea-agents`.
- Node 22.19.0 installed through nvm-windows alongside the existing v20. REA requires Node 22.19 or newer. Node 20 is too old.
- nvm's shell switching is unreliable here. REA is invoked with Node 22 placed first on PATH: `$env:PATH = "C:\Users\willi\AppData\Local\nvm\v22.19.0;" + $env:PATH`.

## Analysis engines

`rea doctor` reports the engine state. Native binary decompilation requires one of Ghidra, Hopper, or IDA. Hopper is macOS only. IDA and Kuna are not configured and Kuna does not accept PE targets. Ghidra 12.1.x is the chosen engine.

- JDK 21 (Microsoft OpenJDK) installed for Ghidra.
- Ghidra 12.1.4 extracted to `C:\Users\willi\tools\ghidra\`.
- `GHIDRA_INSTALL_DIR` points at the extracted release directory.

Engine-free commands that work without Ghidra: `inspect-pe-resources`, `analyze` of JavaScript or Electron trees, `capture-native-ui-scenario`, `observe-native-ui`, `capture-process`, `inspect-pcap-capture`, `decode-signal-recording`.

## The target

LightBurn is installed at `C:\Program Files\LightBurn`. The core is `LightBurn.exe`, a PE32+ x86_64 Qt and C++ binary of 33.6 MB. Notable sibling DLLs, chosen because they map to the subsystems LaserCraft must reimplement:

| DLL | Likely role |
|---|---|
| `acclplanner.dll` | Acceleration and motion planning |
| `executor.dll` | Job execution and streaming |
| `transfer.dll` | Device transfer layer |
| `epilog_print_api_libcpp.dll` | Epilog laser vendor API |
| `lcs2dll.dll`, `lcs2dllx64.dll` | Galvo or fiber controller control |
| `BslCal.dll` | Galvo field calibration |
| `potrace.dll` | Bitmap to vector tracing |

Out of scope: `LexActivator.dll` and the `crypto*.dll` license and activation layer.

## How REA is run

For one-off analysis, the CLI is driven directly, for example:

```
rea inspect-pe-resources "C:\Program Files\LightBurn\LightBurn.exe" --format json
rea analyze "C:\Program Files\LightBurn\LightBurn.exe" --provider ghidra --format json
rea function <address> --format md
rea decompile <selector> --format md
```

Evidence is written into `rea-evidence/`. Each REA result is an evidence bundle with a hash, a provider, and an explicit list of limitations. That explicit provenance is what makes the output defensible as clean-room input.

## MetaMCP onboarding (pending a decision)

REA also runs as an MCP stdio server (`rea --mcp`, or registered with `rea mcp add`). The lab MetaMCP aggregator runs on .88. Two facts shape the onboarding:

1. REA analyzes files local to wherever its server process runs. LightBurn lives on this laptop. A REA server on .88 therefore cannot see the target. REA for this project has to run on the laptop, exposed to MetaMCP over a network transport, or MetaMCP gains a laptop-resident downstream.
2. A single failed downstream zeroes out the whole MetaMCP namespace. REA must be proven to hand-shake cleanly and should sit in its own namespace before it is wired into anything live.

Recommended: register REA locally for Claude Code first, prove the tools, then add a dedicated MetaMCP namespace that points at a laptop-hosted REA endpoint.
