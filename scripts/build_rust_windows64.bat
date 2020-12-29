@ECHO OFF
SETLOCAL
:: Builds the rust code for this project under MS Windows.

:: The root of this project.
SET THIS_DIR=%~dp0
SET ROOT=%THIS_DIR%..\
ECHO OpenCompGroup Root: %ROOT%
CHDIR %ROOT%

:: Build Rust
::
:: Assumes 'cxxbridge-cmd' and 'cbindgen' is installed.
ECHO Generating C++ Headers...
cxxbridge --header --output "%ROOT%\include\rust\cxx.h"
cbindgen --config cbindgen.toml ^
         --crate "opencompgraph-rs" ^
         --output "%ROOT%\include\opencompgraph\_cbindgen.h"
ECHO Building Rust crate...
cargo build --release

:: Return back project root directory.
CHDIR "%ROOT%"
