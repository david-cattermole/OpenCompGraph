#include <iostream>
#include <string>
#include "opencompgraph.h"

namespace ocg = opencompgraph;

int test_d() {
    std::cout << "====================================== test_d()" << std::endl;

    auto g = ocg::Graph();

    auto op = ocg::internal::create_operation_box(
            0, ocg::OperationType::Null);
    std::cout << "op=" << &op << std::endl;

    g.add_op(std::move(op));

    return 0;
}
