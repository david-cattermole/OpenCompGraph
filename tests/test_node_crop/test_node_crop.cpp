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
 */

#include <iostream>
#include <opencompgraph.h>
#include "../generate_frame_range.h"

namespace ocg = open_comp_graph;

int test_node_crop(const bool debug_print,
                    std::shared_ptr<ocg::Cache> cache) {
    if (debug_print) {
        std::cout << "============================== test_node_crop()" << '\n';
    }
    auto bench = ocg::internal::BenchmarkTime();

    auto frames_a = generate_frame_range(1, 1);
    auto frames_b = generate_frame_range(1, 10);
    auto graph = ocg::Graph();

    auto read_image_type = ocg::NodeType::kReadImage;
    auto crop_image_type = ocg::NodeType::kCropImage;
    auto write_image_type = ocg::NodeType::kWriteImage;

    auto read1_node = graph.create_node(read_image_type, "read1");
    auto read2_node = graph.create_node(read_image_type, "read2");
    auto read3_node = graph.create_node(read_image_type, "read3");

    auto crop1_node = graph.create_node(crop_image_type, "crop1");
    auto crop2_node = graph.create_node(crop_image_type, "crop2");
    auto crop3_node = graph.create_node(crop_image_type, "crop3");

    auto write1_node = graph.create_node(write_image_type, "write1");
    auto write2_node = graph.create_node(write_image_type, "write2");
    auto write3_node = graph.create_node(write_image_type, "write3");
    auto write4_node = graph.create_node(write_image_type, "write4");

    // Read Image Attributes
    graph.set_node_attr_str(read1_node, "file_path",
                            "tests/data/oiio-images/tahoe-gps.jpg");
    graph.set_node_attr_str(read2_node, "file_path",
                            "tests/data/oiio-images/grid-overscan.exr");
    graph.set_node_attr_str(read3_node, "file_path",
                            "tests/data/openexr-images/Beachball/multipart.####.exr");

    // Crop Image Attributes
    graph.set_node_attr_i32(crop1_node, "window_min_x", 800);
    graph.set_node_attr_i32(crop1_node, "window_min_y", 900);
    graph.set_node_attr_i32(crop1_node, "window_max_x", 1200);
    graph.set_node_attr_i32(crop1_node, "window_max_y", 1300);
    graph.set_node_attr_i32(crop1_node, "reformat", 0);
    graph.set_node_attr_i32(crop1_node, "black_outside", 0);
    graph.set_node_attr_i32(crop1_node, "intersect", 0);

    graph.set_node_attr_i32(crop2_node, "window_min_x", 0);
    graph.set_node_attr_i32(crop2_node, "window_min_y", 0);
    graph.set_node_attr_i32(crop2_node, "window_max_x", 200);
    graph.set_node_attr_i32(crop2_node, "window_max_y", 100);
    graph.set_node_attr_i32(crop2_node, "reformat", 1);
    graph.set_node_attr_i32(crop2_node, "black_outside", 0);
    graph.set_node_attr_i32(crop2_node, "intersect", 0);

    graph.set_node_attr_i32(crop3_node, "window_min_x", 600);
    graph.set_node_attr_i32(crop3_node, "window_min_y", 600);
    graph.set_node_attr_i32(crop3_node, "window_max_x", 800);
    graph.set_node_attr_i32(crop3_node, "window_max_y", 800);
    graph.set_node_attr_i32(crop3_node, "reformat", 1);
    graph.set_node_attr_i32(crop3_node, "black_outside", 0);
    graph.set_node_attr_i32(crop3_node, "intersect", 0);

    // Write Image Attributes
    graph.set_node_attr_str(write1_node, "file_path",
                            "./tests/data/out/test_node_crop_out1.####.png");
    graph.set_node_attr_str(write2_node, "file_path",
                            "./tests/data/out/test_node_crop_out2.####.png");
    graph.set_node_attr_str(write3_node, "file_path",
                            "./tests/data/out/test_node_crop_out3.####.png");
    graph.set_node_attr_str(write4_node, "file_path",
                            "./tests/data/out/test_node_crop_out4.####.png");
    graph.set_node_attr_i32(write4_node, "crop_on_write", 1);

    graph.connect(read1_node, crop1_node, 0);
    graph.connect(read2_node, crop2_node, 0);
    graph.connect(read3_node, crop3_node, 0);
    graph.connect(crop1_node, write1_node, 0);
    graph.connect(crop2_node, write2_node, 0);
    graph.connect(crop3_node, write3_node, 0);
    graph.connect(read3_node, write4_node, 0);

    // Execute
    if (debug_print) {
        std::cout << "Graph as string (before):\n"
                  << graph.data_debug_string();
    }
    graph.execute(write1_node, frames_a, cache);
    graph.execute(write2_node, frames_a, cache);
    graph.execute(write3_node, frames_b, cache);
    graph.execute(write4_node, frames_b, cache);

    if (debug_print) {
        std::cout << "Graph as string (after):\n"
                  << graph.data_debug_string();

        bench.stop();
        bench.print("Test Node Crop:");
    }

    return 0;
}
