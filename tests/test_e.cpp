#include <iostream>
#include <opencompgraph.h>

namespace ocg = open_comp_graph;

int test_e(std::shared_ptr<ocg::Cache> cache) {
    std::cout << "====================================== test_e()" << '\n';
    auto bench = ocg::internal::BenchmarkTime();

    auto g = ocg::Graph();

    std::cout << "Node() ========================================" << '\n';
    auto read_node = ocg::Node(ocg::NodeType::kReadImage, "read");
    auto null_node1 = ocg::Node(ocg::NodeType::kNull, "null1");
    auto null_node2 = ocg::Node(ocg::NodeType::kNull, "null2");
    auto null_node3 = ocg::Node(ocg::NodeType::kNull, "null3");
    auto null_node4 = ocg::Node(ocg::NodeType::kNull, "null4");
    auto write_node = ocg::Node(ocg::NodeType::kWriteImage, "write");
    std::cout << "read_node=" << &read_node << '\n';
    std::cout << "write_node=" << &write_node << '\n';

    read_node.set_attr_str("file_path", "tests/data/oiio-images/tahoe-gps.jpg");
    write_node.set_attr_str("file_path", "./tests/data/out/test_e_out.png");

    auto read_node_id = g.add_node(read_node);
    auto null_node_id1 = g.add_node(null_node1);
    auto null_node_id2 = g.add_node(null_node2);
    auto null_node_id3 = g.add_node(null_node3);
    auto null_node_id4 = g.add_node(null_node4);
    auto write_node_id = g.add_node(write_node);
    g.connect(read_node_id, null_node_id1, 0);
    g.connect(null_node_id1, null_node_id2, 0);
    g.connect(null_node_id2, null_node_id3, 0);
    g.connect(null_node_id3, null_node_id4, 0);
    g.connect(null_node_id4, write_node_id, 0);

    g.execute(write_node_id, cache);

    bench.stop();
    bench.print("Test E:");

    return 0;
}
