#ifndef OPENCOMPGRAPH_GRAPH_H
#define OPENCOMPGRAPH_GRAPH_H

#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>
#include <opencompgraph/operation.h>

namespace opencompgraph {
    class Graph {
    public:
        Graph() noexcept;
        ~Graph();

        size_t add_op(rust::Box<opencompgraph::internal::OperationImpl> op_box) noexcept;
        size_t add_op(Operation &op) noexcept;
        size_t create_op(opencompgraph::OperationType op_type, size_t id) noexcept;
        void connect(size_t src_op_id, size_t dst_op_id) noexcept;

    private:
        internal::GraphImplShared inner;
    };
}

#endif // OPENCOMPGRAPH_GRAPH_H
