#!/usr/bin/bash

# fail fast
#
set -e

# print each command before it's executed
#
set -x

export RUSTFLAGS="-D warnings"

# --no-default-features is needed to turn of notwasm so this won't try to compile examples
# features don't work in wasm-pack, so using cargo test directly here
#
wasm-pack test --firefox --headless -- --no-default-features
wasm-pack test --firefox --headless -- --no-default-features --features spawn_handle
wasm-pack test --firefox --headless -- --no-default-features --features bindgen
wasm-pack test --firefox --headless -- --no-default-features --features "spawn_handle bindgen"
