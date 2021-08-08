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
 * Read and grade an image.
 */

#include <iostream>
#include <opencompgraph.h>
#include "../generate_frame_range.h"

namespace ocg = open_comp_graph;

int test_node_grade_concat(const bool debug_print,
                           std::shared_ptr<ocg::Cache> cache) {
    if (debug_print) {
        std::cout << "======================= test_node_grade_concat()" << '\n';
    }
    auto bench = ocg::internal::BenchmarkTime();

    auto frames = generate_frame_range(1, 1);
    auto graph = ocg::Graph();

    auto read_type = ocg::NodeType::kReadImage;
    auto grade_type = ocg::NodeType::kGrade;
    auto write_type = ocg::NodeType::kWriteImage;

    auto read_node = graph.create_node(read_type, "read");
    auto grade_node = graph.create_node(grade_type, "grade_forward");
    auto grade_reverse_node = graph.create_node(grade_type, "grade_backward");
    auto write1_node = graph.create_node(write_type, "write1");
    auto write2_node = graph.create_node(write_type, "write2");

    graph.set_node_attr_str(
        read_node, "file_path",
        "tests/data/openexr-images/ScanLines/Desk.exr");
    graph.set_node_attr_str(
        write1_node, "file_path",
        "./tests/data/out/test_node_grade_concat_out1.png");
    graph.set_node_attr_str(
        write2_node, "file_path",
        "./tests/data/out/test_node_grade_concat_out2.png");

    // Grade node (forward)
    graph.set_node_attr_i32(grade_node, "process_r", 1);
    graph.set_node_attr_i32(grade_node, "process_g", 1);
    graph.set_node_attr_i32(grade_node, "process_b", 1);
    graph.set_node_attr_i32(grade_node, "process_a", 0);

    graph.set_node_attr_f32(grade_node, "white_point_r", 2.0f);
    graph.set_node_attr_f32(grade_node, "white_point_g", 2.0f);
    graph.set_node_attr_f32(grade_node, "white_point_b", 2.0f);
    graph.set_node_attr_f32(grade_node, "white_point_a", 2.0f);

    graph.set_node_attr_f32(grade_node, "black_point_r", 0.05f);
    graph.set_node_attr_f32(grade_node, "black_point_g", 0.05f);
    graph.set_node_attr_f32(grade_node, "black_point_b", 0.05f);
    graph.set_node_attr_f32(grade_node, "black_point_a", 0.05f);

    graph.set_node_attr_f32(grade_node, "lift_r", 0.01f);
    graph.set_node_attr_f32(grade_node, "gain_r", 0.8f);

    graph.set_node_attr_f32(grade_node, "offset_r", 0.01f);
    graph.set_node_attr_f32(grade_node, "offset_g", 0.01f);
    graph.set_node_attr_f32(grade_node, "offset_b", 0.01f);
    graph.set_node_attr_f32(grade_node, "offset_a", 0.01f);

    graph.set_node_attr_f32(grade_node, "multiply_g", 1.2f);
    graph.set_node_attr_f32(grade_node, "gamma_r", 1.2f);
    graph.set_node_attr_f32(grade_node, "gamma_b", 0.8f);

    // Grade node (reversed)
    graph.set_node_attr_i32(grade_reverse_node, "reverse", 1);

    graph.set_node_attr_i32(grade_reverse_node, "process_r", 1);
    graph.set_node_attr_i32(grade_reverse_node, "process_g", 1);
    graph.set_node_attr_i32(grade_reverse_node, "process_b", 1);
    graph.set_node_attr_i32(grade_reverse_node, "process_a", 0);

    graph.set_node_attr_f32(grade_reverse_node, "white_point_r", 2.0f);
    graph.set_node_attr_f32(grade_reverse_node, "white_point_g", 2.0f);
    graph.set_node_attr_f32(grade_reverse_node, "white_point_b", 2.0f);
    graph.set_node_attr_f32(grade_reverse_node, "white_point_a", 2.0f);

    graph.set_node_attr_f32(grade_reverse_node, "black_point_r", 0.05f);
    graph.set_node_attr_f32(grade_reverse_node, "black_point_g", 0.05f);
    graph.set_node_attr_f32(grade_reverse_node, "black_point_b", 0.05f);
    graph.set_node_attr_f32(grade_reverse_node, "black_point_a", 0.05f);

    graph.set_node_attr_f32(grade_reverse_node, "lift_r", 0.1f);
    graph.set_node_attr_f32(grade_reverse_node, "gain_r", 0.8f);

    graph.set_node_attr_f32(grade_reverse_node, "offset_r", 0.1f);
    graph.set_node_attr_f32(grade_reverse_node, "offset_g", 0.1f);
    graph.set_node_attr_f32(grade_reverse_node, "offset_b", 0.1f);
    graph.set_node_attr_f32(grade_reverse_node, "offset_a", 0.1f);

    graph.set_node_attr_f32(grade_reverse_node, "multiply_g", 2.0f);
    graph.set_node_attr_f32(grade_reverse_node, "gamma_r", 2.0f);
    graph.set_node_attr_f32(grade_reverse_node, "gamma_b", 2.0f);

    // Connect nodes together
    graph.connect(read_node, grade_node, 0);
    graph.connect(grade_node, grade_reverse_node, 0);
    graph.connect(grade_node, write1_node, 0);
    graph.connect(grade_reverse_node, write2_node, 0);

    // Execute test grade node
    auto exec_status = graph.execute(grade_node, frames, cache);
    if (debug_print) {
        std::cout << "execute status: "
                  << static_cast<uint32_t>(exec_status) << '\n';
        std::cout << "Graph as string:\n"
                  << graph.data_debug_string();
    }

    auto stream_data = graph.output_stream();
    auto display_window = stream_data.display_window();
    if (debug_print) {
        std::cerr << "display_window:"
                  << " min_x=" << display_window.min_x
                  << " min_y=" << display_window.min_y
                  << " max_x=" << display_window.max_x
                  << " max_y=" << display_window.max_y
                  << '\n';
    }
    auto data_window = stream_data.data_window();
    if (debug_print) {
        std::cerr << "data_window:"
                  << " min_x=" << data_window.min_x
                  << " min_y=" << data_window.min_y
                  << " max_x=" << data_window.max_x
                  << " max_y=" << data_window.max_y
                  << '\n';
    }
    auto pixel_buffer = stream_data.pixel_buffer();
    auto pixel_width = stream_data.pixel_width();
    auto pixel_height = stream_data.pixel_height();
    auto pixel_num_channels = stream_data.pixel_num_channels();
    if (debug_print) {
        std::cerr << "pixels: "
                  << pixel_width << "x" << pixel_height
                  << " c="
                  << static_cast<uint32_t>(pixel_num_channels)
                  << " | data=" << &pixel_buffer << '\n';
    }
    // assert(pixel_width == 8);
    // assert(pixel_height == 8);
    // assert(pixel_num_channels == 3);

    // Execute Write nodes
    exec_status = graph.execute(write1_node, frames, cache);
    if (debug_print) {
        std::cout << "execute status: "
                  << static_cast<uint32_t>(exec_status) << '\n';
        std::cout << "Graph as string:\n"
                  << graph.data_debug_string();
    }

    exec_status = graph.execute(write2_node, frames, cache);
    if (debug_print) {
        std::cout << "execute status: "
                  << static_cast<uint32_t>(exec_status) << '\n';
        std::cout << "Graph as string:\n"
                  << graph.data_debug_string();
    }

    if (debug_print) {
        bench.stop();
        bench.print("Test Node Grade:");
    }

    return 0;
}
