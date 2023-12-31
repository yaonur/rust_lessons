!/bin/bash
cargo build --release --target=wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/wasm_sum.wasm --out-dir ../front/src/lib/pkg
