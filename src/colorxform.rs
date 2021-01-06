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
