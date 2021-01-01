#ifndef OPENCOMPGRAPH_CACHE_H
#define OPENCOMPGRAPH_CACHE_H

#include <memory>
#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>
#include "symbol_export.h"

namespace open_comp_graph {

class Cache {
public:
    OPENCOMPGRAPH_SYMBOL_EXPORT
    Cache() noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    ~Cache();

    OPENCOMPGRAPH_SYMBOL_EXPORT
    rust::Box<internal::CacheImpl> get_box() noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    void set_box(rust::Box<internal::CacheImpl> box) noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    std::uint64_t count() const noexcept;

private:
    internal::CacheImplShared inner;

};

}  // namespace open_comp_graph

#endif //OPENCOMPGRAPH_CACHE_H
