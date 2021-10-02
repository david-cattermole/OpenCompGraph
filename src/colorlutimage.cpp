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

// STL
#include <iostream>
#include <string>

// Rust CXX
#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>
#include <opencompgraph/stream.h>
#include <opencompgraph/cache.h>

#include <opencompgraph/colorlutimage.h>

namespace open_comp_graph {

internal::ImageShared get_color_transform_3dlut(
    rust::Str from_color_space,
    rust::Str to_color_space,
    int32_t edge_size,  // Common values; 20, 32 or 64.
    std::shared_ptr<Cache> &cache) noexcept
{
    auto cache_box = cache->get_box();  // Borrow the underlying cache object.
    auto img = internal::get_color_transform_3dlut(
        from_color_space,
        to_color_space,
        edge_size,
        cache_box);
    cache->set_box(std::move(cache_box));  // Return the cache to it's owner.
    return img;
}

internal::ImageShared get_color_ops_lut(
    StreamData &stream_data,
    int32_t edge_size,    // Common values; 20, 32 or 64.
    int32_t num_channels, // Number of channels; must be above 0.
    std::shared_ptr<Cache> &cache) noexcept
{
    // Borrow the underlying objects.
    auto stream_data_box = stream_data.get_box();
    auto cache_box = cache->get_box();

    auto img = internal::get_color_ops_lut(
        stream_data_box,
        edge_size,
        num_channels,
        cache_box);

    // Return the memory to it's owner.
    cache->set_box(std::move(cache_box));
    stream_data.set_box(std::move(stream_data_box));
    return img;
}

} // namespace open_comp_graph
