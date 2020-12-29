#ifndef OPENCOMPGRAPH_STREAM_H
#define OPENCOMPGRAPH_STREAM_H

#include <memory>
#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>

namespace open_comp_graph {

class StreamData {
public:
    StreamData() noexcept;
    StreamData(rust::Box<internal::StreamDataImpl> box) noexcept;
    ~StreamData();

    rust::Box<internal::StreamDataImpl> get_box() noexcept;
    void set_box(rust::Box<internal::StreamDataImpl> box) noexcept;

    StreamDataState state() noexcept;
    uint8_t state_id() noexcept;
    uint64_t hash() noexcept;
    const internal::PixelBlock& pixel_block() noexcept;
    rust::Slice<const float> pixel_buffer() noexcept;
    uint32_t pixel_width() noexcept;
    uint32_t pixel_height() noexcept;

private:
    internal::StreamDataImplShared inner;

};

}  // namespace open_comp_graph

#endif //OPENCOMPGRAPH_STREAM_H
