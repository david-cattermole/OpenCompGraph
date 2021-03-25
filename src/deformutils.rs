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

use log::{debug, error, info, warn};

use crate::cxxbridge::ffi::BBox2Df;
use crate::cxxbridge::ffi::BBox2Di;
use crate::cxxbridge::ffi::DeformerDirection;
use crate::deformer::brownian;
use crate::deformer::Deformer;
use crate::ops;
use crate::pixel::get_pixel_rgb;
use crate::pixel::get_pixel_rgba;
use crate::pixelblock::PixelBlock;

pub fn apply_deformers_to_positions(
    deformers: &Vec<Box<dyn Deformer>>,
    direction: DeformerDirection,
    image_window: BBox2Df,
    buffer: &mut [f32],
) {
    if deformers.len() == 0 {
        // TODO: Copy src to dst.
        return;
    }
    let enabled_deformers: Vec<_> = deformers
        .iter()
        .filter(|x| x.get_attr_i32("enable") == 1)
        .collect();
    if enabled_deformers.len() == 0 {
        // TODO: Copy src to dst.
        return;
    }


    let stride = 3;
    match direction {
        DeformerDirection::Forward => {
            for deformer in enabled_deformers {
                deformer.apply_forward_slice_in_place(buffer, image_window, stride);
            }
        }
        DeformerDirection::Backward => {
            for deformer in enabled_deformers {
                deformer.apply_backward_slice_in_place(buffer, image_window, stride);
            }
        }
        _ => panic!("Invalid deformer direction: {:#?}", direction),
    }
}

pub fn apply_deformers_to_pixels(
    deformers: &Vec<Box<dyn Deformer>>,
    direction: DeformerDirection,
    display_window: BBox2Di,
    src_pixel_block: &PixelBlock,
    src_data_window: BBox2Di,
    dst_pixel_block: &mut PixelBlock,
    dst_data_window: &mut BBox2Di,
) {
    let display_window_f32 = BBox2Df::from(display_window);

    if deformers.len() == 0 {
        // TODO: Copy src to dst.
        return;
    }
    let enabled_deformers: Vec<_> = deformers
        .iter()
        .filter(|x| x.get_attr_i32("enable") == 1)
        .collect();
    if enabled_deformers.len() == 0 {
        // TODO: Copy src to dst.
        return;
    }

    // Get the bounding box of the pixel_coords.
    let display_window_f32 = BBox2Df::from(display_window);
    let mut tmp_data_window = BBox2Df::from(src_data_window);
    match direction {
        DeformerDirection::Forward => {
            for deformer in &enabled_deformers {
                tmp_data_window =
                    deformer.apply_forward_bounding_box(tmp_data_window, display_window_f32);
            }
        }
        DeformerDirection::Backward => {
            for deformer in &enabled_deformers {
                tmp_data_window =
                    deformer.apply_backward_bounding_box(tmp_data_window, display_window_f32);
            }
        }
        _ => panic!("Incorrect deformer direction; {:?}", direction),
    }

    let mut dst_data_window = BBox2Di::new(
        tmp_data_window.min_x.floor() as i32,
        tmp_data_window.min_y.floor() as i32,
        tmp_data_window.max_x.ceil() as i32,
        tmp_data_window.max_y.ceil() as i32,
    );
    dst_pixel_block.data_resize(
        dst_data_window.width(),
        dst_data_window.height(),
        src_pixel_block.num_channels(),
        src_pixel_block.pixel_data_type(),
    );

    let src_width = src_pixel_block.width();
    let src_height = src_pixel_block.height();
    let src_num_pixels = src_width as usize * src_height as usize;

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

    match direction {
        // Calculate the distortion in the opposite direction to what
        // the user has requested, so that we can map the destination
        // to source.
        DeformerDirection::Forward => {
            for deformer in &enabled_deformers {
                let mut buffer = pixel_coords.as_mut_slice();
                deformer.apply_backward_slice_in_place(buffer, display_window_f32, stride);
            }
        }
        DeformerDirection::Backward => {
            for deformer in &enabled_deformers {
                let mut buffer = pixel_coords.as_mut_slice();
                deformer.apply_forward_slice_in_place(buffer, display_window_f32, stride);
            }
        }
        _ => panic!("Incorrect deformer direction; {:?}", direction),
    }

    // Sample the image pixels with the position values.
    ops::pixelremap::pixels_remap_coords(
        display_window,
        &pixel_coords, // pixel coordinates are relative to 'display window'.
        &src_pixel_block,
        src_data_window,
        dst_pixel_block,
        &mut dst_data_window,
    )
}

