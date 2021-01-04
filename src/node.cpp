#include <iostream>
#include <rust/cxx.h>
#include <opencompgraph/node.h>

namespace open_comp_graph {


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
