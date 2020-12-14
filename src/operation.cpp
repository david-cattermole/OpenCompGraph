#include <iostream>
#include <rust/cxx.h>
#include <opencompgraph/operation.h>

namespace opencompgraph {


Operation::Operation(OperationType op_type) noexcept
        : inner{internal::create_operation_shared(op_type)} {
    std::cout << "Operation() with random id"
              << " op_type=" << static_cast<uint32_t>(op_type)
              << std::endl;
}

Operation::Operation(OperationType op_type, const char *name) noexcept
        : inner{internal::create_operation_shared(op_type, name)} {
    std::cout << "Operation() with name"
              << " op_type=" << static_cast<uint32_t>(op_type)
              << std::endl;
}

Operation::Operation(opencompgraph::OperationType op_type, size_t id) noexcept
        : inner{internal::create_operation_shared(op_type, id)} {
    std::cout << "Operation() with id"
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
