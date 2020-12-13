#include <iostream>
#include <opencompgraph.h>

namespace ocg = opencompgraph;

int test_d() {
    std::cout << "====================================== test_d()" << std::endl;

    auto g = ocg::Graph();

    auto read_op = ocg::Operation(0, ocg::OperationType::ReadImage);
    auto write_op = ocg::Operation(1, ocg::OperationType::WriteImage);
    std::cout << "read_op=" << &read_op << std::endl;
    std::cout << "write_op=" << &write_op << std::endl;
    g.add_op(read_op);
    g.add_op(write_op);

    auto read_op_id = g.create_op(0, ocg::OperationType::ReadImage);
    auto write_op_id = g.create_op(1, ocg::OperationType::WriteImage);
    std::cout << "read_op_id=" << read_op_id << std::endl;
    std::cout << "write_op_id=" << write_op_id << std::endl;
    g.connect(read_op_id, write_op_id);

    auto read_op_box = ocg::internal::create_operation_box(
            0, ocg::OperationType::ReadImage);
    auto write_op_box = ocg::internal::create_operation_box(
            1, ocg::OperationType::WriteImage);
    std::cout << "read_op_box=" << &read_op_box << std::endl;
    std::cout << "write_op_box=" << &write_op_box << std::endl;
    auto read_op_box_id = g.add_op(std::move(read_op_box));
    auto write_op_box_id = g.add_op(std::move(write_op_box));
    g.connect(read_op_box_id, write_op_box_id);

    return 0;
}
