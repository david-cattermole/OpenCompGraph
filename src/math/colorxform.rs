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

use nalgebra as na;

use crate::cxxbridge::ffi::Matrix4;
use crate::matrix;

/// Multiply RGB colors, as matrix.
pub fn apply_scale_rgba(
    input_matrix: na::Matrix4<f32>,
    r_scale: f32,
    g_scale: f32,
    b_scale: f32,
    a_scale: f32,
) -> na::Matrix4<f32> {
    let diagonal = na::Vector4::new(r_scale, g_scale, b_scale, a_scale);
    let scale_matrix = na::Matrix4::<f32>::from_diagonal(&diagonal);
    scale_matrix * input_matrix
}

// TODO: Write an exposure matrix wrapper function, which calls
// apply_scale_rgba, but with exposure values (EV) as input.
//
// float multiply = std::pow(2.0, exposure);  // Exposure Value
// let multiply: f32 = (2.0).powf(exposure);  // Exposure Value
