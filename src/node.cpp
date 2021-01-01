#include <iostream>
#include <rust/cxx.h>
#include <opencompgraph/node.h>

namespace open_comp_graph {


Node::Node(NodeType node_type, uint64_t id) noexcept
        : m_id(id), m_node_type(node_type) {
    std::cout << "Node() with id"
              << " node_type=" << static_cast<uint32_t>(node_type)
              << " id=" << id
              << std::endl;
}

Node::~Node() {
    // std::cout << "~Node()" << std::endl;
}

uint64_t Node::get_id() noexcept {
    return m_id;
}

NodeType Node::get_node_type() noexcept {
    return m_node_type;
}


} // namespace open_comp_graph
