#include <iostream>
#include <rust/cxx.h>
#include <opencompgraph/operation.h>

namespace opencompgraph {


Operation::Operation(OperationType op_type) noexcept
        : inner{internal::create_operation_shared(op_type, internal::generate_random_id())} {}

Operation::Operation(OperationType op_type, const char *name) noexcept
        : inner{internal::create_operation_shared(op_type, internal::generate_id_from_name(name))} {}

Operation::Operation(opencompgraph::OperationType op_type, size_t id) noexcept
        : inner{internal::create_operation_shared(op_type, id)} {
    std::cout << "Operation()"
              << " op_type=" << static_cast<uint32_t>(op_type)
              << " id=" << id
              << std::endl;
}

Operation::~Operation() {
    std::cout << "~Operation()" << std::endl;
    std::cout << "done with Operation" << std::endl;
}

size_t Operation::get_id() noexcept {
    return this->inner.inner->get_id();
}

rust::Box<opencompgraph::internal::OperationImpl> Operation::get_box() noexcept {
    return std::move(this->inner.inner);
}

} // namespace opencompgraph
