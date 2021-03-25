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
