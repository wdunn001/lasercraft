# BJJCZ LMC galvo USB protocol (USBLMCV4 / ComMarker B6 MOPA)

Clean-room facts recovered from the shipped EZCAD marking libraries `LMCMIO.dll`
and `Lmc1.dll` (32-bit PE32, from the ComMarker B6 MOPA EZCAD install). Only
protocol facts are recorded here: opcode numbers, packet shape, and call order.
No library code is reproduced.

Method: parse the PE export and import tables with pefile, disassemble with
capstone. Every `LMCMIO.dll` wrapper pushes `[p5,p4,p3,p2,p1], opcode, boardId,
resultPtr` into one shared sender `MIO_Cmd` (`NewCmd` for payload commands), so
the opcode is the third push from the end. `Lmc1.dll` list methods push their
list opcode (0x8000 range) into a shared list appender. Init order was recovered
from the `init_laser` caller in `Lmc1.dll`.

## Transport

- USB vendor 0x9588, product string `USBLMCV4` in the WebUSB picker.
- A control command is one 12-byte packet, little-endian: `u16 opcode` then five
  `u16` params, sent on the bulk OUT endpoint. The host then reads an 8-byte
  reply on the bulk IN endpoint; getters return their value there.
- A marking list is a separate stream of 12-byte list commands (opcodes 0x8000+),
  written to OUT in 0xC00-byte (256-command) chunks and ended by the in-list
  `0x8002`. There is no `SetEndOfList` control command.
- EEPROM, NXP dongle, firmware-version and Z-transfer functions do not use this
  12-byte path. They go through `DeviceIoControl` with IOCTL framing (inline
  constants 0x65/0x66) and are unrelated to marking.

## Control opcodes (LMCMIO.dll, via the 12-byte Cmd/NewCmd sender)

| opcode | MIO_ export | params / notes |
|-------:|-------------|----------------|
| 0x0002 | DisableLaser | |
| 0x0004 | EnableLaser | |
| 0x0005 | ExecuteList | run the written list |
| 0x0006 | SetPwmPulseWidth | |
| 0x0007 | GetVersion | reply on IN |
| 0x0009 | GetSerialNo | reply on IN |
| 0x000A | GetListStatus | reply bits: 0x04 busy, 0x20 ready, 0x40 axis |
| 0x000C | GetPositionXY | |
| 0x000D | GotoXY | direct mirror move, (x,y) device units |
| 0x000E | LaserSignalOff | |
| 0x000F | LaserSignalOn | manual laser / pointer gate |
| 0x0012 | ResetList | |
| 0x0013 | RestartList | |
| 0x0015 | WriteCorTable | uploads the lens correction table (payload) |
| 0x0016 | SetControlMode | |
| 0x0017 | SetDelayMode | |
| 0x0018 | SetMaxPolyDelay | |
| 0x001A | SetFirstPulseKiller | |
| 0x001B | SetLaserMode | |
| 0x001C | SetTiming | |
| 0x001D | SetStandby | two params |
| 0x001E | SetPwmHalfPeriod | |
| 0x001F | StopExecute | |
| 0x0020 | StopList | |
| 0x0021 | WritePort | digital output port |
| 0x0022 | WriteAnalogPort1 | |
| 0x0023 | WriteAnalogPort2 | |
| 0x0024 | WriteAnalogPortX | |
| 0x0025 | ReadPort | reply on IN |
| 0x0026 | SetAxisMotionParam | motor axis |
| 0x0027 | SetAxisOriginParam | motor axis |
| 0x0028 | AxisGoOrigin | motor axis |
| 0x0029 | MoveAxisTo | motor axis |
| 0x002A | GetAxisPos | motor axis |
| 0x002B | GetFlyWaitCount | fly-marking |
| 0x002D | GetMarkCount | |
| 0x002E | SetFpkParam2 | four params (NewCmd) |
| 0x0032 | SetFlyRes | four params (NewCmd) |
| 0x0033 | IPG_OpenMO | fiber master oscillator: 0 close, 1 open |
| 0x0034 | IPG_GetStMO_AP | |
| 0x003A | ENABLEZ | |
| 0x003B | SETZDATA | |
| 0x003C | SetSPISimmerCurrent | |
| 0x0062 | SetFpkParam | |

Host-side or IOCTL (not the 12-byte marking protocol): Open, OpenNewDev, Close,
GetDevHandle, GetClassGUID, GetLmcInfo, GetLmcPartNo, SetLmcPartNo, GetState,
Reset, WriteCmdBuf (streams the list to OUT), EleExecute, SetSysParam,
TransferDataZ, Verify, and the Epprom*/Nxp*/Dongle/GetDrvFwVer/GetChipIDState
licensing and firmware calls.

