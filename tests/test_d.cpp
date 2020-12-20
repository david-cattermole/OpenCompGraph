#include <iostream>
#include <opencompgraph.h>

namespace ocg = opencompgraph;

int test_d() {
    std::cout << "====================================== test_d()" << std::endl;

    auto g = ocg::Graph();

    std::cout << "Node() ========================================" << std::endl;
    auto read_node = ocg::Node(ocg::NodeType::ReadImage);
    auto write_node = ocg::Node(
            ocg::NodeType::WriteImage, "my_write_node");
    std::cout << "read_node=" << &read_node << std::endl;
    std::cout << "write_node=" << &write_node << std::endl;
    auto read_node_id1 = g.add_node(read_node);
    auto write_node_id1 = g.add_node(write_node);
    g.connect(read_node_id1, write_node_id1, 0);

    std::cout << "Graph.create_node() =============================" << std::endl;
    auto read_node_id2 = g.create_node(ocg::NodeType::ReadImage, 0);
    auto write_node_id2 = g.create_node(ocg::NodeType::WriteImage, 1);
    std::cout << "read_node_id2=" << read_node_id2 << std::endl;
    std::cout << "write_node_id2=" << write_node_id2 << std::endl;
    g.connect(read_node_id2, write_node_id2, 0);

    std::cout << "Node.add_node(Box<NodeImpl>) ==========" << std::endl;
    auto read_node_box = ocg::internal::create_node_box(
            ocg::NodeType::ReadImage, 0);
    auto write_node_box = ocg::internal::create_node_box(
            ocg::NodeType::WriteImage, 1);
    std::cout << "read_node_box=" << &read_node_box << std::endl;
    std::cout << "write_node_box=" << &write_node_box << std::endl;
    auto read_node_box_id = g.add_node(std::move(read_node_box));
    auto write_node_box_id = g.add_node(std::move(write_node_box));
    g.connect(read_node_box_id, write_node_box_id, 0);

    g.execute(write_node_id1);
    g.execute(write_node_id2);
    g.execute(write_node_box_id);

    return 0;
}
