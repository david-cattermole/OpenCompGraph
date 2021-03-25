#!/usr/bin/env bash
#
# Copyright (C) 2020 David Cattermole.
#
# This file is part of OpenCompGraph.
#
# OpenCompGraph is free software: you can redistribute it and/or modify it
# under the terms of the GNU Lesser General Public License as
# published by the Free Software Foundation, either version 3 of the
# License, or (at your option) any later version.
#
# OpenCompGraph is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Lesser General Public License for more details.
#
# You should have received a copy of the GNU Lesser General Public License
# along with OpenCompGraph.  If not, see <https://www.gnu.org/licenses/>.
# ---------------------------------------------------------------------
#

# Store the current working directory, to return to.
CWD=`pwd`

# Path to this script.
THIS_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

# The root of this project.
ROOT=`readlink -f ${THIS_DIR}`

# Install directory
INSTALL_DIR="${ROOT}/install"

# Where to find the Rust libraries and headers.
RUST_BUILD_DIR="${ROOT}/target/release/"
RUST_INCLUDE_DIR="${ROOT}/include/"

# Build Rust
#
# Assumes 'cxxbridge-cmd' and 'cbindgen' is installed.
cxxbridge --header --output "${ROOT}/include/rust/cxx.h"
cbindgen --config cbindgen.toml \
         --crate opencompgraph \
         --output "${ROOT}/include/opencompgraph/_cbindgen.h"
cargo build --release

# Build C++
mkdir -p build_linux
cd build_linux
# HACK: Create empty file so that CMake can use add_executable, but
# the file contents have not yet been written.
cmake -E touch "${ROOT}/src/_cxxbridge.cpp"
cmake \
    -DCMAKE_BUILD_TYPE=Release \
    -DCMAKE_INSTALL_PREFIX=${INSTALL_DIR} \
    -DRUST_BUILD_DIR=${RUST_BUILD_DIR} \
    -DRUST_INCLUDE_DIR=${RUST_INCLUDE_DIR} \
    ..
make clean
make all
make install

# Return back project root directory.
cd ${CWD}
