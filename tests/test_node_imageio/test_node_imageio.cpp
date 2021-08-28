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
 * Read EXR and write PNG image.
 */

#include <iostream>
#include <opencompgraph.h>
#include "../generate_frame_range.h"

namespace ocg = open_comp_graph;

int test_node_imageio(const bool debug_print,
                      std::shared_ptr<ocg::Cache> cache) {
    if (debug_print) {
        std::cout << "============================ test_node_imageio()" << '\n';
    }
    auto bench = ocg::internal::BenchmarkTime();

    auto frames = generate_frame_range(1, 1);
    auto graph = ocg::Graph();

    auto read1_node = graph.create_node(ocg::NodeType::kReadImage, "read1");
    auto read2_node = graph.create_node(ocg::NodeType::kReadImage, "read2");
    auto read3_node = graph.create_node(ocg::NodeType::kReadImage, "read3");
    auto read4_node = graph.create_node(ocg::NodeType::kReadImage, "read4");
    // auto read5_node = graph.create_node(ocg::NodeType::kReadImage, "read5");
    // auto read6_node = graph.create_node(ocg::NodeType::kReadImage, "read6");
    auto read7_node = graph.create_node(ocg::NodeType::kReadImage, "read7");
    auto read8_node = graph.create_node(ocg::NodeType::kReadImage, "read8");
    auto read9_node = graph.create_node(ocg::NodeType::kReadImage, "read9");
    // auto read10_node = graph.create_node(ocg::NodeType::kReadImage, "read10");
    auto write1_node = graph.create_node(ocg::NodeType::kWriteImage, "write1");
    auto write2_node = graph.create_node(ocg::NodeType::kWriteImage, "write2");
    auto write3_node = graph.create_node(ocg::NodeType::kWriteImage, "write3");
    auto write4_node = graph.create_node(ocg::NodeType::kWriteImage, "write4");
    // auto write5_node = graph.create_node(ocg::NodeType::kWriteImage, "write5");
    // auto write6_node = graph.create_node(ocg::NodeType::kWriteImage, "write6");
    auto write7_node = graph.create_node(ocg::NodeType::kWriteImage, "write7");
    auto write8_node = graph.create_node(ocg::NodeType::kWriteImage, "write8");
    auto write9_node = graph.create_node(ocg::NodeType::kWriteImage, "write9");
    // auto write10_node = graph.create_node(ocg::NodeType::kWriteImage, "write10");

    graph.set_node_attr_str(
        read1_node, "file_path", "./tests/data/openexr-images/TestImages/AllHalfValues.exr");
    graph.set_node_attr_str(
        read2_node, "file_path", "./tests/data/openexr-images/TestImages/BrightRings.exr");
    graph.set_node_attr_str(
        read3_node, "file_path", "./tests/data/openexr-images/TestImages/BrightRingsNanInf.exr");
    graph.set_node_attr_str(
        read4_node, "file_path", "./tests/data/openexr-images/TestImages/GammaChart.exr.exr");
    // graph.set_node_attr_str(
    //     read5_node, "file_path", "./tests/data/openexr-images/TestImages/GrayRampsDiagonal.exr");
    // graph.set_node_attr_str(
    //     read6_node, "file_path", "./tests/data/openexr-images/TestImages/GrayRampsHorizontal.exr");
    graph.set_node_attr_str(
        read7_node, "file_path", "./tests/data/openexr-images/TestImages/RgbRampsDiagonal.exr");
    graph.set_node_attr_str(
        read8_node, "file_path", "./tests/data/openexr-images/TestImages/SquaresSwirls.exr");
    graph.set_node_attr_str(
        read9_node, "file_path", "./tests/data/openexr-images/TestImages/WideColorGamut.exr");
    // graph.set_node_attr_str(
    //     read10_node, "file_path", "./tests/data/openexr-images/TestImages/WideFloatRange.exr");

    graph.set_node_attr_str(
        write1_node, "file_path", "./tests/data/out/test_node_imageio_exr_out1.png");
    graph.set_node_attr_str(
        write2_node, "file_path", "./tests/data/out/test_node_imageio_exr_out2.png");
    graph.set_node_attr_str(
        write3_node, "file_path", "./tests/data/out/test_node_imageio_exr_out3.png");
    graph.set_node_attr_str(
        write4_node, "file_path", "./tests/data/out/test_node_imageio_exr_out4.png");
    // graph.set_node_attr_str(
    //     write5_node, "file_path", "./tests/data/out/test_node_imageio_exr_out5.png");
    // graph.set_node_attr_str(
    //     write6_node, "file_path", "./tests/data/out/test_node_imageio_exr_out6.png");
    graph.set_node_attr_str(
        write7_node, "file_path", "./tests/data/out/test_node_imageio_exr_out7.png");
    graph.set_node_attr_str(
        write8_node, "file_path", "./tests/data/out/test_node_imageio_exr_out8.png");
    graph.set_node_attr_str(
        write9_node, "file_path", "./tests/data/out/test_node_imageio_exr_out9.png");
    // graph.set_node_attr_str(
    //     write10_node, "file_path", "./tests/data/out/test_node_imageio_exr_out10.png");

    graph.connect(read1_node, write1_node, 0);
    graph.connect(read2_node, write2_node, 0);
    graph.connect(read3_node, write3_node, 0);
    graph.connect(read4_node, write4_node, 0);
    // graph.connect(read5_node, write5_node, 0);
    // graph.connect(read6_node, write6_node, 0);
    graph.connect(read7_node, write7_node, 0);
    graph.connect(read8_node, write8_node, 0);
    graph.connect(read9_node, write9_node, 0);
    // graph.connect(read10_node, write10_node, 0);

    graph.execute(write1_node, frames, cache);
    graph.execute(write2_node, frames, cache);
    graph.execute(write3_node, frames, cache);
    graph.execute(write4_node, frames, cache);
    // graph.execute(write5_node, frames, cache);
    // graph.execute(write6_node, frames, cache);
    graph.execute(write7_node, frames, cache);
    graph.execute(write8_node, frames, cache);
    graph.execute(write9_node, frames, cache);
    // graph.execute(write10_node, frames, cache);

    if (debug_print) {
        std::cout << "Graph as string:\n"
                  << graph.data_debug_string();

        bench.stop();
        bench.print("Test Node Image IO:");
    }

    return 0;
}
