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

#include <iostream>
#include <memory>
#include <rust/cxx.h>
#include <opencompgraph.h>
#include <opencompgraph/internal/oiio_utils.h>
#include <OpenImageIO/imageio.h>

namespace open_comp_graph {
namespace internal {

bool oiio_get_thread_count(int32_t &num_threads) {
    return OIIO::getattribute("threads", num_threads);
}

bool oiio_set_thread_count(const int num_threads) {
    return OIIO::attribute("threads", OIIO::TypeDesc::INT, &num_threads);
}

bool oiio_read_image(const rust::String &file_path, ImageShared &image) {
    auto filename = std::string(file_path);
    auto in = OIIO::ImageInput::open(filename);
    if (!in) {
        return false;
    }
    int subimage = 0;
    int miplevel = 0;
    bool seek_ok = in->seek_subimage(subimage, miplevel);
    if (!seek_ok) {
        return false;
    }

    // Note: OpenImageIO assumes the coordinate system origin is upper
    // left corner of the pixel data
    const OIIO::ImageSpec &spec = in->spec();
    int width = spec.width;
    int height = spec.height;
    int num_channels = spec.nchannels;
    auto oiio_data_type = spec.format;

    // We do not support images with channels less than 3 (RGB) or
    // more than 4 (RGBA).
    if (num_channels < 3) {
        return false;
    }
    std::max(num_channels, 4);

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
    auto pixel_data_type = oiio_format_to_ocg_format(oiio_data_type);
    auto padded_num_channels = stride_num_channels(num_channels, pixel_data_type);
    auto channel_num_bytes = channel_size_bytes(pixel_data_type);
    image.pixel_block->data_resize(
        width, height, padded_num_channels, pixel_data_type);
    auto pixels = image.pixel_block->as_slice_mut();
    auto pixel_data = pixels.data();

    // Read image metadata.
    //
    // https://openimageio.readthedocs.io/en/release-2.2.8.0/stdmetadata.html#cmdoption-arg-oiio-ColorSpace
    int orientation = spec.get_int_attribute("Orientation", 0);
    float pixel_aspect = spec.get_float_attribute("PixelAspectRatio", 1.0f);
    std::string colorspace_text = spec.get_string_attribute("oiio:ColorSpace", "");
    std::string desc_text = spec.get_string_attribute("ImageDescription", "");
    int unassociated_alpha = spec.get_int_attribute("oiio:UnassociatedAlpha", 0);

    // std::cout << "Orientation: " << orientation << '\n';
    // std::cout << "Pixel Aspect: " << pixel_aspect << '\n';
    // std::cout << "Color Space: " << colorspace_text << '\n';
    // std::cout << "Unassociated Alpha: " << unassociated_alpha << '\n';
    // std::cout << "Description: " << desc_text << '\n';

    auto color_space = rust::String(colorspace_text);
    image.spec.color_space = color_space;
    image.spec.pixel_aspect = pixel_aspect;
    image.spec.orientation = static_cast<ImageOrientation>(orientation);
    image.spec.unassociated_alpha = unassociated_alpha != 0;

    int chbegin = 0;
    int chend = num_channels;
    auto pixel_size_bytes = padded_num_channels * channel_num_bytes;
    OIIO::stride_t xstride = pixel_size_bytes;
    OIIO::stride_t ystride = OIIO::AutoStride;
    OIIO::stride_t zstride = OIIO::AutoStride;
    in->read_image(
        subimage, miplevel,
        chbegin, chend,
        oiio_data_type,
        pixel_data,
        xstride, ystride, zstride);
    in->close();
    return true;
}

bool oiio_write_image(const rust::String &file_path, const ImageShared &image) {
    const int32_t xres = image.pixel_block->width();
    const int32_t yres = image.pixel_block->height();
    const int32_t channels = image.pixel_block->num_channels();
    rust::Slice<const float> pixels = image.pixel_block->as_slice();
    const float *pixel_data = pixels.data();

    auto filename = std::string(file_path);
    auto out = OIIO::ImageOutput::create(filename);
    if (!out) {
        return false;
    }

    auto type_desc = ocg_format_to_oiio_format(image.pixel_block->pixel_data_type());
    OIIO::ImageSpec spec(xres, yres, channels, type_desc);
    spec.full_x = image.display_window.min_x;
    spec.full_y = image.display_window.min_y;
    spec.full_width = image.display_window.max_x - image.display_window.min_x;
    spec.full_height = image.display_window.max_y - image.display_window.min_y;
    spec.x = image.data_window.min_x;
    spec.y = image.data_window.min_y;

    out->open(filename, spec);
    out->write_image(type_desc, pixel_data);
    out->close();
    return true;
}

} // namespace internal
} // namespace open_comp_graph
