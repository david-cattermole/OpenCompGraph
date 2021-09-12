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
// #include <sstream>  // stringstream
#include <cctype>   // tolower
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
    auto padded_num_channels =
        static_cast<int32_t>(stride_num_channels(num_channels, pixel_data_type));
    auto channel_num_bytes = channel_size_bytes(pixel_data_type);
    image.pixel_block->data_resize(
        width, height, padded_num_channels, pixel_data_type);
    auto pixels = image.pixel_block->as_slice_mut();
    auto pixel_data = pixels.data();

    // Read image metadata.
    //
    // https://openimageio.readthedocs.io/en/release-2.2.8.0/stdmetadata.html#cmdoption-arg-oiio-ColorSpace
    //
    // TODO: Add more metadata support, such as these commonly used
    // metadata fields:
    //
    // "DateTime"
    // "ImageDescription"
    // "Artist"
    // "DocumentName"
    // "ResolutionUnit"
    // "XResolution"
    // "YResolution"
    //
    std::string colorspace_text = spec.get_string_attribute("oiio:ColorSpace", "");
    float gamma = spec.get_float_attribute("oiio:Gamma", 1.0f);
    float pixel_aspect = spec.get_float_attribute("PixelAspectRatio", 1.0f);
    int orientation = spec.get_int_attribute("Orientation", 0);
    int unassociated_alpha = spec.get_int_attribute("oiio:UnassociatedAlpha", 0);
    std::string desc_text = spec.get_string_attribute("ImageDescription", "");
    int dither = spec.get_int_attribute("oiio:dither", 0);

    // std::cerr << "IN Color Space: " << colorspace_text << '\n';
    // std::cerr << "IN Gamma: " << gamma << '\n';
    // std::cerr << "IN Pixel Aspect: " << pixel_aspect << '\n';
    // std::cerr << "IN Orientation: " << orientation << '\n';
    // std::cerr << "IN Unassociated Alpha: " << unassociated_alpha << '\n';
    // std::cerr << "IN Description: " << desc_text << '\n';
    // std::cerr << "IN Dither: " << dither << '\n';

    auto color_space = rust::String(colorspace_text);
    image.spec.color_space = color_space;
    image.spec.gamma = gamma;
    image.spec.pixel_aspect = pixel_aspect;
    image.spec.orientation = static_cast<ImageOrientation>(orientation);
    image.spec.unassociated_alpha = unassociated_alpha != 0;
    image.spec.dither = dither;

    // // TODO: Find which channels are z-depth and which channels are
    // // alpha.
    //
    // // Read channel names.
    // auto in = ImageInput::open (filename);
    // const ImageSpec &spec = in->spec();
    // for (int i = 0;  i < spec.nchannels;  ++i)
    //     std::cout << "Channel " << i << " is "
    //               << spec.channelnames[i] << "\n";

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

bool name_has_suffix(std::string const &name, std::string const &suffix) {
    if (name.length() >= suffix.length()) {
        return 0 == name.compare(name.length() - suffix.length(), suffix.length(), suffix);
    }
    return false;
}


