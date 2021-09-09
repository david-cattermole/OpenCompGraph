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
#include <rust/cxx.h>
#include <opencompgraph/node.h>

namespace open_comp_graph {

Node::Node() noexcept
        : m_id(0), m_node_type(NodeType::kNull) {
    // std::cout << "Node() with id"
    //           << " node_type=" << static_cast<uint32_t>(node_type)
    //           << " id=" << id
    //           << '\n';
}

Node::Node(NodeType node_type, uint64_t id) noexcept
        : m_id(id), m_node_type(node_type) {
    // std::cout << "Node() with id"
    //           << " node_type=" << static_cast<uint32_t>(node_type)
    //           << " id=" << id
    //           << '\n';
}

Node::~Node() {
    // std::cout << "~Node()" << '\n';
}

uint64_t Node::get_id() const noexcept {
    return m_id;
}

NodeType Node::get_node_type() const noexcept {
    return m_node_type;
}


} // namespace open_comp_graph
