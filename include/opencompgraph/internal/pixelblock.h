/*
 * Copyright (C) 2021 David Cattermole.
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
 * This file is used for functions related to the PixelBlock.
 */

#ifndef OPENCOMPGRAPH_PIXEL_BLOCK_H
#define OPENCOMPGRAPH_PIXEL_BLOCK_H

#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>
#include "opencompgraph/symbol_export.h"

namespace open_comp_graph {
namespace internal {

OCG_API_EXPORT
void*
pixelblock_get_pixel_data_ptr_read_write(
    rust::Box<PixelBlock> &pixel_block);

OCG_API_EXPORT
const void*
pixelblock_get_pixel_data_ptr_read_only(
    const rust::Box<PixelBlock> &pixel_block);

// OCG_API_EXPORT

} // namespace internal
} // namespace open_comp_graph

#endif // OPENCOMPGRAPH_PIXEL_BLOCK_H
