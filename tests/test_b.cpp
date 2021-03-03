#include <iostream>
#include <string>
#include <opencompgraph.h>
#include "generate_mesh.h"
#include "generate_frame_range.h"

namespace ocg = open_comp_graph;


int test_b(const bool debug_print, std::shared_ptr<ocg::Cache> cache) {
    if (debug_print) {
        std::cout << "=========================================== test_b()" << '\n';
    }
    auto bench = ocg::internal::BenchmarkTime();

    // // How to call C++ functions from C++ (exposed via the CXX
    // // API).
    //
    // auto app_name = std::string("my awesome demo");
    // auto x = ocg::cpp::make_thingc(app_name);
    // auto name = ocg::cpp::get_name(*x);
    // if (debug_print) {
    //     std::cout << name << '\n';
    // }

    auto frames = generate_frame_range(1, 1);
    auto graph = ocg::Graph();
    auto read_node = graph.create_node(ocg::NodeType::kReadImage, "my_read_node");
    auto tfm_node = graph.create_node(ocg::NodeType::kTransform, "transform");
    auto write_node = graph.create_node(ocg::NodeType::kWriteImage, "write_node");
    graph.set_node_attr_str(read_node, "file_path", "./tests/data/checker_8bit_rgba_3840x2160.png");
    graph.set_node_attr_f32(tfm_node, "translate_x", 2.0f);
    graph.set_node_attr_str(write_node, "file_path", "./tests/data/out/test_b_out1.jpg");

    graph.connect(read_node, tfm_node, 0);
    graph.connect(tfm_node, write_node, 0);
    auto status = graph.execute(tfm_node, frames, cache);
    if (debug_print) {
        std::cout << "Graph as string:\n"
                  << graph.data_debug_string();
    }
    if (status != ocg::ExecuteStatus::kSuccess) {
        std::cout << "ERROR: Did not compute graph correctly!!!:" << std::endl;
        return 1;
    }

    auto stream_data = graph.output_stream();
    auto tfm_matrix = stream_data.transform_matrix();
    if (debug_print) {
        std::cout << "transform_matrix:\n"
                  << " row0="
                  << tfm_matrix.m00 << ","
                  << tfm_matrix.m01 << ","
                  << tfm_matrix.m02 << "\n"
                  << " row1="
                  << tfm_matrix.m10 << ","
                  << tfm_matrix.m11 << ","
                  << tfm_matrix.m12 << "\n"
                  << " row2="
                  << tfm_matrix.m20 << ","
                  << tfm_matrix.m21 << ","
                  << tfm_matrix.m22 << "\n"
                  << '\n';
    }

    uint32_t divisions_x = 2;
    uint32_t divisions_y = 2;
    size_t pos_count = 0;
    size_t uv_count = 0;
    size_t tri_count = 0;

    // 2 x 2 divisions = 1 face (2 triangles).
    auto direction = ocg::DeformerDirection::kForward;
    divisions_x = 2;
    divisions_y = 2;
    const char* file_path_2x2 = "./tests/data/out/test_b_2x2_out.obj";
    generate_mesh(
        debug_print,
        divisions_x,
        divisions_y,
        stream_data,
        direction,
        file_path_2x2,
        pos_count,
        uv_count,
        tri_count);

    // 3 x 3 divisions = 4 faces (8 triangles).
    stream_data = graph.output_stream();
    divisions_x = 3;
    divisions_y = 3;
    const char* file_path_3x3 = "./tests/data/out/test_b_3x3_out.obj";
    generate_mesh(
        debug_print,
        divisions_x,
        divisions_y,
        stream_data,
        direction,
        file_path_3x3,
        pos_count,
        uv_count,
        tri_count);

    status = graph.execute(write_node, frames, cache);
    if (status != ocg::ExecuteStatus::kSuccess) {
        std::cout << "ERROR=" << static_cast<uint32_t>(status) << '\n';
        return 1;
    }
    if (debug_print) {
        std::cout << "Graph as string:\n"
                  << graph.data_debug_string();

        bench.stop();
        bench.print("Test B:");
    }

    return 0;
}
