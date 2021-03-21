#ifndef OPENCOMPGRAPH_STREAM_H
#define OPENCOMPGRAPH_STREAM_H

#include <memory>
#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>
#include "symbol_export.h"

namespace open_comp_graph {

class StreamData {
public:
    OCG_API_EXPORT
    StreamData() noexcept;

    OCG_API_EXPORT
    explicit StreamData(rust::Box<internal::StreamDataImplRc> box) noexcept;

    OCG_API_EXPORT
    rust::Box<internal::StreamDataImplRc> get_box() noexcept;

    OCG_API_EXPORT
    void set_box(rust::Box<internal::StreamDataImplRc> box) noexcept;

    OCG_API_EXPORT
    StreamDataState state() const noexcept;

    OCG_API_EXPORT
    uint8_t state_id() const noexcept;

    OCG_API_EXPORT
    uint64_t hash() const noexcept;

    OCG_API_EXPORT
    BBox2Di display_window() const noexcept;

    OCG_API_EXPORT
    BBox2Di data_window() const noexcept;

    OCG_API_EXPORT
    Matrix4 color_matrix() const noexcept;

    OCG_API_EXPORT
    Matrix4 transform_matrix() const noexcept;

    OCG_API_EXPORT
    size_t deformers_len() const noexcept;

    OCG_API_EXPORT
    void apply_deformers(rust::Slice<float> &buffer,
                         BBox2Df image_window,
                         DeformerDirection direction) noexcept;

    OCG_API_EXPORT
    rust::Slice<const float> pixel_buffer() const noexcept;

    OCG_API_EXPORT
    int32_t pixel_width() const noexcept;

    OCG_API_EXPORT
    int32_t pixel_height() const noexcept;

    OCG_API_EXPORT
    int32_t pixel_num_channels() const noexcept;

    OCG_API_EXPORT
    PixelDataType pixel_data_type() const noexcept;

private:
    internal::StreamDataImplShared inner;
};

}  // namespace open_comp_graph

#endif //OPENCOMPGRAPH_STREAM_H
