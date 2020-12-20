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

        AttrState attr_exists(rust::Str name) const noexcept;

        float get_attr_f32(rust::Str name) const noexcept;
        int32_t get_attr_i32(rust::Str name) const noexcept;
        rust::Str get_attr_str(rust::Str name) const noexcept;

        void set_attr_f32(rust::Str name, float value) noexcept;
        void set_attr_i32(rust::Str name, int32_t value) noexcept;
        void set_attr_str(rust::Str name, rust::Str value) noexcept;

    private:
        internal::NodeImplShared inner;
    };
}

#endif //OPENCOMPGRAPH_NODE_H
