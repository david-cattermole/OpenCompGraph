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

use log::{debug, warn};

use crate::colorop::ColorOp;
use crate::coloroputils;
use crate::colorspace;
use crate::cxxbridge::ffi::BBox2Di;
use crate::cxxbridge::ffi::BakeOption;
use crate::cxxbridge::ffi::DataType;
use crate::cxxbridge::ffi::ImageSpec;
use crate::cxxbridge::ffi::Matrix4;
use crate::data::COLOR_SPACE_NAME_LINEAR;
use crate::deformutils;
use crate::ops;
use crate::pixelblock::pixelblock::PixelBlock;
use crate::stream::StreamDataImpl;

/// Convert pixels from color space to another color space.
pub fn do_process_colorspace(
    pixel_block: &mut PixelBlock,
    image_spec: &mut ImageSpec,
    from_color_space: &str,
    to_color_space: &str,
) {
    debug!(
        "ops::bake::do_process_colorspace(from_color_space={:#?}, to_color_space={:#?}) START",
        from_color_space, to_color_space
    );

    // Convert to f32 data.
    //
    // Before applying any changes to the pixels we must ensure we
    // work with 32-bit floating-point data.
    pixel_block.convert_into_data_type(DataType::Float32);

    if from_color_space == to_color_space {
        // Nothing to convert.
        return;
    }

    let width = pixel_block.width();
    let height = pixel_block.height();
    let num_channels = pixel_block.num_channels();

    let unassociated_alpha = image_spec.unassociated_alpha();
    let mut alpha_channel_index = -1;
    if num_channels > 3 {
        alpha_channel_index = 3;
    }

    let ok = colorspace::color_convert_inplace(
        &mut pixel_block.as_mut_slice_f32(),
        width,
        height,
        num_channels,
        alpha_channel_index,
        unassociated_alpha,
        from_color_space,
        to_color_space,
    );
    if ok {
        image_spec.set_color_space(to_color_space.to_string())
    } else {
        warn!(
            "Could not convert color space from \"{0}\" to \"{1}\" ",
            from_color_space, to_color_space
        )
    }

    debug!(
        "ops::bake::do_process_colorspace(from_color_space={:#?}, to_color_space={:#?}) END",
        from_color_space, to_color_space
    );
}

/// Convert pixels from color space to another color space.
pub fn do_process_colorgrade(
    pixel_block: &mut PixelBlock,
    color_matrix: Matrix4,
    color_ops: &Vec<Box<dyn ColorOp>>,
) {
    debug!("ops::bake::do_process_colorgrade() START",);

    // Apply Color Matrix (in linear color space)
    let num_channels = pixel_block.num_channels();
    let pixels = &mut pixel_block.as_mut_slice_f32();
    let color_matrix = color_matrix.to_na_matrix();
    ops::xformcolor::apply_color_matrix_inplace(pixels, num_channels, color_matrix);

    // Apply Color Operations (in linear color space)
    debug!("apply color operations: START");
    coloroputils::apply_color_ops_to_pixels(&color_ops, pixels, num_channels);
    debug!("apply color operations: END");

    debug!("ops::bake::do_process_colorgrade() END");
}

/// Convert pixels from color space to another color space.
pub fn do_process_stream_colorgrade(
    pixel_block: &mut PixelBlock,
    stream_data: &mut StreamDataImpl,
) {
    debug!("ops::bake::do_process_stream_colorgrade() START",);

    let color_matrix = stream_data.color_matrix();
    let color_ops = &stream_data.color_ops();
    do_process_colorgrade(pixel_block, color_matrix, color_ops);

    // Reset stream values
    stream_data.set_color_matrix(Matrix4::identity());
    stream_data.clear_color_ops();

    debug!("ops::bake::do_process_stream_colorgrade() END");
}

/// Apply transform matrix, deformations and color corrections before
/// image operations.
pub fn do_process(
    bake_option: BakeOption,
    pixel_block: &mut PixelBlock,
    display_window: BBox2Di,
    data_window: &mut BBox2Di,
    image_spec: &mut ImageSpec,
    stream_data: &mut StreamDataImpl,
    from_color_space: &str,
    to_color_space: &str,
    to_data_type: DataType,
) {
    debug!(
        "ops::bake::do_process(bake_option={:#?} from_color_space={:#?} to_color_space={:#?} pixel_data_type={:#?})",
        bake_option, from_color_space, to_color_space, to_data_type
    );

    // Use the input pixel data type if user didn't specify the data
    // type wanted on output.
    let to_data_type = match to_data_type {
        DataType::Unknown => pixel_block.data_type(),
        _ => to_data_type,
    };

    match bake_option {
        BakeOption::Nothing => {}
        BakeOption::ColorSpace => {
            do_process_colorspace(pixel_block, image_spec, &from_color_space, &to_color_space);

            // Convert to whatever output data type requested.
            pixel_block.convert_into_data_type(to_data_type);
        }
        BakeOption::ColorSpaceAndGrade => {
            // Convert from user color space to linear space.
            let linear_color_space = COLOR_SPACE_NAME_LINEAR;
            do_process_colorspace(
                pixel_block,
                image_spec,
                &from_color_space,
                &linear_color_space,
            );

            do_process_stream_colorgrade(pixel_block, stream_data);

            // Convert from Linear to user given color space.
            do_process_colorspace(
                pixel_block,
                image_spec,
                &linear_color_space,
                &to_color_space,
            );
            image_spec.set_color_space(to_color_space.to_string());

            // Convert to whatever output data type requested.
            pixel_block.convert_into_data_type(to_data_type);
        }
        BakeOption::All => {
            // Convert from user color space to linear space.
            debug!("apply PRE colorspace: START");
            let linear_color_space = COLOR_SPACE_NAME_LINEAR;
            do_process_colorspace(
                pixel_block,
                image_spec,
                &from_color_space,
                &linear_color_space,
            );
            debug!("apply PRE colorspace: END");

            do_process_stream_colorgrade(pixel_block, stream_data);

            // Apply Deformations.
            debug!("apply deformations: START");
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
            debug!("apply deformations: END");

            // Convert from Linear to user given color space.
            debug!("apply POST colorspace: START");
            do_process_colorspace(
                pixel_block,
                image_spec,
                &linear_color_space,
                &to_color_space,
            );
            image_spec.set_color_space(to_color_space.to_string());
            debug!("apply POST colorspace: END");

            // Convert to whatever output data type requested.
            pixel_block.convert_into_data_type(to_data_type);
        }
        _ => panic!("Unknown"),
    }
}
