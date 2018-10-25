#!/bin/sh
cargo build --release --target wasm32-unknown-unknown
#wasm-gc target/wasm32-unknown-unknown/release/rust_wasm.wasm rust_wasm.gc.wasm
wasm-bindgen target/wasm32-unknown-unknown/release/rust_wasm.wasm  --out-dir dist --no-modules --no-typescript
