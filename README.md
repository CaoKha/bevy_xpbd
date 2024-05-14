## Compile Bevy to wasm
```bash
cargo build --example marble_pour --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web \
    --out-dir ./wasm/ \
    --out-name "marble_pour" \
    ./target/wasm32-unknown-unknown/release/examples/marble_pour.wasm
```
