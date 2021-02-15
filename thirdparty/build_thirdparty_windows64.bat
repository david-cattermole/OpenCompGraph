@ECHO OFF
SETLOCAL
::
:: Copyright (C) 2020 David Cattermole.
::
:: This file is part of OpenCompGraph.
::
:: OpenCompGraph is free software: you can redistribute it and/or modify it
:: under the terms of the GNU Lesser General Public License as
:: published by the Free Software Foundation, either version 3 of the
:: License, or (at your option) any later version.
::
:: OpenCompGraph is distributed in the hope that it will be useful,
:: but WITHOUT ANY WARRANTY; without even the implied warranty of
:: MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
:: GNU Lesser General Public License for more details.
::
:: You should have received a copy of the GNU Lesser General Public License
:: along with OpenCompGraph.  If not, see <https://www.gnu.org/licenses/>.
:: ---------------------------------------------------------------------
::
:: Builds the OpenCompGraph project.

:: Clear all build information before re-compiling.
:: Turn this off when wanting to make small changes and recompile.
SET FRESH_BUILD=1

:: Do not edit below, unless you know what you're doing.
::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::

:: What type of build? "Release" or "Debug"?
SET BUILD_TYPE=Release

:: The root of this project.
ECHO THIRDPARTY Root: %CD%

:: The path to where IlmBase will be installed, once it is finally built.
SET ILMBASE_LIB_PATH=%CD%\install\ilmbase_openexr\lib

:: Packages
SET BOOST_ROOT="C:\Program Files\boost_1_61_0"
SET ZLIB_ROOT="%CD%\install\zlib"
SET LIBJPEGTURBO_ROOT="%CD%\install\libjpeg_turbo"
SET LIBPNG_ROOT="%CD%\install\libpng"
SET LIBTIFF_ROOT="%CD%\install\libtiff"
SET ILMBASE_ROOT="%CD%\install\ilmbase_openexr"
SET OPENEXR_ROOT="%CD%\install\ilmbase_openexr"
SET OPENCOLORIO_ROOT="%CD%\install\opencolorio"
SET OPENIMAGEIO_ROOT="%CD%\install\openimageio"
SET PREFIX_PATH=%BOOST_ROOT%;%ZLIB_ROOT%;%LIBJPEGTURBO_ROOT%;%LIBPNG_ROOT%;%LIBTIFF_ROOT%;%ILMBASE_ROOT%;%OPENEXR_ROOT%;%OPENCOLORIO_ROOT%;%OPENIMAGEIO_ROOT%

:: Ignore these paths.
SET IGNORE_PATH="C:\\MinGW\;C:\\MinGW\\bin\;C:\\MinGW\\lib\;C:\\MinGW\\include\;C:\\Program Files\\emacs\\emacs-26.3-x86_64\;C:\\Program Files\\emacs\\emacs-26.3-x86_64\\bin\;C:\\Program Files\\emacs\\emacs-26.3-x86_64\\lib\;C:\\Program Files\\emacs\\emacs-26.3-x86_64\\include\;C:\\Program Files\\Autodesk\\Maya2017\\lib\;C:\\Program Files\\Autodesk\\Maya2018\\lib\;C:\\Program Files\\Autodesk\\Maya2019\\lib\;C:\\Program Files\\Autodesk\\Maya2020\\lib\;"

:: Build plugin
MKDIR build_windows64_thirdparty_%BUILD_TYPE%
CHDIR build_windows64_thirdparty_%BUILD_TYPE%
IF "%FRESH_BUILD%"=="1" (
    DEL /S /Q *
    FOR /D %%G in ("*") DO RMDIR /S /Q "%%~nxG"
)

:: HACK: Adjust the runtime PATH environment variable before CMake
:: runs so that OpenEXR will know where IlmBase libraries will exist,
:: so, that "b44ExpLogTable" and "dwaLookups" executables can exist.
SET PATH=%PATH%\;%ILMBASE_LIB_PATH%

cmake -G "NMake Makefiles" ^
    -DCMAKE_BUILD_TYPE=%BUILD_TYPE% ^
    -DCMAKE_PREFIX_PATH=%PREFIX_PATH% ^
    -DCMAKE_IGNORE_PATH=%IGNORE_PATH% ^
    ..

nmake /F Makefile clean
nmake /F Makefile all
