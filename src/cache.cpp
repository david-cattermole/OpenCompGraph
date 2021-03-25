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

#include <iostream>
#include <utility>
#include <rust/cxx.h>
#include <opencompgraph/cache.h>

namespace open_comp_graph {

Cache::Cache() noexcept
        : inner{internal::create_cache_shared_with_capacity(0)} {
}

rust::Box<internal::CacheImpl> Cache::get_box() noexcept {
    return std::move(this->inner.inner);
}

void Cache::set_box(rust::Box<internal::CacheImpl> box) noexcept {
    this->inner.inner = std::move(box);
}

std::size_t Cache::count() const noexcept {
    return this->inner.inner->len();
}

std::size_t Cache::used_bytes() const noexcept {
    return this->inner.inner->used_bytes();
}

std::size_t Cache::capacity_bytes() const noexcept {
    return this->inner.inner->capacity_bytes();
}

void Cache::set_capacity_bytes(std::size_t value) noexcept {
    return this->inner.inner->set_capacity_bytes(value);
}

std::string Cache::data_debug_string() const noexcept {
    auto rust_string = this->inner.inner->data_debug_string();
    return std::string(rust_string);
}

} // namespace open_comp_graph
