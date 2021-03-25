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

#ifndef OPENCOMPGRAPH_SYSTEM_MEMORY_UTILS_H
#define OPENCOMPGRAPH_SYSTEM_MEMORY_UTILS_H

#include <opencompgraph/symbol_export.h>

namespace open_comp_graph {
namespace internal {

// Returns the number of bytes of system memory available on the
// current machine. If an error occured, 0 (zero) is returned.
OCG_API_EXPORT
size_t get_total_system_memory_as_bytes();

} // namespace internal
} // namespace open_comp_graph

#endif // OPENCOMPGRAPH_SYSTEM_MEMORY_UTILS_H
