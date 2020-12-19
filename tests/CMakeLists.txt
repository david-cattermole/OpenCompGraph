
include(OpenCompGraphUtils)

# Add C++ test executable
add_executable(${target_test_exe}
  main.cpp
  test_a.cpp
  test_b.cpp
  test_c.cpp
  test_d.cpp
  test_e.cpp
  ${opencompgraph_source_files})
target_link_libraries(${target_test_exe} ${target_lib})
target_include_directories(${target_test_exe}
  PUBLIC ${CMAKE_SOURCE_DIR}/include
  PRIVATE ${CMAKE_SOURCE_DIR}/src
  )
set_relative_library_rpath(${target_test_exe} "../lib/")

install(FILES
  DESTINATION "bin")
install(
  TARGETS
  ${target_test_exe}
  ARCHIVE DESTINATION "lib"
  LIBRARY DESTINATION "lib"
  RUNTIME DESTINATION "bin")
