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

int test_node_merge(const bool debug_print,
                    std::shared_ptr<ocg::Cache> cache) {
    if (debug_print) {
        std::cout << "============================== test_node_merge()" << '\n';
    }
    auto bench = ocg::internal::BenchmarkTime();

    auto frames = generate_frame_range(1, 2);
    auto graph = ocg::Graph();

    auto read_image_type = ocg::NodeType::kReadImage;
    auto merge_image_type = ocg::NodeType::kMergeImage;
    auto write_image_type = ocg::NodeType::kWriteImage;

    auto read_empty_node = graph.create_node(read_image_type, "read_empty");
    auto read_bg1_node = graph.create_node(read_image_type, "read_bg1");
    auto read_fg1_node = graph.create_node(read_image_type, "read_fg1");
    auto read_fg2_node = graph.create_node(read_image_type, "read_fg2");
    auto read_fg3_node = graph.create_node(read_image_type, "read_fg3");
    auto merge_add_node = graph.create_node(merge_image_type, "merge_add");
    auto merge_over_node = graph.create_node(merge_image_type, "merge_over");
    auto merge_mult_node = graph.create_node(merge_image_type, "merge_mult");
    auto merge_over2_node = graph.create_node(merge_image_type, "merge_over2");
    auto merge_over3_node = graph.create_node(merge_image_type, "merge_over3");
    auto merge_over4_node = graph.create_node(merge_image_type, "merge_over4");
    auto merge_over5_node = graph.create_node(merge_image_type, "merge_over5");
    auto write_add_node = graph.create_node(write_image_type, "write_add");
    auto write_over_node = graph.create_node(write_image_type, "write_over");
    auto write_mult_node = graph.create_node(write_image_type, "write_mult");
    auto write_concat_node = graph.create_node(write_image_type, "write_concat");
    auto write_empty_node = graph.create_node(write_image_type, "write_empty");
    auto write_beachball_node = graph.create_node(write_image_type, "write_beachball");
    auto write_beachball2_node = graph.create_node(write_image_type, "write_beachball2");

    // Precompute index for enum.
    auto merge_mode_add = static_cast<int32_t>(ocg::MergeImageMode::kAdd);
    auto merge_mode_over = static_cast<int32_t>(ocg::MergeImageMode::kOver);
    auto merge_mode_multiply = static_cast<int32_t>(ocg::MergeImageMode::kMultiply);

    // Read Images
    graph.set_node_attr_str(read_bg1_node, "file_path",
                            "tests/data/oiio-images/tahoe-gps.jpg");
    graph.set_node_attr_str(read_fg1_node, "file_path",
                            "tests/data/oiio-images/grid-overscan.exr");
    graph.set_node_attr_str(read_fg2_node, "file_path",
                            "tests/data/oiio-images/oiio-logo-with-alpha.png");
    graph.set_node_attr_str(read_fg3_node, "file_path",
                            "tests/data/openexr-images/Beachball/multipart.####.exr");

    // A Add B
    graph.set_node_attr_i32(merge_add_node, "mode", merge_mode_add);
    graph.set_node_attr_str(
        write_add_node, "file_path",
        "./tests/data/out/test_node_merge_merge_add_out.####.exr");
    graph.connect(read_fg1_node, merge_add_node, 0); // A
    graph.connect(read_bg1_node, merge_add_node, 1); // B
    graph.connect(merge_add_node, write_add_node, 0);

    // A Over B
    graph.set_node_attr_i32(merge_over_node, "mode", merge_mode_over);
    graph.set_node_attr_str(
        write_over_node, "file_path",
        "./tests/data/out/test_node_merge_merge_over_out.####.exr");
    graph.connect(read_fg2_node, merge_over_node, 0); // A
    graph.connect(read_bg1_node, merge_over_node, 1); // B
    graph.connect(merge_over_node, write_over_node, 0);

    // A Multiply B
    graph.set_node_attr_i32(merge_mult_node, "mode", merge_mode_multiply);
    graph.set_node_attr_str(
        write_mult_node, "file_path",
        "./tests/data/out/test_node_merge_merge_multiply_out.####.exr");
    graph.connect(read_fg2_node, merge_mult_node, 0); // A
    graph.connect(read_bg1_node, merge_mult_node, 1); // B
    graph.connect(merge_mult_node, write_mult_node, 0);

    // Multiple merge operations; A Add B, then A over B.
    graph.set_node_attr_i32(merge_over2_node, "mode", merge_mode_over);
    graph.set_node_attr_str(
        write_concat_node, "file_path",
        "./tests/data/out/test_node_merge_merge_add_over_out.####.exr");
    graph.connect(read_fg2_node, merge_over2_node, 0);  // A
    graph.connect(merge_add_node, merge_over2_node, 1); // B
    graph.connect(merge_over2_node, write_concat_node, 0);

    // Merge empty read node over background.
    graph.set_node_attr_str(
        write_empty_node, "file_path",
        "./tests/data/out/test_node_merge_merge_empty_out.####.exr");
    graph.connect(read_empty_node, merge_over3_node, 0); // A
    graph.connect(read_bg1_node, merge_over3_node, 1);   // B
    graph.connect(merge_over3_node, write_empty_node, 0);

    // Beachball - A Over B
    graph.set_node_attr_i32(merge_over4_node, "mode", merge_mode_over);
    graph.set_node_attr_str(
        write_beachball_node, "file_path",
        "./tests/data/out/test_node_merge_merge_beachball_out.####.exr");
    graph.connect(read_bg1_node, merge_over4_node, 0); // A - tahoe-gps.jpg
    graph.connect(read_fg3_node, merge_over4_node, 1); // B - beachball
    graph.connect(merge_over4_node, write_beachball_node, 0);

    // Beachball - A Over B
    graph.set_node_attr_i32(merge_over4_node, "mode", merge_mode_over);
    graph.set_node_attr_str(
        write_beachball2_node, "file_path",
        "./tests/data/out/test_node_merge_merge_beachball2_out.####.exr");
    graph.connect(read_fg3_node, merge_over5_node, 0); // A - beachball
    graph.connect(read_bg1_node, merge_over5_node, 1); // B - tahoe-gps.jpg
    graph.connect(merge_over5_node, write_beachball2_node, 0);

    // Execute
    if (debug_print) {
        std::cout << "Graph as string (before):\n"
                  << graph.data_debug_string();
    }
    graph.execute(write_add_node, frames, cache);
    graph.execute(write_over_node, frames, cache);
    graph.execute(write_mult_node, frames, cache);
    graph.execute(write_concat_node, frames, cache);
    graph.execute(write_empty_node, frames, cache);
    graph.execute(write_beachball_node, frames, cache);
    graph.execute(write_beachball2_node, frames, cache);

    if (debug_print) {
        std::cout << "Graph as string (after):\n"
                  << graph.data_debug_string();

        bench.stop();
        bench.print("Test Node Merge:");
    }

    return 0;
}
