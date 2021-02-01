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
SET FRESH_BUILD=0

:: Do not edit below, unless you know what you're doing.
::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::

:: What type of build? "Release" or "Debug"?
SET BUILD_TYPE=Release

:: The root of this project.
ECHO THIRDPARTY Root: %CD%

:: The path to where IlmBase will be installed, once it is finally built.
SET ILMBASE_LIB_PATH=%CD%\install\ilmbase\lib

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
SET PATH=%PATH%;%ILMBASE_LIB_PATH%

cmake -G "NMake Makefiles" ^
    -DCMAKE_BUILD_TYPE=%BUILD_TYPE% ^
    ..

nmake /F Makefile clean
nmake /F Makefile all
