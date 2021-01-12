#include <iostream>
#include <vector>
#include <numeric>
#include <random>
#include <algorithm>
#include <iterator>

#include <opencompgraph.h>
#include "generate_frame_range.h"

namespace ocg = open_comp_graph;

// Memory Conversion
const size_t kBytesToGigabytes = 1073741824;  // int(pow(2, 30))


std::vector<int32_t> generate_frame_range(const int32_t start_frame,
                                          const int32_t end_frame) {
    auto total_frame_count = (end_frame - start_frame) + 1;
    std::vector<int32_t> frames(total_frame_count);
    std::iota(frames.begin(), frames.end(), start_frame);
    return frames;
}

std::vector<int32_t> generate_frame_range_loop(const int32_t start_frame,
                                               const int32_t end_frame,
                                               const uint32_t playback_n_times) {
    auto total_frame_count = (end_frame - start_frame) + 1;
    std::vector<int32_t> frames_loop((total_frame_count * playback_n_times));
    for (uint32_t i = 0; i < playback_n_times; ++i) {
        std::iota(frames_loop.begin() + (i * total_frame_count),
                  frames_loop.begin() + ((i + 1) * total_frame_count),
                  start_frame);
    }
    return frames_loop;
}

std::vector<int32_t> generate_frame_range_random(const int32_t start_frame,
                                                 const int32_t end_frame,
                                                 const uint32_t playback_n_times) {
    std::random_device random_device;
    std::mt19937 generator(random_device());

    auto frames_loop = generate_frame_range_loop(
        start_frame, end_frame, playback_n_times);

    std::vector<int32_t> random_frames;
    std::copy(frames_loop.begin(), frames_loop.end(), std::back_inserter(random_frames));
    std::shuffle(random_frames.begin(),
                 random_frames.end(), generator);
    return random_frames;
}

std::vector<int32_t> generate_frame_range_semi_sequential(
    const int32_t start_frame,
    const int32_t end_frame,
    const uint32_t playback_n_times,
    const std::vector<uint32_t> &sequence_skips) {
    auto frames = generate_frame_range(start_frame, end_frame);
    auto frames_loop = generate_frame_range_loop(
        start_frame, end_frame, playback_n_times);

    std::vector<int32_t> random_sequence_frames;
    std::copy(frames_loop.begin(), frames_loop.end(),
              std::back_inserter(random_sequence_frames));

    std::random_device random_device;
    std::mt19937 generator(random_device());

    // Number of times the user skips to another frame sequence.  The
    // more a number is in this array the more often it will be used.
    std::vector<uint32_t> random_sequence_length;
    std::copy(sequence_skips.begin(), sequence_skips.end(),
              std::back_inserter(random_sequence_length));
    std::shuffle(random_sequence_length.begin(),
                 random_sequence_length.end(), generator);

    // Fill the frame range.
    for (auto skip_n_frames : random_sequence_length) {
        auto offset = generator() % random_sequence_frames.size();
        auto new_start_index = generator() % frames.size();
        auto start_value = frames[new_start_index];
        for (uint32_t i = 0; i < skip_n_frames; ++i) {
            auto index = (new_start_index + i) % frames.size();
            auto index2 = (offset + i) % random_sequence_frames.size();
            random_sequence_frames[index2] = frames[index];
        }
    }
    return random_sequence_frames;
}
