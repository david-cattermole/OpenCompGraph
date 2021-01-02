#include <iostream>
#include <opencompgraph.h>

namespace ocg = open_comp_graph;

int test_h(std::shared_ptr<ocg::Cache> cache) {
    std::cout << "=========================================== test_h()" << '\n';
    auto bench = ocg::internal::BenchmarkTime();

    auto graph = ocg::Graph();

    auto read_node = graph.create_node(ocg::NodeType::kReadImage, "read");
    auto grade_node = graph.create_node(ocg::NodeType::kGrade, "grade");

    graph.set_node_attr_str(
        read_node, "file_path", "tests/data/checker_8bit_rgba_8x8.png");
    graph.set_node_attr_f32(grade_node, "multiply", 2.0);

    // Connect nodes together and execute
    graph.connect(read_node, grade_node, 0);
    auto exec_status = graph.execute(grade_node, cache);
    std::cout << "execute status: "
              << static_cast<uint32_t>(exec_status) << '\n';
    std::cout << "Graph as string:\n"
              << graph.data_debug_string();

    auto stream_data = graph.output_stream();
    auto pixel_buffer = stream_data.pixel_buffer();
    auto pixel_width = stream_data.pixel_width();
    auto pixel_height = stream_data.pixel_height();
    auto pixel_num_channels = stream_data.pixel_num_channels();
    std::cerr << "pixels: "
              << pixel_width << "x" << pixel_height
              << " c="
              << static_cast<uint32_t>(pixel_num_channels)
              << " | data=" << &pixel_buffer << '\n';
    assert(pixel_width == 8);
    assert(pixel_height == 8);
    assert(pixel_num_channels == 3);

    bench.stop();
    bench.print("Test H:");

    return 0;
}
