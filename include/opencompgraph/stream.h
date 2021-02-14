#ifndef OPENCOMPGRAPH_STREAM_H
#define OPENCOMPGRAPH_STREAM_H

#include <memory>
#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>
#include "symbol_export.h"

namespace open_comp_graph {

class StreamData {
public:
    OPENCOMPGRAPH_SYMBOL_EXPORT
    StreamData() noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    explicit StreamData(rust::Box<internal::StreamDataImplRc> box) noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    rust::Box<internal::StreamDataImplRc> get_box() noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    void set_box(rust::Box<internal::StreamDataImplRc> box) noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    StreamDataState state() const noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    uint8_t state_id() const noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    uint64_t hash() const noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    BBox2Di display_window() const noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    BBox2Di data_window() const noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    Matrix4 color_matrix() const noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    Matrix4 transform_matrix() const noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    size_t deformers_len() const noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    void apply_deformers(rust::Slice<float> &buffer) noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    rust::Slice<const float> pixel_buffer() const noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    int32_t pixel_width() const noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    int32_t pixel_height() const noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    int32_t pixel_num_channels() const noexcept;

private:
    internal::StreamDataImplShared inner;
};

}  // namespace open_comp_graph

#endif //OPENCOMPGRAPH_STREAM_H
