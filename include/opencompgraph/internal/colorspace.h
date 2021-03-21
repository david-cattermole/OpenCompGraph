#ifndef OPENCOMPGRAPH_COLORSPACE_H
#define OPENCOMPGRAPH_COLORSPACE_H

#include <memory>

#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>
#include "opencompgraph/symbol_export.h"

namespace open_comp_graph {
namespace internal {

OPENCOMPGRAPH_SYMBOL_EXPORT
bool ocio_print_color_spaces();

OPENCOMPGRAPH_SYMBOL_EXPORT
bool ocio_color_convert_inplace(
        rust::Slice<float> pixel_block,
        int width, int height, int num_channels,
        const rust::String &src_color_space,
        const rust::String &dst_color_space);

} // namespace internal
} // namespace open_comp_graph

#endif // OPENCOMPGRAPH_COLORSPACE_H
