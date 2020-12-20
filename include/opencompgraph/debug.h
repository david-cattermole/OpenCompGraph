/*! Debugging Utils - printing and benchmarking.
 */

#ifndef OPENCOMPGRAPH_DEBUG_H
#define OPENCOMPGRAPH_DEBUG_H

// STL
#include <iostream> // cout, cerr
#include <iomanip>  // setfill, setw
#include <string>   // string
#include <chrono>   // high_resolution_clock, time_point, duration, duration_cast

namespace opencompgraph {
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
} // namespace opencompgraph


#endif // OPENCOMPGRAPH_DEBUG_H