## List opcodes (Lmc1.dll, CLmcDev list methods)

| opcode | method | notes |
|-------:|--------|-------|
| 0x8001 | listJumpTo | point scaled by correction, set in the shared appender |
| 0x8002 | listEndOfList | also the chunk pad |
| 0x8003 | listLaserOnPoint | |
| 0x8004 | listDelayTime / listMarkEndDelay | |
| 0x8005 | listMarkTo / listDirectMarkTo | |
| 0x8006 | listJumpSpeed | |
| 0x8007 | listLaserOnDelay | |
| 0x8008 | listLaserOffDelay | |
| 0x800C | listMarkSpeed | |
| 0x800F | listPolygonDelay | |
| 0x8011 | listWritePort (listWritrPort) | laser/aux port inside the list |
| 0x801A | listFlyEnable | |
| 0x801C | listDirectLaserSwitch | |
| 0x801D | listFlyDelay | |
| 0x8021 | listIPGOpenMO | |
| 0x8022 | listWaitForInput | |
| 0x8023 | listChangeMarkCount | |
| 0x8025 | listEnableWeldPowerWave | |
| 0x8026 | listIPGYLPMPulseWidth | |
| 0x8029 | listSetDaZWord | |

Computed in the appender, not literal in the method: listMarkFreq,
listMarkPowerRatio, listMarkCurrent, listMarkPulseWidth.

Cross-check against our `craft-galvo` `lmc` module: JumpTo, EndOfList, MarkTo,
JumpSpeed, LaserOnDelay, LaserOffDelay, MarkSpeed and PolygonDelay all match. Our
module's `LIST_JUMP_DELAY 0x800D`, `LIST_MARK_POWER_RATIO 0x800B` and
`LIST_QSWITCH_PERIOD 0x801B` do not line up with the DLL (which uses 0x8004 for
delays, a computed power-ratio opcode, and 0x801C/0x801D in that range). Those
three need review when the list generator is rebuilt.

## Init order (Lmc1.dll init_laser; params are config-loaded, not immediate)

1. WriteCorTable (0x0015) with the `.cor` table
2. EnableLaser (0x0004)
3. SetControlMode (0x0016)
4. SetLaserMode (0x001B)
5. SetDelayMode (0x0017)
6. SetTiming (0x001C)
7. SetStandby (0x001D)
8. SetFirstPulseKiller (0x001A)
9. SetPwmHalfPeriod (0x001E)
10. SetPwmPulseWidth (0x0006)

The single most important gap versus our web driver: EZCAD uploads the lens
correction table first on every init; our driver skips it. The marking parameter
values come from `markcfg7` / `EZCAD.CFG`, not hardcoded, so those files are the
source of the real control-mode, laser-mode, timing, PWM and power-scale values.
Note that EZCAD's init does not call a control-level Reset; our `initLaser` does,
which has no authoritative basis here.

## Correction table upload (WriteCorTable)

EZCAD uploads a 65x65 = 4225-cell lens correction grid at the top of every init,
before EnableLaser. The sequence: WriteCorTable (0x0015, param 1) to start, then
one per-cell command WriteCorLine (0x0010) carrying `(dx, dy)` as two u16 and a
flag (0 for the first cell, 1 for the rest). The `.cor` file is read as a 22-byte
label, then a header, then the grid. Label "LMC1COR_1.0" is the float format (a
0x1FA-byte header, cells are two f64); otherwise the legacy int format (a 0x0E
header, cells are two i32). Each value encodes as `v >= 0 ? v : (-v + 0x8000)`,
masked to 16 bits. (Cross-checked against balor and against our `cor_real` test.)

## Marking handshake

ResetList (0x0012), write the list in 0xC00 chunks. The list must open with the
start-of-list marker 0x8051 (listReadyMark) or the board does not arm it; it ends
with 0x8002. Then IPG_OpenMO (0x0033, 1), ExecuteList (0x0005), wait for not-busy,
then IPG_OpenMO (0x0033, 0).

Busy is read from the GetVersion (0x0007) reply, not GetListStatus. The 8-byte
reply unpacks as four little-endian u16 words; the status is the fourth word
(bytes 6..7), busy bit 0x04, ready bit 0x20. A real board reply observed here was
`00 00 21 84 09 00 26 02` (version 0x8421 in word 1, 0x0226 in word 3), so the bit
positions still need a live busy/idle comparison to confirm against this unit.

## Framing / red pointer

GotoXY (0x000D) drives the mirrors directly; LaserSignalOn/Off (0x000E/0x000F)
plus WriteAnalogPort gate the beam. Our Frame loops GotoXY around the job's
bounding box with the fiber MO held closed, so the mirrors trace the box under
the guide beam and the fiber cannot fire.
