#!/usr/bin/bash

# fail fast
#
set -e

# print each command before it's executed
#
set -x

export RUSTFLAGS="-D warnings"

cargo check
cargo check --features async_std
cargo check --features tokio_tp
cargo check --features tokio_ct
cargo check --features spawn_handle

cargo test --all-features

# checking with rustup for when not running on travis.
#
if [[ "$TRAVIS_RUST_VERSION" == nightly ]] || [[ $(rustup default) =~ "nightly" ]]
then

	# will run doc tests which requires nightly.
	#
	cargo doc --all-features --no-deps

fi
