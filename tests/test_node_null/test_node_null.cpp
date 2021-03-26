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
 * Nodes connected like so; "Read image -> Null -> Write". Many Null
 * nodes are used to test if the streams are forwarded correctly.
 */

#include <iostream>
#include <opencompgraph.h>
#include "../generate_frame_range.h"

namespace ocg = open_comp_graph;

int test_node_null(const bool debug_print,
                   std::shared_ptr<ocg::Cache> cache) {
    if (debug_print) {
        std::cout << "=============================== test_node_null()" << '\n';
    }
    auto bench = ocg::internal::BenchmarkTime();

    auto frames = generate_frame_range(1, 1);
    auto graph = ocg::Graph();

    auto read_node = graph.create_node(ocg::NodeType::kReadImage, "read");
    auto null_node1 = graph.create_node(ocg::NodeType::kNull, "null1");
    auto null_node2 = graph.create_node(ocg::NodeType::kNull, "null2");
    auto null_node3 = graph.create_node(ocg::NodeType::kNull, "null3");
    auto null_node4 = graph.create_node(ocg::NodeType::kNull, "null4");
    auto write_node = graph.create_node(ocg::NodeType::kWriteImage, "write");
    if (debug_print) {
        std::cout << "read_node=" << &read_node << '\n';
        std::cout << "write_node=" << &write_node << '\n';
    }

    graph.set_node_attr_str(read_node,"file_path", "tests/data/oiio-images/tahoe-gps.jpg");
    graph.set_node_attr_str(write_node,"file_path", "./tests/data/out/test_node_null_out.png");

    graph.connect(read_node, null_node1, 0);
    graph.connect(null_node1, null_node2, 0);
    graph.connect(null_node2, null_node3, 0);
    graph.connect(null_node3, null_node4, 0);
    graph.connect(null_node4, write_node, 0);

    graph.execute(write_node, frames, cache);
    if (debug_print) {
        std::cout << "Graph as string:\n"
                  << graph.data_debug_string();

        bench.stop();
        bench.print("Test Node Null:");
    }

    return 0;
}
