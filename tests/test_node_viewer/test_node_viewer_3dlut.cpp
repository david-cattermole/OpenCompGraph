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
 * Viewer node to bake and visualise the node.
 */

#include <iostream>
#include <opencompgraph.h>
#include "../generate_frame_range.h"

namespace ocg = open_comp_graph;

int test_node_viewer_3dlut(const bool debug_print,
                           std::shared_ptr<ocg::Cache> cache) {
    if (debug_print) {
        std::cout << "======================= test_node_viewer_3dlut()" << '\n';
    }
    auto bench = ocg::internal::BenchmarkTime();

    auto frames = generate_frame_range(1001, 1001);
    auto graph = ocg::Graph();

    auto read_type = ocg::NodeType::kReadImage;
    auto grade_type = ocg::NodeType::kGrade;
    auto viewer_type = ocg::NodeType::kViewer;
    auto write_type = ocg::NodeType::kWriteImage;

    auto read_node = graph.create_node(read_type, "read");
    auto grade_node = graph.create_node(grade_type, "grade1");
    auto viewer1_node = graph.create_node(viewer_type, "viewer1");
    auto write1_node = graph.create_node(write_type, "write1");

    // Read node
    graph.set_node_attr_str(
        read_node, "file_path",
        "./tests/data/ocg-testdata/images/color_bars/3840x2160_png/color_bars.####.png");

    // Grade node
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

    graph.set_node_attr_f32(grade_node, "multiply_r", 2.0f);
    graph.set_node_attr_f32(grade_node, "multiply_g", 1.2f);
    graph.set_node_attr_f32(grade_node, "gamma_r", 1.2f);
    graph.set_node_attr_f32(grade_node, "gamma_b", 0.8f);

    // Viewer Nodes
    graph.set_node_attr_i32(viewer1_node, "bake_option", 0);
    graph.set_node_attr_i32(viewer1_node, "crop_to_format", 0);

    // Write Nodes
    graph.set_node_attr_str(
        write1_node, "file_path",
        "./tests/data/out/test_node_viewer_3dlut_out1.####.jpg");

    // Connect nodes together
    graph.connect(read_node, grade_node, 0);
    graph.connect(grade_node, viewer1_node, 0);
    graph.connect(viewer1_node, write1_node, 0);

    // Execute Viewer node
    auto exec_status = graph.execute(viewer1_node, frames, cache);
    if (debug_print) {
        std::cout << "execute status: "
                  << static_cast<uint32_t>(exec_status) << '\n';
        std::cout << "Graph as string:\n"
                  << graph.data_debug_string();
    }

    auto stream_data = graph.output_stream();
    std::cout << "stream_data.color_ops_len="
              << stream_data.color_ops_len() << '\n';
    std::cout << "stream_data.color_ops_hash="
              << stream_data.color_ops_hash() << '\n';

    // Generate a 3D volume texture to be used to look
    // up color space transforms.
    auto lut_edge_size = 32;
    auto from_color_space = stream_data.clone_image_spec().color_space;
    auto to_color_space = "Linear";
    auto use_3dlut = from_color_space != to_color_space;

    auto lut_image = ocg::get_color_transform_3dlut(
        from_color_space, to_color_space,
        lut_edge_size, cache);
    if (debug_print) {
        std::cout << "lut_edge_size=" << lut_edge_size << '\n';
        std::cout << "from_color_space=" << from_color_space << '\n';
        std::cout << "to_color_space=" << to_color_space << '\n';
        std::cout << "lut_image.spec.color_space=" << lut_image.spec.color_space << '\n';
        std::cout << "lut_image.pixel_block.width()=" << lut_image.pixel_block->width() << '\n';
        std::cout << "lut_image.pixel_block.height()=" << lut_image.pixel_block->height() << '\n';
        std::cout << "lut_image.pixel_block.num_channels()=" << lut_image.pixel_block->num_channels() << '\n';

        auto pixel_buffer = lut_image.pixel_block->as_slice_f32();
        std::cout << "lut_image.pixel_block length=" << pixel_buffer.length() << '\n';
        auto max_index = pixel_buffer.length();
        if (max_index >= 32) {
            max_index = 32;
        }
        for (auto i = 0; i < max_index; ++i) {
            std::cout << "num: " << i << "=" << pixel_buffer[i] << "\n";
        }
    }

    auto num_channels = 3;
    auto lut_3d_ops_image = ocg::get_color_ops_lut(
        stream_data, lut_edge_size, num_channels, cache);
    if (debug_print) {
        std::cout << "ColorOps 3D LUT (RGB)" << '\n';
        std::cout << "lut_edge_size=" << lut_edge_size << '\n';
        std::cout << "lut_3d_ops_image.spec.color_space="
                  << lut_3d_ops_image.spec.color_space << '\n';
        std::cout << "lut_3d_ops_image.pixel_block.width()="
                  << lut_3d_ops_image.pixel_block->width() << '\n';
        std::cout << "lut_3d_ops_image.pixel_block.height()="
                  << lut_3d_ops_image.pixel_block->height() << '\n';
        std::cout << "lut_3d_ops_image.pixel_block.num_channels()="
                  << lut_3d_ops_image.pixel_block->num_channels() << '\n';

        auto pixel_buffer = lut_3d_ops_image.pixel_block->as_slice_f32();
        std::cout << "lut_3d_ops_image.pixel_block length=" << pixel_buffer.length() << '\n';
        auto max_index = pixel_buffer.length();
        if (max_index >= 32) {
            max_index = 32;
        }
        for (auto i = 0; i < max_index; ++i) {
            std::cout << "num: " << i << "=" << pixel_buffer[i] << "\n";
        }
    }

    num_channels = 1;
    auto lut_1d_ops_image = ocg::get_color_ops_lut(
        stream_data, lut_edge_size, num_channels, cache);
    if (debug_print) {
        std::cout << "ColorOps 1D LUT (Alpha)" << '\n';
        std::cout << "lut_edge_size=" << lut_edge_size << '\n';
        std::cout << "lut_1d_ops_image.spec.color_space="
                  << lut_1d_ops_image.spec.color_space << '\n';
        std::cout << "lut_1d_ops_image.pixel_block.width()="
                  << lut_1d_ops_image.pixel_block->width() << '\n';
        std::cout << "lut_1d_ops_image.pixel_block.height()="
                  << lut_1d_ops_image.pixel_block->height() << '\n';
        std::cout << "lut_1d_ops_image.pixel_block.num_channels()="
                  << lut_1d_ops_image.pixel_block->num_channels() << '\n';

        auto pixel_buffer = lut_1d_ops_image.pixel_block->as_slice_f32();
        std::cout << "lut_1d_ops_image.pixel_block length=" << pixel_buffer.length() << '\n';
        auto max_index = pixel_buffer.length();
        if (max_index >= 32) {
            max_index = 32;
        }
        for (auto i = 0; i < max_index; ++i) {
            std::cout << "num: " << i << "=" << pixel_buffer[i] << "\n";
        }
    }

    // Execute Write nodes
    exec_status = graph.execute(write1_node, frames, cache);
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
