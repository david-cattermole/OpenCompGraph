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

use std::string::String;

use crate::colorspace;
use crate::cxxbridge::ffi::BBox2Di;
use crate::cxxbridge::ffi::Matrix4;
use crate::deformer::Deformer;
use crate::deformutils;
use crate::ops;
use crate::pixelblock::PixelBlock;

pub fn do_process(
    pixel_block: &mut PixelBlock,
    display_window: BBox2Di,
    data_window: &mut BBox2Di,
    deformers: &Vec<Box<dyn Deformer>>,
    color_matrix: Matrix4,
    from_color_space: String,
    to_color_space: String,
) {
    {
        // Convert to f32 data and convert to linear color space.
        //
        // Before applying any changes to the pixels we
        // must ensure we work in 32-bit floating-point linear
        // color space.
        pixel_block.convert_into_f32_data();
        let width = pixel_block.width();
        let height = pixel_block.height();
        let num_channels = pixel_block.num_channels();
        colorspace::color_convert_inplace(
            &mut pixel_block.as_slice_mut(),
            width,
            height,
            num_channels,
            &from_color_space,
            &to_color_space,
        );
    }

    {
        // Apply Deformations.
        let src_data_window = data_window.clone();
        let ref_pixel_block = pixel_block.clone();
        deformutils::apply_deformers_to_pixels(
            &deformers,
            display_window,
            &ref_pixel_block,
            src_data_window,
            pixel_block,
            data_window,
        );
    }

    // Apply Color Matrix
    {
        let num_channels = pixel_block.num_channels();
        let pixels = &mut pixel_block.as_slice_mut();
        ops::xformcolor::apply_color_matrix_inplace(
            pixels,
            num_channels,
            color_matrix.to_na_matrix(),
        );
    }
}
