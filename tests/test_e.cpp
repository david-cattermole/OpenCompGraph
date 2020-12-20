#include <iostream>
#include <opencompgraph.h>

namespace ocg = opencompgraph;

int test_e() {
    std::cout << "====================================== test_e()" << std::endl;

    auto g = ocg::Graph();

    std::cout << "Node() ========================================" << std::endl;
    auto read_op = ocg::Node(ocg::NodeType::ReadImage, "read");
    auto null_op1 = ocg::Node(ocg::NodeType::Null, "null1");
    auto null_op2 = ocg::Node(ocg::NodeType::Null, "null2");
    auto null_op3 = ocg::Node(ocg::NodeType::Null, "null3");
    auto null_op4 = ocg::Node(ocg::NodeType::Null, "null4");
    auto write_op = ocg::Node(ocg::NodeType::WriteImage, "write");
    std::cout << "read_op=" << &read_op << std::endl;
    std::cout << "write_op=" << &write_op << std::endl;
    auto read_op_id = g.add_op(read_op);
    auto null_op_id1 = g.add_op(null_op1);
    auto null_op_id2 = g.add_op(null_op2);
    auto null_op_id3 = g.add_op(null_op3);
    auto null_op_id4 = g.add_op(null_op4);
    auto write_op_id = g.add_op(write_op);
    g.connect(read_op_id, null_op_id1, 0);
    g.connect(null_op_id1, null_op_id2, 0);
    g.connect(null_op_id2, null_op_id3, 0);
    g.connect(null_op_id3, null_op_id4, 0);
    g.connect(null_op_id4, write_op_id, 0);

    g.execute(write_op_id);
    return 0;
}
