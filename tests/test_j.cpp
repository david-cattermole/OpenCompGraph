#include <iostream>
#include <vector>
#include <numeric>
#include <random>
#include <algorithm>
#include <iterator>

#include <opencompgraph.h>
#include "generate_frame_range.h"

namespace ocg = open_comp_graph;


// Test the sequence execution and caching mechanisms.
int test_j(const bool debug_print) {
    if (debug_print) {
        std::cout << "=========================================== test_j()" << '\n';
    }
    auto bench = ocg::internal::BenchmarkTime();
    auto graph = ocg::Graph();

    auto read_node = graph.create_node(ocg::NodeType::kReadImage, "read");
    auto write_node = graph.create_node(ocg::NodeType::kWriteImage, "write");
    graph.set_node_attr_str(read_node, "file_path", "tests/data/color_bars_3840x2160_png/color_bars.####.png");
    graph.set_node_attr_str(write_node, "file_path", "./tests/data/out/test_j_out_color_bars_3840x2160.####.png");

    graph.connect(read_node, write_node, 0);

    if (debug_print) {
        std::cout << "Graph as string:\n"
                  << graph.data_debug_string();
    }

    auto start_frame = 951;
    auto end_frame = 1001;
    auto total_frame_count = (end_frame - start_frame) + 1;
    std::cout << "start_frame=" << start_frame << '\n';
    std::cout << "end_frame=" << end_frame << '\n';
    std::cout << "total_frame_count=" << total_frame_count << '\n';

    // frame range start_frame to end_frame.
    auto frames = generate_frame_range(start_frame, end_frame);

    // frame range with frames that do not have valid inputs.
    auto frames_out_of_range = generate_frame_range(start_frame - 10, start_frame + 10);

    // Frame numbers, looped N times, to simulate a user playing back.
    const auto playback_n_times = 4;
    auto frames_loop = generate_frame_range_loop(
        start_frame, end_frame, playback_n_times);

    // Create an entirely random sequence of frame numbers (of the
    // valid frames we already had). Each frame will be visited used
    // exactly once.
    auto random_frames = generate_frame_range_random(
        start_frame, end_frame, playback_n_times);

    // Generate a sequence of frame numbers that sometimes skips to
    // another sequence of frames.
    std::vector<uint32_t> sequence_skips{
        5, 5, 5, 5, 5,
        10, 10, 10, 10, 10,
        15,
        20,
        25, 25,
        30,
        50,
        100, 100};
    auto random_sequence_frames = generate_frame_range_semi_sequential(
        start_frame, end_frame, playback_n_times, sequence_skips);

    const size_t kBytesToGigabytes = 1073741824;  // int(pow(2, 30))
    auto capacity = 1 * kBytesToGigabytes;

    // Execute the *Read node*, in sequential order.
    {
        std::cout << "Executing... (sequential order)\n";
        auto cache = std::make_shared<ocg::Cache>();
        cache->set_capacity_bytes(capacity);
        graph.execute(read_node, frames, cache);
        std::cout << "Cache: Count (sequential order): "
                  << cache->data_debug_string() << '\n';
    }

    // Execute the *Read node*, in sequential order, with out of range
    // values.
    {
        std::cout << "Executing... (sequential order out of range)\n";
        auto cache = std::make_shared<ocg::Cache>();
        cache->set_capacity_bytes(capacity);
        graph.execute(read_node, frames_out_of_range, cache);
        std::cout << "Cache: Count (sequential order out of range): "
                  << cache->data_debug_string() << '\n';
    }

    // Execute the *Read node*, in sequential order (looped).
    {
        std::cout << "Executing... (sequential loop order)\n";
        auto cache = std::make_shared<ocg::Cache>();
        cache->set_capacity_bytes(capacity);
        graph.execute(read_node, frames_loop, cache);
        std::cout << "Cache: Count (sequential loop order): "
                  << cache->data_debug_string() << '\n';
    }

    // Execute the read node, in random order.
    {
        std::cout << "Executing... (random order)\n";
        auto cache = std::make_shared<ocg::Cache>();
        cache->set_capacity_bytes(capacity);
        graph.execute(read_node, random_frames, cache);
        std::cout << "Cache Count (random order): "
                  << cache->data_debug_string() << '\n';
    }

    // Execute the read node, in semi-sequential order.
    {
        std::cout << "Executing... (semi-sequential order)\n";
        auto cache = std::make_shared<ocg::Cache>();
        cache->set_capacity_bytes(capacity);
        graph.execute(read_node, random_sequence_frames, cache);
        std::cout << "Cache Count (semi-sequential): "
                  << cache->data_debug_string() << '\n';
    }

    if (debug_print) {
        bench.stop();
        bench.print("Test J:");
    }
    return 0;
}