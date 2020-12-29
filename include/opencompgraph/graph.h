#ifndef OPENCOMPGRAPH_GRAPH_H
#define OPENCOMPGRAPH_GRAPH_H

#include <memory>
#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>
#include <opencompgraph/cache.h>
#include <opencompgraph/node.h>
#include <opencompgraph/stream.h>

namespace open_comp_graph {
    class Graph {
    public:
        Graph() noexcept;
        ~Graph();

        size_t add_node(rust::Box<open_comp_graph::internal::NodeImpl> node_box) noexcept;
        size_t add_node(Node &node) noexcept;
        void connect(size_t src_index, size_t dst_index, uint8_t input_num) noexcept;
        ExecuteStatus execute(size_t start_node_index, std::shared_ptr<Cache> &cache) noexcept;
        StreamData output_stream() noexcept;

    private:
        internal::GraphImplShared inner;
    };
}

#endif // OPENCOMPGRAPH_GRAPH_H
