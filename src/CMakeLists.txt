
# Expected inputs.
set(RUST_BUILD_DIR "/path/to/rust/build/directory/" CACHE PATH
  "The path to the compiled rust directory.")
set(RUST_INCLUDE_DIR "/path/to/rust/include/directory/" CACHE PATH
  "The path to the generated include.")

include(OpenCompGraphUtils)

# Find the Rust Library
set(rust_linktime_file "NOT-FOUND")
find_rust_library("opencompgraph_rs" ${RUST_BUILD_DIR} rust_linktime_file)
message(STATUS "Rust Library: " ${rust_linktime_file})

# Generate CXX Bridge files.
find_program(CXXBRIDGE cxxbridge REQUIRED PATHS $ENV{HOME}/.cargo/bin)
message(STATUS "Using cxxbridge: ${CXXBRIDGE}")
set(cxxbridge_in ${CMAKE_SOURCE_DIR}/src/cxxbridge.rs)
set(cxxbridge_source_out ${CMAKE_SOURCE_DIR}/src/_cxxbridge.cpp)
set(cxxbridge_header_out ${CMAKE_SOURCE_DIR}/include/opencompgraph/_cxxbridge.h)
if (MSVC)
  set(cxxbridge_args "--cxx-impl-annotations \"__declspec(dllexport)\"")
elseif (UNIX) # GCC
  set(cxxbridge_args "--cxx-impl-annotations" "\"__attribute__((visibility(\\\"default\\\")))\"")
endif()
add_custom_command(
  OUTPUT ${cxxbridge_source_out} ${cxxbridge_header_out}
  COMMAND ${CXXBRIDGE} ${cxxbridge_in} ${cxxbridge_args} --header --output ${cxxbridge_header_out}
  COMMAND ${CXXBRIDGE} ${cxxbridge_in} ${cxxbridge_args} --output ${cxxbridge_source_out}
  COMMENT "Generating CXX Bridge: ${cxxbridge_header_out} ${cxxbridge_source_out}"
  DEPENDS ${cxxbridge_in})

set(opencompgraph_source_files
  ${CMAKE_CURRENT_SOURCE_DIR}/_cxxbridge.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/cpp.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/cache.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/node.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/graph.cpp)

# Add 'opencompgraph' library as a C++ wrapper around the Rust
# library.
add_library(${target_lib} SHARED ${opencompgraph_source_files})
target_link_libraries(${target_lib}
  ${rust_linktime_file})
target_include_directories(${target_lib}
  PUBLIC ${CMAKE_SOURCE_DIR}/include
  PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/ )
target_include_directories(${target_lib} PUBLIC ${RUST_INCLUDE_DIR})
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
  ARCHIVE DESTINATION "lib"
  LIBRARY DESTINATION "lib"
  RUNTIME DESTINATION "bin")

set(opencompgraph_source_files ${opencompgraph_source_files} PARENT_SCOPE)