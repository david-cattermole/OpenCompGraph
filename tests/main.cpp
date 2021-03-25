/*
 * Copyright (C) 2020, 2021 David Cattermole.
 *
 * This file is part of OpenCompGraph.
 *
 * OpenCompGraph is free software: you can redistribute it and/or modify it
 * under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * OpenCompGraph is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with OpenCompGraph.  If not, see <https://www.gnu.org/licenses/>.
 * ====================================================================
 *
 */

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
#include "test_j.h"
#include "test_k.h"
#include "test_l.h"
#include "test_m.h"

namespace ocg = open_comp_graph;

int main() {
    std::cout << "Starting Tests...\n";
    ocg::log::initialize();
    const bool debug_print = true;

    test_k(debug_print);

    // Run single frame tests.
    {
        std::cout << "Starting Single-Frame Tests...\n";
        auto bench = ocg::internal::BenchmarkTime();
        bench.start();

        auto cache = std::make_shared<ocg::Cache>();
        const size_t kBytesToGigabytes = 1073741824;  // int(pow(2, 30))
        auto capacity = 1 * kBytesToGigabytes;
        cache->set_capacity_bytes(capacity);

        auto count = 1;
        for (auto i = 0; i < count; ++i) {
            test_a(debug_print, cache);
            test_b(debug_print, cache);
            test_c(debug_print);
            test_d(debug_print, cache);
            test_e(debug_print, cache);
            test_f(debug_print, cache);
            test_g(debug_print, cache);
            test_h(debug_print, cache);
            test_i(debug_print);
            test_l(debug_print, cache);
            test_m(debug_print, cache);
        }

        bench.stop();
        bench.print("Single Frame Tests:");
        bench.print("Single Frame Tests:", count);
        std::cout << "Single Frame Cache Count: "
                  << cache->count() << '\n';
    }

    // Multi-Frame tests.
    {
        std::cout << "Starting Multi-Frame Tests...\n";
        auto bench = ocg::internal::BenchmarkTime();
        bench.start();

        test_j(debug_print);

        bench.stop();
        bench.print("Mult-Frame Tests:");
    }

    std::cout << "Completed Tests!" << std::endl;

    return 0;
}
