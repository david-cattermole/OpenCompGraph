@ECHO OFF
SETLOCAL
:: Builds project under MS Windows

:: Clear all build information before re-compiling.
:: Turn this off when wanting to make small changes and recompile.
SET FRESH_BUILD=1

:: The root of this project.
SET THIS_DIR=%~dp0
SET OCG_ROOT=%THIS_DIR%
ECHO OpenCompGraph Root: %OCG_ROOT%
CHDIR %OCG_ROOT%

:: Install directory
SET INSTALL_DIR="%OCG_ROOT%\install"
SET THIRDPARTY_INSTALL_DIR="%OCG_ROOT%\thirdparty\install"

:: Should we build tests?
SET BUILD_TESTS=1

:: Where to find the Rust libraries and headers.
SET RUST_BUILD_DIR="%OCG_ROOT%\target\release"
SET RUST_INCLUDE_DIR="%OCG_ROOT%\include"

:: Install root paths for various third-party components.
SET ZLIB_ROOT="%THIRDPARTY_INSTALL_DIR%\zlib"
SET TIFF_ROOT="%THIRDPARTY_INSTALL_DIR%\libtiff"
SET PNG_ROOT="%THIRDPARTY_INSTALL_DIR%\libpng"
SET JPEG_TURBO_ROOT="%THIRDPARTY_INSTALL_DIR%\libjpeg_turbo"
SET ILMBASE_ROOT="%THIRDPARTY_INSTALL_DIR%\ilmbase_openexr"
SET OPENEXR_ROOT="%THIRDPARTY_INSTALL_DIR%\ilmbase_openexr"
SET OPENCOLORIO_ROOT="%THIRDPARTY_INSTALL_DIR%\opencolorio"
SET OPENIMAGEIO_ROOT="%THIRDPARTY_INSTALL_DIR%\openimageio"

:: Special env-vars that is picked up by build scripts.
SET OpenImageIO_ROOT=%OPENIMAGEIO_ROOT%

:: Build Rust
::
:: Assumes 'cxxbridge-cmd' is installed.
call %OCG_ROOT%\scripts\build_rust_windows64.bat

:: Build C++
MKDIR build_windows
CHDIR build_windows
IF "%FRESH_BUILD%"=="1" (
    DEL /S /Q *
    FOR /D %%G in ("*") DO RMDIR /S /Q "%%~nxG"
)
:: HACK: Create empty file so that CMake can use add_executable, but
:: the file contents have not yet been written.
cmake -E touch "%OCG_ROOT%\src\_cxxbridge.cpp"
cmake -G "NMake Makefiles" ^
    -DCMAKE_BUILD_TYPE=Release ^
    -DCMAKE_INSTALL_PREFIX=%INSTALL_DIR% ^
    -DCMAKE_CXX_STANDARD=11 ^
    -DOpenCompGraph_BUILD_TESTS=%BUILD_TESTS% ^
    -DOpenCompGraph_RUST_BUILD_DIR=%RUST_BUILD_DIR% ^
    -DOpenCompGraph_RUST_INCLUDE_DIR=%RUST_INCLUDE_DIR% ^
    -DOPENIMAGEIO_ROOT=%OPENIMAGEIO_ROOT% ^
    ..

nmake /F Makefile clean
nmake /F Makefile all
nmake /F Makefile install

:: Return back project root directory.
CHDIR "%OCG_ROOT%"
