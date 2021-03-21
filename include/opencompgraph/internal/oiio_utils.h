// This file is used for C++ only functions related to OpenImageIO.

#ifndef OPENCOMPGRAPH_OIIO_UTILS_H
#define OPENCOMPGRAPH_OIIO_UTILS_H

#include <opencompgraph/_cxxbridge.h>
#include <OpenImageIO/typedesc.h>

namespace open_comp_graph {
namespace internal {

// Not exported to the API.
PixelDataType oiio_format_to_ocg_format(OIIO::TypeDesc oiio_type_desc);

// Not exported to the API.
OIIO::TypeDesc ocg_format_to_oiio_format(PixelDataType ocg_pixel_data_type);

} // namespace internal
} // namespace open_comp_graph

#endif // OPENCOMPGRAPH_OIIO_UTILS_H
