#include <iostream>
#include <rust/cxx.h>
#include <opencompgraph/node.h>

namespace opencompgraph {


Node::Node(NodeType node_type) noexcept
        : inner{internal::create_node_shared(node_type)} {
    std::cout << "Node() with random id"
              << " node_type=" << static_cast<uint32_t>(node_type)
              << std::endl;
}

Node::Node(NodeType node_type, const char *name) noexcept
        : inner{internal::create_node_shared(node_type, name)} {
    std::cout << "Node() with name"
              << " node_type=" << static_cast<uint32_t>(node_type)
              << std::endl;
}

Node::Node(opencompgraph::NodeType node_type, size_t id) noexcept
        : inner{internal::create_node_shared(node_type, id)} {
    std::cout << "Node() with id"
              << " node_type=" << static_cast<uint32_t>(node_type)
              << " id=" << id
              << std::endl;
}

Node::~Node() {
    std::cout << "~Node()" << std::endl;
    std::cout << "done with Node" << std::endl;
}

size_t Node::get_id() noexcept {
    return this->inner.inner->get_id();
}

rust::Box<opencompgraph::internal::NodeImpl> Node::get_box() noexcept {
    return std::move(this->inner.inner);
}

} // namespace opencompgraph
