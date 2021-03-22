#ifndef OPENCOMPGRAPH_IMAGEIO_H
#define OPENCOMPGRAPH_IMAGEIO_H

#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>
#include "opencompgraph/symbol_export.h"

namespace open_comp_graph {
namespace internal {

// Forward declare.
struct ImageShared;

OCG_API_EXPORT
bool oiio_get_thread_count(int &num_threads);

OCG_API_EXPORT
bool oiio_set_thread_count(int32_t num_threads);

OCG_API_EXPORT
bool oiio_read_image(const rust::String &file_path, ImageShared &image);

OCG_API_EXPORT
bool oiio_write_image(const rust::String &file_path, const ImageShared &image);

} // namespace internal
} // namespace open_comp_graph

#endif // OPENCOMPGRAPH_IMAGEIO_H