#ifndef OPENCOMPGRAPH_STREAM_H
#define OPENCOMPGRAPH_STREAM_H

#include <memory>
#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>

namespace open_comp_graph {

class StreamData {
public:
    __declspec(dllexport) StreamData() noexcept;
    __declspec(dllexport) StreamData(rust::Box<internal::StreamDataImpl> box) noexcept;
    __declspec(dllexport) ~StreamData();

    __declspec(dllexport) rust::Box<internal::StreamDataImpl> get_box() noexcept;
    __declspec(dllexport) void set_box(rust::Box<internal::StreamDataImpl> box) noexcept;

    __declspec(dllexport) StreamDataState state() noexcept;
    __declspec(dllexport) uint8_t state_id() noexcept;
    __declspec(dllexport) uint64_t hash() noexcept;
    __declspec(dllexport) const internal::PixelBlock& pixel_block() noexcept;
    __declspec(dllexport) rust::Slice<const float> pixel_buffer() noexcept;
    __declspec(dllexport) uint32_t pixel_width() noexcept;
    __declspec(dllexport) uint32_t pixel_height() noexcept;
    uint8_t pixel_num_channels() noexcept;

private:
    internal::StreamDataImplShared inner;

};

}  // namespace open_comp_graph

#endif //OPENCOMPGRAPH_STREAM_H
