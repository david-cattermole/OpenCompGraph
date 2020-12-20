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
    auto grade_dark_op = ocg::Node(ocg::NodeType::Grade, "grade3");
    auto null1_op = ocg::Node(ocg::NodeType::Null, "null1");
    auto null2_op = ocg::Node(ocg::NodeType::Null, "null2");

    std::cout << "read_op=" << &read_op << std::endl;
    std::cout << "grade_overexpose_op=" << &grade_overexpose_op << std::endl;
    std::cout << "grade_underexpose_op=" << &grade_underexpose_op << std::endl;
    std::cout << "grade_dark_op=" << &grade_dark_op << std::endl;
    std::cout << "null1_op=" << &null1_op << std::endl;
    std::cout << "null2_op=" << &null2_op << std::endl;

    read_op.set_attr_str("file_name", "tests/data/oiio-images/tahoe-gps.jpg");
    grade_overexpose_op.set_attr_f32("multiply", 2.0);
    grade_underexpose_op.set_attr_f32("multiply", 0.5);
    grade_dark_op.set_attr_f32("multiply", 0.1);

    auto read_id = g.add_op(read_op);
    auto grade_overexpose_id = g.add_op(grade_overexpose_op);
    auto grade_underexpose_id = g.add_op(grade_underexpose_op);
    auto grade_dark_id = g.add_op(grade_dark_op);
    auto null1_id = g.add_op(null1_op);
    auto null2_id = g.add_op(null2_op);

    g.connect(read_id, grade_overexpose_id, 0);
    g.connect(grade_overexpose_id, grade_underexpose_id, 0);
    g.connect(grade_underexpose_id, null1_id, 0);

    g.connect(grade_underexpose_id, grade_dark_id, 0);
    g.connect(grade_dark_id, null2_id, 0);

    g.execute(null1_id);
    g.execute(null2_id);

    return 0;
}
