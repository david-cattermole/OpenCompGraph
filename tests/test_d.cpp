#include <iostream>
#include <opencompgraph.h>

namespace ocg = opencompgraph;

int test_d() {
    std::cout << "====================================== test_d()" << std::endl;

    auto g = ocg::Graph();

    std::cout << "Node() ========================================" << std::endl;
    auto read_op = ocg::Node(ocg::NodeType::ReadImage);
    auto write_op = ocg::Node(
            ocg::NodeType::WriteImage, "my_write_node");
    std::cout << "read_op=" << &read_op << std::endl;
    std::cout << "write_op=" << &write_op << std::endl;
    auto read_op_id1 = g.add_op(read_op);
    auto write_op_id1 = g.add_op(write_op);
    g.connect(read_op_id1, write_op_id1, 0);

    std::cout << "Graph.create_op() =============================" << std::endl;
    auto read_op_id2 = g.create_op(ocg::NodeType::ReadImage, 0);
    auto write_op_id2 = g.create_op(ocg::NodeType::WriteImage, 1);
    std::cout << "read_op_id2=" << read_op_id2 << std::endl;
    std::cout << "write_op_id2=" << write_op_id2 << std::endl;
    g.connect(read_op_id2, write_op_id2, 0);

    std::cout << "Node.add_op(Box<NodeImpl>) ==========" << std::endl;
    auto read_op_box = ocg::internal::create_node_box(
            ocg::NodeType::ReadImage, 0);
    auto write_op_box = ocg::internal::create_node_box(
            ocg::NodeType::WriteImage, 1);
    std::cout << "read_op_box=" << &read_op_box << std::endl;
    std::cout << "write_op_box=" << &write_op_box << std::endl;
    auto read_op_box_id = g.add_op(std::move(read_op_box));
    auto write_op_box_id = g.add_op(std::move(write_op_box));
    g.connect(read_op_box_id, write_op_box_id, 0);

    g.execute(write_op_id1);
    g.execute(write_op_id2);
    g.execute(write_op_box_id);

    return 0;
}
