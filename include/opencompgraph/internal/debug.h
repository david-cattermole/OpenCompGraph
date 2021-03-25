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
 * Debugging Utils - printing and benchmarking.
 */

#ifndef OPENCOMPGRAPH_DEBUG_H
#define OPENCOMPGRAPH_DEBUG_H

// STL
#include <iostream> // cout, cerr
#include <iomanip>  // setfill, setw
#include <string>   // string
#include <chrono>   // high_resolution_clock, time_point, duration, duration_cast

namespace open_comp_graph {
namespace internal {

// Wrapper class around 'get_timestamp' timer.
class BenchmarkTime {
public:
    typedef std::chrono::high_resolution_clock clock_t;
    typedef std::chrono::time_point<clock_t> time_point_t;
    typedef std::chrono::duration<double, std::ratio<1> > second_t;

    BenchmarkTime() :
            start_time(),
            total_time(0.0) {
        start();
    }

    void start() {
        start_time = clock_t::now();
    }

    void reset() {
        total_time = 0.0;
        start_time = clock_t::now();
    }

    double stop() {
        return total_time += std::chrono::duration_cast<second_t> (
                clock_t::now() - start_time).count();
    }

    void print(const std::string& heading, uint32_t loop_count = 0) {
        auto secs = total_time;
        if (loop_count <= 1) {
            std::cout << heading << " Time: ";
        } else if (loop_count > 0) {
            secs /= loop_count;
            std::cout << heading << " Time (per-loop): ";
        }
        std::cout << secs << " seconds";
        std::cout << "\n";
    }

    time_point_t start_time;
    double total_time;
};

} // namespace internal
} // namespace open_comp_graph


#endif // OPENCOMPGRAPH_DEBUG_H
