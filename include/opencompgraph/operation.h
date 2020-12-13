#ifndef OPENCOMPGRAPH_OPERATION_H
#define OPENCOMPGRAPH_OPERATION_H

#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>

namespace opencompgraph {
    class Operation {
    public:
        Operation(OperationType op_type) noexcept;
        Operation(OperationType op_type, const char *name) noexcept;
        Operation(OperationType op_type, size_t id) noexcept;

        ~Operation();

        size_t get_id() noexcept;
        rust::Box<opencompgraph::internal::OperationImpl> get_box() noexcept;

    private:
        internal::OperationImplShared inner;
    };
}

#endif // OPENCOMPGRAPH_OPERATION_H
