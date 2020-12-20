#include <iostream>
#include "test_a.h"
#include "test_b.h"
#include "test_c.h"
#include "test_d.h"
#include "test_e.h"
#include "test_f.h"

int main() {
    std::cout << "Starting Tests..." << std::endl;

    // Run tests.
    test_a();
    test_b();
    test_c();
    test_d();
    test_e();
    test_f();

    std::cout << "Completed Tests!" << std::endl;
    return 0;
}
