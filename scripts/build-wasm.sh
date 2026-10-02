#!/usr/bin/env bash
# Builds the WebAssembly binding for Node and for browsers into dist/.
# Needs: rustup target add wasm32-unknown-unknown
#        cargo install wasm-bindgen-cli --version 0.2.100 --locked
set -euo pipefail
cd "$(dirname "$0")/.."
cargo build -p understory-wasm --release --target wasm32-unknown-unknown
WASM=target/wasm32-unknown-unknown/release/understory_wasm.wasm
rm -rf dist
wasm-bindgen "$WASM" --target nodejs --out-dir dist/nodejs
wasm-bindgen "$WASM" --target web --out-dir dist/web
echo "built dist/nodejs and dist/web ($(wc -c < dist/nodejs/understory_wasm_bg.wasm) bytes)"