bool oiio_write_image(const rust::String &file_path, const ImageShared &image,
                      const ImageCompression &compress) {
    const int32_t xres = image.pixel_block->width();
    const int32_t yres = image.pixel_block->height();
    const int32_t channels = image.pixel_block->num_channels();
    rust::Slice<const float> pixels = image.pixel_block->as_slice();
    const float *pixel_data = pixels.data();

    auto filename = std::string(file_path);
    auto out = OIIO::ImageOutput::create(filename);
    if (!out) {
        std::cerr
            << "No valid OIIO output plug-in could be found. "
            << "Cannot write image!\n";
        return false;
    }
    // std::cerr << "Output Format: " << out->format_name() << '\n';

    auto type_desc = ocg_format_to_oiio_format(image.pixel_block->pixel_data_type());
    OIIO::ImageSpec spec(xres, yres, channels, type_desc);
    spec.full_x = image.display_window.min_x;
    spec.full_y = image.display_window.min_y;
    spec.full_width = image.display_window.max_x - image.display_window.min_x;
    spec.full_height = image.display_window.max_y - image.display_window.min_y;
    spec.x = image.data_window.min_x;
    spec.y = image.data_window.min_y;

    // Set the 'colorspace', and other metadata, so that the image
    // writing can correctly convert the data for the intended format.
    auto color_space_str = std::string(image.spec.color_space);
    auto gamma = static_cast<float>(image.spec.gamma);
    auto pixel_aspect = static_cast<float>(image.spec.pixel_aspect);
    auto orientation = static_cast<int32_t>(image.spec.orientation);
    auto unassociated_alpha = static_cast<int32_t>(image.spec.unassociated_alpha);
    auto dither = static_cast<int32_t>(image.spec.dither);
    // std::cerr << "OUT Color Space: " << color_space_str << '\n';
    // std::cerr << "OUT Gamma: " << gamma << '\n';
    // std::cerr << "OUT Pixel Aspect: " << pixel_aspect << '\n';
    // std::cerr << "OUT Orientation: " << orientation << '\n';
    // std::cerr << "OUT Unassociated Alpha: " << unassociated_alpha << '\n';
    // std::cerr << "OUT Dither: " << dither << '\n';
    spec.attribute("oiio:ColorSpace", color_space_str);
    spec.attribute("oiio:Gamma", gamma);
    spec.attribute("PixelAspectRatio", pixel_aspect);
    spec.attribute("Orientation", orientation);
    spec.attribute("oiio:dither", dither);
    spec.attribute("oiio:UnassociatedAlpha", unassociated_alpha);

    // Image Compression
    {
        std::string filename_lower = filename;
        std::transform(
            filename_lower.begin(),
            filename_lower.end(),
            filename_lower.begin(),
            [](unsigned char c){ return std::tolower(c); }
        );

        if (name_has_suffix(filename_lower, ".exr")) {
            auto exr_compress_name = "Compression";
            auto exr_dwa_level_name = "openexr:dwaCompressionLevel";
            auto dwa_level = static_cast<float>(compress.exr_dwa_compression_level);
            if (compress.exr_compression == open_comp_graph::ExrCompression::kNone) {
                spec.attribute(exr_compress_name, "none");
            } else if (compress.exr_compression == open_comp_graph::ExrCompression::kRle) {
                spec.attribute(exr_compress_name, "rle");
            } else if (compress.exr_compression == open_comp_graph::ExrCompression::kZip) {
                spec.attribute(exr_compress_name, "zip");
            } else if (compress.exr_compression == open_comp_graph::ExrCompression::kZipScanline) {
                spec.attribute(exr_compress_name, "zips");
            } else if (compress.exr_compression == open_comp_graph::ExrCompression::kPiz) {
                spec.attribute(exr_compress_name, "piz");
            } else if (compress.exr_compression == open_comp_graph::ExrCompression::kPxr24) {
                spec.attribute(exr_compress_name, "pxr24");
            } else if (compress.exr_compression == open_comp_graph::ExrCompression::kB44) {
                spec.attribute(exr_compress_name, "b44");
            } else if (compress.exr_compression == open_comp_graph::ExrCompression::kB44a) {
                spec.attribute(exr_compress_name, "b44a");
            } else if (compress.exr_compression == open_comp_graph::ExrCompression::kDwaa) {
                spec.attribute(exr_compress_name, "dwaa");
                spec.attribute(exr_dwa_level_name, dwa_level);
            } else if (compress.exr_compression == open_comp_graph::ExrCompression::kDwab) {
                spec.attribute(exr_compress_name, "dwab");
                spec.attribute(exr_dwa_level_name, dwa_level);
            }

        } else if (name_has_suffix(filename_lower, ".png")) {
            spec.attribute(
                "png:compressionLevel",
                compress.png_compression_level);

        } else if (name_has_suffix(filename_lower, ".jpg")
                   || name_has_suffix(filename_lower, ".jpeg")
                   || name_has_suffix(filename_lower, ".jpe")
                   || name_has_suffix(filename_lower, ".jif")
                   || name_has_suffix(filename_lower, ".jfif")
                   || name_has_suffix(filename_lower, ".jfi")) {
            spec.attribute("Compression", "jpeg");
            spec.attribute("CompressionQuality", compress.jpeg_compression_level);

            auto jpeg_subsamp_name = "jpeg:subsampling";
            if (compress.jpeg_subsampling == open_comp_graph::JpegChromaSubSampling::kNone444) {
                spec.attribute(jpeg_subsamp_name, "4:4:4");
            } else if (compress.jpeg_subsampling == open_comp_graph::JpegChromaSubSampling::kSample422) {
                spec.attribute(jpeg_subsamp_name, "4:2:2");
            } else if (compress.jpeg_subsampling == open_comp_graph::JpegChromaSubSampling::kSample420) {
                spec.attribute(jpeg_subsamp_name, "4:2:0");
            } else if (compress.jpeg_subsampling == open_comp_graph::JpegChromaSubSampling::kSample421) {
                spec.attribute(jpeg_subsamp_name, "4:2:1");
            }

            spec.attribute(
                "jpeg:progressive",
                static_cast<int32_t>(compress.jpeg_progressive));
        }
    }

    // // Mark the file that it's been created using OpenCompGraph.
    // auto software = std::string("OpenCompGraph");
    // spec.attribute("Software", software);

    out->open(filename, spec);
    auto ok = out->write_image(type_desc, pixel_data);
    if (!ok) {
        auto error_message = out->geterror();
        std::cerr << "ERROR: " << error_message << '\n';
    }
    out->close();
    return true;
}

} // namespace internal
} // namespace open_comp_graph
