# Open Compositing Graph project
#
cmake_minimum_required(VERSION 2.8)

project(OpenCompGraph)
set(CMAKE_CXX_STANDARD 11)

# Project configuration.
project(compgraph)
set(PROJECT_VERSION_MAJOR 0)
set(PROJECT_VERSION_MINOR 1)
set(PROJECT_VERSION_PATCH 0)
set(PROJECT_VERSION "${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}.${PROJECT_VERSION_PATCH}")
set(PROJECT_HOMEPAGE_URL "https://github.com/david-cattermole/OpenCompGraph")
set(PROJECT_AUTHOR "David Cattermole")
set(PROJECT_COPYRIGHT "2019, David Cattermole.")

# Source
set(SOURCE_FILES
        src/main.cc
        )

# 'OpenCompGraph' library
add_library(OpenCompGraph SHARED ${SOURCE_FILES})
target_include_directories(OpenCompGraph
        PRIVATE src
        )
