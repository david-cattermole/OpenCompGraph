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

std::unique_ptr<Cache> make_unique_cache() {
    // std::cout << "make_unique_cache()" << '\n';
    return std::unique_ptr<Cache>(new Cache());
}

std::shared_ptr<Cache> make_shared_cache() {
    // std::cout << "make_shared_cache()" << '\n';
    return std::make_shared<Cache>();
}

} // namespace open_comp_graph
