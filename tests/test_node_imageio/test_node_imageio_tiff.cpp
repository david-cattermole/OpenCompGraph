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
 * Read TIFF and write PNG image.
 */

#include <iostream>
#include <opencompgraph.h>
#include "../generate_frame_range.h"

namespace ocg = open_comp_graph;

int test_node_imageio_tiff(const bool debug_print,
                      std::shared_ptr<ocg::Cache> cache) {
    if (debug_print) {
        std::cout << "======================= test_node_imageio_tiff()" << '\n';
    }
    auto bench = ocg::internal::BenchmarkTime();

    auto frames = generate_frame_range(1, 1);
    auto graph = ocg::Graph();

    auto read1_node = graph.create_node(ocg::NodeType::kReadImage, "read1");
    auto read2_node = graph.create_node(ocg::NodeType::kReadImage, "read2");
    auto write1_node = graph.create_node(ocg::NodeType::kWriteImage, "write1");
    auto write2_node = graph.create_node(ocg::NodeType::kWriteImage, "write2");

    graph.set_node_attr_str(
        read1_node, "file_path", "./tests/data/oiio-images/checker.tif");
    graph.set_node_attr_str(
        read2_node, "file_path", "./tests/data/oiio-images/grid.tif");

    graph.set_node_attr_str(
        write1_node, "file_path", "./tests/data/out/test_node_imageio_tiff_out1.png");
    graph.set_node_attr_str(
        write2_node, "file_path", "./tests/data/out/test_node_imageio_tiff_out2.png");

    graph.connect(read1_node, write1_node, 0);
    graph.connect(read2_node, write2_node, 0);
    graph.execute(write1_node, frames, cache);
    graph.execute(write2_node, frames, cache);
    if (debug_print) {
        std::cout << "Graph as string:\n"
                  << graph.data_debug_string();

        bench.stop();
        bench.print("Test Node Image IO:");
    }

    return 0;
}
