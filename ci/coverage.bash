#!/usr/bin/bash

# fail fast
#
set -e

# print each command before it's executed
#
set -x

bash <(curl https://raw.githubusercontent.com/xd009642/tarpaulin/master/travis-install.sh)
cargo tarpaulin --features "async_std tokio_tp tokio_ct localpool threadpool" --exclude-files src/bindgen.rs --out Xml
bash <(curl -s https://codecov.io/bash)
