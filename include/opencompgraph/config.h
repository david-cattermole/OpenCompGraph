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
