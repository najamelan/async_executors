#!/usr/bin/bash

# fail fast
#
set -e

# print each command before it's executed
#
set -x

export RUSTFLAGS="-D warnings"

cargo check
cargo check --features async_global
cargo check --features async_std
cargo check --features glommio
cargo check --features tokio_tp
cargo check --features tokio_ct
cargo check --features localpool
cargo check --features threadpool

# Currently doc tests in readme will fail without all features, because we have no way of turning on
# the features for the doctest.
#
cargo test --all-features

cargo test --features "async_global async_std localpool threadpool tokio_ct tokio_tp glommio"
cargo test --features "timer async_global async_std localpool threadpool tokio_ct tokio_tp glommio"
cargo test --features "tokio_io async_global async_std tokio_ct tokio_tp"
cargo test --features "tokio_timer tokio_ct tokio_tp"
cargo test --features "timer tokio_ct tokio_tp"

# checking with rustup for when not running on travis.
#
if [[ "$TRAVIS_RUST_VERSION" == nightly ]] || [[ $(rustup default) =~ "nightly" ]]
then

	# will run doc tests which requires nightly.
	#
	cargo doc --all-features --no-deps

fi
