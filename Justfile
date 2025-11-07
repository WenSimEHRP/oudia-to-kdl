default:
    cargo run --release

wasm:
    cargo build --target wasm32-unknown-unknown --release --lib
    wasm-bindgen target/wasm32-unknown-unknown/release/oudia_to_kdl.wasm --out-dir web/pkg --target web --no-typescript
