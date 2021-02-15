#include <iostream>
#include <memory>
#include <rust/cxx.h>
#include <opencompgraph.h>
#include <opencompgraph/oiio_utils.h>
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

    // Allocate pixel memory with Rust data structure.
    auto pixel_data_type = oiio_format_to_ocg_format(oiio_data_type);
    image.pixel_block->data_resize(width, height, num_channels, pixel_data_type);
    auto pixels = image.pixel_block->as_slice_mut();
    auto pixel_data = pixels.data();

    int chbegin = 0;
    int chend = num_channels;
    OIIO::stride_t xstride = OIIO::AutoStride;
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

    auto filename = std::string(file_path);
    const int32_t xres = image.pixel_block->width();
    const int32_t yres = image.pixel_block->height();
    const int32_t channels = image.pixel_block->num_channels();
    rust::Slice<const float> pixels = image.pixel_block->as_slice();
    const float *pixel_data = pixels.data();

    auto out = OIIO::ImageOutput::create(filename);
    if (!out) {
        return false;
    }

    OIIO::ImageSpec spec(xres, yres, channels, OIIO::TypeDesc::UINT8);
    out->open(filename, spec);
    out->write_image(OIIO::TypeDesc::UINT8, pixel_data);
    out->close();
    return true;
}

} // namespace internal
} // namespace open_comp_graph
