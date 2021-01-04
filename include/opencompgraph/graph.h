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
    OPENCOMPGRAPH_SYMBOL_EXPORT Graph() noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    Node create_node(NodeType node_type) noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    Node create_node(NodeType node_type, const char* name) noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    Node create_node(NodeType node_type, uint64_t id) noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    bool delete_node(const Node &node) noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    bool delete_node(uint64_t node_id) noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    AttrState node_attr_exists(const Node &node, rust::Str name) const noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    float get_node_attr_f32(const Node &node, rust::Str name) const noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    int32_t get_node_attr_i32(const Node &node, rust::Str name) const noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    rust::Str get_node_attr_str(const Node &node, rust::Str name) const noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    void set_node_attr_f32(const Node &node, rust::Str name, float value) noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    void set_node_attr_i32(const Node &node, rust::Str name, int32_t value) noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    void set_node_attr_str(const Node &node, rust::Str name, rust::Str value) noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    bool node_exists(const Node& node) noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    void connect(const Node& src_node, const Node& dst_node, uint8_t input_num) noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    ExecuteStatus execute(const Node& node, std::shared_ptr<Cache> &cache) noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    NodeStatus node_status(const Node &node) const noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    std::string data_debug_string() const noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    StreamData output_stream() noexcept;

private:
    internal::GraphImplShared inner;
};

} // namespace open_comp_graph

#endif // OPENCOMPGRAPH_GRAPH_H
