#ifndef OPENCOMPGRAPH_NODE_H
#define OPENCOMPGRAPH_NODE_H

#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>
#include "symbol_export.h"

namespace open_comp_graph {

class Node {
public:
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
