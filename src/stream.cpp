#include <iostream>
#include <utility>
#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>
#include <opencompgraph/stream.h>

namespace open_comp_graph {

StreamData::StreamData() noexcept
        : inner{internal::create_stream_data_shared()} {
    std::cout << "StreamData()" << '\n';
}

StreamData::StreamData(rust::Box<internal::StreamDataImpl> box) noexcept
        : inner{internal::create_stream_data_shared_box(std::move(box))} {
    std::cout << "StreamData(box)" << '\n';
}

rust::Box<internal::StreamDataImpl> StreamData::get_box() noexcept {
    return std::move(this->inner.inner);
}

void StreamData::set_box(rust::Box<internal::StreamDataImpl> box) noexcept {
    this->inner.inner = std::move(box);
}

StreamDataState StreamData::state() const noexcept {
    return this->inner.inner->get_state();
}

uint8_t StreamData::state_id() const noexcept {
    return this->inner.inner->get_state_id();
}

uint64_t StreamData::hash() const noexcept {
    return this->inner.inner->get_hash();
}

const internal::PixelBlock& StreamData::pixel_block() const noexcept {
    return this->inner.inner->get_pixel_block();
}

rust::Slice<const float> StreamData::pixel_buffer() const noexcept {
    return this->inner.inner->get_pixel_buffer();
};

uint32_t StreamData::pixel_width() const noexcept {
    return this->inner.inner->get_pixel_width();
};

uint32_t StreamData::pixel_height() const noexcept {
    return this->inner.inner->get_pixel_height();
};

uint8_t StreamData::pixel_num_channels() const noexcept {
    return this->inner.inner->get_pixel_num_channels();
};

} // namespace open_comp_graph
