#include <iostream>
#include <string>
#include "opencompgraph.h"

namespace ocg = opencompgraph;

int test_b() {
    std::cout << "test_b()" << std::endl;
    auto app_name = std::string("my awesome demo");
    auto x = ocg::make_thingc(app_name);
    auto name = ocg::get_name(*x);
    std::cout << name << std::endl;
    return 0;
}
