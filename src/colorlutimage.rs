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
use crate::colorop::ColorOp;
use crate::cxxbridge::ffi::BBox2Di;
use crate::cxxbridge::ffi::BlockSize;
use crate::cxxbridge::ffi::DataType;
use crate::cxxbridge::ffi::ImageShared;
use crate::cxxbridge::ffi::ImageSpec;
use crate::cxxbridge::ffi::Matrix4;
use crate::data::COLOR_SPACE_NAME_LINEAR;
use crate::ops::bake;
use crate::pixelblock::pixelblock::PixelBlock;
use crate::stream::StreamDataImplRc;

/// Compute the image width / height
fn get_3dlut_image_size(cube_size: i32, num_channels: i32) -> (i32, i32) {
    match num_channels {
        1 => {
            let width = cube_size;
            let height: i32 = 1;
            (width, height)
        }
        2 => {
            let width = cube_size;
            let height = cube_size;
            (width, height)
        }
        3 => {
            let width = cube_size * cube_size;
            let pixel_count: i32 = cube_size * cube_size * cube_size;
            let height = ((pixel_count as f32) / (width as f32)).ceil() as i32;
            (width, height)
        }
        _ => panic!("Unsupported LUT size."),
    }
}

/// Generate a 3D (cube) LUT that has no effect - just an identity 3D
/// LUT. Only supports 3D LUTs with 3 or 4 channels (RGB or RGBA).
fn generate_identity_3dlut(img: &mut [f32], edge_length: i32, num_channels: i32) {
    assert!(num_channels == 1 || num_channels == 3);
    let num_channels = num_channels as usize;
    let edge_length = edge_length as usize;

    let c: f32 = 1.0 / ((edge_length as f32) - 1.0);

    match num_channels {
        1 => {
            let pixel_count = edge_length;
            for i in 0..pixel_count {
                img[i] = (i % edge_length) as f32 * c;
            }
        }
        3 => {
            let pixel_count = edge_length * edge_length * edge_length;
            for i in 0..pixel_count {
                img[num_channels * i + 0] = (i % edge_length) as f32 * c;
                img[num_channels * i + 1] = ((i / edge_length) % edge_length) as f32 * c;
                img[num_channels * i + 2] =
                    ((i / edge_length / edge_length) % edge_length) as f32 * c;
            }
        }
        _ => panic!("Unsupported LUT size."),
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
    let (width, height) = get_3dlut_image_size(cube_size, num_channels);

    let mut image_spec = ImageSpec::new();
    image_spec.set_color_space(from_color_space.to_string());

    let blocksize = BlockSize::new(width, height, num_channels);
    let data_type = DataType::Float32;
    let mut pixel_block_box = Box::new(PixelBlock::new(blocksize, data_type));
    generate_identity_3dlut(
        &mut pixel_block_box.as_mut_slice_f32(),
        cube_size,
        num_channels,
    );

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

/// Generate a new 3D LUT image that converts a an input color to an
/// output color.
fn generate_color_ops_lut(
    cube_size: i32,
    num_channels: i32,
    color_ops: &Vec<Box<dyn ColorOp>>,
) -> ImageShared {
    debug!("Generating Color Ops 3D LUT...");
    let (width, height) = get_3dlut_image_size(cube_size, num_channels);
    debug!("size: {} x {}", width, height);

    let mut image_spec = ImageSpec::new();
    image_spec.set_color_space(COLOR_SPACE_NAME_LINEAR.to_string());

    let blocksize = BlockSize::new(width, height, num_channels);
    let data_type = DataType::Float32;
    let mut pixel_block_box = Box::new(PixelBlock::new(blocksize, data_type));
    generate_identity_3dlut(
        &mut pixel_block_box.as_mut_slice_f32(),
        cube_size,
        num_channels,
    );

    // Transform the identity LUT into the color space.
    let color_matrix = Matrix4::identity();
    bake::do_process_colorgrade(&mut pixel_block_box, color_matrix, color_ops);

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
pub fn get_color_ops_lut(
    stream_data: &StreamDataImplRc,
    cube_size: i32,
    num_channels: i32,
    cache: &mut Box<CacheImpl>,
) -> ImageShared {
    debug!(
        "colorlutimage::get_color_ops_lut: cube_size={} num_channels={}",
        cube_size, num_channels
    );

    let mut hasher = DefaultHasher::new();
    hasher.write(b"Color Ops LUT");
    hasher.write(b"LUT Size:");
    hasher.write_i32(cube_size);
    hasher.write(b"Number of channels:");
    hasher.write_i32(num_channels);
    hasher.write(b"Color Ops Hash");
    let color_ops_hash = stream_data.color_ops_hash();
    hasher.write_u64(color_ops_hash);
    let hash_value = hasher.finish();

    match cache.get(&hash_value) {
        Some(cached_img) => {
            debug!("Color Operations LUT - Cache Hit");
            ImageShared {
                pixel_block: Box::new((*cached_img.pixel_block.clone()).clone()),
                spec: cached_img.spec.clone(),
                data_window: cached_img.data_window,
                display_window: cached_img.display_window,
            }
        }
        _ => {
            debug!("Color Operations LUT - Cache Miss");
            let color_ops = stream_data.color_ops();
            let img = generate_color_ops_lut(cube_size, num_channels, color_ops);
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
