#ifndef OPENCOMPGRAPH_CONFIG_H
#define OPENCOMPGRAPH_CONFIG_H

#include <memory>
#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>
#include "symbol_export.h"

namespace open_comp_graph {

class Config {
public:
    OPENCOMPGRAPH_SYMBOL_EXPORT
    Config() noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    Config(const char* file_name) noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    rust::Box<internal::ConfigImpl> get_box() noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    void set_box(rust::Box<internal::ConfigImpl> box) noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    float cache_ram_capacity_percent() const noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    std::size_t cache_ram_capacity_bytes() const noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    std::string data_debug_string() const noexcept;

private:
    internal::ConfigImplShared inner;

};

}  // namespace open_comp_graph

#endif //OPENCOMPGRAPH_CONFIG_H
