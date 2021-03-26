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
    graph.set_node_attr_str(
        read_node, "file_path", "./tests/data/checker_8bit_rgba_3840x2160.png");
    graph.set_node_attr_f32(tfm_node, "translate_x", 2.0f);
    graph.set_node_attr_str(
        write_node, "file_path",
        "./tests/data/out/test_node_transform_out1.jpg");

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
    const char* file_path_2x2 = "./tests/data/out/test_node_transform_2x2_out.obj";
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
    const char* file_path_3x3 = "./tests/data/out/test_node_transform_3x3_out.obj";
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
        bench.print("Test Node Transform:");
    }

    return 0;
}
