#!/usr/bin/bash

# fail fast
#
set -e

# print each command before it's executed
#
set -x

# Required for io_uring.
#
sudo bash -c "ulimit -l 512"
