#include <iostream>
#include <string>
#include "opencompgraph.h"

namespace ocg = opencompgraph;

int test_b() {
    std::cout << "====================================== test_b()" << '\n';
    auto bench = ocg::internal::BenchmarkTime();

    auto app_name = std::string("my awesome demo");
    auto x = ocg::cpp::make_thingc(app_name);
    auto name = ocg::cpp::get_name(*x);
    std::cout << name << '\n';

    bench.stop();
    bench.print("Test B:");

    return 0;
}
