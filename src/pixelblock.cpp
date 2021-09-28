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

#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>
#include <opencompgraph/internal/pixelblock.h>

namespace open_comp_graph {
namespace internal {

// Get pixel data pointer from underlying DataBlock.
void*
pixelblock_get_pixel_data_ptr_read_write(
        rust::Box<PixelBlock> &pixel_block) {
    void *pixel_data = nullptr;
    auto pixel_data_type = pixel_block->data_type();
    if (pixel_data_type == open_comp_graph::DataType::kFloat32) {
        auto pixels = pixel_block->as_mut_slice_f32();
        pixel_data = pixels.data();
    } else if (pixel_data_type == open_comp_graph::DataType::kHalf16) {
        auto pixels = pixel_block->as_mut_slice_f16_fake();
        pixel_data = pixels.data();
    } else if (pixel_data_type == open_comp_graph::DataType::kUInt8) {
        auto pixels = pixel_block->as_mut_slice_u8();
        pixel_data = pixels.data();
    } else if (pixel_data_type == open_comp_graph::DataType::kUInt16) {
        auto pixels = pixel_block->as_mut_slice_u16();
        pixel_data = pixels.data();
    }
    return pixel_data;
}

// Get pixel data pointer from underlying DataBlock.
const void*
pixelblock_get_pixel_data_ptr_read_only(
        const rust::Box<PixelBlock> &pixel_block) {
    const void *pixel_data = nullptr;
    auto pixel_data_type = pixel_block->data_type();
    if (pixel_data_type == open_comp_graph::DataType::kFloat32) {
        auto pixels = pixel_block->as_slice_f32();
        pixel_data = pixels.data();
    } else if (pixel_data_type == open_comp_graph::DataType::kHalf16) {
        auto pixels = pixel_block->as_slice_f16_fake();
        pixel_data = pixels.data();
    } else if (pixel_data_type == open_comp_graph::DataType::kUInt8) {
        auto pixels = pixel_block->as_slice_u8();
        pixel_data = pixels.data();
    } else if (pixel_data_type == open_comp_graph::DataType::kUInt16) {
        auto pixels = pixel_block->as_slice_u16();
        pixel_data = pixels.data();
    }
    return pixel_data;
}


} // namespace internal
} // namespace open_comp_graph
