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

#ifndef OPENCOMPGRAPH_GRAPH_H
#define OPENCOMPGRAPH_GRAPH_H

#include <memory>
#include <string>
#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>
#include <opencompgraph/cache.h>
#include <opencompgraph/node.h>
#include <opencompgraph/stream.h>
#include "symbol_export.h"

namespace open_comp_graph {

class Graph {
public:
    OCG_API_EXPORT Graph() noexcept;

    OCG_API_EXPORT
    GraphState state() const noexcept;

    OCG_API_EXPORT
    ExecuteStatus execute_status() const noexcept;

    OCG_API_EXPORT
    Node create_node(NodeType node_type) noexcept;

    OCG_API_EXPORT
    Node create_node(NodeType node_type, const char* name) noexcept;

    OCG_API_EXPORT
    Node create_node(NodeType node_type, uint64_t id) noexcept;

    OCG_API_EXPORT
    bool delete_node(const Node &node) noexcept;

    OCG_API_EXPORT
    bool delete_node(uint64_t node_id) noexcept;

    OCG_API_EXPORT
    AttrState node_attr_exists(const Node &node, rust::Str name) const noexcept;

    OCG_API_EXPORT
    float get_node_attr_f32(const Node &node, rust::Str name) const noexcept;

    OCG_API_EXPORT
    int32_t get_node_attr_i32(const Node &node, rust::Str name) const noexcept;

    OCG_API_EXPORT
    rust::Str get_node_attr_str(const Node &node, rust::Str name) const noexcept;

    OCG_API_EXPORT
    void set_node_attr_f32(const Node &node, rust::Str name, float value) noexcept;

    OCG_API_EXPORT
    void set_node_attr_i32(const Node &node, rust::Str name, int32_t value) noexcept;

    OCG_API_EXPORT
    void set_node_attr_str(const Node &node, rust::Str name, rust::Str value) noexcept;

    OCG_API_EXPORT
    bool node_exists(const Node& node) noexcept;

    OCG_API_EXPORT
    void connect(const Node& src_node, const Node& dst_node, uint8_t input_num) noexcept;

    OCG_API_EXPORT
    ExecuteStatus execute(
        const Node& node,
        std::vector<int32_t> &frames,
        std::shared_ptr<Cache> &cache) noexcept;

    OCG_API_EXPORT
    ExecuteStatus execute(
        const Node& node,
        std::vector<double> &frames,
        std::shared_ptr<Cache> &cache) noexcept;

    OCG_API_EXPORT
    NodeStatus node_status(const Node &node) const noexcept;

    OCG_API_EXPORT
    std::string data_debug_string() const noexcept;

    OCG_API_EXPORT
    StreamData output_stream() noexcept;

private:
    internal::GraphImplShared inner;
};

} // namespace open_comp_graph

#endif // OPENCOMPGRAPH_GRAPH_H
