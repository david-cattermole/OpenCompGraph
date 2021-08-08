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

pub mod matrix;
pub mod vector4f32;
pub mod vector4i32;

use petgraph;

pub type GraphIdx = usize;
pub type NodeWeight = u64;
pub type EdgeWeight = u8;
pub type NodeIdx = petgraph::graph::NodeIndex<GraphIdx>;
pub type EdgeIdx = petgraph::graph::EdgeIndex<GraphIdx>;
pub type HashValue = u64;
pub type Identifier = u64;

// Memory Conversion
pub const BYTES_TO_KILOBYTES: usize = 1024; // int(pow(2, 10))
pub const BYTES_TO_MEGABYTES: usize = 1048576; // int(pow(2, 20))
pub const BYTES_TO_GIGABYTES: usize = 1073741824; // int(pow(2, 30))
pub const KILOBYTES_TO_MEGABYTES: usize = 1024; // int(pow(2, 10))
pub const KILOBYTES_TO_GIGABYTES: usize = 1048576; // int(pow(2, 20))

// Color Bars Texture, for debug.
//
// https://en.wikipedia.org/wiki/SMPTE_color_bars
//
// ------------------------------
//  R  -  G  -  B   - COLOR NAME
// ------------------------------
// 235 - 235 - 235  - 100% White
// 180 - 180 - 180  - 75% White
// 235 - 235 - 16   - Yellow
// 16  - 235 - 235  - Cyan
// 16  - 235 - 16   - Green
// 235 - 16  - 235  - Magenta
// 235 - 16  - 16   - Red
// 16  - 16  - 235  - Blue
// 16  - 16  - 16   - Black
// ------------------------------
//
// The texture block (below) starts at the lower-left (zeroth index)
// and continues the upper-right (last index).
//
// Note: To make things even (only 8 entries), we skip the "75% white"
// value.
//
pub static COLOR_BARS_WIDTH: i32 = 4;
pub static COLOR_BARS_HEIGHT: i32 = 4;
pub static COLOR_BARS_NUM_CHANNELS: i32 = 3;
pub static COLOR_BARS: &'static [f32] = &[
    // Row 0
    0.9215, 0.9215, 0.9215, // 235, 235, 235 - 100% White
    0.9215, 0.9215, 0.0627, // 235, 235, 16  - Yellow
    0.0627, 0.9215, 0.9215, // 16, 235, 235  - Cyan
    0.0627, 0.9215, 0.0627, // 16, 235, 16   - Green
    // Row 1
    0.9215, 0.9215, 0.9215, // 235, 235, 235 - 100% White
    0.9215, 0.9215, 0.0627, // 235, 235, 16  - Yellow
    0.0627, 0.9215, 0.9215, // 16, 235, 235  - Cyan
    0.0627, 0.9215, 0.0627, // 16, 235, 16   - Green
    // Row 2
    0.9215, 0.0627, 0.9215, // 235, 16, 235  - Magenta
    0.9215, 0.0627, 0.0627, // 235, 16, 16   - Red
    0.0627, 0.0627, 0.9215, // 16, 16, 235   - Blue
    0.0627, 0.0627, 0.0627, // 16, 16, 16    - Black
    // Row 3
    0.9215, 0.0627, 0.9215, // 235, 16, 235  - Magenta
    0.9215, 0.0627, 0.0627, // 235, 16, 16   - Red
    0.0627, 0.0627, 0.9215, // 16, 16, 235   - Blue
    0.0627, 0.0627, 0.0627, // 16, 16, 16    - Black
];

pub enum ErrorCode {
    Failure,
    Invalid,
    Uninitialized,
}
