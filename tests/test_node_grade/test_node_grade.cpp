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
 * Read and grade an image. Never writes out the image.
 */

#include <iostream>
#include <opencompgraph.h>
#include "../generate_frame_range.h"

namespace ocg = open_comp_graph;

int test_node_grade(const bool debug_print,
                    std::shared_ptr<ocg::Cache> cache) {
    if (debug_print) {
        std::cout << "============================== test_node_grade()" << '\n';
    }
    auto bench = ocg::internal::BenchmarkTime();

    auto frames = generate_frame_range(1, 1);
    auto graph = ocg::Graph();

    auto read_node = graph.create_node(ocg::NodeType::kReadImage, "read");
    auto grade_node = graph.create_node(ocg::NodeType::kGrade, "grade");

    graph.set_node_attr_str(
        read_node, "file_path", "tests/data/ocg-testdata/images/checker/checker_8bit_rgba_8x8.png");
    graph.set_node_attr_f32(grade_node, "multiply_g", 2.0);

    // Connect nodes together and execute
    graph.connect(read_node, grade_node, 0);
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
    assert(pixel_width == 8);
    assert(pixel_height == 8);
    assert(pixel_num_channels == 3);

    if (debug_print) {
        bench.stop();
        bench.print("Test Node Grade:");
    }

    return 0;
}
