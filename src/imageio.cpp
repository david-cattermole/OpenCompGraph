#include <iostream>
#include <memory>
#include <rust/cxx.h>
#include <opencompgraph.h>
#include <OpenImageIO/imageio.h>

namespace open_comp_graph {
namespace internal {

void oiio_read_image(const rust::String &file_path, ImageShared &image) {
    // TODO: Read and set the display and data windows.
    const int num_threads = 0;
    bool ok = OIIO::attribute("threads", OIIO::TypeDesc::INT, &num_threads);
    if (!ok){
        return;
    }

    auto filename = std::string(file_path);
    auto in = OIIO::ImageInput::open(filename);
    if (!in) {
        return;
    }
    int subimage = 0;
    int miplevel = 0;
    bool seek_ok = in->seek_subimage(subimage, miplevel);
    if (!seek_ok) {
        return;
    }

    const OIIO::ImageSpec &spec = in->spec();
    int width = spec.width;
    int height = spec.height;
    int num_channels = spec.nchannels;

    // Allocate pixel memory with Rust data structure.
    auto pixel_data_type = PixelDataType::kFloat32;
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
        // For now, we will force convert to float/f32 data types. In
        // the future it would be nice to stay as the native type
        // instead.
        OIIO::TypeDesc::FLOAT,
        pixel_data,
        xstride, ystride, zstride);
    in->close();
}

bool oiio_write_image(const rust::String &file_path, const ImageShared &image) {
    const int num_threads = 0;
    bool ok = OIIO::attribute("threads", OIIO::TypeDesc::INT, &num_threads);
    if (!ok){
        return false;
    }

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
