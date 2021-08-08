/*
 * Copyright (C) 2021 David Cattermole.
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

use crate::cxxbridge::ffi::Vector4f32;
use crate::cxxbridge::ffi::Vector4i32;

fn grade_single(
    pixel: f32,
    black_point: f32,
    white_point: f32,
    lift: f32,
    gain: f32,
    multiply: f32,
    offset: f32,
    gamma: f32,
    reverse: bool,
    clamp_black: bool,
    clamp_white: bool,
    premult: bool,
) -> f32 {
    let mut a = multiply * (gain - lift) / (white_point - black_point);
    let mut b = offset + lift - a * black_point;
    if reverse == false {
        (a * pixel + b).powf(1.0 / gamma)
    } else {
        // Reverse gamma.
        let mut value = if gamma <= 0.0 {
            if pixel > 0.0 {
                1.0
            } else {
                0.0
            }
        } else if gamma != 1.0 {
            let mut v = pixel;
            if (v <= 0.0) { // v = 0.0;
            } else if (v < 1.0) {
                v = v.powf(gamma);
            } else {
                v = 1.0 + (v - 1.0) * gamma;
            }
            v
        } else {
            pixel
        };

        // Reverse the linear part.
        if (a != 1.0) || (b != 0.0) {
            if a != 0.0 {
                a = 1.0 / a;
            } else {
                a = 1.0;
            }
            b = -b * a;
            value = pixel * a + b;
        }
        value
    }
}

fn grade(
    pixel: Vector4f32,
    mask: Vector4f32,
    black_point: Vector4f32,
    white_point: Vector4f32,
    lift: Vector4f32,
    gain: Vector4f32,
    multiply: Vector4f32,
    offset: Vector4f32,
    gamma: Vector4f32,
    reverse: bool,
    clamp_black: bool,
    clamp_white: bool,
    premult: bool,
) -> Vector4f32 {
    let r = grade_single(
        pixel.x,
        black_point.x,
        white_point.x,
        lift.x,
        gain.x,
        multiply.x,
        offset.x,
        gamma.x,
        reverse,
        clamp_black,
        clamp_white,
        premult,
    );
    let g = grade_single(
        pixel.y,
        black_point.y,
        white_point.y,
        lift.y,
        gain.y,
        multiply.y,
        offset.y,
        gamma.y,
        reverse,
        clamp_black,
        clamp_white,
        premult,
    );
    let b = grade_single(
        pixel.z,
        black_point.z,
        white_point.z,
        lift.z,
        gain.z,
        multiply.z,
        offset.z,
        gamma.z,
        reverse,
        clamp_black,
        clamp_white,
        premult,
    );
    let a = grade_single(
        pixel.w,
        black_point.w,
        white_point.w,
        lift.w,
        gain.w,
        multiply.w,
        offset.w,
        gamma.w,
        reverse,
        clamp_black,
        clamp_white,
        premult,
    );
    Vector4f32::new(
        (r * mask.x) + (pixel.x * (1.0 - mask.x)),
        (g * mask.y) + (pixel.y * (1.0 - mask.y)),
        (b * mask.z) + (pixel.z * (1.0 - mask.z)),
        (a * mask.w) + (pixel.w * (1.0 - mask.w)),
    )
}

pub fn apply_color_grade_inplace(
    pixels: &mut [f32],
    num_channels: i32,
    process: Vector4i32,
    black_point: Vector4f32,
    white_point: Vector4f32,
    lift: Vector4f32,
    gain: Vector4f32,
    multiply: Vector4f32,
    offset: Vector4f32,
    gamma: Vector4f32,
    reverse: bool,
    clamp_black: bool,
    clamp_white: bool,
    premult: bool,
) {
    let pixel_mask = Vector4f32::new(
        process.x as f32,
        process.y as f32,
        process.z as f32,
        process.w as f32,
    );

    let pixel_count = pixels.len() / (num_channels as usize);
    match num_channels {
        3 => {
            for i in 0..pixel_count {
                let index = i * (num_channels as usize);
                let r = pixels[index + 0];
                let g = pixels[index + 1];
                let b = pixels[index + 2];

                let pixel = Vector4f32::new(r, g, b, 1.0);
                let rgba = grade(
                    pixel,
                    pixel_mask,
                    black_point,
                    white_point,
                    lift,
                    gain,
                    multiply,
                    offset,
                    gamma,
                    reverse,
                    clamp_black,
                    clamp_white,
                    premult,
                );

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

                let pixel = Vector4f32::new(r, g, b, a);
                let rgba = grade(
                    pixel,
                    pixel_mask,
                    black_point,
                    white_point,
                    lift,
                    gain,
                    multiply,
                    offset,
                    gamma,
                    reverse,
                    clamp_black,
                    clamp_white,
                    premult,
                );

                pixels[index + 0] = rgba.x;
                pixels[index + 1] = rgba.y;
                pixels[index + 2] = rgba.z;
                pixels[index + 3] = rgba.w;
            }
        }
        _ => panic!(
            "apply colorgrade to {} channel image is not implemented.",
            num_channels
        ),
    };
}
