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

        void add_op(rust::Box<opencompgraph::internal::OperationImpl> op_box);

    private:

        internal::GraphImplShared inner;
    };

}


#endif // OPENCOMPGRAPH_H
