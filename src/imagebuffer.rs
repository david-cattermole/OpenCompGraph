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
use log::{debug, log_enabled, Level};

use crate::colorutils::convert_linear_to_srgb;
use crate::cxxbridge::ffi::ImageShared;

pub fn create_image_buffer_rgb_u8(
    image: &ImageShared,
) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let pixel_block = &image.pixel_block;
    let width = pixel_block.width();
    let height = pixel_block.height();
    let pixels = &pixel_block.as_slice();

    // Get pixel statistics
    if log_enabled!(Level::Debug) {
        let min = pixels.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let max = pixels.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        let avg = pixels.iter().sum::<f32>() / (pixels.len() as f32);
        debug!("Min value: {}", min);
        debug!("Max value: {}", max);
        debug!("Avg value: {}", avg);
    }

    // Convert f32 pixel image to u8 ImageBuffer.
    let pixels_u8: Vec<u8> = pixels
        .iter()
        .map(|x| (convert_linear_to_srgb(*x as f32) * (u8::max_value() as f32)) as u8)
        .collect();

    match image::ImageBuffer::from_raw(width as u32, height as u32, pixels_u8) {
        Some(data) => data,
        _ => panic!("invalid image."),
    }
}

pub fn create_image_buffer_rgba_u8(
    image: &ImageShared,
) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    let pixel_block = &image.pixel_block;
    let width = pixel_block.width();
    let height = pixel_block.height();
    let pixels = &pixel_block.as_slice();

    // Get pixel statistics
    if log_enabled!(Level::Debug) {
        let min = pixels.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let max = pixels.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        let avg = pixels.iter().sum::<f32>() / (pixels.len() as f32);
        debug!("Min value: {}", min);
        debug!("Max value: {}", max);
        debug!("Avg value: {}", avg);
    }

    // Convert f32 pixel image to u8 ImageBuffer.
    let pixels_u8: Vec<u8> = pixels
        .iter()
        .map(|x| (convert_linear_to_srgb(*x as f32) * (u8::max_value() as f32)) as u8)
        .collect();

    debug!(
        "RGBA u8 width={} height={} num_channels={} pixels.size()={}",
        width,
        height,
        pixel_block.num_channels(),
        pixels_u8.len()
    );
    match image::ImageBuffer::from_raw(width as u32, height as u32, pixels_u8) {
        Some(data) => data,
        _ => panic!("invalid image."),
    }
}
