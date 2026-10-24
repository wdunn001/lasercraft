#!/usr/bin/env bash
# Build the LaserCraft web app: the engine compiled to WebAssembly plus the JS
# bindings. Run from the repo root. Output lands in web/pkg next to index.html.
#
# Needs the wasm32 target and a wasm-bindgen-cli whose version matches the
# wasm-bindgen crate (see Cargo.lock). The versions must agree or the glue and
# the module disagree at load time.
set -euo pipefail

VERSION="0.2.129"

rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown -p lasercraft-wasm

if ! command -v wasm-bindgen >/dev/null || [ "$(wasm-bindgen --version | awk '{print $2}')" != "$VERSION" ]; then
  cargo install wasm-bindgen-cli --version "$VERSION"
fi

rm -rf web/pkg
wasm-bindgen target/wasm32-unknown-unknown/release/lasercraft_wasm.wasm \
  --out-dir web/pkg --target web --no-typescript

echo "Built web/pkg. Serve the web/ directory over HTTP, for example:"
echo "  python3 -m http.server 8099 --directory web"
