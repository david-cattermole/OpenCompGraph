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

#ifndef OPENCOMPGRAPH_COLOR_LUT_IMAGE_H
#define OPENCOMPGRAPH_COLOR_LUT_IMAGE_H

#include <memory>
#include <string>
#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>
#include <opencompgraph/cache.h>
#include <opencompgraph/stream.h>
#include "symbol_export.h"

namespace open_comp_graph {

OCG_API_EXPORT
internal::ImageShared get_color_transform_3dlut(
    rust::Str from_color_space,
    rust::Str to_color_space,
    int32_t edge_size,  // Common values; 20, 32 or 64.
    std::shared_ptr<Cache> &cache) noexcept;

OCG_API_EXPORT
internal::ImageShared get_color_ops_lut(
    StreamData &stream_data,
    int32_t edge_size,    // Common values; 20, 32 or 64.
    int32_t num_channels, // Number of channels; must be above 0.
    std::shared_ptr<Cache> &cache) noexcept;

} // namespace open_comp_graph

#endif // OPENCOMPGRAPH_COLOR_LUT_IMAGE_H
