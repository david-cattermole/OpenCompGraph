#ifndef OPENCOMPGRAPH_GRAPH_H
#define OPENCOMPGRAPH_GRAPH_H

#include <memory>
#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>
#include <opencompgraph/cache.h>
#include <opencompgraph/node.h>
#include <opencompgraph/stream.h>
#include "symbol_export.h"

namespace open_comp_graph {

class Graph {
public:
    OPENCOMPGRAPH_SYMBOL_EXPORT Graph() noexcept;
    OPENCOMPGRAPH_SYMBOL_EXPORT ~Graph();

    OPENCOMPGRAPH_SYMBOL_EXPORT
    size_t add_node(rust::Box<open_comp_graph::internal::NodeImpl> node_box) noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    size_t add_node(Node &node) noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    void connect(size_t src_index, size_t dst_index, uint8_t input_num) noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    ExecuteStatus execute(size_t start_node_index,
                          std::shared_ptr<Cache> &cache) noexcept;

    OPENCOMPGRAPH_SYMBOL_EXPORT
    StreamData output_stream() noexcept;

private:
    internal::GraphImplShared inner;
};

} // namespace open_comp_graph

#endif // OPENCOMPGRAPH_GRAPH_H
