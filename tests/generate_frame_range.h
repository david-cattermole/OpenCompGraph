#ifndef OPENCOMPGRAPH_TEST_GENERATE_FRAME_RANGE_H
#define OPENCOMPGRAPH_TEST_GENERATE_FRAME_RANGE_H

#include <vector>

// Fill a vector with frame numbers, starting at start_frame and
// ending at end_frame.
std::vector<int32_t> generate_frame_range(
    const int32_t start_frame,
    const int32_t end_frame);

// Frame numbers, looped N times, to simulate a user playing back.
std::vector<int32_t> generate_frame_range_loop(
    const int32_t start_frame,
    const int32_t end_frame,
    const uint32_t playback_n_times);

// Create an entirely random sequence of frame numbers (of the valid
// frames we already had). Each frame will be visited used exactly
// 'playback_n_times' number of times.
std::vector<int32_t> generate_frame_range_random(
    const int32_t start_frame,
    const int32_t end_frame,
    const uint32_t playback_n_times);


// Generate a sequence of frame numbers that sometimes skips to
// another sequence of frames.
//
// This is assumed to be a more common access pattern of a sequence of
// frames for the users. Some frames will not be read, others will be
// read multiple times.
std::vector<int32_t> generate_frame_range_semi_sequential(
    const int32_t start_frame,
    const int32_t end_frame,
    const uint32_t playback_n_times,
    const std::vector<uint32_t> &sequence_skips);

#endif //OPENCOMPGRAPH_TEST_GENERATE_FRAME_RANGE_H
