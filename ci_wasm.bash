#!/usr/bin/bash

# fail fast
#
set -e

# print each command before it's executed
#
set -x

# --no-default-features is needed to turn of notwasm so this won't try to compile examples
# features don't work in wasm-pack, so using cargo test directly here
#
cargo test --target wasm32-unknown-unknown --no-default-features
cargo test --target wasm32-unknown-unknown --no-default-features --features spawn_handle
cargo test --target wasm32-unknown-unknown --no-default-features --features "spawn_handle bindgen"
