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
