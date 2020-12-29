#ifndef OPENCOMPGRAPH_CBINDGEN_H
#define OPENCOMPGRAPH_CBINDGEN_H

/* Generated with cbindgen:0.16.0 */

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>
#include "symbol_export.h"

namespace open_comp_graph {
namespace cbindgen {

/// The Scene Graph.
struct SceneGraph {
  int id;
};

extern "C" {

OPENCOMPGRAPH_SYMBOL_EXPORT SceneGraph *scene_graph_new(int id);

OPENCOMPGRAPH_SYMBOL_EXPORT
void scene_graph_delete(SceneGraph *scene_graph_ptr);

} // extern "C"

} // namespace cbindgen
} // namespace open_comp_graph

#endif // OPENCOMPGRAPH_CBINDGEN_H
