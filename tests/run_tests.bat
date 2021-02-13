@ECHO OFF
SETLOCAL
:: MS Windows test build script.
::
:: Sets up the PATH for the required libraries needed by OpenCompGraph.

:: The root of this project.
SET THIS_DIR=%~dp0
SET OCG_ROOT=%THIS_DIR%..
ECHO OpenCompGraph Root: %OCG_ROOT%
CHDIR %OCG_ROOT%

:: Install directory
SET INSTALL_DIR=%OCG_ROOT%\install
SET THIRDPARTY_INSTALL_DIR=%OCG_ROOT%\thirdparty\install\

:: Install root paths for various third-party components.
SET ZLIB_ROOT=%THIRDPARTY_INSTALL_DIR%\zlib
SET JPEG_TURBO_ROOT=%THIRDPARTY_INSTALL_DIR%\libjpeg_turbo
SET PNG_ROOT=%THIRDPARTY_INSTALL_DIR%\libpng
SET TIFF_ROOT=%THIRDPARTY_INSTALL_DIR%\libtiff
SET ILMBASE_ROOT=%THIRDPARTY_INSTALL_DIR%\ilmbase_openexr
SET OPENEXR_ROOT=%THIRDPARTY_INSTALL_DIR%\ilmbase_openexr
SET OPENCOLORIO_ROOT=%THIRDPARTY_INSTALL_DIR%\opencolorio
SET OPENIMAGEIO_ROOT=%THIRDPARTY_INSTALL_DIR%\openimageio
SET BOOST_ROOT=C:\Program Files\boost_1_61_0

:: Set PATH to find runtime libraries.
::
:: NOTE: These paths override any others in the PATH.
SET PATH=%ZLIB_ROOT%\bin;%TIFF_ROOT%\bin;%JPEG_TURBO_ROOT%\bin;%PNG_ROOT%\bin;%ILMBASE_ROOT%\lib;%OPENEXR_ROOT%\lib;%OPENCOLORIO_ROOT%\bin;%OPENIMAGEIO_ROOT%\bin;%BOOST_ROOT%\lib64-msvc-14.0;%PATH%

:: Run test!
SET OPENCOMPGRAPH_LOG=warn
CALL %INSTALL_DIR%\bin\opencompgraph_tests
