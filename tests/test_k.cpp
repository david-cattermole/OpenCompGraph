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

    const size_t kBytesToGigabytes = 1073741824;  // int(pow(2, 30))
    auto config = ocg::Config();
    auto num_bytes = config.cache_ram_capacity_bytes();
    auto percent = config.cache_ram_capacity_percent();
    auto num_gigabytes = num_bytes / kBytesToGigabytes;

    std::cout << "Config: cache_ram_capacity_percent: " << percent << '\n';
    std::cout << "Config: cache_ram_capacity_bytes: " << num_bytes << '\n';
    std::cout << "Config: cache_ram_capacity_gigabytes: " << num_gigabytes << '\n';

    if (debug_print) {
        bench.stop();
        bench.print("Test K:");
    }
    return 0;
}
