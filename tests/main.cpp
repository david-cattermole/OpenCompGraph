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

namespace ocg = open_comp_graph;

int main() {
    std::cout << "Starting Tests..." << '\n';
    ocg::log::initialize();
    auto bench = ocg::internal::BenchmarkTime();
    bench.start();

    // Create a cache for each test.
    auto cache_d = std::make_shared<ocg::Cache>();
    auto cache_e = std::make_shared<ocg::Cache>();
    auto cache_f = std::make_shared<ocg::Cache>();
    auto cache_g = std::make_shared<ocg::Cache>();
    auto cache_h = std::make_shared<ocg::Cache>();

    // Run tests.
    auto count = 20;
    for (auto i = 0; i < count; ++i) {
        test_a();
        test_b();
        test_c();
        test_d(cache_d);
        test_e(cache_e);
        test_f(cache_f);
        test_g(cache_g);
        test_h(cache_h);
    }

    bench.stop();
    bench.print("All Tests:");
    bench.print("All Tests:", count);
    std::cout << "CacheD Count: " << cache_d->count() << '\n';
    std::cout << "CacheE Count: " << cache_e->count() << '\n';
    std::cout << "CacheF Count: " << cache_f->count() << '\n';
    std::cout << "CacheG Count: " << cache_g->count() << '\n';
    std::cout << "CacheH Count: " << cache_h->count() << '\n';
    std::cout << "Completed Tests!" << std::endl;

    return 0;
}
