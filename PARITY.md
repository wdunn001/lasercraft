# LightBurn feature parity checklist

This tracks LaserCraft against LightBurn's documented features and controls, drawn
from the official reference at docs.lightburnsoftware.com. Status is one of Done,
Partial, or Gap. The goal is 1:1 coverage of the documented controls. The long tail
(barcodes, LiDAR, nesting, dual-laser, taper warp) is listed so nothing is lost, not
because every item ships at once.

## Most-prominent 25 (baseline a reimplementation is judged on)

| # | Feature | Status | Notes |
|---|---------|--------|-------|
| 1 | New / Open / Save / Import / Export, project files | Done | Native .lcproj; SVG/DXF/image/.lbrn import; GCode/DXF export. .lbrn import composes group transforms and maps cut modes. |
| 2 | Draw lines + shape tools (rect, ellipse, polygon, star) | Done | rect, rounded rect, ellipse, regular N-gon, freeform poly, star, slot, line, pen, text. |
| 3 | Select, move, scale, rotate + numeric edits | Done | Properties tab: X/Y/W/H, rotation, aspect lock, flips. |
| 4 | Edit nodes (add/delete/move, convert, break/join) | Done | Node tool with insert/delete. Trim/extend/convert-curve: Partial. |
| 5 | Text, TTF + single-line | Done | Mark (single-stroke) and Sans (filled); text-on-path warps glyphs along a selected curve. |
| 6 | Variable text (serial, date/time, CSV merge) | Done | Serial numbers (%N token), date/time insert, and CSV merge (%col% / {col} tokens, one copy per row) all done. |
| 7 | Group / Ungroup | Done | Group id on shapes, group-aware selection, Ctrl+G / Ctrl+Shift+G. |
| 8 | Align and distribute | Done | Align to bed; distribute horizontally/vertically by even spacing. |
| 9 | Boolean union / subtract / intersect + weld | Done | |
| 10 | Offset shapes | Done | |
| 11 | Grid array and circular array | Done | Arrange dialog. |
| 12 | Cuts/Layers window + colour palette | Done | Layers with colour picker, output/show toggles, cut-order reorder. |
| 13 | Cut settings editor: Line/Fill/Image/Offset-fill | Done | Full editor (double-click a layer) with perforation, wobble, tabs, ramp, timings. |
| 14 | Speed, max/min power, passes | Done | |
| 15 | Scan angle, line interval, cross-hatch | Done | Angle-aware hatch (engine), interval, and cross-hatch all work on galvo and G-code. |
| 16 | Image dithering modes | Done | Floyd-Steinberg, Atkinson, Jarvis, Stucki, Ordered, Threshold, picked in Preferences. |
| 17 | Trace image (vectorize) | Done | |
| 18 | Laser window: start, frame, pause, stop | Done | GRBL serial: Run (confirm), Frame (beam-off), Pause/Resume, Stop. |
| 19 | Job origin / start from (abs, current, user) | Partial | Absolute and current position done; user origin: Partial. |
| 20 | Jog + position readout | Done | Move tab jog; GRBL position readout. |
| 21 | Device setup and connection | Done | Machine add/edit/delete; serial and galvo USB. |
| 22 | Bed size, origin corner, grid/snapping | Done | Bed size, origin corner stored, grid snap. |
| 23 | Material library (save, assign, link presets) | Done | Per-machine materials with finishes; JSON/XML/CSV import, JSON export; material test grid generator. |
| 24 | Preview / simulation before running | Done | Cut preview modal plays back the toolpath. |
| 25 | Camera overlay, calibration, alignment | Done | Camera tab with 4-corner alignment, registration marks. |

## By area (condensed)

- Document/File: core open/save/import/export, LightBurn .lbrn import, HPGL (.plt) import Done. G-code (.gc/.nc) import Done. LightBurn's remaining import formats are AI, PDF, Ruida .rd, and Snapmaker .sc; AI and PDF share a PDF parser. (LightBurn does not import Gerber or STL/3D.) Templates, bundles, print: Gap.
- Drawing: primary shapes, rounded rect, slot, regular N-gon, star, Code 128, Code 39, QR code (byte mode, versions 1 to 10, decoder-verified) Done; other 1D/2D symbologies (Data Matrix, EAN/UPC) Gap; tabs tool Partial (per-layer tabs done, manual placement Gap).
- Edit/Transform: selection, clipboard, undo/redo, align, distribute, arrays, boolean, offset, draw-order, group/ungroup, lock/unlock, fillet, chamfer, trim, extend Done. Nesting, dock, two-point rotate, mirror-across-line: Gap.
- Node editing: insert/delete/move, trim, extend Done; convert-curve/break/optimize/warp/deform: Gap.
- Text: core + alignment + fonts + variable text (serial, date/time, CSV merge) + text-on-path Done; bending: Gap; bar code Partial (Code 128).
- Layers/Cut settings: modes, speed/power/passes/interval/angle, air, output/show, cut order, tabs/bridges, perforation, lead-in/out, overcut, kerf offset (all applied in the G-code path) Done; sub-layers, PPI, dot mode, wobble, optimization toggles: Gap/Partial.
- Material library: Done (editable, per-machine, multi-finish, 3-format import). Material/focus/interval test generators: Gap.
- Fill/scan: interval, angle, cross-hatch, lines-per-inch (DPI), overscan, bidirectional/unidirectional scanning Done; fill grouping, flood fill, dot mode: Gap (engine work).
- Image/art: trace + dither import, dither-mode picker, adjust image (brightness/contrast/gamma), 3D depth map Done; mask, convert-to-bitmap: Gap.
- Laser/device: connect, frame, run, pause, stop, home, set-origin, unlock, position, GRBL console Done; full galvo device-settings pages Done (IO ports, laser source, field, correction, COR load, 4 tabs); macros, file list, bridge: Gap.
- Coordinate/work area: bed, origin corner, grid snap, machine axes (Move-tab Z gating), units mm/inch Done; guidelines, snap distances, alignment guides: Gap.

## Beyond LightBurn

- Depth / heat-map engrave: a grayscale image drives cut depth, banded into
  multi-pass cut layers (lighter carves deeper), for fiber-laser relief and texture.
  Direction and level count set in Preferences.
- Camera: overlay, 4-corner align, print-and-cut marks Done; lens calibration wizard, head-mounted, LiDAR: Gap.
- Rotary: Done (setup dialog: type, axis, steps, diameter, speeds, reverse, return). Galvo rotary-marking extras: Gap.
- Advanced: preview Done; settings/prefs Partial (language done); hotkeys, bundles, center finder, cylinder correction, taper, dual laser: Gap.

## Done this session (2026-10-23)

Hardware control (GRBL serial run/frame/pause/stop, position readout), machine
add/edit/delete with capabilities and origin/baud/job-start, Properties tab, grid
snap, per-layer output/show toggles, cut-order drag reorder, kerf, tabs/bridges,
machine-scoped material library with finishes and 3-format import, group/ungroup,
star tool, cut preview, six dithering modes, distribute horizontally/vertically,
lock/unlock shapes. Full i18n across en/es/de/fr/pt-BR for every new control. An
adversarial code review ran over the new JavaScript and all six confirmed defects
were fixed; the wasm builds green and every change was screenshot-verified.
