#include <iostream>
#include <opencompgraph/graph.h>


namespace opencompgraph {

Graph::Graph() noexcept : inner{internal::create_graph_shared()} {
    std::cout << "Graph()" << '\n';
}

Graph::~Graph() {
    std::cout << "~Graph()" << '\n';
}

size_t Graph::add_node(rust::Box<opencompgraph::internal::NodeImpl> node_box) noexcept {
    auto index = this->inner.inner->add_node(std::move(node_box));
    return index;
}

size_t Graph::add_node(Node &node) noexcept {
    auto node_box = node.get_box();
    return this->inner.inner->add_node(std::move(node_box));
}

void Graph::connect(size_t src_index, size_t dst_index, uint8_t input_num) noexcept {
    this->inner.inner->connect(src_index, dst_index, input_num);
}

void Graph::execute(size_t start_index, std::shared_ptr<Cache> &cache) noexcept {
    auto cache_box = cache->get_box();  // Borrow the underlying cache object.
    this->inner.inner->execute(start_index, cache_box);
    cache->set_box(std::move(cache_box));  // Return the cache to it's owner.
}

} // namespace opencompgraph
