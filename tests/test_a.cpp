#include <iostream>
#include <opencompgraph.h>
#include "generate_mesh.h"

namespace ocg = open_comp_graph;


int test_a(const bool debug_print, std::shared_ptr<ocg::Cache> cache) {
    if (debug_print) {
        std::cout << "=========================================== test_a()" << '\n';
    }
    auto bench = ocg::internal::BenchmarkTime();

    auto graph = ocg::Graph();
    auto read_node = graph.create_node(ocg::NodeType::kReadImage, "my_read_node");
    auto lens_node = graph.create_node(ocg::NodeType::kLensDistort, "lens_node");
    auto write_node = graph.create_node(ocg::NodeType::kWriteImage, "write_node");
    graph.set_node_attr_str(read_node, "file_path", "./tests/data/checker_8bit_rgba_3840x2160.png");
    graph.set_node_attr_f32(lens_node, "k1", 0.2f);
    graph.set_node_attr_f32(lens_node, "k2", -0.05f);
    graph.set_node_attr_str(write_node, "file_path", "./tests/data/out/test_a_out1.jpg");

    graph.connect(read_node, lens_node, 0);
    graph.connect(lens_node, write_node, 0);
    auto status = graph.execute(lens_node, cache);
    if (debug_print) {
        std::cout << "Graph as string:\n"
                  << graph.data_debug_string();
    }
    if (status != ocg::ExecuteStatus::kSuccess) {
        std::cout << "ERROR: Did not compute graph correctly!!!:" << std::endl;
        return 1;
    }

    auto stream_data = graph.output_stream();
    auto num_deformers = stream_data.deformers_len();
    if (debug_print) {
        std::cout << "num_deformers=" << num_deformers << '\n';
    }

    uint32_t divisions_x = 2;
    uint32_t divisions_y = 2;
    size_t pos_count = 0;
    size_t uv_count = 0;
    size_t tri_count = 0;

    // 2 x 2 divisions = 1 face (2 triangles).
    divisions_x = 2;
    divisions_y = 2;
    const char* file_path_2x2 = "./tests/data/out/test_a_2x2_out.obj";
    generate_mesh(
        debug_print,
        divisions_x,
        divisions_y,
        stream_data,
        file_path_2x2,
        pos_count,
        uv_count,
        tri_count);

    // 3 x 3 divisions = 4 faces (8 triangles).
    stream_data = graph.output_stream();
    divisions_x = 3;
    divisions_y = 3;
    const char* file_path_3x3 = "./tests/data/out/test_a_3x3_out.obj";
    generate_mesh(
        debug_print,
        divisions_x,
        divisions_y,
        stream_data,
        file_path_3x3,
        pos_count,
        uv_count,
        tri_count);

    // 4 x 4 divisions = 9 faces (18 triangles).
    stream_data = graph.output_stream();
    divisions_x = 4;
    divisions_y = 4;
    const char* file_path_4x4 = "./tests/data/out/test_a_4x4_out.obj";
    generate_mesh(
        debug_print,
        divisions_x,
        divisions_y,
        stream_data,
        file_path_4x4,
        pos_count,
        uv_count,
        tri_count);

    // 2 x 4 divisions = 8 faces (16 triangles).
    stream_data = graph.output_stream();
    divisions_x = 2;
    divisions_y = 4;
    const char* file_path_2x4 = "./tests/data/out/test_a_2x4_out.obj";
    generate_mesh(
        debug_print,
        divisions_x,
        divisions_y,
        stream_data,
        file_path_2x4,
        pos_count,
        uv_count,
        tri_count);

    // 16 x 32 divisions = 512 faces (1024 triangles).
    stream_data = graph.output_stream();
    divisions_x = 16;
    divisions_y = 32;
    const char* file_path_16x32 = "./tests/data/out/test_a_16x32_out.obj";
    generate_mesh(
        debug_print,
        divisions_x,
        divisions_y,
        stream_data,
        file_path_16x32,
        pos_count,
        uv_count,
        tri_count);

    // 32 x 16 divisions = 512 faces (1024 triangles).
    stream_data = graph.output_stream();
    divisions_x = 32;
    divisions_y = 16;
    const char* file_path_32x16 = "./tests/data/out/test_a_32x16_out.obj";
    generate_mesh(
        debug_print,
        divisions_x,
        divisions_y,
        stream_data,
        file_path_32x16,
        pos_count,
        uv_count,
        tri_count);

    // 192 x 108 divisions = 20736 faces (41472 triangles).
    //
    // This represents 1 vertex per-10 pixels in a HD 1920 x 1080 image.
    stream_data = graph.output_stream();
    divisions_x = 192;
    divisions_y = 108;
    const char* file_path_192x108 = "./tests/data/out/test_a_192x108_out.obj";
    generate_mesh(
        debug_print,
        divisions_x,
        divisions_y,
        stream_data,
        file_path_192x108,
        pos_count,
        uv_count,
        tri_count);

    status = graph.execute(write_node, cache);
    if (status != ocg::ExecuteStatus::kSuccess) {
        return 1;
    }
    if (debug_print) {
        std::cout << "Graph as string:\n"
                  << graph.data_debug_string();
    }

    if (debug_print) {
        bench.stop();
        bench.print("Test A:");
    }

    return 0;
}
