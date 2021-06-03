#!/usr/bin/bash

# fail fast
#
set -e

# print each command before it's executed
#
set -x

export RUSTFLAGS="-D warnings"

# only works on nightly because of features like doc_cfg and external_doc
#
cargo doc --all-features --no-deps
cargo test --doc --all-features
