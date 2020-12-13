#include <iostream>
#include <string>
#include "opencompgraph.h"

namespace ocg = opencompgraph;

int test_b() {
    std::cout << "test_b()" << std::endl;

    auto app_name = std::string("my awesome demo");
    auto x = ocg::make_thingc(app_name);
    auto name = ocg::get_name(*x);
    std::cout << name << std::endl;

    ocg::my_func();

    auto g = ocg::create_shared_graph();

    auto op = ocg::internal::create_operation_box(
            0, ocg::OperationType::Null);
    std::cout << "read_op=" << &op << std::endl;

    g.inner->add_op(std::move(op));

    return 0;
}
