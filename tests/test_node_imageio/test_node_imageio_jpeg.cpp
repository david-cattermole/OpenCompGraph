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
 * Read JPEG and write JPEG image.
 */

#include <iostream>
#include <opencompgraph.h>
#include "../generate_frame_range.h"

namespace ocg = open_comp_graph;

int test_node_imageio_jpeg(const bool debug_print,
                      std::shared_ptr<ocg::Cache> cache) {
    if (debug_print) {
        std::cout << "======================= test_node_imageio_jpeg()" << '\n';
    }
    auto bench = ocg::internal::BenchmarkTime();

    auto frames = generate_frame_range(1, 1);
    auto graph = ocg::Graph();

    auto read1_node = graph.create_node(ocg::NodeType::kReadImage, "read1");
    auto read2_node = graph.create_node(ocg::NodeType::kReadImage, "read2");
    auto read3_node = graph.create_node(ocg::NodeType::kReadImage, "read3");
    auto read4_node = graph.create_node(ocg::NodeType::kReadImage, "read4");
    auto read5_node = graph.create_node(ocg::NodeType::kReadImage, "read5");
    auto write1_node = graph.create_node(ocg::NodeType::kWriteImage, "write1");
    auto write2_node = graph.create_node(ocg::NodeType::kWriteImage, "write2");
    auto write3_node = graph.create_node(ocg::NodeType::kWriteImage, "write3");
    auto write4_node = graph.create_node(ocg::NodeType::kWriteImage, "write4");
    auto write5_node = graph.create_node(ocg::NodeType::kWriteImage, "write5");

    graph.set_node_attr_str(
        read1_node, "file_path", "./tests/data/oiio-images/tahoe-gps.jpg");
    graph.set_node_attr_str(
        read2_node, "file_path", "./tests/data/ocg-testdata/images/vancouver_stills/full_res_jpg/DSC05947.jpg");
    graph.set_node_attr_str(
        read3_node, "file_path", "./tests/data/ocg-testdata/images/vancouver_stills/full_res_jpg/DSC05954.jpg");
    graph.set_node_attr_str(
        read4_node, "file_path", "./tests/data/ocg-testdata/images/vancouver_stills/full_res_jpg/DSC05974.jpg");
    graph.set_node_attr_str(
        read5_node, "file_path", "./tests/data/ocg-testdata/images/color_bars/3840x2160_jpg/color_bars.0951.jpg");

    graph.set_node_attr_str(
        write1_node, "file_path", "./tests/data/out/test_node_imageio_jpeg_out1.jpg");

    graph.set_node_attr_str(
        write2_node, "file_path", "./tests/data/out/test_node_imageio_jpeg_out2_comp_level_90.jpg");
    graph.set_node_attr_i32(
        write2_node, "jpeg_compression_level", 90);

    graph.set_node_attr_str(
        write3_node, "file_path", "./tests/data/out/test_node_imageio_jpeg_out3_comp_level_10.jpg");
    graph.set_node_attr_i32(
        write3_node, "jpeg_compression_level", 10);

    graph.set_node_attr_str(
        write4_node, "file_path", "./tests/data/out/test_node_imageio_jpeg_out4_subsample444.jpg");
    graph.set_node_attr_i32(
        write4_node, "jpeg_subsampling",
        static_cast<int32_t>(ocg::JpegChromaSubSampling::kNone444));

    graph.set_node_attr_str(
        write5_node, "file_path", "./tests/data/out/test_node_imageio_jpeg_out5_progressive.jpg");
    graph.set_node_attr_i32(
        write5_node, "jpeg_progressive", 1);

    graph.connect(read1_node, write1_node, 0);
    graph.connect(read2_node, write2_node, 0);
    graph.connect(read3_node, write3_node, 0);
    graph.connect(read4_node, write4_node, 0);
    graph.connect(read5_node, write5_node, 0);
    graph.execute(write1_node, frames, cache);
    graph.execute(write2_node, frames, cache);
    graph.execute(write3_node, frames, cache);
    graph.execute(write4_node, frames, cache);
    graph.execute(write5_node, frames, cache);
    if (debug_print) {
        std::cout << "Graph as string:\n"
                  << graph.data_debug_string();

        bench.stop();
        bench.print("Test Node Image IO:");
    }

    return 0;
}
