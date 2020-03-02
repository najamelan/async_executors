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

# checking with rustup for when not running on travis.
#
if [[ "$TRAVIS_RUST_VERSION" == nightly ]] || [[ $(rustup default) =~ "nightly" ]]
then

	# will run doc tests which require nightly.
	#
	cargo test --all-features
	cargo doc --all-features --no-deps

else

	cargo test --features "spawn_handle async_std tokio_tp tokio_ct localpool threadpool"

fi
