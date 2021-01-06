#include <iostream>
#include <opencompgraph.h>

namespace ocg = open_comp_graph;

int test_a(const bool debug_print) {
    if (debug_print) {
        std::cout << "=========================================== test_a()" << '\n';
    }
    auto bench = ocg::internal::BenchmarkTime();

    // TODO: Add an interesting test here.

    bench.stop();
    if (debug_print) {
        bench.print("Test A:");
    }

    return 0;
}
