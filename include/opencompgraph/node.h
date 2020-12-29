#ifndef OPENCOMPGRAPH_NODE_H
#define OPENCOMPGRAPH_NODE_H

#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>

namespace open_comp_graph {

    class Node {
    public:
        __declspec(dllexport) explicit Node(NodeType node_type) noexcept;
        __declspec(dllexport) Node(NodeType node_type, const char *name) noexcept;
        __declspec(dllexport) Node(NodeType node_type, size_t id) noexcept;

        __declspec(dllexport) ~Node();

        __declspec(dllexport) size_t get_id() noexcept;
        __declspec(dllexport) rust::Box<open_comp_graph::internal::NodeImpl> get_box() noexcept;

        __declspec(dllexport) AttrState attr_exists(rust::Str name) const noexcept;

        __declspec(dllexport) float get_attr_f32(rust::Str name) const noexcept;
        __declspec(dllexport) int32_t get_attr_i32(rust::Str name) const noexcept;
        __declspec(dllexport) rust::Str get_attr_str(rust::Str name) const noexcept;

        __declspec(dllexport) void set_attr_f32(rust::Str name, float value) noexcept;
        __declspec(dllexport) void set_attr_i32(rust::Str name, int32_t value) noexcept;
        __declspec(dllexport) void set_attr_str(rust::Str name, rust::Str value) noexcept;

    private:
        internal::NodeImplShared inner;
    };

} // namespace open_comp_graph

#endif //OPENCOMPGRAPH_NODE_H
