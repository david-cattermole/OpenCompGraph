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

use crate::cxxbridge::ffi::BBox2Df;
use crate::cxxbridge::ffi::BBox2Di;
use crate::deformer::Deformer;
use crate::ops;
use crate::pixelblock::PixelBlock;

pub fn apply_deformers_to_positions(
    deformers: &Vec<Box<dyn Deformer>>,
    _display_window: BBox2Df,
    data_window: BBox2Df,
    buffer: &mut [f32],
) {
    if deformers.len() == 0 {
        return;
    }
    let enabled_deformers: Vec<_> = deformers
        .iter()
        .filter(|x| x.get_attr_i32("enable") == 1)
        .collect();
    if enabled_deformers.len() == 0 {
        return;
    }

    let stride = 3;
    let inverse = false;
    for deformer in enabled_deformers {
        deformer.apply_slice_in_place(buffer, data_window, inverse, stride);
    }
}

pub fn apply_deformers_to_pixels(
    deformers: &Vec<Box<dyn Deformer>>,
    display_window: BBox2Di,
    src_pixel_block: &PixelBlock,
    src_data_window: BBox2Di,
    dst_pixel_block: &mut PixelBlock,
    dst_data_window: &mut BBox2Di,
) {
    // TODO: Check if all deformers are 'uniform' (same deformation
    // for all points - can be simplified to a matrix), then compute
    // the matrix and perform the computation once.

    if deformers.len() == 0 {
        // Copy src to dst.
        *dst_pixel_block = src_pixel_block.clone();
        *dst_data_window = src_data_window.clone();
        return;
    }
    let enabled_deformers: Vec<_> = deformers
        .iter()
        .filter(|x| x.get_attr_i32("enable") == 1)
        .collect();
    if enabled_deformers.len() == 0 {
        // Copy src to dst.
        *dst_pixel_block = src_pixel_block.clone();
        *dst_data_window = src_data_window.clone();
        return;
    }

    // Get the bounding box of the pixel_coords.
    let display_window_f32 = BBox2Df::from(display_window);
    let mut tmp_data_window = BBox2Df::from(src_data_window);
    for deformer in &enabled_deformers {
        tmp_data_window = deformer.apply_bounding_box(tmp_data_window, display_window_f32);
    }

    dst_data_window.min_x = tmp_data_window.min_x.floor() as i32;
    dst_data_window.min_y = tmp_data_window.min_y.floor() as i32;
    dst_data_window.max_x = tmp_data_window.max_x.ceil() as i32;
    dst_data_window.max_y = tmp_data_window.max_y.ceil() as i32;

    dst_pixel_block.data_resize(
        dst_data_window.width(),
        dst_data_window.height(),
        src_pixel_block.num_channels(),
        src_pixel_block.pixel_data_type(),
    );

    let dst_width = dst_pixel_block.width();
    let dst_height = dst_pixel_block.height();
    let dst_num_pixels = dst_width as usize * dst_height as usize;

    // Fill pixel coordinates with identity input values.
    let stride = 2 as usize;
    let mut pixel_coords = vec![0.0 as f32; dst_num_pixels * stride];
    let mut index = 0;
    for row in dst_data_window.min_y..dst_data_window.max_y {
        for col in dst_data_window.min_x..dst_data_window.max_x {
            // Position of pixel, with lower-left=(0.0, 0.0), and with
            // upper-right=(width-1, height-1), then include a
            // half-pixel offset, so we sample the lens center of the
            // pixel, rather than the edge.
            let pixel_x = (col as f32) + 0.5;
            let pixel_y = (row as f32) + 0.5;
            pixel_coords[index + 0] = pixel_x;
            pixel_coords[index + 1] = pixel_y;
            index += stride;
        }
    }

    // Apply deformers to 2D position buffer.
    let stride = 2;
    // Calculate the distortion in the opposite direction to what the
    // user has requested, so that we can map the destination to
    // source.
    let inverse = true;
    for deformer in &enabled_deformers {
        let buffer = pixel_coords.as_mut_slice();
        deformer.apply_slice_in_place(buffer, display_window_f32, inverse, stride);
    }

    // Sample the image pixels with the position values.
    ops::pixelremap::pixels_remap_coords(
        display_window,
        &pixel_coords, // pixel coordinates are relative to 'display window'.
        &src_pixel_block,
        src_data_window,
        dst_pixel_block,
        dst_data_window,
    )
}
