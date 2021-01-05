#include <iostream>
#include <opencompgraph.h>

namespace ocg = open_comp_graph;

int test_d(const bool debug_print, std::shared_ptr<ocg::Cache> cache) {
    if (debug_print) {
        std::cout << "=========================================== test_d()" << '\n';
    }
    auto bench = ocg::internal::BenchmarkTime();

    auto graph = ocg::Graph();

    auto read_node = graph.create_node(ocg::NodeType::kReadImage, "my_read_node");
    auto write_node = graph.create_node(ocg::NodeType::kWriteImage, "my_write_node");
    if (debug_print) {
        std::cout << "read_node=" << &read_node << '\n';
        std::cout << "write_node=" << &write_node << '\n';
    }
    auto read_node_id1 = read_node.get_id();
    auto write_node_id1 = write_node.get_id();
    graph.set_node_attr_str(read_node, "file_path", "./tests/data/oiio-images/checker.tif");
    graph.set_node_attr_str(write_node, "file_path", "./tests/data/out/test_d_out1.png");

    graph.connect(read_node, write_node, 0);
    graph.execute(write_node, cache);
    if (debug_print) {
        std::cout << "Graph as string:\n"
                  << graph.data_debug_string();

        bench.stop();
        bench.print("Test D:");
    }

    return 0;
}
