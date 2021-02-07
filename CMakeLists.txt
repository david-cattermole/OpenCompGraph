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
set(OpenCompGraph_INSTALL_PREFIX "" CACHE STRING "String added to the install path for libraries and binaries.")

add_subdirectory(src)
add_subdirectory(include)
add_subdirectory(tests)
