/*
 * Copyright (C) 2020, 2021 David Cattermole.
 *
 * This file is part of OpenCompGraph.
 *
 * OpenCompGraph is free software: you can redistribute it and/or modify it
 * under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * OpenCompGraph is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with OpenCompGraph.  If not, see <https://www.gnu.org/licenses/>.
 * ====================================================================
 *
 * Basic "Read image -> transform" connection and execution, with
 * geometry output. Write node is connected by not executed.
 */

#include <iostream>
#include <string>
#include <opencompgraph.h>
#include "../generate_mesh.h"
#include "../generate_frame_range.h"

namespace ocg = open_comp_graph;


int test_node_transform(const bool debug_print,
                        std::shared_ptr<ocg::Cache> cache) {
    if (debug_print) {
        std::cout << "========================== test_node_transform()" << '\n';
    }
    auto bench = ocg::internal::BenchmarkTime();

    auto frames = generate_frame_range(1, 1);
    auto graph = ocg::Graph();
    auto read_node = graph.create_node(ocg::NodeType::kReadImage, "my_read_node");
    auto tfm1_node = graph.create_node(ocg::NodeType::kTransform, "transform1");
    auto tfm2_node = graph.create_node(ocg::NodeType::kTransform, "transform2");
    auto tfm3_node = graph.create_node(ocg::NodeType::kTransform, "transform3");
    auto write1_node = graph.create_node(ocg::NodeType::kWriteImage, "write_node1");
    auto write2_node = graph.create_node(ocg::NodeType::kWriteImage, "write_node2");
    auto write3_node = graph.create_node(ocg::NodeType::kWriteImage, "write_node3");
    graph.set_node_attr_str(
        read_node, "file_path", "./tests/data/checker_8bit_rgba_3840x2160.png");

    // Test 1
    graph.set_node_attr_f32(tfm1_node, "translate_x", 0.5f);
    graph.set_node_attr_f32(tfm1_node, "translate_y", 0.5f);
    graph.set_node_attr_str(
        write1_node, "file_path",
        "./tests/data/out/test_node_transform_out1_translate.jpg");

    // Test 2
    graph.set_node_attr_f32(tfm2_node, "rotate", 45.0f);
    graph.set_node_attr_str(
        write2_node, "file_path",
        "./tests/data/out/test_node_transform_out2_rotate.jpg");

    // Test 3
    graph.set_node_attr_f32(tfm3_node, "scale_x", 2.0f);
    graph.set_node_attr_f32(tfm3_node, "scale_y", 2.0f);
    graph.set_node_attr_str(
        write3_node, "file_path",
        "./tests/data/out/test_node_transform_out3_scale.jpg");

    // Graph Connections
    graph.connect(read_node, tfm1_node, 0);
    graph.connect(read_node, tfm2_node, 0);
    graph.connect(read_node, tfm3_node, 0);
    graph.connect(tfm1_node, write1_node, 0);
    graph.connect(tfm2_node, write2_node, 0);
    graph.connect(tfm3_node, write3_node, 0);

    auto status = graph.execute(tfm1_node, frames, cache);
    if (debug_print) {
        std::cout << "Graph as string:\n"
                  << graph.data_debug_string();
    }
    if (status != ocg::ExecuteStatus::kSuccess) {
        std::cout << "ERROR: Did not compute graph correctly!!!:" << std::endl;
        return 1;
    }

    uint32_t divisions_x = 2;
    uint32_t divisions_y = 2;
    size_t pos_count = 0;
    size_t uv_count = 0;
    size_t tri_count = 0;

    // 2 x 2 divisions = 1 face (2 triangles).
    auto stream_data = graph.output_stream();
    divisions_x = 2;
    divisions_y = 2;
    const char* file_path_2x2 = "./tests/data/out/test_node_transform_2x2_out.obj";
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
    const char* file_path_3x3 = "./tests/data/out/test_node_transform_3x3_out.obj";
    generate_mesh(
        debug_print,
        divisions_x,
        divisions_y,
        stream_data,
        file_path_3x3,
        pos_count,
        uv_count,
        tri_count);

    status = graph.execute(write1_node, frames, cache);
    if (status != ocg::ExecuteStatus::kSuccess) {
        std::cout << "ERROR=" << static_cast<uint32_t>(status) << '\n';
        return 1;
    }

    status = graph.execute(write2_node, frames, cache);
    if (status != ocg::ExecuteStatus::kSuccess) {
        std::cout << "ERROR=" << static_cast<uint32_t>(status) << '\n';
        return 1;
    }

    status = graph.execute(write3_node, frames, cache);
    if (status != ocg::ExecuteStatus::kSuccess) {
        std::cout << "ERROR=" << static_cast<uint32_t>(status) << '\n';
        return 1;
    }

    if (debug_print) {
        std::cout << "Graph as string:\n"
                  << graph.data_debug_string();

        bench.stop();
        bench.print("Test Node Transform:");
    }

    return 0;
}
