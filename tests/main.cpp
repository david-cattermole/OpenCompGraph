#include <iostream>
#include <opencompgraph.h>
#include "test_a.h"
#include "test_b.h"
#include "test_c.h"
#include "test_d.h"
#include "test_e.h"
#include "test_f.h"
#include "test_g.h"
#include "test_h.h"
#include "test_i.h"

namespace ocg = open_comp_graph;

int main() {
    std::cout << "Starting Tests..." << '\n';
    ocg::log::initialize();
    auto bench = ocg::internal::BenchmarkTime();
    bench.start();

    const bool debug_print = false;

    // Create a cache for each test.
    auto cache_a = std::make_shared<ocg::Cache>();
    auto cache_d = std::make_shared<ocg::Cache>();
    auto cache_e = std::make_shared<ocg::Cache>();
    auto cache_f = std::make_shared<ocg::Cache>();
    auto cache_g = std::make_shared<ocg::Cache>();
    auto cache_h = std::make_shared<ocg::Cache>();

    // Run tests.
    auto count = 20;
    for (auto i = 0; i < count; ++i) {
        test_a(debug_print, cache_a);
        test_b(debug_print);
        test_c(debug_print);
        test_d(debug_print, cache_d);
        test_e(debug_print, cache_e);
        test_f(debug_print, cache_f);
        test_g(debug_print, cache_g);
        test_h(debug_print, cache_h);
        test_i(debug_print);
    }

    bench.stop();
    bench.print("All Tests:");
    bench.print("All Tests:", count);
    std::cout << "CacheA Count: " << cache_a->count() << '\n';
    std::cout << "CacheD Count: " << cache_d->count() << '\n';
    std::cout << "CacheE Count: " << cache_e->count() << '\n';
    std::cout << "CacheF Count: " << cache_f->count() << '\n';
    std::cout << "CacheG Count: " << cache_g->count() << '\n';
    std::cout << "CacheH Count: " << cache_h->count() << '\n';
    std::cout << "Completed Tests!" << std::endl;

    return 0;
}
