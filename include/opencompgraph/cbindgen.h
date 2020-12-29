#pragma once

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

namespace open_comp_graph {
namespace cbindgen {

/// The Scene Graph.
struct SceneGraph {
  int id;
};

extern "C" {

__declspec(dllexport) void scene_graph_delete(SceneGraph *scene_graph_ptr);

__declspec(dllexport) SceneGraph *scene_graph_new(int id);

} // extern "C"

} // namespace cbindgen
} // namespace open_comp_graph
