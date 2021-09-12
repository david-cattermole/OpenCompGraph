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
 * Read PNG and write PNG image.
 */

#include <iostream>
#include <opencompgraph.h>
#include "../generate_frame_range.h"

namespace ocg = open_comp_graph;

int test_node_imageio_png(const bool debug_print,
                      std::shared_ptr<ocg::Cache> cache) {
    if (debug_print) {
        std::cout << "======================= test_node_imageio_png()" << '\n';
    }
    auto bench = ocg::internal::BenchmarkTime();

    auto frames = generate_frame_range(1, 1);
    auto graph = ocg::Graph();

    auto read1_node = graph.create_node(ocg::NodeType::kReadImage, "read1");
    auto read2_node = graph.create_node(ocg::NodeType::kReadImage, "read2");
    auto read3_node = graph.create_node(ocg::NodeType::kReadImage, "read3");
    auto read4_node = graph.create_node(ocg::NodeType::kReadImage, "read4");
    auto read5_node = graph.create_node(ocg::NodeType::kReadImage, "read5");
    auto read6_node = graph.create_node(ocg::NodeType::kReadImage, "read6");
    auto read7_node = graph.create_node(ocg::NodeType::kReadImage, "read7");
    auto read8_node = graph.create_node(ocg::NodeType::kReadImage, "read8");
    auto read9_node = graph.create_node(ocg::NodeType::kReadImage, "read9");
    auto read10_node = graph.create_node(ocg::NodeType::kReadImage, "read10");
    auto read11_node = graph.create_node(ocg::NodeType::kReadImage, "read11");

    auto write1_node = graph.create_node(ocg::NodeType::kWriteImage, "write1");
    auto write2_node = graph.create_node(ocg::NodeType::kWriteImage, "write2");
    auto write3_node = graph.create_node(ocg::NodeType::kWriteImage, "write3");
    auto write4_node = graph.create_node(ocg::NodeType::kWriteImage, "write4");
    auto write5_node = graph.create_node(ocg::NodeType::kWriteImage, "write5");
    auto write6_node = graph.create_node(ocg::NodeType::kWriteImage, "write6");
    auto write7_node = graph.create_node(ocg::NodeType::kWriteImage, "write7");
    auto write8_node = graph.create_node(ocg::NodeType::kWriteImage, "write8");
    auto write9_node = graph.create_node(ocg::NodeType::kWriteImage, "write9");
    auto write10_node = graph.create_node(ocg::NodeType::kWriteImage, "write10");
    auto write11_node = graph.create_node(ocg::NodeType::kWriteImage, "write11");

    graph.set_node_attr_str(
        read1_node, "file_path",
        "./tests/data/ocg-testdata/images/checker/checker_8bit_rgba_3840x2160.png");
    graph.set_node_attr_str(
        read2_node, "file_path",
        "./tests/data/ocg-testdata/images/color_bars/3840x2160_png/color_bars.0951.png");
    graph.set_node_attr_str(
        read3_node, "file_path",
        "./tests/data/ocg-testdata/images/color_bars/3840x2160_png/color_bars.0952.png");
    graph.set_node_attr_str(
        read4_node, "file_path",
        "./tests/data/ocg-testdata/images/color_bars/3840x2160_png/color_bars.0953.png");
    graph.set_node_attr_str(
        read5_node, "file_path",
        "./tests/data/ocg-testdata/images/color_bars/3840x2160_png/color_bars.0954.png");
    graph.set_node_attr_str(
        read6_node, "file_path",
        "./tests/data/ocg-testdata/images/color_bars/3840x2160_png/color_bars.0955.png");
    graph.set_node_attr_str(
        read7_node, "file_path",
        "./tests/data/ocg-testdata/images/color_bars/3840x2160_png/color_bars.0956.png");
    graph.set_node_attr_str(
        read8_node, "file_path",
        "./tests/data/ocg-testdata/images/color_bars/3840x2160_png/color_bars.0957.png");
    graph.set_node_attr_str(
        read9_node, "file_path",
        "./tests/data/ocg-testdata/images/color_bars/3840x2160_png/color_bars.0958.png");
    graph.set_node_attr_str(
        read10_node, "file_path",
        "./tests/data/ocg-testdata/images/color_bars/3840x2160_png/color_bars.0959.png");

    graph.set_node_attr_str(
        write1_node, "file_path", "./tests/data/out/test_node_imageio_png_out1.png");

    graph.set_node_attr_str(
        write2_node, "file_path", "./tests/data/out/test_node_imageio_png_out2_compress_level_0.png");
    graph.set_node_attr_i32(
        write2_node, "png_compression_level", 0);

    graph.set_node_attr_str(
        write3_node, "file_path", "./tests/data/out/test_node_imageio_png_out3_compress_level_1.png");
    graph.set_node_attr_i32(
        write3_node, "png_compression_level", 1);

    graph.set_node_attr_str(
        write4_node, "file_path", "./tests/data/out/test_node_imageio_png_out4_compress_level_2.png");
    graph.set_node_attr_i32(
        write4_node, "png_compression_level", 2);

    graph.set_node_attr_str(
        write5_node, "file_path", "./tests/data/out/test_node_imageio_png_out5_compress_level_3.png");
    graph.set_node_attr_i32(
        write5_node, "png_compression_level", 3);

    graph.set_node_attr_str(
        write6_node, "file_path", "./tests/data/out/test_node_imageio_png_out6_compress_level_4.png");
    graph.set_node_attr_i32(
        write6_node, "png_compression_level", 4);

    graph.set_node_attr_str(
        write7_node, "file_path", "./tests/data/out/test_node_imageio_png_out7_compress_level_5.png");
    graph.set_node_attr_i32(
        write7_node, "png_compression_level", 5);

    graph.set_node_attr_str(
        write8_node, "file_path", "./tests/data/out/test_node_imageio_png_out8_compress_level_6.png");
    graph.set_node_attr_i32(
        write8_node, "png_compression_level", 6);

    graph.set_node_attr_str(
        write9_node, "file_path", "./tests/data/out/test_node_imageio_png_out9_compress_level_7.png");
    graph.set_node_attr_i32(
        write9_node, "png_compression_level", 7);

    graph.set_node_attr_str(
        write10_node, "file_path", "./tests/data/out/test_node_imageio_png_out10_compress_level_8.png");
    graph.set_node_attr_i32(
        write10_node, "png_compression_level", 8);

    graph.set_node_attr_str(
        write11_node, "file_path", "./tests/data/out/test_node_imageio_png_out11_compress_level_9.png");
    graph.set_node_attr_i32(
        write11_node, "png_compression_level", 9);

    graph.connect(read1_node, write1_node, 0);
    graph.connect(read2_node, write2_node, 0);
    graph.connect(read3_node, write3_node, 0);
    graph.connect(read4_node, write4_node, 0);
    graph.connect(read5_node, write5_node, 0);
    graph.connect(read6_node, write6_node, 0);
    graph.connect(read7_node, write7_node, 0);
    graph.connect(read8_node, write8_node, 0);
    graph.connect(read9_node, write9_node, 0);
    graph.connect(read10_node, write10_node, 0);
    graph.connect(read11_node, write11_node, 0);

    graph.execute(write1_node, frames, cache);
    graph.execute(write2_node, frames, cache);
    graph.execute(write3_node, frames, cache);
    graph.execute(write4_node, frames, cache);
    graph.execute(write5_node, frames, cache);
    graph.execute(write6_node, frames, cache);
    graph.execute(write7_node, frames, cache);
    graph.execute(write8_node, frames, cache);
    graph.execute(write9_node, frames, cache);
    graph.execute(write10_node, frames, cache);
    graph.execute(write11_node, frames, cache);

    if (debug_print) {
        std::cout << "Graph as string:\n"
                  << graph.data_debug_string();

        bench.stop();
        bench.print("Test Node Image IO:");
    }

    return 0;
}
