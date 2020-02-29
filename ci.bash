#!/usr/bin/bash

# fail fast
#
set -e

# print each command before it's executed
#
set -x

export RUSTFLAGS="-D warnings"

cargo test
cargo test --features async_std
cargo test --features tokio_tp
cargo test --features tokio_ct
cargo test --features spawn_handle
cargo test --features "docs spawn_handle async_std tokio_tp tokio_ct localpool threadpool"



# we would like to get doc tests for the examples in the readme, but rustdoc does not
# seem to enable the features, so they don't work
#
# cargo test --features external_doc async_std juliex localpool

# --no-default-features is needed to turn of notwasm so this won't try to compile examples
# features don't work in wasm-pack, so using cargo test directly here
#
cargo test --target wasm32-unknown-unknown --no-default-features
cargo test --target wasm32-unknown-unknown --no-default-features --features spawn_handle
cargo test --target wasm32-unknown-unknown --no-default-features --features "spawn_handle bindgen"


