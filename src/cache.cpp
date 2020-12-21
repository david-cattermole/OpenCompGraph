#include <iostream>
#include <utility>
#include <rust/cxx.h>
#include <opencompgraph/cache.h>

namespace opencompgraph {

Cache::Cache() noexcept
        : inner{internal::create_cache_shared()} {
    std::cout << "Cache()" << '\n';
}

Cache::~Cache() {
    std::cout << "~Cache()" << '\n';
}

rust::Box<opencompgraph::internal::CacheImpl> Cache::get_box() noexcept {
    return std::move(this->inner.inner);
}

void Cache::set_box(rust::Box<opencompgraph::internal::CacheImpl> box) noexcept {
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

} // namespace opencompgraph
