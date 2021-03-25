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

use crate::pixelblock::get_pixel_index;

// This assumes the 'pixels' have RGBA channels.
#[inline]
pub fn get_pixel_rgba(
    pixels: &[f32],
    width: i32,
    height: i32,
    x_stride: i32,
    y_stride: i32,
    x: f32,
    y: f32,
) -> (f32, f32, f32, f32) {
    // Clamp the pixel dimensions to the image.
    let sx = (x.round() as i32).max(0).min(width - 1);
    let sy = (y.round() as i32).max(0).min(height - 1);

    let index = get_pixel_index(width, height, x_stride, y_stride, sx, sy);

    // TODO: Since we clamp the pixel dimensions it should be
    // impossible to have an incorrect index.
    let mut out_r = 0.0;
    let mut out_g = 0.0;
    let mut out_b = 0.0;
    let mut out_a = 0.0;
    if index >= 0 {
        let i = index as usize;
        out_r = pixels[i + 0];
        out_g = pixels[i + 1];
        out_b = pixels[i + 2];
        out_a = pixels[i + 3];
    }

    return (out_r, out_g, out_b, out_a);
}

// This assumes the 'pixels' have RGB channels.
#[inline]
pub fn get_pixel_rgb(
    pixels: &[f32],
    width: i32,
    height: i32,
    x_stride: i32,
    y_stride: i32,
    x: f32,
    y: f32,
) -> (f32, f32, f32) {
    // Clamp the pixel dimensions to the image.
    let sx = (x.round() as i32).max(0).min(width - 1);
    let sy = (y.round() as i32).max(0).min(height - 1);

    let index = get_pixel_index(width, height, x_stride, y_stride, sx, sy);

    let mut out_r = 0.0;
    let mut out_g = 0.0;
    let mut out_b = 0.0;
    if index >= 0 {
        let i = index as usize;
        out_r = pixels[i + 0];
        out_g = pixels[i + 1];
        out_b = pixels[i + 2];
    }

    return (out_r, out_g, out_b);
}
