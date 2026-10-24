# WebAssembly build target

LaserCraft's planning pipeline runs in the browser. The engine and every crate
beneath it carry no filesystem, thread, time or network calls. They compile to
`wasm32-unknown-unknown` without change. The `lasercraft-wasm` crate wraps them
for JavaScript through `wasm-bindgen`.

## Building

Add the target once:

```
rustup target add wasm32-unknown-unknown
```

Build the release artifact:

```
cargo build --release --target wasm32-unknown-unknown -p lasercraft-wasm
```

The output is `target/wasm32-unknown-unknown/release/lasercraft_wasm.wasm`, about
240 KB before optimisation. Generate the JavaScript glue with `wasm-bindgen`:

```
wasm-bindgen target/wasm32-unknown-unknown/release/lasercraft_wasm.wasm \
  --out-dir pkg --target web
```

## API

Every entry point takes JSON in and returns a string or a number out. The page
never holds a Rust value across the boundary. Each function parses its input. A
bad input returns a thrown `Error` and the module keeps running.

| Function | Input | Output |
|:--|:--|:--|
| `generate_gcode(project, motion, flavor)` | project JSON, motion JSON, controller name | GCode program |
| `generate_frame(project, flavor)` | project JSON, controller name | framing-pass GCode |
| `preview_svg(project)` | project JSON | SVG preview |
| `estimate_seconds(project, motion)` | project JSON, motion JSON | job time in seconds |
| `material_test_project(spec)` | material-test spec JSON | project JSON |

`flavor` is one of `grbl`, `grbl-m3`, `marlin` or `smoothie`. An unknown name
falls back to plain GRBL.

## Example

```js
import init, { generate_gcode } from "./pkg/lasercraft_wasm.js";

await init();
const project = JSON.stringify({ shapes: [/* ... */] });
const motion = JSON.stringify({
  max_speed_mm_s: 300,
  accel_mm_s2: 3000,
  accel_to_decel_mm_s2: 1500,
});
const gcode = generate_gcode(project, motion, "grbl");
```
