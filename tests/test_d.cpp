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
#include "generate_frame_range.h"

namespace ocg = open_comp_graph;

int test_d(const bool debug_print, std::shared_ptr<ocg::Cache> cache) {
    if (debug_print) {
        std::cout << "=========================================== test_d()" << '\n';
    }
    auto bench = ocg::internal::BenchmarkTime();

    auto frames = generate_frame_range(1, 1);
    auto graph = ocg::Graph();

    auto read_node = graph.create_node(ocg::NodeType::kReadImage, "my_read_node");
    auto write_node = graph.create_node(ocg::NodeType::kWriteImage, "my_write_node");
    if (debug_print) {
        std::cout << "read_node=" << &read_node << '\n';
        std::cout << "write_node=" << &write_node << '\n';
    }
    auto read_node_id1 = read_node.get_id();
    auto write_node_id1 = write_node.get_id();
    graph.set_node_attr_str(read_node, "file_path", "./tests/data/oiio-images/checker.tif");
    graph.set_node_attr_str(write_node, "file_path", "./tests/data/out/test_d_out1.png");

    graph.connect(read_node, write_node, 0);
    graph.execute(write_node, frames, cache);
    if (debug_print) {
        std::cout << "Graph as string:\n"
                  << graph.data_debug_string();

        bench.stop();
        bench.print("Test D:");
    }

    return 0;
}
