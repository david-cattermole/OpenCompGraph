#include <iostream>
#include <string>
#include "opencompgraph.h"

namespace ocg = opencompgraph;

int test_d() {
    std::cout << "====================================== test_d()" << std::endl;

    auto g = ocg::Graph();

    auto read_op = ocg::internal::create_operation_box(
            0, ocg::OperationType::ReadImage);
    std::cout << "read_op=" << &read_op << std::endl;

    auto write_op = ocg::internal::create_operation_box(
            0, ocg::OperationType::WriteImage);
    std::cout << "write_op=" << &write_op << std::endl;

    auto read_op_id = g.add_op(std::move(read_op));
    auto write_op_id = g.add_op(std::move(write_op));
    g.connect(read_op_id, write_op_id);

    return 0;
}
