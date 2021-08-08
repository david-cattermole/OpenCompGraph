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

pub fn apply_color_matrix_inplace(
    pixels: &mut [f32],
    num_channels: i32,
    matrix: na::Matrix4<f32>,
) {
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
        _ => panic!(
            "apply matrix to {} channel image is not implemented.",
            num_channels
        ),
    };
}
