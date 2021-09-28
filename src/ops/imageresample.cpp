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
#include <cmath>

#include <rust/cxx.h>

#include <opencompgraph.h>
#include <opencompgraph/internal/oiio_utils.h>
#include <opencompgraph/internal/pixelblock.h>

// OIIO
#include <OpenImageIO/imagebufalgo.h>

namespace open_comp_graph {
namespace internal {

bool oiio_image_resample(
        ImageShared &src_image,
        ImageShared &dst_image,
        int factor_num,
        bool interpolate) {
    bool ok = false;
    auto pixel_data_type = src_image.pixel_block->data_type();
    auto num_channels = src_image.pixel_block->num_channels();
    auto src_width = src_image.pixel_block->width();
    auto src_height = src_image.pixel_block->height();

    // Wrap source image as buffer
    OIIO::ImageSpec src_spec;
    ok = oiio_construct_spec(
        src_width, src_height,
        num_channels,
        pixel_data_type,
        src_spec);
    if (!ok) {
        return ok;
    }

    auto src_pixel_data = pixelblock_get_pixel_data_ptr_read_write(
        src_image.pixel_block);
    if (src_pixel_data == nullptr) {
        return false;
    }
    auto src_image_buffer = OIIO::ImageBuf::ImageBuf(src_spec, src_pixel_data);
    auto src_roi = OIIO::get_roi_full(src_spec);

    // Calculate new image dimensions. New image cannot be more than
    // 16K, or less than 1 pixel.
    auto new_width = static_cast<int32_t>(src_width * std::pow(2, factor_num));
    auto new_height = static_cast<int32_t>(src_height * std::pow(2, factor_num));
    new_width = std::max(1, new_width);
    new_height = std::max(1, new_height);
    new_width = std::min(16384, new_width);
    new_height = std::min(16384, new_height);

    // Construct destination ImageSpec.
    OIIO::ImageSpec dst_spec;
    ok = oiio_construct_spec(
        new_width, new_height,
        num_channels,
        pixel_data_type,
        dst_spec);
    if (!ok) {
        return ok;
    }

    // Allocate OCG Image memory to be written into by OIIO.
    ok = oiio_allocate_image(dst_spec, dst_image);
    if (!ok) {
        return ok;
    }
    auto dst_pixel_data = pixelblock_get_pixel_data_ptr_read_write(
        dst_image.pixel_block);
    if (dst_pixel_data == nullptr) {
        return false;
    }
    auto dst_image_buffer = OIIO::ImageBuf::ImageBuf(dst_spec, dst_pixel_data);

    // Do image processing.
    auto dst_roi = OIIO::get_roi_full(dst_spec);
    ok = OIIO::ImageBufAlgo::resample(
        dst_image_buffer,
        src_image_buffer,
        interpolate,
        dst_roi);
    return ok;
}


} // namespace internal
} // namespace open_comp_graph
