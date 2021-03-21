@ECHO OFF
SETLOCAL
:: Builds the rust code for this project under MS Windows.

:: The root of this project.
SET THIS_DIR=%~dp0
SET ROOT=%THIS_DIR%..\
ECHO OpenCompGroup Root: %ROOT%
CHDIR %ROOT%

:: Install the needed cxxbridge.exe command to be installed with the
:: exact version we need.
cargo install cxxbridge-cmd --version 1.0.33

:: Build Rust
::
:: Assumes 'cxxbridge' (cxxbridge-cmd) is installed.
ECHO Generating C++ Headers...
cxxbridge --header --output "%ROOT%\include\rust\cxx.h"
ECHO Building Rust crate...
cargo build --release

:: Return back project root directory.
CHDIR "%ROOT%"
