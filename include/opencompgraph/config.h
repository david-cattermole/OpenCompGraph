#ifndef OPENCOMPGRAPH_CONFIG_H
#define OPENCOMPGRAPH_CONFIG_H

#include <memory>
#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>
#include "symbol_export.h"

namespace open_comp_graph {

class Config {
public:
    OCG_API_EXPORT
    Config() noexcept;

    OCG_API_EXPORT
    Config(const char* file_name) noexcept;

    OCG_API_EXPORT
    rust::Box<internal::ConfigImpl> get_box() noexcept;

    OCG_API_EXPORT
    void set_box(rust::Box<internal::ConfigImpl> box) noexcept;

    OCG_API_EXPORT
    float cache_ram_capacity_percent() const noexcept;

    OCG_API_EXPORT
    std::size_t cache_ram_capacity_bytes() const noexcept;

    OCG_API_EXPORT
    std::string data_debug_string() const noexcept;

private:
    internal::ConfigImplShared inner;

};

}  // namespace open_comp_graph

#endif //OPENCOMPGRAPH_CONFIG_H
