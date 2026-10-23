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

## The wire encoder and its framing

Scoring the overrides for coordinate shifts and append calls points at the real
encoders. They shift coordinates and append through `QByteArray`. The byte stores
pass through the append helper. A direct-memory-store score misses them.

| controller | encoder VA |
|---|---|
| Trocen | `0x1408e1c20`, `0x1408e01a0` |
| Topwisdom | `0x1408dfa50`, `0x1408e1980` |

The Trocen encoder at `0x1408e1c20` builds a packet by appending one byte per
`append(char)` call. The opening sequence loads the byte into `dl` and calls the
append helper:

```
mov dl, 1    ; append 0x01
mov dl, 0xd7 ; append 0xD7
             ; append a computed byte
mov dl, 4    ; append 0x04
mov dl, 0x22 ; append 0x22
mov dl, 0xd7 ; append 0xD7
             ; append a computed byte
mov dl, 4    ; append 0x04
mov dl, 0x32 ; append 0x32
```

So the command bytes include `0xD7` as a prefix and `0x22`, `0x32` as opcodes,
with `0x04` as a count or type field. The coordinates that follow are split into
7-bit groups, the run of `shl r8d, 7` repeated five times. That is the same 7-bit
coordinate packing the DSP family uses, the one the Ruida codec already implements.

## What is still open

One encoder method shows the framing shape. The complete command set for Trocen and
Topwisdom spans several encoder methods, each a different packet. The checksum or
terminator is not yet confirmed. The remaining work is to map every command with its
coordinate and checksum layout before a byte-correct reimplementation.

## Boundary

Per the clean-room rule, the actual wire bytes get a clean reimplementation from a
confirmed protocol description, the way the Ruida codec was done. A laser control
protocol is also a safety surface. `craft-dsp` carries no Trocen or Topwisdom
framing until the encoding is confirmed byte-correct.
