# Open Compositing Graph project
#
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

set(OPENIMAGEIO_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/thirdparty/install/openimageio"
  CACHE STRING "Root path to installed OpenImageIO.")

# Third-party packages.
find_package(OpenImageIO REQUIRED
  HINTS ${OPENIMAGEIO_ROOT})
message(STATUS "OpenImageIO Found: ${OpenImageIO_FOUND}")
message(STATUS "OpenImageIO Version: ${OpenImageIO_VERSION}")

find_package(OpenColorIO REQUIRED
  HINTS ${OPENCOLORIO_ROOT})
message(STATUS "OpenColorIO Found: ${OpenColorIO_FOUND}")
message(STATUS "OpenColorIO Library: ${OpenColorIO_LIBRARY}")
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

add_subdirectory(src)
add_subdirectory(include)
if(OpenCompGraph_BUILD_TESTS)
  message(STATUS "Build Tests: ON")
  add_subdirectory(tests)
elseif()
  message(STATUS "Build Tests: OFF")
endif()
