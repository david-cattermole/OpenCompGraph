#include <iostream>
#include <string>
#include "opencompgraph.h"

namespace ocg = open_comp_graph;

int test_b(const bool debug_print) {
    if (debug_print) {
        std::cout << "=========================================== test_b()" << '\n';
    }
    auto bench = ocg::internal::BenchmarkTime();

    // auto app_name = std::string("my awesome demo");
    // auto x = ocg::cpp::make_thingc(app_name);
    // auto name = ocg::cpp::get_name(*x);
    // if (debug_print) {
    //     std::cout << name << '\n';
    // }

    bench.stop();
    if (debug_print) {
        bench.print("Test B:");
    }

    return 0;
}
