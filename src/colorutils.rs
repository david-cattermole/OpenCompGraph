use fastapprox::faster;
use log::{debug, error, info, warn};
use nalgebra as na;

use crate::colorxform;
use crate::cxxbridge::ffi::Matrix4;
use crate::pixelblock::PixelBlock;

/// https://www.excamera.com/sphinx/article-srgb.html
pub fn convert_srgb_to_linear(x: f32) -> f32 {
    let a: f32 = 0.055;
    if x <= 0.04045 {
        x * (1.0 / 12.92)
    } else {
        faster::pow((x + a) * (1.0 / (1.0 + a)), 2.4)
    }
}

/// https://www.excamera.com/sphinx/article-srgb.html
pub fn convert_linear_to_srgb(x: f32) -> f32 {
    let a = 0.055;
    if x <= 0.0031308 {
        x * 12.92
    } else {
        (1.0 + a) * faster::pow(x, 1.0 / 2.4) - a
    }
}

pub fn apply_color_matrix_inplace(pixels: &mut [f32], num_channels: u8, matrix: na::Matrix4<f32>) {
    let pixel_count = pixels.len() / (num_channels as usize);
    match num_channels {
        3 => {
            for i in 0..pixel_count {
                let index = i * (num_channels as usize);
                let r = pixels[index + 0];
                let g = pixels[index + 1];
                let b = pixels[index + 2];
                // if i < 3 {
                //     debug!("before r={} g={} b={}", r, g, b);
                // }
                let rgba = matrix * na::Vector4::new(r, g, b, 1.0);
                // if i < 3 {
                //     debug!("after  r={} g={} b={}", rgba.x, rgba.y, rgba.z);
                // }
                pixels[index + 0] = rgba.x;
                pixels[index + 1] = rgba.y;
                pixels[index + 2] = rgba.z;
            }
        }
        4 => {
            for i in 0..pixel_count {
                let index = i * (num_channels as usize);
                let r = pixels[index + 0];
                let g = pixels[index + 1];
                let b = pixels[index + 2];
                let a = pixels[index + 3];
                let rgba = matrix * na::Vector4::new(r, g, b, a);
                pixels[index + 0] = rgba.x;
                pixels[index + 1] = rgba.y;
                pixels[index + 2] = rgba.z;
                pixels[index + 3] = rgba.w;
            }
        }
        _ => panic!("apply matrix to {} channel image is not implemented."),
    };
}
