@ECHO OFF
SETLOCAL
:: Builds project under MS Windows

:: The root of this project.
SET THIS_DIR=%~dp0
SET OCG_ROOT=%THIS_DIR%
ECHO OpenCompGraph Root: %OCG_ROOT%
CHDIR %OCG_ROOT%

:: Install directory
SET INSTALL_DIR="%OCG_ROOT%\install"

:: Where to find the Rust libraries and headers.
SET RUST_BUILD_DIR="%OCG_ROOT%\target\release"
SET RUST_INCLUDE_DIR="%OCG_ROOT%\include"

:: Build Rust
::
:: Assumes 'cxxbridge-cmd' and 'cbindgen' is installed.
cxxbridge --header --output "%OCG_ROOT%\include\rust\cxx.h"
cbindgen --config cbindgen.toml ^
         --crate opencompgraph ^
         --output "%OCG_ROOT%\include\opencompgraph\_cbindgen.h"
cargo build --release

:: Build C++
MKDIR build_windows
CHDIR build_windows
:: HACK: Create empty file so that CMake can use add_executable, but
:: the file contents have not yet been written.
cmake -E touch "%OCG_ROOT%\src\_cxxbridge.cpp"
cmake -G "NMake Makefiles" ^
    -DCMAKE_BUILD_TYPE=Release ^
    -DCMAKE_INSTALL_PREFIX=%INSTALL_DIR% ^
    -DRUST_BUILD_DIR=%RUST_BUILD_DIR% ^
    -DRUST_INCLUDE_DIR=%RUST_INCLUDE_DIR% ^
    ..

nmake /F Makefile clean
nmake /F Makefile all
nmake /F Makefile install

:: Return back project root directory.
CHDIR "%OCG_ROOT%"
