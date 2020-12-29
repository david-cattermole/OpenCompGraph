#ifndef OPENCOMPGRAPH_CACHE_H
#define OPENCOMPGRAPH_CACHE_H

#include <memory>
#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>

namespace open_comp_graph {

class Cache {
public:
    __declspec(dllexport) Cache() noexcept;
    __declspec(dllexport) ~Cache();

    __declspec(dllexport) rust::Box<internal::CacheImpl> get_box() noexcept;
    __declspec(dllexport) void set_box(rust::Box<internal::CacheImpl> box) noexcept;
    __declspec(dllexport) std::uint64_t count() noexcept;

private:
    internal::CacheImplShared inner;

};

}  // namespace open_comp_graph

#endif //OPENCOMPGRAPH_CACHE_H
