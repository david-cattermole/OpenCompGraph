use nalgebra as na;

use crate::cxxbridge::ffi::Matrix4;
use crate::matrix;

fn create_translate_2d(translate_x: f32, translate_y: f32) -> na::Matrix4<f32> {
    na::Matrix4::<f32>::new(
        1.0,
        0.0,
        0.0,
        0.0,
        //
        0.0,
        1.0,
        0.0,
        0.0,
        //
        0.0,
        0.0,
        1.0,
        0.0,
        translate_x,
        translate_y,
        0.0,
        1.0,
    )
}

fn create_rotate_2d(center_x: f32, center_y: f32, angle_degree: f32) -> na::Matrix4<f32> {
    let angle_radian: f32 = angle_degree * std::f32::consts::PI / 180.0;
    let c = (angle_radian).cos();
    let s = (angle_radian).sin();

    na::Matrix4::<f32>::new(
        c,
        -s,
        center_x * (1.0 - c) + center_y * s,
        0.0,
        //
        s,
        c,
        center_y * (1.0 - c) - center_x * s,
        0.0,
        //
        0.0,
        0.0,
        1.0,
        0.0,
        //
        0.0,
        0.0,
        0.0,
        1.0,
    )
}

fn create_scale_2d(scale_x: f32, scale_y: f32) -> na::Matrix4<f32> {
    na::Matrix4::<f32>::new(
        scale_x, 0.0, 0.0, 0.0, //
        0.0, scale_y, 0.0, 0.0, //
        0.0, 0.0, 1.0, 0.0, //
        0.0, 0.0, 0.0, 1.0, //
    )
}

// The transformation order is "TRS".
pub fn apply_transform_2d(
    input_matrix: na::Matrix4<f32>,
    translate_x: f32,
    translate_y: f32,
    rotate_center_x: f32,
    rotate_center_y: f32,
    rotate_degree: f32,
    scale_x: f32,
    scale_y: f32,
) -> na::Matrix4<f32> {
    let translate = create_translate_2d(translate_x, translate_y);
    let rotate = create_rotate_2d(rotate_center_x, rotate_center_y, rotate_degree);
    let scale = create_scale_2d(scale_x, scale_y);
    scale * rotate * translate * input_matrix
}