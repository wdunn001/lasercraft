# REA findings

Running record of what the reverse engineering has established. Every number here comes from a REA evidence bundle in `rea-evidence/`, produced by Ghidra 12.1.4 on 2026-10-22.

## Binary inventory

| Binary | Role target | Functions | Strings | PDB reference | Snapshot |
|---|---|---|---|---|---|
| `LightBurn.exe` | job model, Ruida and GRBL framing | 41432 | 47362 | stripped | `lightburn.snap` |
| `transfer.dll` | transport layer, `craft-device` | 651 | 429 | `transfer.pdb`, stripped | `transfer.snap` |
| `executor.dll` | galvo execution and calibration, `craft-galvo` | 3810 | 998 | `executor.pdb`, stripped | `executor.snap` |
| `acclplanner.dll` | motion planning and GCode emission, `craft-motion`/`craft-gcode` | 3757 | 2084 | stripped | `acclplanner.snap` |

All three DLLs are native PE32+ x86_64 with image base `0x180000000`. The shipped binaries carry no PDB. Ghidra function names are therefore auto-generated. MSVC name mangling survives in the export and RTTI strings. That recovers class and method names. The analyzed string inventory is the primary signal for locating behavior.

## String analysis

The string inventory corrected the subsystem assumptions. The Ruida and GRBL framing expected from the plan does not appear in these three DLLs. It most likely lives in `LightBurn.exe`. That binary still needs a long analysis run. What these DLLs actually contain:

### transfer.dll is the transport layer

Mangled class names recover a clean transport stack. `TcpTransfer` and `UdpTransfer` sit over a `utils` namespace holding `SocketBase`, `TcpClient`, `UdpSocket`, and `SocketAddr`. The UDP side has broadcast bind variants (`setBindByBroadcast`, `setIPAddrByBroadcast`, `setIPAddrAllByBroadcast`). That is the standard network-laser discovery pattern. USB appears through descriptor handling (`USBD_STATUS_PID_CHECK_FAILURE`, firmware BOS descriptor messages). This maps to `craft-device`'s transport trait with TCP, UDP, and USB backends.

Evidence: `0x180013c40` `TcpTransfer::OpenDev socket fail err=%x`, `0x180014da0` `UdpTransfer::setBindByBroadcast socket bind fail`, `0x18001c780` `?recvfrom@UdpSocket@@...`.

### executor.dll is the galvo and fiber execution engine

The content is dominated by galvo field correction. A `CalibFunc` class exposes `gridCorCompute`, `logicPtToDevPt` and `logicPtToDevPtFly` (logical to device point mapping), `loadUcf` and `saveUcf` (a UCF galvo correction file), and `readCor`/`saveCor`. `Executor` and `Executor7` send device parameters through `sendGalvoBoxParam`, `sendGridCorrectCoeff`, and `sendGalvoEquiScaleParam`. Types include `GalvoParam`, `GalvoBox`, `GalvoCalPar`, and `GalvoGridCorPar`. This is the `craft-galvo` subsystem, a later phase.

Evidence: `0x1800b18a0` `?gridCorCompute@CalibFunc@@...`, `0x1800b1fb6` `?logicPtToDevPt@CalibFunc@@...`, `0x1800b2b55` `?sendGalvoBoxParam@Executor7@@...`.

### acclplanner.dll plans motion and emits Klipper and binary GCode

The planner emits Klipper extended GCode. Strings include `SET_VELOCITY_LIMIT ACCEL=`, `SET_VELOCITY_LIMIT ACCEL_TO_DECEL=`, `RESET_G92 AXIS=Z`, `G90`, and `G1 U0 V0 W0 F` with U, V, W axes. It also writes a binary GCode output (`binary_gcode_file:`, `Binary gcode header size mismatch`, `Binary gcode scales must be positive`). The motion model is exposed through config keys (`motion_accel`, `machine_galvo_max_speed`, `is_accel_a_axis`, `start_gcode`, `end_gcode`) and per-axis planning diagnostics (`Global Axis %c: distance {min,max}, velocity {min,max}`). Galvo fill modes appear as `fill_galvo_line`, `fill_galvo_dot`, `vector_galvo`, `offset_galvo`, built by `addImageLineFillGalvo`, `addVectorPathGalvo`, and `addOffsetFillGalvo`.

This splits into two crates. `craft-motion` owns the acceleration and velocity planning. `craft-gcode` owns the Klipper dialect and the binary GCode writer. Ruida and GRBL dialects remain to be found in `LightBurn.exe`.

Evidence: `0x18028eea0` `SET_VELOCITY_LIMIT ACCEL=`, `0x18028ef70` `\n;Start GCode\nG90\n`, `0x18029c2f0` `binary_gcode_file: `, `0x18028c370` `[acclplanner] addImageLineFillGalvo()`.

## Decompiled functions

Three acclplanner functions were located by cross-referencing the GCode string anchors and decompiled into `rea-evidence/decompile-*.md`. Names are Ghidra auto-generated because the PDB is stripped.

### FUN_180034dd0: the Start GCode builder (xref from 0x180034e85)

This function assembles the job header. It appends the literal `\n;Start GCode\nG90\n`. The `G90` sets absolute-coordinate mode. It then appends `G1 U0 V0 W0 F` followed by a feedrate read from config. This shows U, V, and W axis naming with a configured feedrate. A `\nG00 P100\n` move follows. Config values are read through a key lookup helper (`FUN_180055f90` tests a key, `FUN_180023c70` and `FUN_180023b10` fetch it) and a numeric getter (`FUN_180023f50` returns a double). This is the reference for the `craft-gcode` header writer.

### FUN_1800365d0: the config reader behind the velocity limit (xref from 0x180037456)

