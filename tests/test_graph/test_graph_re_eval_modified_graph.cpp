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
 * Re-execute the same graph with different values. Ensure the graph
 * can re-evaluate correctly.
 */

#include <iostream>
#include <random>
#include <sstream>  // stringstream
#include <cmath>    // fabs
#include <opencompgraph.h>
#include "../generate_frame_range.h"

namespace ocg = open_comp_graph;

template<typename NUM_TYPE>
inline
std::string number_to_string(NUM_TYPE num) {
    std::stringstream ss;
    ss << num;
    return ss.str();
}

int test_graph_re_eval_modified_graph(const bool debug_print,
                                      std::shared_ptr<ocg::Cache> cache) {
    if (debug_print) {
        std::cout << "============ test_graph_re_eval_modified_graph()" << '\n';
    }
    auto bench = ocg::internal::BenchmarkTime();

    auto frames = generate_frame_range(1, 1);
    auto graph = ocg::Graph();

    auto read_node = graph.create_node(ocg::NodeType::kReadImage, "read");
    auto grade_node = graph.create_node(ocg::NodeType::kGrade, "grade");
    auto write_node = graph.create_node(ocg::NodeType::kWriteImage, "write");

    graph.set_node_attr_str(
            read_node, "file_path",
            "tests/data/openexr-images/TestImages/SquaresSwirls.exr");

    // Connect nodes together and execute
    graph.connect(read_node, grade_node, 0);
    graph.connect(grade_node, write_node, 0);

    std::random_device random_device;
    std::mt19937 generator(random_device());
    std::vector<int32_t> random_frames;
    std::vector<float> mult_r_values{0.5f, 1.0f, 0.8f, 0.2f, 0.0f, 2.0f, 1.5f};
    std::vector<float> mult_g_values{0.5f, 1.0f, 0.8f, 0.2f, 0.0f, 2.0f, 1.5f};
    std::vector<float> mult_b_values{0.5f, 1.0f, 0.8f, 0.2f, 0.0f, 2.0f, 1.5f};
    std::shuffle(mult_r_values.begin(), mult_r_values.end(), generator);
    std::shuffle(mult_g_values.begin(), mult_g_values.end(), generator);
    std::shuffle(mult_b_values.begin(), mult_b_values.end(), generator);

    // Execute graph multiple times, with different Grade values
    for (size_t i = 0; i < mult_r_values.size(); i++) {
        auto mult_r = mult_r_values[i];
        auto mult_g = mult_g_values[i];
        auto mult_b = mult_b_values[i];
        graph.set_node_attr_f32(grade_node, "multiply_r", mult_r);
        graph.set_node_attr_f32(grade_node, "multiply_g", mult_g);
        graph.set_node_attr_f32(grade_node, "multiply_b", mult_b);

        auto out_file = "./tests/data/out/test_graph_re_eval_modified_graph_out" + number_to_string(i) + ".png";
        graph.set_node_attr_str(write_node, "file_path", out_file);

        auto exec_status = graph.execute(write_node, frames, cache);
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
    }

    std::cout << "Cache: capacity_bytes: "
              << cache->capacity_bytes() << '\n';
    std::cout << "Cache: used_bytes: "
              << cache->used_bytes() << '\n';
    std::cout << "Cache: count: "
              << cache->count() << '\n';
    std::cout << "Cache: \n"
              << cache->data_debug_string() << '\n';

    if (debug_print) {
        bench.stop();
        bench.print("Test Graph Re-Eval Modified Graph:");
    }

    return 0;
}
