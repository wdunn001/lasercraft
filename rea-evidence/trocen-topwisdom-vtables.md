# Trocen and Topwisdom protocol classes, located by RTTI

Date 2026-10-23. Target `LightBurn.exe` (33.6 MB, PE x86-64, image base
0x140000000). The PDB `LightBurn.pdb` is not loaded. Functions are unnamed.

## Fast method, no full Ghidra run

The REA/Ghidra path re-analyses the whole binary on every query, about 25 minutes
each. A direct MSVC RTTI parse with `pefile` plus `capstone` recovers the same
addresses in seconds. The script walks the type descriptor for a class name. It finds the Complete
Object Locator that points at it. It finds the pointer to that locator in `.rdata`.
The vtable sits just above that pointer.

## Confirmed classes

Both DSP controllers carry full RTTI and 80-entry vtables.

- `Protocol_Trocen`  / `TrocenDescriptor`
- `Protocol_Topwisdom` / `TopWisdomDescriptor`

They share the base `Protocol` vtable shape with the other controllers. Slots
`0x1400f87a0` and `0x140028de0` repeat across the table as shared base or stub
methods.

## Trocen methods with high literal-byte density

Scoring each override by how many small immediates it stores to memory, the
Trocen leaders are:

| VA | role |
|---|---|
| `0x1408dbea0` | machine-settings descriptor table (not the wire encoder) |
| `0x14090b490` | struct copy and controller-state transfer |

The Topwisdom counterparts are `0x1408d89d0` and `0x140917920`.

`0x1408dbea0` registers named device settings through a helper at `0x140619e00`,
loading the names with `lea rdx, [rip+...]`. The recovered names are the Trocen
User Settings surface:

```
"User Settings"
"Idle speed (mm/s)"
"Start speed (mm/s)"
"Min acceleration (mm/s^2)"
"Idle acceleration (mm/s^2)"
"Cutting acceleration (mm/s^2)"
"Engraving acceleration (mm/s^2)"
"Idle jerk limit (mm/s^3)"
```

The numeric fields beside each name (small integers like 4, 5, 2, 3 and values
like 0x201, 0x209 that step by 8) are the setting type and the controller config
address. This is the machine-parameter model. `craft-dsp` can carry it as a
descriptor table.

## What is still open

The per-move wire framing, the bytes a cut or a jog turns into with its header and
checksum, lives in a different override on the 80-entry vtable, below the job-send
path. It is reachable from these vtable addresses, but pinning the exact encoder
and reversing its coordinate and checksum maths is a continued effort.

## Boundary

Per the clean-room rule, the actual wire bytes get a clean reimplementation from a
confirmed protocol description, the way the Ruida codec was done. A laser control
protocol is also a safety surface. `craft-dsp` carries no Trocen or Topwisdom
framing until the encoding is confirmed byte-correct.
