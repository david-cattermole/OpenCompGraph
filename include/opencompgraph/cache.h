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
