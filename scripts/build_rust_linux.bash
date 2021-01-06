#!/usr/bin/env bash

# Store the current working directory, to return to.
CWD=`pwd`

# Path to this script.
THIS_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

# The root of this project.
ROOT=`readlink -f ${THIS_DIR}`

# Build Rust
#
# Assumes 'cxxbridge' (cxxbridge-cmd) is installed.
echo "Generating C++ Headers..."
cxxbridge --header --output "${ROOT}/include/rust/cxx.h"
echo "Building Rust crate..."
cargo build --release

# Return back project root directory.
cd ${CWD}
