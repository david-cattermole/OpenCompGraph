#ifndef OPENCOMPGRAPH_CACHE_H
#define OPENCOMPGRAPH_CACHE_H

#include <memory>
#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>
#include "symbol_export.h"

namespace open_comp_graph {

class Cache {
public:
    OCG_API_EXPORT
    Cache() noexcept;

    OCG_API_EXPORT
    rust::Box<internal::CacheImpl> get_box() noexcept;

    OCG_API_EXPORT
    void set_box(rust::Box<internal::CacheImpl> box) noexcept;

    OCG_API_EXPORT
    std::uint64_t count() const noexcept;

    OCG_API_EXPORT
    std::size_t used_bytes() const noexcept;

    OCG_API_EXPORT
    std::size_t capacity_bytes() const noexcept;

    OCG_API_EXPORT
    void set_capacity_bytes(std::size_t value) noexcept;

    OCG_API_EXPORT
    std::string data_debug_string() const noexcept;

private:
    internal::CacheImplShared inner;

};

}  // namespace open_comp_graph

#endif //OPENCOMPGRAPH_CACHE_H
