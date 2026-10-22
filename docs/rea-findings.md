# REA findings

Running record of what the reverse engineering has established. Every number here comes from a REA evidence bundle in `rea-evidence/`, produced by Ghidra 12.1.4 on 2026-10-22.

## Binary inventory

| Binary | Role target | Functions | Strings | PDB reference | Snapshot |
|---|---|---|---|---|---|
| `LightBurn.exe` | job model, GCode framing | pending | pending | n/a | timed out, needs long run |
| `transfer.dll` | transport layer, `craft-device` | 651 | 429 | `transfer.pdb`, stripped | `transfer.snap` |
| `executor.dll` | galvo execution and calibration, `craft-galvo` | 3810 | 998 | `executor.pdb`, stripped | `executor.snap` |
| `acclplanner.dll` | motion planning and GCode emission, `craft-motion`/`craft-gcode` | 3757 | 2084 | stripped | `acclplanner.snap` |

All three DLLs are native PE32+ x86_64 with image base `0x180000000`. The shipped binaries carry no PDB, so Ghidra function names are auto-generated. MSVC name mangling survives in the export and RTTI strings, which recovers class and method names. The analyzed string inventory is the primary signal for locating behavior.

## String analysis

The string inventory corrected the subsystem assumptions. The Ruida and GRBL framing expected from the plan does not appear in these three DLLs. It most likely lives in `LightBurn.exe`, which still needs a long analysis run. What these DLLs actually contain:

### transfer.dll is the transport layer

Mangled class names recover a clean transport stack. `TcpTransfer` and `UdpTransfer` sit over a `utils` namespace holding `SocketBase`, `TcpClient`, `UdpSocket`, and `SocketAddr`. The UDP side has broadcast bind variants (`setBindByBroadcast`, `setIPAddrByBroadcast`, `setIPAddrAllByBroadcast`), which is the standard network-laser discovery pattern. USB appears through descriptor handling (`USBD_STATUS_PID_CHECK_FAILURE`, firmware BOS descriptor messages). This maps to `craft-device`'s transport trait with TCP, UDP, and USB backends.

Evidence: `0x180013c40` `TcpTransfer::OpenDev socket fail err=%x`, `0x180014da0` `UdpTransfer::setBindByBroadcast socket bind fail`, `0x18001c780` `?recvfrom@UdpSocket@@...`.

### executor.dll is the galvo and fiber execution engine

The content is dominated by galvo field correction. A `CalibFunc` class exposes `gridCorCompute`, `logicPtToDevPt` and `logicPtToDevPtFly` (logical to device point mapping), `loadUcf` and `saveUcf` (a UCF galvo correction file), and `readCor`/`saveCor`. `Executor` and `Executor7` send device parameters through `sendGalvoBoxParam`, `sendGridCorrectCoeff`, and `sendGalvoEquiScaleParam`. Types include `GalvoParam`, `GalvoBox`, `GalvoCalPar`, and `GalvoGridCorPar`. This is the `craft-galvo` subsystem, a later phase.

Evidence: `0x1800b18a0` `?gridCorCompute@CalibFunc@@...`, `0x1800b1fb6` `?logicPtToDevPt@CalibFunc@@...`, `0x1800b2b55` `?sendGalvoBoxParam@Executor7@@...`.

### acclplanner.dll plans motion and emits Klipper and binary GCode

The planner emits Klipper extended GCode. Strings include `SET_VELOCITY_LIMIT ACCEL=`, `SET_VELOCITY_LIMIT ACCEL_TO_DECEL=`, `RESET_G92 AXIS=Z`, `G90`, and `G1 U0 V0 W0 F` with U, V, W axes. It also writes a binary GCode output (`binary_gcode_file:`, `Binary gcode header size mismatch`, `Binary gcode scales must be positive`). The motion model is exposed through config keys (`motion_accel`, `machine_galvo_max_speed`, `is_accel_a_axis`, `start_gcode`, `end_gcode`) and per-axis planning diagnostics (`Global Axis %c: distance {min,max}, velocity {min,max}`). Galvo fill modes appear as `fill_galvo_line`, `fill_galvo_dot`, `vector_galvo`, `offset_galvo`, built by `addImageLineFillGalvo`, `addVectorPathGalvo`, and `addOffsetFillGalvo`.

This splits into two crates. `craft-motion` owns the acceleration and velocity planning. `craft-gcode` owns the Klipper dialect and the binary GCode writer. Ruida and GRBL dialects remain to be found in `LightBurn.exe`.

Evidence: `0x18028eea0` `SET_VELOCITY_LIMIT ACCEL=`, `0x18028ef70` `\n;Start GCode\nG90\n`, `0x18029c2f0` `binary_gcode_file: `, `0x18028c370` `[acclplanner] addImageLineFillGalvo()`.

## Decompiled functions

Pending. The functions anchored by the protocol and motion strings will be decompiled into `rea-evidence/` as the Phase 3 reference for `craft-ruida`, `craft-gcode`, and `craft-motion`.
