## Installing

```
rustup default nightly
rustup target add wasm32-unknown-unknown
cargo install wasm-gc wasm-bindgen-cli https
```

## Building

```
./build.sh
```

## Running

Start the `http` webserver in the `wasmbox` directory. Then open `http://localhost:8000` in the browser.



