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

use std::rc::Rc;

use crate::cxxbridge::ffi::BBox2Di;
use crate::cxxbridge::ffi::ImageShared;
use crate::pixelblock::PixelBlock;
use crate::stream::StreamDataImpl;

pub fn do_image_process(
    inputs: &Vec<Rc<StreamDataImpl>>,
    crop_window: BBox2Di,
    reformat: bool,
    black_outside: bool,
    _intersect: bool,
) -> ImageShared {
    debug_assert!(inputs.len() == 1);

    let new_data_window = match black_outside {
        true => BBox2Di::new(
            // Add extra edge of pixels around the edge of the image.
            crop_window.min_x - 1,
            crop_window.min_y - 1,
            crop_window.max_x + 1,
            crop_window.max_y + 1,
        ),
        false => BBox2Di::new(
            crop_window.min_x,
            crop_window.min_y,
            crop_window.max_x,
            crop_window.max_y,
        ),
    };

    let input = &inputs[0].clone();
    let new_display_window = match reformat {
        false => input.display_window(),
        true => crop_window,
    };

    // TODO: Remove the clone once pixel_data_types other than f32 are
    // supported.
    let mut src_pixel_block = input.clone_pixel_block();
    src_pixel_block.convert_into_f32_data();
    let pixel_block_box = Box::new(PixelBlock::from_pixel_block(
        &src_pixel_block,
        input.data_window(),
        crop_window,
    ));

    let image_out = ImageShared {
        pixel_block: pixel_block_box,
        display_window: new_display_window,
        data_window: new_data_window,
    };

    image_out
}
