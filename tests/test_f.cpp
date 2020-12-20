#include <iostream>
#include <opencompgraph.h>

namespace ocg = opencompgraph;

int test_f() {
    std::cout << "====================================== test_f()" << std::endl;

    auto g = ocg::Graph();

    std::cout << "Node() ========================================" << std::endl;
    auto read_node = ocg::Node(ocg::NodeType::ReadImage, "readimage1");
    auto grade_overexpose_node = ocg::Node(ocg::NodeType::Grade, "grade1");
    auto grade_underexpose_node = ocg::Node(ocg::NodeType::Grade, "grade2");
    auto grade_dark_node = ocg::Node(ocg::NodeType::Grade, "grade3");
    auto null1_node = ocg::Node(ocg::NodeType::Null, "null1");
    auto null2_node = ocg::Node(ocg::NodeType::Null, "null2");

    std::cout << "read_node=" << &read_node << std::endl;
    std::cout << "grade_overexpose_node=" << &grade_overexpose_node << std::endl;
    std::cout << "grade_underexpose_node=" << &grade_underexpose_node << std::endl;
    std::cout << "grade_dark_node=" << &grade_dark_node << std::endl;
    std::cout << "null1_node=" << &null1_node << std::endl;
    std::cout << "null2_node=" << &null2_node << std::endl;

    read_node.set_attr_str("file_name", "tests/data/oiio-images/tahoe-gps.jpg");
    grade_overexpose_node.set_attr_f32("multiply", 2.0);
    grade_underexpose_node.set_attr_f32("multiply", 0.5);
    grade_dark_node.set_attr_f32("multiply", 0.1);

    auto read_id = g.add_node(read_node);
    auto grade_overexpose_id = g.add_node(grade_overexpose_node);
    auto grade_underexpose_id = g.add_node(grade_underexpose_node);
    auto grade_dark_id = g.add_node(grade_dark_node);
    auto null1_id = g.add_node(null1_node);
    auto null2_id = g.add_node(null2_node);

    g.connect(read_id, grade_overexpose_id, 0);
    g.connect(grade_overexpose_id, grade_underexpose_id, 0);
    g.connect(grade_underexpose_id, null1_id, 0);

    g.connect(grade_underexpose_id, grade_dark_id, 0);
    g.connect(grade_dark_id, null2_id, 0);

    g.execute(null1_id);
    g.execute(null2_id);

    return 0;
}
