cargo build --target=wasm32-unknown-unknown
wasm-bindgen  --out-dir ../../front/src/lib/pkg/greet ./target/wasm32-unknown-unknown/debug/greet.wasm