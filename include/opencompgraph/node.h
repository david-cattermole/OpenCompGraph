#ifndef OPENCOMPGRAPH_NODE_H
#define OPENCOMPGRAPH_NODE_H

#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>
#include "symbol_export.h"

namespace open_comp_graph {

class Node {
public:
    OPENCOMPGRAPH_SYMBOL_EXPORT
    explicit Node(NodeType node_type) noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    Node(NodeType node_type, const char *name) noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    Node(NodeType node_type, size_t id) noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    ~Node();

    OPENCOMPGRAPH_SYMBOL_EXPORT
    size_t get_id() noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    rust::Box<open_comp_graph::internal::NodeImpl> get_box() noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    AttrState attr_exists(rust::Str name) const noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    float get_attr_f32(rust::Str name) const noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    int32_t get_attr_i32(rust::Str name) const noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    rust::Str get_attr_str(rust::Str name) const noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    void set_attr_f32(rust::Str name, float value) noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    void set_attr_i32(rust::Str name, int32_t value) noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    void set_attr_str(rust::Str name, rust::Str value) noexcept;

private:
    internal::NodeImplShared inner;
};

} // namespace open_comp_graph

#endif //OPENCOMPGRAPH_NODE_H
