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
    StreamData(rust::Box<internal::StreamDataImpl> box) noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    ~StreamData();

    OPENCOMPGRAPH_SYMBOL_EXPORT
    rust::Box<internal::StreamDataImpl> get_box() noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    void set_box(rust::Box<internal::StreamDataImpl> box) noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    StreamDataState state() noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    uint8_t state_id() noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    uint64_t hash() noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    const internal::PixelBlock& pixel_block() noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    rust::Slice<const float> pixel_buffer() noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    uint32_t pixel_width() noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    uint32_t pixel_height() noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    uint8_t pixel_num_channels() noexcept;

private:
    internal::StreamDataImplShared inner;

};

}  // namespace open_comp_graph

#endif //OPENCOMPGRAPH_STREAM_H
