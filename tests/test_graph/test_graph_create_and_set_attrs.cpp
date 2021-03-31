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
 * Create some nodes and set attributes. No connections are made.
 */

#include <cstdint>
#include <iostream>
#include <string>
#include <opencompgraph.h>

namespace ocg = open_comp_graph;


int test_graph_create_and_set_attrs(const bool debug_print) {
    if (debug_print) {
        std::cout << "============== test_graph_create_and_set_attrs()" << '\n';
    }
    auto bench = ocg::internal::BenchmarkTime();

    // Graph
    auto graph = ocg::Graph();

    // Read Node...
    size_t id = 0;
    auto read_node = graph.create_node(ocg::NodeType::kReadImage, id++);
    if (debug_print) {
        std::cout << "read_node=" << &read_node << '\n';
    }

    auto read_id2 = read_node.get_id();
    if (debug_print) {
    std::cout << "read_id2=" << read_id2 << '\n';
    }

    auto read_node_type = read_node.get_node_type() == ocg::NodeType::kReadImage;
    if (debug_print) {
        std::cout << "read_node_type=" << read_node_type << '\n';
        std::cout << "read_node_type_id=" << static_cast<uint32_t>(read_node_type)
                  << '\n';
    }

    // Set string attribute
    auto read_attr = "file_path";
    if (graph.node_attr_exists(read_node, read_attr) == ocg::AttrState::kExists) {
        auto read_path1 = graph.get_node_attr_str(read_node, read_attr);
        if (debug_print) {
        std::cout << "read_path1=" << read_path1 << '\n';
        }

        graph.set_node_attr_str(
            read_node, read_attr, "tests/data/checker_8bit_rgba_8x8.png");
        auto read_path2 = graph.get_node_attr_str(read_node, read_attr);
        if (debug_print) {
            std::cout << "read_path2=" << read_path2 << '\n';
        }
    }

    // Set float attribute
    auto read_attr_x = "x";
    auto read_exists = graph.node_attr_exists(read_node, read_attr_x);
    if (read_exists == ocg::AttrState::kExists) {
        auto read_x = graph.get_node_attr_f32(read_node, read_attr_x);
        if (debug_print) {
            std::cout << "read_x=" << read_x << '\n';
        }

        graph.set_node_attr_f32(read_node, read_attr_x, 3.147f);
        auto read_x2 = graph.get_node_attr_f32(read_node, read_attr_x);
        if (debug_print) {
            std::cout << "read_x2=" << read_x2 << '\n';
        }
    }

    // Set integer attribute
    auto read_attr_y = "y";
    if (graph.node_attr_exists(read_node, read_attr_y) == ocg::AttrState::kExists) {
        auto read_y = graph.get_node_attr_f32(read_node, read_attr_y);
        if (debug_print) {
            std::cout << "read_y=" << read_y << '\n';
        }

        graph.set_node_attr_i32(read_node, read_attr_y, 42);
        auto read_y2 = graph.get_node_attr_f32(read_node, read_attr_y);
        if (debug_print) {
            std::cout << "read_y2=" << read_y2 << '\n';
        }
    }

    // Write Node...
    auto write_node = graph.create_node(ocg::NodeType::kWriteImage, id++);
    if (debug_print) {
    std::cout << "write_node=" << &write_node << '\n';
    }

    auto write_id2 = write_node.get_id();
    if (debug_print) {
        std::cout << "write_id2=" << write_id2 << '\n';
    }
    auto write_node_type = write_node.get_node_type();
    if (debug_print) {
        std::cout << "write_node_type="
                  << (write_node_type == ocg::NodeType::kWriteImage)
                  << '\n';
        std::cout << "write_node_type_id="
                  << static_cast<uint32_t>(write_node_type)
                  << '\n';
    }

    auto write_attr = "file_path";
    auto write_exists = graph.node_attr_exists(write_node, write_attr);
    if (write_exists == ocg::AttrState::kExists) {
        auto write_path1 = graph.get_node_attr_str(write_node, write_attr);
        if (debug_print) {
        std::cout << "write_path1=" << write_path1 << '\n';
        }

        graph.set_node_attr_str(
            write_node, write_attr,
            "./tests/data/out/test_graph_create_and_set_attrs_out.png");
        auto write_path2 = graph.get_node_attr_str(write_node, write_attr);
            if (debug_print) {
        std::cout << "write_path2=" << write_path2 << '\n';
            }
    }

    if (debug_print) {
        std::cout << "Graph as string:\n"
                  << graph.data_debug_string();

        bench.stop();
        bench.print("Test Graph Create and Set Attrs:");
    }

    return 0;
}
