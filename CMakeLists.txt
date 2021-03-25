#
# Copyright (C) 2020, 2021 David Cattermole.
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
# along with OpenCompGraph.  If not, see <https:#www.gnu.org/licenses/>.
# ====================================================================
#

# Open Compositing Graph project
cmake_minimum_required(VERSION 3.12.0)

# Project configuration.
project(OpenCompGraph
        VERSION 0.1.0)

# Project configuration.
set(PROJECT_HOMEPAGE_URL "https://github.com/david-cattermole/OpenCompGraph")
set(PROJECT_DESCRIPTION "Image compositing node graph library.")
set(PROJECT_AUTHOR "David Cattermole")
set(PROJECT_COPYRIGHT "2019, 2020, David Cattermole.")

# Add custom modules to the module path
set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} ${CMAKE_CURRENT_SOURCE_DIR}/cmake/modules)
include(OpenCompGraphUtils)

# Enable C++11 as minimum
set(CMAKE_CXX_STANDARD_REQUIRED ON)
if (CMAKE_CXX_STANDARD EQUAL "")
  info("OpenCompGraph does not support less than C++11.")
  set (CMAKE_CXX_STANDARD 11)
endif()

set(target_lib "opencompgraph")
set(target_test_exe "opencompgraph_tests")
set(OpenCompGraph_INSTALL_PREFIX ""
  CACHE STRING "String added to the install path for libraries and binaries.")
set(OpenCompGraph_BUILD_TESTS OFF
  CACHE BOOL "builds Tests for OpenCompGraph?.")


# Third-party packages.
set(OPENIMAGEIO_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/thirdparty/install/openimageio"
  CACHE STRING "Root path to installed OpenImageIO.")
find_package(OpenImageIO REQUIRED
  HINTS ${OPENIMAGEIO_ROOT})
message(STATUS "OpenImageIO Found: ${OpenImageIO_FOUND}")
message(STATUS "OpenImageIO Version: ${OpenImageIO_VERSION}")

set(OPENCOLORIO_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/thirdparty/install/opencolorio"
  CACHE STRING "Root path to installed OpenColorIO.")
find_package(OpenColorIO REQUIRED
  HINTS ${OPENCOLORIO_ROOT})
message(STATUS "OpenColorIO Found: ${OpenColorIO_FOUND}")
message(STATUS "OpenColorIO Libraries: ${OpenColorIO_LIBRARIES}")
message(STATUS "OpenColorIO Include: ${OpenColorIO_INCLUDE_DIRS}")

# HACK: OpenColorIO's installed CMake configs set
# "IMPORTED_LINK_INTERFACE_LIBRARIES_RELEASE" to
# "TINYXML_LIB;YAML_CPP_LIB" which are invalid library file names
# because they are statically linked with OpenColorIO. Invalid library
# names cause a linking error, therefore we need to remove these
# libraries before they are linked and cause the problem.
set_target_properties(OpenColorIO PROPERTIES
  IMPORTED_LINK_INTERFACE_LIBRARIES_RELEASE "")

set(LDPK_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/thirdparty/install/ldpk"
  CACHE STRING "Root path to installed LDPK.")
find_package(ldpk REQUIRED
  HINTS ${LDPK_ROOT})
message(STATUS "LDPK Found: ${ldpk_FOUND}")
message(STATUS "LDPK Libraries: ${ldpk_LIBRARIES}")
message(STATUS "LDPK Include: ${ldpk_INCLUDE_DIRS}")

set(DLFCN_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/thirdparty/install/dlfcn"
  CACHE STRING "Root path to installed dlfcn.")
find_package(dlfcn-win32 REQUIRED
  HINTS ${DLFCN_ROOT})
message(STATUS "DLFCN Found: ${dlfcn-win32_FOUND}")
message(STATUS "DLFCN Libraries: ${dlfcn-win32_LIBRARIES}")
message(STATUS "DLFCN Include: ${dlfcn-win32_INCLUDE_DIRS}")

add_subdirectory(src)
add_subdirectory(include)
if(OpenCompGraph_BUILD_TESTS)
  message(STATUS "Build Tests: ON")
  add_subdirectory(tests)
elseif()
  message(STATUS "Build Tests: OFF")
endif()
