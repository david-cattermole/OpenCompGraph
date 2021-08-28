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
#include "test_graph/test_graph_create_and_set_attrs.h"
#include "test_graph/test_graph_non_linear_graph.h"
#include "test_graph/test_graph_no_ops.h"
#include "test_graph/test_graph_re_eval_modified_graph.h"
#include "test_stream/test_stream_empty_write_geom.h"
#include "test_cache/test_cache_read_image_seq.h"
#include "test_cache/test_cache_init.h"
#include "test_node_null/test_node_null.h"
#include "test_node_imageio/test_node_imageio.h"
#include "test_node_imageio/test_node_imageio_jpeg.h"
#include "test_node_imageio/test_node_imageio_tiff.h"
#include "test_node_lens/test_node_lens.h"
#include "test_node_transform/test_node_transform.h"
#include "test_node_grade/test_node_grade.h"
#include "test_node_grade/test_node_grade_concat.h"
#include "test_node_merge/test_node_merge.h"
#include "test_node_crop/test_node_crop.h"
#include "test_node_viewer/test_node_viewer.h"
#include "test_node_viewer/test_node_viewer_3dlut.h"

namespace ocg = open_comp_graph;

int main() {
    std::cout << "Starting Tests...\n";
    ocg::log::initialize();
    const bool debug_print = true;

    test_cache_init(debug_print);

    // Run single frame tests.
    {
        std::cout << "Starting Single-Frame Tests...\n";
        auto bench = ocg::internal::BenchmarkTime();
        bench.start();

        auto cache = std::make_shared<ocg::Cache>();
        const size_t kBytesToGigabytes = 1073741824;  // int(pow(2, 30))
        auto capacity = 4 * kBytesToGigabytes;
        cache->set_capacity_bytes(capacity);

        auto count = 1;  // Increase the 'count' to test cache read performance.
        for (auto i = 0; i < count; ++i) {
            test_stream_empty_write_geom(debug_print);

            test_graph_create_and_set_attrs(debug_print);
            test_graph_non_linear_graph(debug_print, cache);
            test_graph_no_ops(debug_print, cache);
            test_graph_re_eval_modified_graph(debug_print, cache);

            test_node_lens(debug_print, cache);
            test_node_transform(debug_print, cache);
            test_node_imageio(debug_print, cache);
            test_node_imageio_jpeg(debug_print, cache);
            test_node_imageio_tiff(debug_print, cache);
            test_node_null(debug_print, cache);
            test_node_grade(debug_print, cache);
            test_node_grade_concat(debug_print, cache);
            test_node_merge(debug_print, cache);
            test_node_crop(debug_print, cache);
            test_node_viewer(debug_print, cache);
            test_node_viewer_3dlut(debug_print, cache);
        }

        bench.stop();
        bench.print("Single Frame Tests:");
        bench.print("Single Frame Tests:", count);
        std::cout << "Single Frame Cache Count: "
                  << cache->count() << '\n';
    }

    // Multi-Frame tests.
    {
        std::cout << "Starting Multi-Frame Tests...\n";
        auto bench = ocg::internal::BenchmarkTime();
        bench.start();

        test_cache_read_image_seq(debug_print);

        bench.stop();
        bench.print("Mult-Frame Tests:");
    }

    std::cout << "Completed Tests!" << std::endl;
    return 0;
}
