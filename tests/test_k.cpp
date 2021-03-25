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
#include <vector>
#include <numeric>
#include <random>
#include <algorithm>
#include <iterator>

#include <opencompgraph.h>
#include "generate_frame_range.h"

namespace ocg = open_comp_graph;


// Test the sequence execution and caching mechanisms.
int test_k(const bool debug_print) {
    if (debug_print) {
        std::cout << "=========================================== test_k()" << '\n';
    }
    auto bench = ocg::internal::BenchmarkTime();

    const char* config_file_name = "open_comp_graph.yaml";
    const size_t kBytesToGigabytes = 1073741824;  // int(pow(2, 30))
    auto config = ocg::Config(config_file_name);
    auto num_bytes = config.cache_ram_capacity_bytes();
    auto percent = config.cache_ram_capacity_percent();
    auto num_gigabytes =
        static_cast<float>(num_bytes) / static_cast<float>(kBytesToGigabytes);
    std::cout << "Config: cache_ram_capacity_percent: "
              << percent << '\n';
    std::cout << "Config: cache_ram_capacity_bytes: "
              << num_bytes << '\n';
    std::cout << "Config: cache_ram_capacity_gigabytes: "
              << num_gigabytes << '\n';
    std::cout << "Config: \n"
              << config.data_debug_string() << '\n';

    auto cache = std::make_shared<ocg::Cache>();
    cache->set_capacity_bytes(num_bytes);
    std::cout << "Cache: capacity_bytes: "
              << cache->capacity_bytes() << '\n';
    std::cout << "Cache: used_bytes: "
              << cache->used_bytes() << '\n';
    std::cout << "Cache: count: "
              << cache->count() << '\n';
    std::cout << "Cache: \n"
              << cache->data_debug_string() << '\n';

    if (debug_print) {
        bench.stop();
        bench.print("Test K:");
    }
    return 0;
}
