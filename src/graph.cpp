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
#include <string>
#include <opencompgraph/graph.h>
#include <opencompgraph/node.h>
#include <opencompgraph/stream.h>


namespace open_comp_graph {

Graph::Graph() noexcept
        : inner{internal::create_graph_shared()} {
    // std::cout << "Graph()" << '\n';
}

GraphState Graph::state() const noexcept {
    return this->inner.inner->state();
}

ExecuteStatus Graph::execute_status() const noexcept {
    return this->inner.inner->execute_status();
}

Node Graph::create_node(NodeType node_type) noexcept {
    auto id = internal::generate_random_id();
    auto node = Node(node_type, id);
    auto node_box = internal::create_node_box(node_type, id);
    this->inner.inner->add_node(std::move(node_box));
    return node;
}

Node Graph::create_node(NodeType node_type, const char* name) noexcept {
    auto id = internal::generate_id_from_name(name);
    auto node = Node(node_type, id);
    auto node_box = internal::create_node_box(node_type, id);
    this->inner.inner->add_node(std::move(node_box));
    return node;
}

Node Graph::create_node(NodeType node_type, uint64_t id) noexcept {
    auto node = Node(node_type, id);
    auto node_box = internal::create_node_box(node_type, id);
    this->inner.inner->add_node(std::move(node_box));
    return node;
}

bool Graph::delete_node(const Node &node) noexcept {
    auto node_id = node.get_id();
    return Graph::delete_node(node_id);
}

bool Graph::delete_node(uint64_t node_id) noexcept {
    return this->inner.inner->remove_node(node_id);
}

AttrState Graph::node_attr_exists(const Node &node, rust::Str name) const noexcept {
    auto node_id = node.get_id();
    return this->inner.inner->node_attr_exists(node_id, name);
}

float Graph::get_node_attr_f32(const Node &node, rust::Str name) const noexcept {
    auto node_id = node.get_id();
    return this->inner.inner->get_node_attr_f32(node_id, name);
}

int32_t Graph::get_node_attr_i32(const Node &node, rust::Str name) const noexcept {
    auto node_id = node.get_id();
    return this->inner.inner->get_node_attr_i32(node_id, name);
}

rust::Str Graph::get_node_attr_str(const Node &node, rust::Str name) const noexcept {
    auto node_id = node.get_id();
    return this->inner.inner->get_node_attr_str(node_id, name);
}

void Graph::set_node_attr_f32(const Node &node, rust::Str name, float value) noexcept {
    auto node_id = node.get_id();
    return this->inner.inner->set_node_attr_f32(node_id, name, value);
}

void Graph::set_node_attr_i32(const Node &node, rust::Str name, int32_t value) noexcept {
    auto node_id = node.get_id();
    return this->inner.inner->set_node_attr_i32(node_id, name, value);
}

void Graph::set_node_attr_str(const Node &node, rust::Str name, rust::Str value) noexcept {
    auto node_id = node.get_id();
    return this->inner.inner->set_node_attr_str(node_id, name, value);
}

NodeStatus Graph::node_status(const Node &node) const noexcept {
    auto node_id = node.get_id();
    return this->inner.inner->node_status(node_id);
}

bool Graph::node_exists(const Node &node) noexcept {
    auto node_id = node.get_id();
    return this->inner.inner->node_exists(node_id);
}

void Graph::disconnect_input(const Node& dst_node, uint8_t input_num) noexcept {
    auto dst_node_id = dst_node.get_id();
    this->inner.inner->disconnect_input(dst_node_id, input_num);
}

void Graph::connect(const Node& src_node, const Node& dst_node, uint8_t input_num) noexcept {
    auto src_node_id = src_node.get_id();
    auto dst_node_id = dst_node.get_id();
    this->inner.inner->connect(src_node_id, dst_node_id, input_num);
}

ExecuteStatus Graph::execute(const Node &node,
                             std::vector<int32_t> &frames,
                             std::shared_ptr<Cache> &cache) noexcept {
    std::vector<double> float_frames(frames.size());
    for (uint32_t i = 0; i < frames.size(); ++i) {
        float_frames.push_back(frames[i]);
    }
    return Graph::execute(
        node,
        float_frames,
        cache);
}

ExecuteStatus Graph::execute(const Node &node,
                             std::vector<double> &frames,
                             std::shared_ptr<Cache> &cache) noexcept {
    auto node_id = node.get_id();
    rust::Slice<const double> slice_frames{frames.data(), frames.size()};
    auto cache_box = cache->get_box();  // Borrow the underlying cache object.
    auto status = this->inner.inner->execute(node_id, slice_frames, cache_box);
    cache->set_box(std::move(cache_box));  // Return the cache to it's owner.
    return status;
}

std::string Graph::data_debug_string() const noexcept {
    auto rust_string = this->inner.inner->data_debug_string();
    return std::string(rust_string);
}

StreamData Graph::output_stream() noexcept {
    auto data = this->inner.inner->output_stream();
    return StreamData(std::move(data.inner));
}

} // namespace open_comp_graph
