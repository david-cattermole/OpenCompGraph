#ifndef OPENCOMPGRAPH_SYSTEM_MEMORY_UTILS_H
#define OPENCOMPGRAPH_SYSTEM_MEMORY_UTILS_H

#include <opencompgraph/symbol_export.h>

namespace open_comp_graph {
namespace internal {

// Returns the number of bytes of system memory available on the
// current machine. If an error occured, 0 (zero) is returned.
OCG_API_EXPORT
size_t get_total_system_memory_as_bytes();

} // namespace internal
} // namespace open_comp_graph

#endif // OPENCOMPGRAPH_SYSTEM_MEMORY_UTILS_H
