# Open Compositing Graph project
#
cmake_minimum_required(VERSION 3.0.0)

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

set_cxx_standard_cpp_11()

set(target_lib "opencompgraph")
set(target_test_exe "opencompgraph_tests")

add_subdirectory(src)
add_subdirectory(include)
add_subdirectory(tests)
