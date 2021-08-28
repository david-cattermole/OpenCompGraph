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

use log::debug;
use std::rc::Rc;

use crate::cxxbridge::ffi::BBox2Di;
use crate::cxxbridge::ffi::ImageShared;
use crate::cxxbridge::ffi::ImageSpec;
use crate::cxxbridge::ffi::PixelDataType;
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

    let input = &inputs[0].clone();
    let input_pixel_block = input.clone_pixel_block();

    // let metadata = create_metadata_shared();
    let mut image_out = ImageShared {
        pixel_block: Box::new(input_pixel_block),
        display_window: input.display_window(),
        data_window: input.data_window(),
        spec: ImageSpec::new(),
    };

    let _ok = crop_image_in_place(
        &mut image_out,
        crop_window,
        reformat,
        black_outside,
        _intersect,
    );

    image_out
}

pub fn crop_image_in_place(
    image: &mut ImageShared,
    crop_window: BBox2Di,
    reformat: bool,
    black_outside: bool,
    // TODO: Implement 'intersect'.
    //
    // When enabled, the output bounding box is an intersection of the
    // crop bounding box and the incoming bounding box.
    //
    // When disabled, the output bounding box matches the crop
    // bounding box and can extend outside the incoming bounding box.
    //
    _intersect: bool,
) -> bool {
    let src_pixel_block = &mut *image.pixel_block;
    let src_display_window = image.display_window;
    let src_data_window = image.data_window;

    debug!("Crop Window: {:?}", crop_window);
    debug!("Source Display Window: {:?}", src_display_window);
    debug!("Source Data Window: {:?}", src_data_window);

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

    let new_display_window = match reformat {
        false => src_display_window,
        true => crop_window,
    };

    // TODO: Remove the clone once pixel_data_types other than f32 are
    // supported.
    src_pixel_block.convert_into_pixel_data_type(PixelDataType::Float32);
    let pixel_block_box = Box::new(PixelBlock::from_pixel_block(
        &src_pixel_block,
        src_data_window,
        crop_window,
    ));

    debug!("Display Window: {:?}", new_display_window);
    debug!("Data Window: {:?}", new_data_window);
    debug!(
        "PixelBlock (WxH): {}x{}",
        pixel_block_box.width(),
        pixel_block_box.height()
    );

    image.pixel_block = pixel_block_box;
    image.display_window = new_display_window;
    image.data_window = new_data_window;
    assert!(image.data_window.width() == image.pixel_block.width());
    assert!(image.data_window.height() == image.pixel_block.height());

    true
}
