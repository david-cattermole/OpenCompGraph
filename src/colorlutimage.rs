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

/// Generate a 3D cube texture for fast preview of colour
/// space conversions.
///
/// Popular resolutions are: 20x20x20, 32x32x32 or 64x64x64
//
use log::debug;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::rc::Rc;

use crate::cache::CacheImpl;
use crate::cache::CachedImage;
use crate::cxxbridge::ffi::BBox2Di;
use crate::cxxbridge::ffi::ImageShared;
use crate::cxxbridge::ffi::ImageSpec;
use crate::cxxbridge::ffi::PixelDataType;
use crate::ops::bake;
use crate::pixelblock::PixelBlock;

/// Compute the image width / height
fn get_3dlut_image_size(cube_size: i32) -> (i32, i32) {
    let width = cube_size * cube_size;
    let pixel_count: i32 = cube_size * cube_size * cube_size;
    let height = ((pixel_count as f32) / (width as f32)).ceil() as i32;
    (width, height)
}

/// Generate a 3D (cube) LUT that has no effect - just an identity 3D
/// LUT.
fn generate_identity_3dlut(img: &mut [f32], edge_length: i32, num_channels: i32) {
    assert!(num_channels > 2);
    let num_channels = num_channels as usize;
    let edge_length = edge_length as usize;

    let c: f32 = 1.0 / ((edge_length as f32) - 1.0);

    let pixel_count = edge_length * edge_length * edge_length;
    for i in 0..pixel_count {
        img[num_channels * i + 0] = (i % edge_length) as f32 * c;
        img[num_channels * i + 1] = ((i / edge_length) % edge_length) as f32 * c;
        img[num_channels * i + 2] = ((i / edge_length / edge_length) % edge_length) as f32 * c;
    }
}

/// Generate a new 3D LUT image that converts a an input color to an
/// output color.
fn generate_color_transform_3dlut(
    cube_size: i32,
    num_channels: i32,
    from_color_space: &str,
    to_color_space: &str,
) -> ImageShared {
    debug!("Generating Color Transform 3D LUT...");
    let (width, height) = get_3dlut_image_size(cube_size);

    let mut image_spec = ImageSpec::new();
    image_spec.set_color_space(from_color_space.to_string());

    let mut pixel_block_box = Box::new(PixelBlock::new(
        width,
        height,
        num_channels,
        PixelDataType::Float32,
    ));
    generate_identity_3dlut(&mut pixel_block_box.as_slice_mut(), cube_size, num_channels);

    // Transform the identity LUT into the color space.
    bake::do_process_colorspace(
        &mut pixel_block_box,
        &mut image_spec,
        from_color_space,
        to_color_space,
    );

    ImageShared {
        pixel_block: pixel_block_box,
        display_window: BBox2Di::new(0, 0, width, height),
        data_window: BBox2Di::new(0, 0, width, height),
        spec: image_spec,
    }
}

/// Look up the transform from the cache and return it, otherwise
/// generate the color transfrom from scratch (and store it in the
/// cache).
pub fn get_color_transform_3dlut(
    from_color_space: &str,
    to_color_space: &str,
    cube_size: i32,
    cache: &mut Box<CacheImpl>,
) -> ImageShared {
    let num_channels: i32 = 3;

    // Calculate the hash value.
    let mut hasher = DefaultHasher::new();
    hasher.write(b"3DLUT Size:");
    hasher.write_i32(cube_size);
    hasher.write(b"From:");
    hasher.write(from_color_space.as_bytes());
    hasher.write(b"To:");
    hasher.write(to_color_space.as_bytes());
    let hash_value = hasher.finish();

    match cache.get(&hash_value) {
        Some(cached_img) => {
            debug!("Color LUT Transform - Cache Hit");
            ImageShared {
                pixel_block: Box::new((*cached_img.pixel_block.clone()).clone()),
                spec: cached_img.spec.clone(),
                data_window: cached_img.data_window,
                display_window: cached_img.display_window,
            }
        }
        _ => {
            debug!("Color LUT Transform - Cache Miss");
            let img = generate_color_transform_3dlut(
                cube_size,
                num_channels,
                from_color_space,
                to_color_space,
            );
            let pixel_block_rc = Rc::new(*img.pixel_block.clone());
            let cached_img = CachedImage {
                pixel_block: pixel_block_rc.clone(),
                spec: img.spec.clone(),
                data_window: img.data_window,
                display_window: img.display_window,
            };
            cache.insert(hash_value, cached_img);
            img
        }
    }
}
