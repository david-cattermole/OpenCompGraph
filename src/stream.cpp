#include <iostream>
#include <utility>
#include <rust/cxx.h>
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

StreamData::~StreamData() {
    std::cout << "~StreamData()" << '\n';
}

rust::Box<internal::StreamDataImpl> StreamData::get_box() noexcept {
    return std::move(this->inner.inner);
}

void StreamData::set_box(rust::Box<internal::StreamDataImpl> box) noexcept {
    this->inner.inner = std::move(box);
}

StreamDataState StreamData::state() noexcept {
    return this->inner.inner->get_state();
}

uint8_t StreamData::state_id() noexcept {
    return this->inner.inner->get_state_id();
}

uint64_t StreamData::hash() noexcept {
    return this->inner.inner->get_hash();
}

} // namespace open_comp_graph
