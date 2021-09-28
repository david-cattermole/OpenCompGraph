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

#include <opencompgraph/internal/oiio_utils.h>
#include <OpenImageIO/typedesc.h>
#include <OpenImageIO/imageio.h>

namespace open_comp_graph {
namespace internal {

DataType oiio_format_to_ocg_format(const OIIO::TypeDesc oiio_type_desc) {
    auto data_type = DataType::kUnknown;
    if (oiio_type_desc == OIIO::TypeDesc::UINT8) {
        data_type = DataType::kUInt8;
    } else if (oiio_type_desc == OIIO::TypeDesc::HALF) {
        data_type = DataType::kHalf16;
    } else if (oiio_type_desc == OIIO::TypeDesc::FLOAT) {
        data_type = DataType::kFloat32;
    } else if (oiio_type_desc == OIIO::TypeDesc::UINT16) {
        data_type = DataType::kUInt16;
    }
    return data_type;
}


OIIO::TypeDesc ocg_format_to_oiio_format(const DataType ocg_data_type) {
    auto oiio_type_desc = OIIO::TypeDesc::UNKNOWN;
    if (ocg_data_type == DataType::kUInt8) {
        oiio_type_desc = OIIO::TypeDesc::UINT8;
    } else if (ocg_data_type == DataType::kHalf16) {
        oiio_type_desc = OIIO::TypeDesc::HALF;
    } else if (ocg_data_type == DataType::kFloat32) {
        oiio_type_desc = OIIO::TypeDesc::FLOAT;
    } else if (ocg_data_type == DataType::kUInt16) {
        oiio_type_desc = OIIO::TypeDesc::UINT16;
    }
    return oiio_type_desc;
}

bool oiio_construct_spec(
        int data_window_min_x,
        int data_window_min_y,
        int data_width,
        int data_height,
        int display_window_min_x,
        int display_window_min_y,
        int display_window_max_x,
        int display_window_max_y,
        int num_channels,
        const DataType data_type,
        OIIO::ImageSpec &spec) {
    spec.width = data_width;
    spec.height = data_height;
    spec.nchannels = num_channels;

    auto type_desc = ocg_format_to_oiio_format(data_type);
    spec.format = type_desc;

    spec.full_x = display_window_min_x;
    spec.full_y = display_window_min_y;
    spec.full_width = display_window_max_x - display_window_min_x;
    spec.full_height = display_window_max_y - display_window_min_y;
    spec.x = data_window_min_x;
    spec.y = data_window_min_y;

    return true;
}

bool oiio_construct_spec(
        int data_width,
        int data_height,
        int num_channels,
        const DataType data_type,
        OIIO::ImageSpec &spec) {
    return oiio_construct_spec(
        0, 0,
        data_width, data_height,
        0, 0,
        data_width, data_height,
        num_channels,
        data_type,
        spec);
}

bool oiio_allocate_image(
        const OIIO::ImageSpec &spec,
        ImageShared &image) {

    // Read the data window.
    image.data_window.min_x = spec.x;
    image.data_window.min_y = spec.y;
    image.data_window.max_x = spec.x + spec.width;
    image.data_window.max_y = spec.y + spec.height;

    // Read the display window.
    image.display_window.min_x = spec.full_x;
    image.display_window.min_y = spec.full_y;
    image.display_window.max_x = spec.full_x + spec.full_width;
    image.display_window.max_y = spec.full_y + spec.full_height;

    // Ensure the display window corner starts at 0,0 by removing any
    // non-zero values and pushing the values into the data window.
    image.data_window.min_x += image.display_window.min_x;
    image.data_window.min_y += image.display_window.min_y;
    image.data_window.max_x += image.display_window.min_x;
    image.data_window.max_y += image.display_window.min_y;
    image.display_window.max_x -= image.display_window.min_x;
    image.display_window.max_y -= image.display_window.min_y;
    image.display_window.min_x = 0;
    image.display_window.min_y = 0;

    // Allocate pixel memory with Rust data structure.
    //
    // Make sure the data read is compatible with OpenGL without
    // needing "GL_UNPACK_ALIGNMENT". Maya does not support any pixel
    // formats that align to 48-bytes (such as RGB 8-bit), so we must
    // pad the channels.
    auto data_type = oiio_format_to_ocg_format(spec.format);
    auto padded_num_channels =
        static_cast<int32_t>(stride_num_channels(spec.nchannels, data_type));
    auto channel_num_bytes = channel_size_bytes(data_type);
    auto blocksize = open_comp_graph::BlockSize{
        spec.width,
        spec.height,
        padded_num_channels};
    image.pixel_block->data_resize(
        blocksize, data_type);

    return true;
}

} // namespace internal
} // namespace open_comp_graph
