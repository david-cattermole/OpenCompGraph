#ifndef OPENCOMPGRAPH_CACHE_H
#define OPENCOMPGRAPH_CACHE_H

#include <memory>
#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>

namespace open_comp_graph {

class Cache {
public:
    Cache() noexcept;
    ~Cache();

    rust::Box<open_comp_graph::internal::CacheImpl> get_box() noexcept;
    void set_box(rust::Box<open_comp_graph::internal::CacheImpl> box) noexcept;
    std::uint64_t count() noexcept;

private:
    internal::CacheImplShared inner;

};

std::unique_ptr<Cache> make_unique_cache();
std::shared_ptr<Cache> make_shared_cache();

}

#endif //OPENCOMPGRAPH_CACHE_H
