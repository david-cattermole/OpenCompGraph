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
}

size_t Node::get_id() noexcept {
    return this->inner.inner->get_id();
}

rust::Box<opencompgraph::internal::NodeImpl> Node::get_box() noexcept {
    return std::move(this->inner.inner);
}

AttrState Node::attr_exists(rust::Str name) const noexcept {
    return this->inner.inner->attr_exists(name);
}

float Node::get_attr_f32(rust::Str name) const noexcept {
    return this->inner.inner->get_attr_f32(name);
}

int32_t Node::get_attr_i32(rust::Str name) const noexcept {
    return this->inner.inner->get_attr_i32(name);
}

rust::Str Node::get_attr_str(rust::Str name) const noexcept {
    return this->inner.inner->get_attr_str(name);
}

void Node::set_attr_f32(rust::Str name, float value) noexcept {
    return this->inner.inner->set_attr_f32(name, value);
}

void Node::set_attr_i32(rust::Str name, int32_t value) noexcept {
    return this->inner.inner->set_attr_i32(name, value);
}

void Node::set_attr_str(rust::Str name, rust::Str value) noexcept {
    return this->inner.inner->set_attr_str(name, value);
}

} // namespace opencompgraph
