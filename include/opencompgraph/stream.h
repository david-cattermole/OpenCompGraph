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
    explicit StreamData(rust::Box<internal::StreamDataImpl> box) noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    rust::Box<internal::StreamDataImpl> get_box() noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    void set_box(rust::Box<internal::StreamDataImpl> box) noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    StreamDataState state() const noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    uint8_t state_id() const noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    uint64_t hash() const noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    BBox2Df display_window() const noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    void set_display_window(BBox2Df value) noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    BBox2Df data_window() const noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    void set_data_window(BBox2Df value) noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    Matrix4 color_matrix() const noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    const internal::PixelBlock& pixel_block() const noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    rust::Slice<const float> pixel_buffer() const noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    uint32_t pixel_width() const noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    uint32_t pixel_height() const noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    uint8_t pixel_num_channels() const noexcept;

private:
    internal::StreamDataImplShared inner;
};

}  // namespace open_comp_graph

#endif //OPENCOMPGRAPH_STREAM_H
