#ifndef OPENCOMPGRAPH_GRAPH_H
#define OPENCOMPGRAPH_GRAPH_H

#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>
#include <opencompgraph/node.h>

namespace opencompgraph {
    class Graph {
    public:
        Graph() noexcept;
        ~Graph();

        size_t add_node(rust::Box<opencompgraph::internal::NodeImpl> node_box) noexcept;
        size_t add_node(Node &node) noexcept;
        size_t create_node(opencompgraph::NodeType node_type, size_t id) noexcept;
        void connect(size_t src_index, size_t dst_index, uint8_t input_num) noexcept;
        void execute(size_t start_index) noexcept;

    private:
        internal::GraphImplShared inner;
    };
}

#endif // OPENCOMPGRAPH_GRAPH_H
