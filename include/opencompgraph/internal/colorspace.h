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
 */

#ifndef OPENCOMPGRAPH_COLORSPACE_H
#define OPENCOMPGRAPH_COLORSPACE_H

#include <memory>

#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>
#include "opencompgraph/symbol_export.h"

namespace open_comp_graph {
namespace internal {

OCG_API_EXPORT
bool ocio_print_color_spaces();

OCG_API_EXPORT
bool oiio_color_convert_inplace(
        rust::Slice<float> pixel_block,
        int width, int height, int num_channels,
        int alpha_channel_index,
        bool unassociated_alpha,
        const rust::String &src_color_space,
        const rust::String &dst_color_space);

} // namespace internal
} // namespace open_comp_graph

#endif // OPENCOMPGRAPH_COLORSPACE_H
