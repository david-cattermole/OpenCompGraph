#include <iostream>
#include <opencompgraph.h>

namespace ocg = opencompgraph;

int test_f() {
    std::cout << "====================================== test_f()" << std::endl;

    auto g = ocg::Graph();

    std::cout << "Node() ========================================" << std::endl;
    auto read_op = ocg::Node(ocg::NodeType::ReadImage, "readimage1");
    auto grade_overexpose_op = ocg::Node(ocg::NodeType::Grade, "grade1");
    auto grade_underexpose_op = ocg::Node(ocg::NodeType::Grade, "grade2");
    auto null_op = ocg::Node(ocg::NodeType::Null, "null1");
    std::cout << "read_op=" << &read_op << std::endl;
    std::cout << "grade_overexpose_op=" << &grade_overexpose_op << std::endl;
    std::cout << "grade_underexpose_op=" << &grade_underexpose_op << std::endl;
    std::cout << "null_op=" << &null_op << std::endl;
    auto read_id = g.add_op(read_op);
    auto grade_overexpose_id = g.add_op(grade_overexpose_op);
    auto grade_underexpose_id = g.add_op(grade_underexpose_op);
    auto null_id = g.add_op(null_op);
    g.connect(read_id, grade_overexpose_id, 0);
    g.connect(grade_overexpose_id, grade_underexpose_id, 0);
    g.connect(grade_underexpose_id, null_id, 0);

    g.execute(null_id);

    return 0;
}
