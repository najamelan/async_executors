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


# Do not try to test glommio on github actions.
#
if [ -z "$GITHUB_ACTION" ]
then

	# Currently doc tests in readme will fail without all features, because we have no way of turning on
	# the features for the doctest.
	#
	cargo test --all-features

	# All executors but nothing extra.
	#
	cargo test --features "async_global async_std bindgen localpool threadpool tokio_ct tokio_tp glommio"

	# timer
	#
	cargo test --features "timer async_global async_std localpool threadpool tokio_ct tokio_tp glommio"

else # CI

	# all features without glommio:
	#
	cargo test --features "async_global async_global_async_io async_global_tokio async_std async_std_tokio localpool threadpool tokio_ct tokio_tp tokio_io tokio_timer timer tracing bindgen notwasm"

	cargo test --features "async_global async_std bindgen localpool threadpool tokio_ct tokio_tp"
	cargo test --features "timer async_global async_std localpool threadpool tokio_ct tokio_tp"
fi


cargo test --features "tokio_io async_global async_std tokio_ct tokio_tp"
cargo test --features "tokio_timer tokio_ct tokio_tp"
