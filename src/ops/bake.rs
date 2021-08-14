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

use log::warn;
use std::string::String;

use crate::colorspace;
use crate::cxxbridge::ffi::BBox2Di;
use crate::cxxbridge::ffi::BakeOption;
use crate::cxxbridge::ffi::ImageSpec;
use crate::cxxbridge::ffi::Matrix4;
use crate::cxxbridge::ffi::Vector4f32;
use crate::deformer::Deformer;
use crate::deformutils;
use crate::ops;
use crate::pixelblock::PixelBlock;
use crate::stream::StreamDataImpl;

// Convert from user color space to linear space.
fn do_process_colorspace(
    pixel_block: &mut PixelBlock,
    image_spec: &mut ImageSpec,
    from_color_space: &String,
    to_color_space: &String,
) {
    let width = pixel_block.width();
    let height = pixel_block.height();
    let num_channels = pixel_block.num_channels();

    let unassociated_alpha = image_spec.unassociated_alpha();
    let mut alpha_channel_index = -1;
    if num_channels > 3 {
        alpha_channel_index = 3;
    }

    // Convert to f32 data and convert to linear color space.
    //
    // Before applying any changes to the pixels we must ensure we
    // work with 32-bit floating-point data.
    pixel_block.convert_into_f32_data();

    let ok = colorspace::color_convert_inplace(
        &mut pixel_block.as_slice_mut(),
        width,
        height,
        num_channels,
        alpha_channel_index,
        unassociated_alpha,
        &from_color_space,
        &to_color_space,
    );
    if ok {
        image_spec.set_color_space(to_color_space.clone())
    } else {
        warn!(
            "Could not convert color space from \"{0}\" to \"{1}\" ",
            from_color_space, to_color_space
        )
    }
}

// Apply transform matrix, deformations and color corrections before
// image operations.
pub fn do_process(
    bake_option: BakeOption,
    pixel_block: &mut PixelBlock,
    display_window: BBox2Di,
    data_window: &mut BBox2Di,
    image_spec: &mut ImageSpec,
    stream_data: &mut StreamDataImpl,
    from_color_space: &String,
    to_color_space: &String,
) {
    match bake_option {
        // BakeOption::Nothing => panic!("Nothing"),
        BakeOption::ColorSpace => {
            do_process_colorspace(pixel_block, image_spec, &from_color_space, &to_color_space)
        }
        BakeOption::ColorSpaceAndGrade => {
            // Convert from user color space to linear space.
            let linear_color_space = "Linear".to_string();
            do_process_colorspace(
                pixel_block,
                image_spec,
                &from_color_space,
                &linear_color_space,
            );

            // Apply Color Matrix (in linear color space)
            let num_channels = pixel_block.num_channels();
            let pixels = &mut pixel_block.as_slice_mut();
            let color_matrix = stream_data.color_matrix().to_na_matrix();
            ops::xformcolor::apply_color_matrix_inplace(pixels, num_channels, color_matrix);
            stream_data.set_color_matrix(Matrix4::identity());

            // Convert from Linear to user given color space.
            do_process_colorspace(
                pixel_block,
                image_spec,
                &linear_color_space,
                &to_color_space,
            );
            image_spec.set_color_space(to_color_space.clone());
        }
        BakeOption::All => {
            // Convert from user color space to linear space.
            let linear_color_space = "Linear".to_string();
            do_process_colorspace(
                pixel_block,
                image_spec,
                &from_color_space,
                &linear_color_space,
            );

            // Apply Color Matrix (in linear color space)
            let num_channels = pixel_block.num_channels();
            let pixels = &mut pixel_block.as_slice_mut();
            let color_matrix = stream_data.color_matrix().to_na_matrix();
            ops::xformcolor::apply_color_matrix_inplace(pixels, num_channels, color_matrix);
            stream_data.set_color_matrix(Matrix4::identity());

            // Apply Deformations.
            let src_data_window = data_window.clone();
            let ref_pixel_block = pixel_block.clone();
            let deformers = &stream_data.deformers();
            deformutils::apply_deformers_to_pixels(
                &deformers,
                display_window,
                &ref_pixel_block,
                src_data_window,
                pixel_block,
                data_window,
            );
            stream_data.clear_deformers();

            // Convert from Linear to user given color space.
            do_process_colorspace(
                pixel_block,
                image_spec,
                &linear_color_space,
                &to_color_space,
            );
            image_spec.set_color_space(to_color_space.clone());
        }
        _ => panic!("Unknown"),
    }
}
