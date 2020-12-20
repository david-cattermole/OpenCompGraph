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
    // read_node.set_attr_str("file_path", "./tests/data/checker_8bit_rgba_3840x2160.png");
    read_node.set_attr_str("file_path", "./tests/data/oiio-images/checker.tif");
    write_node.set_attr_str("file_path", "./tests/data/out/test_d_out1.png");
    auto read_node_id1 = g.add_node(read_node);
    auto write_node_id1 = g.add_node(write_node);
    g.connect(read_node_id1, write_node_id1, 0);

    // std::cout << "Node.add_node(Box<NodeImpl>) ==========" << std::endl;
    // auto read_node_box = ocg::internal::create_node_box(
    //         ocg::NodeType::ReadImage, 0);
    // auto write_node_box = ocg::internal::create_node_box(
    //         ocg::NodeType::WriteImage, 1);
    // std::cout << "read_node_box=" << &read_node_box << std::endl;
    // std::cout << "write_node_box=" << &write_node_box << std::endl;
    // read_node_box->set_attr_str("file_path", "./tests/data/checker_8bit_rgba_3840x2160.png");
    // write_node_box->set_attr_str("file_path", "./tests/data/out/test_d_out2.png");
    // auto read_node_box_id = g.add_node(std::move(read_node_box));
    // auto write_node_box_id = g.add_node(std::move(write_node_box));
    // g.connect(read_node_box_id, write_node_box_id, 0);

    g.execute(write_node_id1);
    // g.execute(write_node_box_id);

    return 0;
}
