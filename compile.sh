cargo +nightly build --target wasm32-unknown-unknown --release
wasm-gc target/wasm32-unknown-unknown/release/autismarchia_back.wasm -o backend.wasm
