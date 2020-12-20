#include <iostream>
#include <opencompgraph/graph.h>


namespace opencompgraph {

Graph::Graph() noexcept : inner{internal::create_graph_shared()} {
    std::cout << "Graph()" << std::endl;
}

Graph::~Graph() {
    std::cout << "~Graph()" << std::endl;
    std::cout << "done with Graph" << std::endl;
}

size_t Graph::add_node(rust::Box<opencompgraph::internal::NodeImpl> node_box) noexcept {
    auto index = this->inner.inner->add_node(std::move(node_box));
    return index;
}

size_t Graph::add_node(Node &node) noexcept {
    auto node_box = node.get_box();
    return this->inner.inner->add_node(std::move(node_box));
}

size_t Graph::create_node(opencompgraph::NodeType node_type, size_t id) noexcept {
    auto node = internal::create_node_shared(node_type, id);
    return this->inner.inner->add_node(std::move(node.inner));
}

void Graph::connect(size_t src_index, size_t dst_index, uint8_t input_num) noexcept {
    this->inner.inner->connect(src_index, dst_index, input_num);
}

void Graph::execute(size_t start_index) noexcept {
    this->inner.inner->execute(start_index);
}

} // namespace opencompgraph
