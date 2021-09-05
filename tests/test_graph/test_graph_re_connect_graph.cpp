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
 * Re-execute the same graph with a different set of node connections.
 * This ensures the graph is correct even when new connections are
 * made.
 *
 */

#include <iostream>
#include <random>
#include <sstream>  // stringstream
#include <cmath>    // fabs
#include <opencompgraph.h>
#include "../generate_frame_range.h"

namespace ocg = open_comp_graph;

template<typename NUM_TYPE>
inline
std::string number_to_string(NUM_TYPE num) {
    std::stringstream ss;
    ss << num;
    return ss.str();
}

int test_graph_re_connect_graph(const bool debug_print,
                                      std::shared_ptr<ocg::Cache> cache) {
    if (debug_print) {
        std::cout << "============ test_graph_re_connect_graph()" << '\n';
    }
    auto bench = ocg::internal::BenchmarkTime();

    auto frames = generate_frame_range(1, 1);
    auto graph = ocg::Graph();

    auto read_node = graph.create_node(ocg::NodeType::kReadImage, "read");
    auto grade_node = graph.create_node(ocg::NodeType::kGrade, "grade");
    auto write_node = graph.create_node(ocg::NodeType::kWriteImage, "write");

    graph.set_node_attr_str(
            read_node, "file_path",
            "tests/data/openexr-images/TestImages/SquaresSwirls.exr");

    // Connect nodes together and execute
    graph.connect(read_node, grade_node, 0);
    graph.connect(grade_node, write_node, 0);

    graph.set_node_attr_str(
        write_node,
        "file_path",
        "./tests/data/out/test_graph_re_connect_graph_out1.png");

    auto exec_status = graph.execute(write_node, frames, cache);
    if (debug_print) {
        std::cout << "execute status: "
            << static_cast<uint32_t>(exec_status) << '\n';
        std::cout << "Graph as string:\n"
                  << graph.data_debug_string();
    }

    graph.set_node_attr_str(
        write_node,
        "file_path",
        "./tests/data/out/test_graph_re_connect_graph_out2.png");

    // Modify the structure of the graph, this could cause a crash if
    // the graph does not evaluate correctly.
    graph.connect(read_node, write_node, 0);

    exec_status = graph.execute(write_node, frames, cache);
    if (debug_print) {
        std::cout << "execute status: "
            << static_cast<uint32_t>(exec_status) << '\n';
        std::cout << "Graph as string:\n"
                  << graph.data_debug_string();
    }

    if (debug_print) {
        bench.stop();
        bench.print("Test Graph Re-Eval Modified Graph:");
    }

    return 0;
}
