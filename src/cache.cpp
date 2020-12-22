#include <iostream>
#include <utility>
#include <rust/cxx.h>
#include <opencompgraph/cache.h>

namespace open_comp_graph {

Cache::Cache() noexcept
        : inner{internal::create_cache_shared()} {
    std::cout << "Cache()" << '\n';
}

Cache::~Cache() {
    std::cout << "~Cache()" << '\n';
}

rust::Box<open_comp_graph::internal::CacheImpl> Cache::get_box() noexcept {
    return std::move(this->inner.inner);
}

void Cache::set_box(rust::Box<open_comp_graph::internal::CacheImpl> box) noexcept {
    this->inner.inner = std::move(box);
}

std::size_t Cache::count() noexcept {
    return this->inner.inner->len();
}

} // namespace open_comp_graph
