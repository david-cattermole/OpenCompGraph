/*
 * Copyright (C) 2020, 2021 David Cattermole.
 *
 * This file is part of OpenCompGraph.
 *
 * OpenCompGraph is free software: you can redistribute it and/or modify it
 * under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * OpenCompGraph is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with OpenCompGraph.  If not, see <https://www.gnu.org/licenses/>.
 * ====================================================================
 *
 */

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
