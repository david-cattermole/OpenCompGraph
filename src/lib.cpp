#include <iostream>
#include <opencompgraph.h>

namespace opencompgraph {

Graph::Graph() noexcept : inner{internal::create_graph_shared()} {
    std::cout << "Graph()" << std::endl;
}

Graph::~Graph() {
    std::cout << "~Graph()" << std::endl;
    std::cout << "done with Graph" << std::endl;
}

size_t Graph::add_op(rust::Box<opencompgraph::internal::OperationImpl> op_box) noexcept {
    auto id = op_box->get_id();
    this->inner.inner->add_op(std::move(op_box));
    return id;
}

void Graph::connect(size_t src_op_id, size_t dst_op_id) noexcept {
    this->inner.inner->connect(src_op_id, dst_op_id);
}


} // namespace opencompgraph
