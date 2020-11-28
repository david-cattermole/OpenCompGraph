
# Expected inputs.
set(RUST_BUILD_DIR "/path/to/rust/build/directory/" CACHE PATH
  "The path to the compiled rust directory.")
set(RUST_INCLUDE_DIR "/path/to/rust/include/directory/" CACHE PATH
  "The path to the generated include.")

include(OpenCompGraphUtils)

set(rust_linktime_file "EMPTY")
find_rust_library(
  "opencompgraph_rs"
  ${RUST_BUILD_DIR}
  rust_linktime_file)
message("Rust Library: " ${rust_linktime_file})

# Add 'opencompgraph' library as a C++ wrapper around the Rust
# library.
add_library(${target_lib} SHARED
  lib.cpp
  _cxxbridge.cpp)
target_link_libraries(${target_lib}
  ${rust_linktime_file})
target_include_directories(${target_lib}
  PUBLIC ${CMAKE_SOURCE_DIR}/include
  PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/
  )
target_include_directories(${target_lib}
  PUBLIC ${RUST_INCLUDE_DIR}
  )
if (MSVC)
  target_link_libraries(${target_lib} ws2_32 userenv advapi32 shell32 msvcrt)
elseif (UNIX)
  target_link_libraries(${target_lib} pthread m dl)
endif ()

install(
  TARGETS
  ${target_lib}
  ARCHIVE DESTINATION "lib"
  LIBRARY DESTINATION "lib"
  RUNTIME DESTINATION "bin")
