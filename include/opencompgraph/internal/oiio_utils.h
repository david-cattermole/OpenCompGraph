/*
 * Copyright (C) 2020, 2021 David Cattermole.
 *
 * This file is part of OpenCompGraph.
 *
 * OpenCompGraph is free software: you can redistribute it and/or modify it
 * under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * OpenCompGraph is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with OpenCompGraph.  If not, see <https://www.gnu.org/licenses/>.
 * ====================================================================
 *
 * This file is used for C++ only functions related to OpenImageIO.
 */

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
