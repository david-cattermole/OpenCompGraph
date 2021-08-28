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

int test_node_viewer_disk_cache(const bool debug_print,
                                std::shared_ptr<ocg::Cache> cache) {
    if (debug_print) {
        std::cout << "================== test_node_viewer_disk_cache()" << '\n';
    }
    auto bench = ocg::internal::BenchmarkTime();

    auto frames = generate_frame_range(1001, 1001);
    auto graph = ocg::Graph();

    auto read_type = ocg::NodeType::kReadImage;
    auto lens_type = ocg::NodeType::kLensDistort;
    auto grade_type = ocg::NodeType::kGrade;
    auto viewer_type = ocg::NodeType::kViewer;
    auto write_type = ocg::NodeType::kWriteImage;

    auto read_node = graph.create_node(read_type, "read");
    auto lens_node = graph.create_node(lens_type, "undistort");
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

    graph.set_node_attr_f32(grade_node, "multiply_g", 1.2f);
    graph.set_node_attr_f32(grade_node, "gamma_r", 1.2f);
    graph.set_node_attr_f32(grade_node, "gamma_b", 0.8f);

    // Lens Distortion test values.
    auto undistort_dir = static_cast<int32_t>(ocg::LensDistortDirection::kUndistort);
    auto k1 = 0.2f;     // Distortion (degree 2)
    auto k2 = 0.05f;    // Quartic Distortion (degree 4)
    auto lco_x = 0.0f;  // Lens Center Offset X
    auto lco_y = 0.0f;  // Lens Center Offset Y
    graph.set_node_attr_i32(lens_node, "direction", undistort_dir);
    graph.set_node_attr_f32(lens_node, "distortion", k1);
    graph.set_node_attr_f32(lens_node, "quartic_distortion", k2);
    graph.set_node_attr_f32(lens_node, "lens_center_offset_x", lco_x);
    graph.set_node_attr_f32(lens_node, "lens_center_offset_y", lco_y);

    // Viewer Nodes
    auto bake_option = static_cast<int32_t>(ocg::BakeOption::kColorSpace);
    auto image_exr_lossy = static_cast<int32_t>(ocg::DiskCacheImageType::kExrLossyHalf16);
    graph.set_node_attr_i32(viewer1_node, "bake_option", bake_option);
    graph.set_node_attr_i32(viewer1_node, "crop_to_format", 0);
    graph.set_node_attr_i32(viewer1_node, "disk_cache", 1);
    graph.set_node_attr_i32(viewer1_node, "disk_cache_image_type", image_exr_lossy);
    graph.set_node_attr_str(
        viewer1_node, "disk_cache_dir", "./tests/data/out/disk_cache/");

    // Write Nodes
    graph.set_node_attr_str(
        write1_node, "file_path",
        "./tests/data/out/test_node_viewer_disk_cache_out1.####.jpg");

    // Connect nodes together
    graph.connect(read_node, grade_node, 0);
    graph.connect(grade_node, lens_node, 0);
    graph.connect(lens_node, viewer1_node, 0);
    graph.connect(viewer1_node, write1_node, 0);

    // Execute Viewer node
    auto exec_status = graph.execute(viewer1_node, frames, cache);
    if (debug_print) {
        std::cout << "execute status: "
                  << static_cast<uint32_t>(exec_status) << '\n';
        std::cout << "Graph as string:\n"
                  << graph.data_debug_string();
    }

    // Change disk cache image format.
    auto image_jpg_lossy = static_cast<int32_t>(ocg::DiskCacheImageType::kJpegLossyUInt8);
    graph.set_node_attr_i32(viewer1_node, "disk_cache_image_type", image_jpg_lossy);

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
