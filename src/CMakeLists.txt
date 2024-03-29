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

# Expected inputs.
set(OpenCompGraph_RUST_BUILD_DIR "/path/to/rust/build/directory/" CACHE PATH
  "The path to the compiled rust directory.")
set(OpenCompGraph_RUST_INCLUDE_DIR "/path/to/rust/include/directory/" CACHE PATH
  "The path to the generated include.")

include(OpenCompGraphUtils)

# Find the Rust Library
set(rust_linktime_file "NOT-FOUND")
find_rust_library("opencompgraph_rs" ${OpenCompGraph_RUST_BUILD_DIR} rust_linktime_file)
message(STATUS "Rust Library: " ${rust_linktime_file})

# Generate CXX Bridge files.
find_program(CXXBRIDGE cxxbridge REQUIRED PATHS $ENV{HOME}/.cargo/bin)
message(STATUS "Using cxxbridge: ${CXXBRIDGE}")
set(cxxbridge_in ${CMAKE_CURRENT_SOURCE_DIR}/cxxbridge.rs)
set(cxxbridge_source_out ${CMAKE_CURRENT_SOURCE_DIR}/_cxxbridge.cpp)
set(cxxbridge_header_out ${CMAKE_CURRENT_SOURCE_DIR}/../include/opencompgraph/_cxxbridge.h)
set(cxxbridge_args "--cxx-impl-annotations" "OCG_API_EXPORT")
add_custom_command(
  OUTPUT ${cxxbridge_source_out} ${cxxbridge_header_out}
  COMMAND ${CXXBRIDGE} ${cxxbridge_in} ${cxxbridge_args} --header --output ${cxxbridge_header_out}
  COMMAND ${CXXBRIDGE} ${cxxbridge_in} ${cxxbridge_args} --output ${cxxbridge_source_out}
  COMMENT "Generating CXX Bridge: ${cxxbridge_header_out} ${cxxbridge_source_out}"
  DEPENDS ${cxxbridge_in})

set(opencompgraph_source_files
  ${CMAKE_CURRENT_SOURCE_DIR}/_cxxbridge.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/cpp.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/colorspace.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/colorlutimage.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/systemmemory.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/config.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/cache.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/imageio.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/deformer/ldpk_utils.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/ops/imageresample.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/oiio_utils.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/pixelblock.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/node.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/graph.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/stream.cpp)

# Add 'opencompgraph' library as a C++ wrapper around the Rust
# library.
add_library(${target_lib} SHARED ${opencompgraph_source_files})
target_link_libraries(${target_lib}
  ${rust_linktime_file}
  OpenImageIO::OpenImageIO
  OpenColorIO
)

target_include_directories(${target_lib}
    PUBLIC
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/../include>
        $<INSTALL_INTERFACE:include>
        ${OpenCompGraph_RUST_INCLUDE_DIR}
    PRIVATE
        ${CMAKE_CURRENT_SOURCE_DIR}
        ${OPENIMAGEIO_INCLUDES}
        ${OpenColorIO_INCLUDE_DIR}
        ${ldpk_INCLUDE_DIRS}
        ${dlfcn-win32_INCLUDE_DIRS}
)

if (MSVC)
  target_link_libraries(${target_lib} ws2_32 userenv advapi32 shell32)
  if (CMAKE_BUILD_TYPE STREQUAL "Release")
    target_link_libraries(${target_lib} msvcrt)
  else ()
    target_link_libraries(${target_lib} msvcrtd)
  endif()
elseif (UNIX)
  target_link_libraries(${target_lib} pthread m dl)
endif ()

install(
  TARGETS
  ${target_lib}
  ARCHIVE DESTINATION "${OpenCompGraph_INSTALL_PREFIX}lib"
  LIBRARY DESTINATION "${OpenCompGraph_INSTALL_PREFIX}lib"
  RUNTIME DESTINATION "${OpenCompGraph_INSTALL_PREFIX}bin")
