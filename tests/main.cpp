#include <iostream>
#include <opencompgraph.h>
#include "test_a.h"
#include "test_b.h"
#include "test_c.h"
#include "test_d.h"
#include "test_e.h"
#include "test_f.h"

namespace ocg = open_comp_graph;

int main() {
    std::cout << "Starting Tests..." << '\n';
    auto bench = ocg::internal::BenchmarkTime();
    bench.start();

    // Create a cache.
    auto cache_d = ocg::make_shared_cache();
    auto cache_e = ocg::make_shared_cache();
    auto cache_f = ocg::make_shared_cache();

    // Run tests.
    auto count = 20;
    for (auto i = 0; i < count; ++i) {
        test_a();
        test_b();
        test_c();
        test_d(cache_d);
        test_e(cache_e);
        test_f(cache_f);
    }

    bench.stop();
    bench.print("All Tests:");
    bench.print("All Tests:", count);
    std::cout << "CacheD Count: " << cache_d->count() << '\n';
    std::cout << "CacheE Count: " << cache_e->count() << '\n';
    std::cout << "CacheF Count: " << cache_f->count() << '\n';
    std::cout << "Completed Tests!" << std::endl;

    return 0;
}
