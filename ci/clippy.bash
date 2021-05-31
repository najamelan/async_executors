#!/usr/bin/bash

# fail fast
#
set -e

# print each command before it's executed
#
set -x

cargo clippy --tests --examples --benches --all-features -- -D warnings
cargo clippy --tests --target wasm32-unknown-unknown --features "bindgen timer async_std async_global" -- -D warnings
