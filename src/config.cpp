#include <sstream>  // stringstream

#include <opencompgraph/config.h>

namespace open_comp_graph {

Config::Config() noexcept
        : inner{internal::get_config("open_comp_graph.yaml")} {
}

Config::Config(const char* file_name) noexcept
        : inner{internal::get_config(file_name)} {
}

rust::Box<internal::ConfigImpl> Config::get_box() noexcept {
    return std::move(this->inner.inner);
}

void Config::set_box(rust::Box<internal::ConfigImpl> box) noexcept {
    this->inner.inner = std::move(box);
}

float Config::cache_ram_capacity_percent() const noexcept {
    return this->inner.inner->cache_ram_capacity_percent();
}

std::size_t Config::cache_ram_capacity_bytes() const noexcept {
    return this->inner.inner->cache_ram_capacity_bytes();
}

std::string Config::data_debug_string() const noexcept {
    auto rust_string = this->inner.inner->data_debug_string();
    return std::string(rust_string);
}

} // namespace open_comp_graph
