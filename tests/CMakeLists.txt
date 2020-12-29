
include(OpenCompGraphUtils)

# Add C++ test executable
add_executable(${target_test_exe}
  ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/test_a.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/test_b.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/test_c.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/test_d.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/test_e.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/test_f.cpp)
target_link_libraries(${target_test_exe} ${target_lib})
target_include_directories(${target_test_exe}
  PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/../include/
  PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/../src/
  )
# Required on Linux and MacOS, does nothing on Windows.
set_relative_library_rpath(${target_test_exe} "../lib/")

install(FILES
  DESTINATION "bin")
install(
  TARGETS
  ${target_test_exe}
  ARCHIVE DESTINATION "${OpenCompGraph_INSTALL_PREFIX}lib"
  LIBRARY DESTINATION "${OpenCompGraph_INSTALL_PREFIX}lib"
  RUNTIME DESTINATION "${OpenCompGraph_INSTALL_PREFIX}bin")
