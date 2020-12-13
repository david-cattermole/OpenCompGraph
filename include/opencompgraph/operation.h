#ifndef OPENCOMPGRAPH_OPERATION_H
#define OPENCOMPGRAPH_OPERATION_H

#include <rust/cxx.h>
#include <opencompgraph/_cxxbridge.h>

namespace opencompgraph {
    class Operation {
    public:
        Operation(size_t id, OperationType op_type) noexcept;
        ~Operation();

        size_t get_id() noexcept;
        rust::Box<opencompgraph::internal::OperationImpl> get_box() noexcept;

    private:
        internal::OperationImplShared inner;
    };
}

#endif // OPENCOMPGRAPH_OPERATION_H
