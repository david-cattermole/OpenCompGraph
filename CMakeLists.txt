# Open Compositing Graph project
#
cmake_minimum_required(VERSION 2.8.12)

project(OpenCompGraph)

# Project configuration.
set(PROJECT_VERSION_MAJOR 0)
set(PROJECT_VERSION_MINOR 1)
set(PROJECT_VERSION_PATCH 0)
set(PROJECT_VERSION "${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}.${PROJECT_VERSION_PATCH}")
set(PROJECT_HOMEPAGE_URL "https://github.com/david-cattermole/OpenCompGraph")
set(PROJECT_DESCRIPTION "Image compositing node graph library.")
set(PROJECT_AUTHOR "David Cattermole")
set(PROJECT_COPYRIGHT "2019, 2020, David Cattermole.")

# Add custom modules to the module path
set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} ${CMAKE_CURRENT_SOURCE_DIR}/cmake/modules)
include(OpenCompGraphUtils)

# set(CMAKE_CXX_STANDARD 11)
set_cxx_standard_cpp_11()

set(target_lib "opencompgraph")
set(target_test_exe "opencompgraph_tests")

add_subdirectory(src)
add_subdirectory(include)
add_subdirectory(tests)
