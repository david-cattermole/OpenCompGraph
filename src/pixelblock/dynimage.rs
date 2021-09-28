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

use image;
use image::GenericImageView;
use log::{debug, log_enabled, Level};

use crate::colorutils::convert_srgb_to_linear;
use crate::cxxbridge::ffi::BlockSize;
use crate::pixelblock::datablock::DataBlock;
use crate::pixelblock::pixelblock::PixelBlock;

/// Consumes a image::DynamicImage and produces a PixelBlock.
//
// TODO: This function should return a newly created 'ImageShared'.
//
pub fn from_dynamic_image(img: image::DynamicImage) -> PixelBlock {
    let (_width, _height) = img.dimensions();
    let width = _width as i32;
    let height = _height as i32;
    let color_type = img.color();
    debug!("Resolution: {:?}x{:?}", width, height);
    debug!("Color Type: {:?}", color_type);
    let num_channels = match color_type {
        image::ColorType::Rgb8 => 3,
        image::ColorType::Rgba8 => 4,
        _ => 0,
    };
    debug!("Num Channels: {:?}", num_channels);

    // Convert the image to f32 "flat sample" values
    let flat_samples = match color_type {
        image::ColorType::Rgb8 => {
            let rgb_img = img.into_rgb8();
            rgb_img.into_flat_samples()
        }
        image::ColorType::Rgba8 => {
            let rgba_img = img.into_rgba8();
            rgba_img.into_flat_samples()
        }
        _ => panic!("unsupported color type: {:#?}", color_type),
    };
    // TODO: skip converting the alpha channel, because it is not
    // stored in sRGB. We can use the "chunks" iterator to work on
    // 4 channels at once, then only modify the RGB channels.
    let pixels: Vec<f32> = flat_samples
        .as_slice()
        .into_iter()
        .map(|x| convert_srgb_to_linear((*x as f32) / (u8::max_value() as f32)))
        .collect();

    // Get pixel statistics
    if log_enabled!(Level::Debug) {
        let min = pixels.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let max = pixels.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        let avg = pixels.iter().sum::<f32>() / (pixels.len() as f32);
        debug!("Min value: {}", min);
        debug!("Max value: {}", max);
        debug!("Avg value: {}", avg);
    }

    let blocksize = BlockSize::new(width, height, num_channels);
    let datablock = DataBlock::from_slice_f32(&pixels[..]);
    PixelBlock::from_datablock(blocksize, datablock)
}
