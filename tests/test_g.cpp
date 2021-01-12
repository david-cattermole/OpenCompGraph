#include <iostream>
#include <opencompgraph.h>
#include "generate_frame_range.h"

namespace ocg = open_comp_graph;

int test_g(const bool debug_print, std::shared_ptr<ocg::Cache> cache) {
    if (debug_print) {
        std::cout << "=========================================== test_g()" << '\n';
    }
    auto bench = ocg::internal::BenchmarkTime();

    auto frames = generate_frame_range(1, 1);
    auto graph = ocg::Graph();

    auto read_node = graph.create_node(ocg::NodeType::kReadImage, "read");
    auto null_node1 = graph.create_node(ocg::NodeType::kNull, "null1");
    auto null_node2 = graph.create_node(ocg::NodeType::kNull, "null2");
    auto write_node = graph.create_node(ocg::NodeType::kWriteImage, "write");
    if (debug_print) {
        std::cout << "read_node=" << &read_node << '\n';
        std::cout << "write_node=" << &write_node << '\n';
    }

    graph.set_node_attr_str(read_node, "file_path", "tests/data/checker_8bit_rgba_3840x2160.png");
    graph.set_node_attr_str(write_node, "file_path", "./tests/data/out/test_g_out.png");

    // Ensure the node exists in the graph, check by finding based on
    // the id queried before.
    auto read_node_exists = graph.node_exists(read_node);
    auto null_node1_exists = graph.node_exists(null_node1);
    auto null_node2_exists = graph.node_exists(null_node2);
    auto write_node_exists = graph.node_exists(write_node);
    if (debug_print) {
        std::cout << "read_node_exists=" << read_node_exists << '\n';
        std::cout << "null_node1_exists=" << null_node1_exists << '\n';
        std::cout << "null_node2_exists=" << null_node2_exists << '\n';
        std::cout << "write_node_exists=" << write_node_exists << '\n';
    }
    assert(read_node_exists);
    assert(null_node1_exists);
    assert(null_node2_exists);
    assert(write_node_exists);
    
    // Connect nodes together.
    graph.connect(read_node, null_node1, 0);
    graph.connect(null_node1, null_node2, 0);
    graph.connect(null_node2, write_node, 0);    

    graph.execute(write_node, frames, cache);
    if (debug_print) {
        std::cout << "Graph as string:\n"
                  << graph.data_debug_string();


        bench.stop();
        bench.print("Test G:");
    }
    return 0;
}
