
include(OpenCompGraphUtils)

# Add C++ test executable
add_executable(${target_test_exe}
  ${CMAKE_CURRENT_SOURCE_DIR}/main.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/generate_mesh.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/generate_frame_range.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/test_graph/test_graph_create_and_set_attrs.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/test_graph/test_graph_non_linear_graph.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/test_graph/test_graph_no_ops.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/test_graph/test_graph_re_eval_modified_graph.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/test_stream/test_stream_empty_write_geom.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/test_cache/test_cache_read_image_seq.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/test_cache/test_cache_init.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/test_node_lens/test_node_lens.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/test_node_transform/test_node_transform.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/test_node_imageio/test_node_imageio.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/test_node_null/test_node_null.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/test_node_grade/test_node_grade.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/test_node_merge/test_node_merge.cpp
  )
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
