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

#ifndef OPENCOMPGRAPH_NODE_H
#define OPENCOMPGRAPH_NODE_H

#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>
#include "symbol_export.h"

namespace open_comp_graph {

class Node {
public:
    OCG_API_EXPORT
    Node() noexcept;

    OCG_API_EXPORT
    Node(NodeType node_type, size_t id) noexcept;

    OCG_API_EXPORT
    ~Node();

    OCG_API_EXPORT
    uint64_t get_id() const noexcept;

    OCG_API_EXPORT
    NodeType get_node_type() const noexcept;

private:
    uint64_t m_id;
    NodeType m_node_type;
};

} // namespace open_comp_graph

#endif //OPENCOMPGRAPH_NODE_H
