#include <iostream>
#include <rust/cxx.h>
#include <opencompgraph/operation.h>

namespace opencompgraph {

Operation::Operation(size_t id, opencompgraph::OperationType op_type) noexcept
        : inner{internal::create_operation_shared(id, op_type)} {
    std::cout << "Operation()"
            << " id=" << id
            << " op_type=" << static_cast<uint8_t>(op_type)
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
