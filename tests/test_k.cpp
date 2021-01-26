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
