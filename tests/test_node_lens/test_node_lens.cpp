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
 * Basic "read image -> lens distortion" connection and execution.
 * Geometry files are output for testing the lens distortion output.
 * Write node is connected, but not evaluated.
 */

#include <iostream>
#include <opencompgraph.h>
#include "../generate_mesh.h"
#include "../generate_frame_range.h"

namespace ocg = open_comp_graph;


int test_node_lens(const bool debug_print,
                   std::shared_ptr<ocg::Cache> cache) {
    if (debug_print) {
        std::cout << "=============================== test_node_lens()" << '\n';
    }
    auto bench = ocg::internal::BenchmarkTime();

    auto frames = generate_frame_range(1, 1);
    auto graph = ocg::Graph();

    auto read_type = ocg::NodeType::kReadImage;
    auto lens_type = ocg::NodeType::kLensDistort;
    auto write_type = ocg::NodeType::kWriteImage;

    auto read_node = graph.create_node(read_type, "read_node");
    auto lens_distort_node = graph.create_node(lens_type, "distort");
    auto lens_undistort_node = graph.create_node(lens_type, "undistort");
    auto lens_redistort_node = graph.create_node(lens_type, "redistort");
    auto write_distort_png_node = graph.create_node(write_type, "write_distort_png");
    auto write_undistort_png_node = graph.create_node(write_type, "write_undistort_png");
    auto write_redistort_png_node = graph.create_node(write_type, "write_redistort_png");
    auto write_distort_exr_node = graph.create_node(write_type, "write_distort_exr");
    auto write_undistort_exr_node = graph.create_node(write_type, "write_undistort_exr");
    auto write_redistort_exr_node = graph.create_node(write_type, "write_redistort_exr");

    // Distortion test values.
    auto undistort_dir = static_cast<int32_t>(ocg::LensDistortDirection::kUndistort);
    auto distort_dir = static_cast<int32_t>(ocg::LensDistortDirection::kDistort);
    auto k1 = 0.2f;     // Distortion (degree 2)
    auto k2 = 0.05f;    // Quartic Distortion (degree 4)
    auto lco_x = 0.0f;  // Lens Center Offset X
    auto lco_y = 0.0f;  // Lens Center Offset Y

    graph.set_node_attr_str(
        read_node, "file_path",
        "./tests/data/ocg-testdata/images/checker/checker_8bit_rgba_3840x2160.png");

    graph.set_node_attr_i32(lens_distort_node, "direction", distort_dir);
    graph.set_node_attr_f32(lens_distort_node, "distortion", k1);
    graph.set_node_attr_f32(lens_distort_node, "quartic_distortion", k2);
    graph.set_node_attr_f32(lens_distort_node, "lens_center_offset_x", lco_x);
    graph.set_node_attr_f32(lens_distort_node, "lens_center_offset_y", lco_y);

    graph.set_node_attr_i32(lens_undistort_node, "direction", undistort_dir);
    graph.set_node_attr_f32(lens_undistort_node, "distortion", k1);
    graph.set_node_attr_f32(lens_undistort_node, "quartic_distortion", k2);
    graph.set_node_attr_f32(lens_undistort_node, "lens_center_offset_x", lco_x);
    graph.set_node_attr_f32(lens_undistort_node, "lens_center_offset_y", lco_y);

    graph.set_node_attr_i32(lens_redistort_node, "direction", distort_dir);
    graph.set_node_attr_f32(lens_redistort_node, "distortion", k1);
    graph.set_node_attr_f32(lens_redistort_node, "quartic_distortion", k2);
    graph.set_node_attr_f32(lens_redistort_node, "lens_center_offset_x", lco_x);
    graph.set_node_attr_f32(lens_redistort_node, "lens_center_offset_y", lco_y);

    graph.set_node_attr_str(
        write_distort_png_node, "file_path",
        "./tests/data/out/test_node_lens_out_distort.png");
    graph.set_node_attr_str(
        write_undistort_png_node, "file_path",
        "./tests/data/out/test_node_lens_out_undistort.png");
    graph.set_node_attr_str(
        write_redistort_png_node, "file_path",
        "./tests/data/out/test_node_lens_out_redistort.png");
    graph.set_node_attr_str(
        write_distort_exr_node, "file_path",
        "./tests/data/out/test_node_lens_out_distort.exr");
    graph.set_node_attr_str(
        write_undistort_exr_node, "file_path",
        "./tests/data/out/test_node_lens_out_undistort.exr");
    graph.set_node_attr_str(
        write_redistort_exr_node, "file_path",
        "./tests/data/out/test_node_lens_out_redistort.exr");

    graph.connect(read_node, lens_distort_node, 0);
    graph.connect(read_node, lens_undistort_node, 0);
    graph.connect(lens_distort_node, write_distort_png_node, 0);
    graph.connect(lens_distort_node, write_distort_exr_node, 0);
    graph.connect(lens_undistort_node, write_undistort_png_node, 0);
    graph.connect(lens_undistort_node, write_undistort_exr_node, 0);
    graph.connect(lens_undistort_node, lens_redistort_node, 0);
    graph.connect(lens_redistort_node, write_redistort_png_node, 0);
    graph.connect(lens_redistort_node, write_redistort_exr_node, 0);

    auto status = graph.execute(lens_undistort_node, frames, cache);
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
    const char* file_path_2x2 = "./tests/data/out/test_node_lens_2x2_out.obj";
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
    const char* file_path_3x3 = "./tests/data/out/test_node_lens_3x3_out.obj";
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
    const char* file_path_4x4 = "./tests/data/out/test_node_lens_4x4_out.obj";
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
    const char* file_path_2x4 = "./tests/data/out/test_node_lens_2x4_out.obj";
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
    const char* file_path_16x32 = "./tests/data/out/test_node_lens_16x32_out.obj";
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
    const char* file_path_32x16 = "./tests/data/out/test_node_lens_32x16_out.obj";
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
    const char* file_path_192x108 = "./tests/data/out/test_node_lens_192x108_out.obj";
    generate_mesh(
        debug_print,
        divisions_x,
        divisions_y,
        stream_data,
        file_path_192x108,
        pos_count,
        uv_count,
        tri_count);

    status = graph.execute(write_distort_png_node, frames, cache);
    if (status != ocg::ExecuteStatus::kSuccess) {
        std::cout << "ERROR=" << static_cast<uint32_t>(status) << '\n';
        return 1;
    }

    status = graph.execute(write_undistort_png_node, frames, cache);
    if (status != ocg::ExecuteStatus::kSuccess) {
        std::cout << "ERROR=" << static_cast<uint32_t>(status) << '\n';
        return 1;
    }

    status = graph.execute(write_redistort_png_node, frames, cache);
    if (status != ocg::ExecuteStatus::kSuccess) {
        std::cout << "ERROR=" << static_cast<uint32_t>(status) << '\n';
        return 1;
    }

    status = graph.execute(write_distort_exr_node, frames, cache);
    if (status != ocg::ExecuteStatus::kSuccess) {
        std::cout << "ERROR=" << static_cast<uint32_t>(status) << '\n';
        return 1;
    }

    status = graph.execute(write_undistort_exr_node, frames, cache);
    if (status != ocg::ExecuteStatus::kSuccess) {
        std::cout << "ERROR=" << static_cast<uint32_t>(status) << '\n';
        return 1;
    }

    status = graph.execute(write_redistort_exr_node, frames, cache);
    if (status != ocg::ExecuteStatus::kSuccess) {
        std::cout << "ERROR=" << static_cast<uint32_t>(status) << '\n';
        return 1;
    }

    if (debug_print) {
        std::cout << "Graph as string:\n"
                  << graph.data_debug_string();

        bench.stop();
        bench.print("Test Node Lens:");
    }

    return 0;
}
