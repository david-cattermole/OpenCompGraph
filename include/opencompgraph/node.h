#ifndef OPENCOMPGRAPH_NODE_H
#define OPENCOMPGRAPH_NODE_H

#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>

namespace opencompgraph {
    class Node {
    public:
        Node(NodeType node_type) noexcept;
        Node(NodeType node_type, const char *name) noexcept;
        Node(NodeType node_type, size_t id) noexcept;

        ~Node();

        size_t get_id() noexcept;
        rust::Box<opencompgraph::internal::NodeImpl> get_box() noexcept;

    private:
        internal::NodeImplShared inner;
    };
}

#endif //OPENCOMPGRAPH_NODE_H