This function reads a block of laser configuration. The keys are compiled as packed little-endian immediates. They decode to `laser_id`, `mopa_enable`, `mopa_freq`, and `airflow`. MOPA is a fiber-laser pulse mode. These keys confirm fiber and galvo support alongside air-assist control. The same lookup helpers appear here. They pin down the config access pattern `craft-motion` and `craft-gcode` must reproduce.

Decoded immediates for the record: `0x64695f726573616c` is `laser_id`, `0x616e655f61706f6d` is `mopa_ena`, `0x6572665f61706f6d` is `mopa_fre`, `0x5f776f6c66726961` is `airflow_`.

### FUN_180060a30: the binary GCode writer (xref from 0x180060f80)

This function emits binary GCode records in a loop with an 88-byte (`0x58`) stride per record. So the binary format is fixed-width records. `craft-gcode` can model them as a packed struct once the field layout is recovered from this function's stores.

## LightBurn.exe protocol architecture

The whole-exe Ghidra analysis succeeded at 41432 functions and 47362 strings. Reopening that saved snapshot to query it is killed on load every time, most likely an out-of-memory during project load. The workaround was direct string extraction from the PE. That needs no analysis engine and runs in seconds. The full inventory is saved at `rea-evidence/strings-lightburn-ascii.txt` and the device and protocol class list at `rea-evidence/exe-protocol-classes.txt`.

The MSVC RTTI class names recover LightBurn's entire controller layer. It is a `Protocol` base class with a `ProtocolDescriptor` registry, one descriptor per supported device. The families are clear.

| Family | Protocol classes | LaserCraft crate |
|---|---|---|
| GCode base and dialects | `Protocol_GCode_Base`, `Protocol_GRBL` (+ `_LPC`, `_M3`, `_iLaser`), `Protocol_Marlin`, `Protocol_Smoothie`, `Protocol_GCode_Thread_RepRap`, `Protocol_LinuxCNC`, `Protocol_GCode_Custom` | `craft-gcode` |
| Vendor GCode flavors | `Protocol_Flavor_Creality_Falcon`, `Protocol_Flavor_RRF_HTTP`, `Protocol_Flavor_WeCreat_Http`, `Protocol_Emblaser*`, `Protocol_SnapMaker`, `Protocol_Cubiio`, `Protocol_FabKit`, `Protocol_ACCELaser`, `Protocol_CanCam` | `craft-gcode` flavors |
| DSP binary controllers | `Protocol_Ruida`, `Protocol_Trocen`, `Protocol_Topwisdom` | `craft-dsp` |
| Galvo and fiber | `Protocol_Galvo`, `Protocol_EZCad3`, `Protocol_JCZFiber`, `Protocol_BSLFiber` | `craft-galvo` |
| xTool network | `Protocol_XTool`, `Protocol_XTool_HTTP`, `Protocol_Worker_XTool_V1`, `Protocol_Worker_XTool_V2` | `craft-xtool` |
| Other | `Protocol_Epilog`, `Protocol_LBP`, `Protocol_Gerbil_STM`, `Protocol_SnapMaker` | per-device |

Two design points carry straight into LaserCraft. The GCode controllers all descend from one base with per-dialect subclasses. `craft-gcode` should follow that shape with a core emitter and a dialect trait, where each dialect is a small implementation of the trait. The device list is data-driven through descriptors and embedded device-profile JSON, for example the Emblaser profiles carrying `"GCodeFlavor": "grbl"` and a `StartGCode` template. That matches the house rule against hardcoded device registries. `craft-device` holds the descriptor registry and the transport, and each protocol crate implements one `Protocol` trait.

The xTool path is unusual. It speaks HTTP and a WebSocket status bus, carrying its own auth, credentials, and camera commands. The serial and DSP controllers share none of that. xTool earns its own `craft-xtool` crate.

## The .lbrn file model

The project format is plain XML. The root element is `<LightBurnProject>` and geometry is carried in `<Shape>` elements with attributes such as `Type="Group"` and `CutIndex="0"`. Exported GCode embeds a preview through a `;thumbnail: data:image/png;base64,` comment. Bundled sample projects (`GalvoCalibration.lbrn2`, `alignmenttargets.lbrn`) confirm both extensions. This is parsing work for `craft-laser-doc`, not deep reverse engineering.

## The exe decompile is unblocked on .57

The `LightBurn.exe` snapshot reopen that was killed on the laptop was an out-of-memory on a machine with little free RAM. The .57 build host has 62 GB. Re-run there, the analysis and every snapshot reopen succeed. The decompile toolchain is staged on .57 at `/srv/data/re` (Temurin 21, Ghidra 12.1.4, rea-agents, the snapshot `lightburn.snap`, and `LightBurn.exe`), run through a `node:22-bookworm` container with `_JAVA_OPTIONS=-Xmx24g`. Byte-level RE of the exe is now an on-demand drill-down.

Proof of the path end to end: searching the Ruida transport string, cross-referencing it, and decompiling the referencing function all succeeded on .57. The function `FUN_1405783a0` (evidence in `rea-evidence/decompile-ruida-transport.md`) is LightBurn's controller-connection registry builder. It installs about twenty per-controller factories in sequence, each a `FUN_140xxxxxx` assigned and registered, with the `Ethernet or Wifi/TCP Ruida` label marking the Ruida entry. This confirms the data-driven descriptor model that `craft-device` already reproduces, and it gives the exact Ruida factory addresses to drill into for the byte-level framing when it is needed.

## What still needs deeper work

The DSP byte scramble itself (Ruida, Trocen, Topwisdom) is one or two decompile hops past the registry, down each controller's send path. It is now reachable on the staged .57 toolchain. The cut-settings and material-library schema can be read from the XML and the embedded device-profile JSON without the decompiler.
