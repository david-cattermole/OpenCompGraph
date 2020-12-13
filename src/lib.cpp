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

void Graph::add_op(rust::Box<opencompgraph::internal::OperationImpl> op_box) {
    this->inner.inner->add_op(std::move(op_box));
}

} // namespace opencompgraph
