#ifndef OPENCOMPGRAPH_H
#define OPENCOMPGRAPH_H

#include <cxx.h>
#include <opencompgraph/cbindgen.h>
#include <opencompgraph/_cxxbridge.h>
#include <opencompgraph/cpp.h>

namespace opencompgraph {

    class Graph {
    public:
        Graph() noexcept;

        ~Graph();

        size_t add_op(rust::Box<opencompgraph::internal::OperationImpl> op_box) noexcept ;

        void connect(size_t src_op_id, size_t dst_op_id) noexcept ;

    private:

        internal::GraphImplShared inner;
    };

}


#endif // OPENCOMPGRAPH_H
