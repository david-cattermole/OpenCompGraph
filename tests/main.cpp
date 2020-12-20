#include <iostream>
#include <opencompgraph.h>
#include "test_a.h"
#include "test_b.h"
#include "test_c.h"
#include "test_d.h"
#include "test_e.h"
#include "test_f.h"

int main() {
    std::cout << "Starting Tests..." << '\n';
    auto bench = opencompgraph::internal::BenchmarkTime();
    bench.start();

    // Run tests.
    auto count = 4;
    for (auto i = 0; i < count; ++i) {
        test_a();
        test_b();
        test_c();
        test_d();
        test_e();
        test_f();
    }

    bench.stop();
    bench.print("All Tests:");
    bench.print("All Tests:", count);
    std::cout << "Completed Tests!" << std::endl;

    return 0;
}
