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
 * Read and then up/down-res (resample) an image.
 */

#include <iostream>
#include <opencompgraph.h>
#include "../generate_frame_range.h"

namespace ocg = open_comp_graph;

int test_node_resample(const bool debug_print,
                    std::shared_ptr<ocg::Cache> cache) {
    if (debug_print) {
        std::cout << "============================== test_node_resample()" << '\n';
    }
    auto bench = ocg::internal::BenchmarkTime();

    auto frames = generate_frame_range(1, 1);
    auto graph = ocg::Graph();

    auto read_type = ocg::NodeType::kReadImage;
    auto resample_type = ocg::NodeType::kResampleImage;
    auto write_type = ocg::NodeType::kWriteImage;

    auto read_node = graph.create_node(read_type, "read");
    auto resample_node = graph.create_node(resample_type, "resample");
    auto write_node = graph.create_node(write_type, "write");

    graph.connect(read_node, resample_node, 0);
    graph.connect(resample_node, write_node, 0);

    graph.set_node_attr_str(
        read_node, "file_path",
        "tests/data/openexr-images/ScanLines/Desk.exr");

    // No change
    graph.set_node_attr_i32(resample_node, "interpolate", 0);
    graph.set_node_attr_i32(resample_node, "factor", 0);
    graph.set_node_attr_str(
        write_node, "file_path",
        "./tests/data/out/test_node_resample_out1_noChange.png");
    auto exec_status = graph.execute(write_node, frames, cache);

    // Down-res 1
    graph.set_node_attr_i32(resample_node, "interpolate", 0);
    graph.set_node_attr_i32(resample_node, "factor", -1);
    graph.set_node_attr_str(
        write_node, "file_path",
        "./tests/data/out/test_node_resample_out2_downRes1_interpOff.png");
    exec_status = graph.execute(write_node, frames, cache);

    // Down-res 2
    graph.set_node_attr_i32(resample_node, "interpolate", 1);
    graph.set_node_attr_i32(resample_node, "factor", -2);
    graph.set_node_attr_str(
        write_node, "file_path",
        "./tests/data/out/test_node_resample_out3_downRes2_interpOn.png");
    exec_status = graph.execute(write_node, frames, cache);

    // Up-res 1
    graph.set_node_attr_i32(resample_node, "interpolate", 1);
    graph.set_node_attr_i32(resample_node, "factor", 1);
    graph.set_node_attr_str(
        write_node, "file_path",
        "./tests/data/out/test_node_resample_out4_upRes1_interpOn.png");
    exec_status = graph.execute(write_node, frames, cache);

    // Up-res 2
    graph.set_node_attr_i32(resample_node, "interpolate", 0);
    graph.set_node_attr_i32(resample_node, "factor", 2);
    graph.set_node_attr_str(
        write_node, "file_path",
        "./tests/data/out/test_node_resample_out5_upRes2_interpOff.png");
    exec_status = graph.execute(write_node, frames, cache);

    // Down-res 4 (single pixel)
    graph.set_node_attr_i32(resample_node, "interpolate", 1);
    graph.set_node_attr_i32(resample_node, "factor", -12);
    graph.set_node_attr_str(
        write_node, "file_path",
        "./tests/data/out/test_node_resample_out6_downRes12_interpOn.png");
    exec_status = graph.execute(write_node, frames, cache);

    // Up-res 14 (maximum extent of image size)
    graph.set_node_attr_i32(resample_node, "interpolate", 0);
    graph.set_node_attr_i32(resample_node, "factor", 14);
    graph.set_node_attr_str(
        write_node, "file_path",
        "./tests/data/out/test_node_resample_out7_upRes14_interpOff.png");
    exec_status = graph.execute(write_node, frames, cache);

    if (debug_print) {
        bench.stop();
        bench.print("Test Node Resample:");
    }

    return 0;
}
