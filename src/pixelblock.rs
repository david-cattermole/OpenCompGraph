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

use half::f16;
use image;
use image::GenericImageView;
use log::{debug, error, log_enabled, Level};
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::colorutils::convert_srgb_to_linear;
use crate::cxxbridge::ffi::BBox2Di;
use crate::cxxbridge::ffi::PixelDataType;
use crate::data::COLOR_BARS;
use crate::data::COLOR_BARS_HEIGHT;
use crate::data::COLOR_BARS_NUM_CHANNELS;
use crate::data::COLOR_BARS_WIDTH;

#[inline]
fn size_bytes_aligned_to<T>(size_bytes: usize) -> usize {
    let align_type = std::mem::align_of::<T>();
    let extra_align_bytes = align_type - size_bytes.rem_euclid(align_type).rem_euclid(align_type);
    let size_bytes_aligned = size_bytes + extra_align_bytes;
    size_bytes_aligned
}

#[inline]
fn size_aligned_to<T>(size_bytes: usize) -> usize {
    let align_type = std::mem::align_of::<T>();
    let size = size_bytes_aligned_to::<T>(size_bytes) / align_type;
    size
}

#[inline]
pub fn stride_num_channels(num_channels: i32, pixel_data_type: PixelDataType) -> usize {
    match pixel_data_type {
        // Force all 8-bit image pixels to be 4-byte aligned. Add
        // an extra channel for RGB images.
        //
        // 8 + 8 + 8 + 8 = 32 bytes per-pixel (aligns to 4-bytes)
        PixelDataType::UInt8 => 4,

        // Force all 16-bit image pixels to be 4-byte aligned. Add
        // an extra channel for RGB images.
        //
        // 16 + 16 + 16 + 16 = 64 bytes per-pixel (aligns to 4-bytes)
        PixelDataType::Half16 => 4,
        PixelDataType::UInt16 => 4,

        // A 32-bit-per-channel image pixel is always 4-byte aligned,
        // so we can handle either 3 or 4 channels without any
        // padding.
        //
        // 32 + 32 + 32 + 32 = 128 bytes per-pixel (aligns to 4-bytes)
        // 32 + 32 + 32      = 96 bytes per-pixel (aligns to 4-bytes)
        PixelDataType::Float32 => num_channels as usize,
        _ => panic!("Invalid PixelDataType: {:?}", pixel_data_type),
    }
}

#[inline]
pub fn channel_size_bytes(pixel_data_type: PixelDataType) -> usize {
    let num_bytes = match pixel_data_type {
        PixelDataType::UInt8 => std::mem::size_of::<u8>(),
        PixelDataType::Half16 => std::mem::size_of::<f16>(),
        PixelDataType::Float32 => std::mem::size_of::<f32>(),
        PixelDataType::UInt16 => std::mem::size_of::<u16>(),
        _ => {
            error!("Invalid PixelDataType: {:?}", pixel_data_type);
            0
        }
    };
    num_bytes
}

#[inline]
fn pixel_block_size(
    width: i32,
    height: i32,
    num_channels: i32,
    pixel_data_type: PixelDataType,
) -> usize {
    let stride_num_channels: usize = stride_num_channels(num_channels, pixel_data_type);
    (width * height) as usize * stride_num_channels
}

#[inline]
fn pixel_block_size_bytes(
    width: i32,
    height: i32,
    num_channels: i32,
    pixel_data_type: PixelDataType,
) -> usize {
    let num_bytes = channel_size_bytes(pixel_data_type);
    let size_bytes =
        num_bytes * pixel_block_size(width, height, num_channels, pixel_data_type) as usize;
    size_bytes
}

#[inline]
fn convert_value_to_f32_bytes(value: f32, pixel_data_type: PixelDataType) -> f32 {
    match pixel_data_type {
        PixelDataType::UInt8 => {
            let v: u8 = (value * (u8::MAX as f32)) as u8;
            let array: [u8; 4] = [v, v, v, v];
            f32::from_ne_bytes(array)
        }
        PixelDataType::UInt16 => {
            let v: u16 = (value * (u16::MAX as f32)) as u16;
            let array = v.to_ne_bytes();
            let bytes: [u8; 4] = [array[0], array[1], array[0], array[1]];
            f32::from_ne_bytes(bytes)
        }
        PixelDataType::Half16 => {
            let v: f16 = f16::from_f32(value);
            let array = v.to_ne_bytes();
            let bytes: [u8; 4] = [array[0], array[1], array[0], array[1]];
            f32::from_ne_bytes(bytes)
        }
        PixelDataType::Float32 => value as f32,
        _ => panic!("Invalid pixel data type"),
    }
}

/// Get the pixel index into a packed data structure of RGB(A) pixel
/// data.
///
/// Returns the index if the x,y coordinates are with-in bounds,
/// otherwise -1 is returned.
#[inline]
pub fn get_pixel_index(
    width: i32,
    height: i32,
    x_stride: i32,
    y_stride: i32,
    x: i32,
    y: i32,
) -> isize {
    let index_x = x * x_stride;
    let index_y = y * y_stride;
    let max_x = (width - 1) * x_stride;
    let max_y = (height - 1) * y_stride;
    let mut index = index_x + index_y;
    if (index_x < 0) || (index_x > max_x) || (index_y < 0) || (index_y > max_y) {
        index = -1;
    }
    return index as isize;
}

#[inline]
pub fn transmute_slice_f32_to_u8(pixel_slice: &[f32]) -> &[u8] {
    // SAFETY: It is safe to convert the pixel data
    // from 'f32' to 'u8' because there can be no
    // alignment issues. f32 is 4-byte aligned. u8 is
    // 1-byte aligned which fits into 4-bytes evenly.
    let pixels_u8 = unsafe { std::mem::transmute::<&[f32], &[u8]>(pixel_slice) };
    pixels_u8
}

#[inline]
pub fn transmute_slice_f32_to_u16(pixel_slice: &[f32]) -> &[u16] {
    // SAFETY: It is safe to convert the pixel data
    // from 'f32' to 'u16' because there can be no
    // alignment issues. f32 is 4-byte aligned. u16 is
    // 2-byte aligned, which fits into 4-bytes evenly.
    let pixels_u16 = unsafe { std::mem::transmute::<&[f32], &[u16]>(pixel_slice) };
    pixels_u16
}

#[inline]
pub fn transmute_slice_f32_to_f16(pixel_slice: &[f32]) -> &[f16] {
    // SAFETY: It is safe to convert the pixel data
    // from 'f32' to 'f16' because there can be no
    // alignment issues. f32 is 4-byte aligned. f16 is
    // 2-byte aligned, which fits into 4-bytes evenly.
    let pixels_f16 = unsafe { std::mem::transmute::<&[f32], &[f16]>(pixel_slice) };
    pixels_f16
}

/// Convert a slice of pixels from T to f32 type.
///
/// The results are
/// stored in a Vec<f32>, but the real bytes are of type T. Therefore
/// the number of elements in the Vec<f32> will be less than the
/// pixel_slice.
#[inline]
pub fn convert_pixel_data_to_f32<T, F>(
    pixel_slice: &[f32],
    new_pixels: &mut Vec<f32>,
    convert_func: F,
) where
    T: Copy,
    F: Fn(T) -> f32,
{
    assert_eq!(new_pixels.len(), 0);
    let num_items_per_f32 = std::mem::align_of::<f32>() / std::mem::align_of::<T>();
    let pixels_native_count = pixel_slice.len() * num_items_per_f32;
    new_pixels.reserve_exact(pixels_native_count);
    let pixels_native = unsafe { std::mem::transmute::<&[f32], &[T]>(pixel_slice) };
    let ptr = pixels_native.as_ptr();
    for i in 0..pixels_native_count {
        let x = unsafe { *ptr.add(i) };
        let v = convert_func(x);
        new_pixels.push(v);
    }
}

#[derive(Clone)]
pub struct PixelBlock {
    width: i32,
    height: i32,
    num_channels: i32,
    pixel_data_type: PixelDataType,

    // TODO: Add a 'single_pixel' flag to indicate the entire image
    // has a single colour. This allows large images that are constant
    // to allocate 1 pixel, but represent many pixels.
    //
    // _single_pixel: bool,
    //
    /// The pixel data may *not* be f32 values, but rather the values
    /// may be u8 or u16 or 'half'. We use 'f32' here as the maximum
    /// number of bytes that could be needed. Rust's Vec will ensure
    /// that the f32 values are a byte aligned for us.
    pixels: Vec<f32>,
}

impl PixelBlock {
    pub fn new(
        width: i32,
        height: i32,
        num_channels: i32,
        pixel_data_type: PixelDataType,
    ) -> PixelBlock {
        let size = pixel_block_size_bytes(width, height, num_channels, pixel_data_type);
        let size_aligned = size_aligned_to::<f32>(size);
        let pixels = vec![0.0 as f32; size_aligned];

        PixelBlock {
            width,
            height,
            num_channels,
            pixel_data_type,
            pixels,
        }
    }

    pub fn empty(pixel_data_type: PixelDataType) -> PixelBlock {
        let width = 0;
        let height = 0;
        let num_channels = 0;
        PixelBlock::new(width, height, num_channels, pixel_data_type)
    }

    pub fn new_constant_pixel_rgba_f32(r: f32, g: f32, b: f32, a: f32) -> PixelBlock {
        let width = 1;
        let height = 1;
        let num_channels = 4;
        let pixel_data_type = PixelDataType::Float32;

        let size = pixel_block_size_bytes(width, height, num_channels, pixel_data_type);
        let size_aligned = size_aligned_to::<f32>(size);
        let mut pixels = Vec::<f32>::with_capacity(size_aligned);
        let single_pixel = [r, g, b, a];
        pixels.extend_from_slice(&single_pixel);

        PixelBlock {
            width,
            height,
            num_channels,
            pixel_data_type,
            pixels,
        }
    }

    pub fn new_color_bars() -> PixelBlock {
        let width = COLOR_BARS_WIDTH;
        let height = COLOR_BARS_HEIGHT;
        let num_channels = COLOR_BARS_NUM_CHANNELS;
        let pixels = COLOR_BARS.to_vec();
        let pixel_data_type = PixelDataType::Float32;
        PixelBlock {
            width,
            height,
            num_channels,
            pixel_data_type,
            pixels,
        }
    }

    /// Construct a pixel block from an existing PixelBlock, with a
    /// new data window.
    ///
    /// The pixels inside the data window are copied.
    pub fn from_pixel_block(
        pixel_block: &PixelBlock,
        data_window: BBox2Di,
        crop_window: BBox2Di,
    ) -> PixelBlock {
        assert!(pixel_block.width() == data_window.width());
        assert!(pixel_block.height() == data_window.height());

        let diff_window = BBox2Di::intersection(data_window, crop_window);

        let width = crop_window.width();
        let height = crop_window.height();
        let num_channels = pixel_block.num_channels();

        let pixel_data_type = pixel_block.pixel_data_type();
        let mut new_pixel_block = PixelBlock::new(width, height, num_channels, pixel_data_type);

        // Detect if there are any pixels to copy over. If not, simply
        // return the transparent black pixels for the crop window.
        if diff_window.area() == 0 {
            return new_pixel_block;
        }

        // Copy over pixels, scanline by scanline.
        //
        // Each scanline may live inside or outside the original data
        // window.
        let dst_offset_y = crop_window.min_y;
        let dst_offset_x = diff_window.min_x - crop_window.min_x;
        let src_offset_y = data_window.min_y;
        let src_offset_start_x = (diff_window.min_x - data_window.min_x) * num_channels;
        let src_offset_end_x = src_offset_start_x + (diff_window.width()) * num_channels;

        for y in diff_window.min_y..diff_window.max_y {
            // Destination indexes into the cropped output image.
            let dst_row = y - dst_offset_y;
            let dst_start_index =
                (((dst_row * crop_window.width()) + dst_offset_x) * num_channels) as usize;
            let dst_end_index = dst_start_index + (diff_window.width() * num_channels) as usize;
            let dst = &mut new_pixel_block.pixels[dst_start_index..dst_end_index];

            // Source indexes into the input image.
            let src_row = y - src_offset_y;
            let src_row_index = (src_row * data_window.width() * num_channels) as usize;
            let src_start_index = src_row_index + src_offset_start_x as usize;
            let src_end_index = src_row_index + src_offset_end_x as usize;
            let src = &pixel_block.pixels[src_start_index..src_end_index];

            dst.copy_from_slice(src);
        }

        new_pixel_block
    }

    fn convert_into_u8_data(&mut self) {
        let old_pixel_data_type = self.pixel_data_type;
        let new_pixel_data_type = PixelDataType::UInt8;
        assert!(old_pixel_data_type != new_pixel_data_type);

        let pixel_slice = self.pixels.as_slice();
        let mut new_pixels = Vec::<f32>::new();
        match old_pixel_data_type {
            // PixelDataType::Half16 => {}
            // PixelDataType::UInt16 => {}
            PixelDataType::Float32 => {
                let num_items_per_f32 = std::mem::align_of::<f32>() / std::mem::align_of::<u8>();
                let pixels_native_count = pixel_slice.len() / num_items_per_f32;
                new_pixels.reserve_exact(pixels_native_count);
                for v in pixel_slice.chunks_exact(num_items_per_f32) {
                    // TODO: Should this data be stored in sRGB colour space?
                    let bytes: [u8; 4] = [
                        (v[0] * (u8::max_value() as f32)) as u8,
                        (v[1] * (u8::max_value() as f32)) as u8,
                        (v[2] * (u8::max_value() as f32)) as u8,
                        (v[3] * (u8::max_value() as f32)) as u8,
                    ];
                    let value = f32::from_ne_bytes(bytes);
                    new_pixels.push(value);
                }
            }
            _ => panic!("Unsupported pixel data type: {:#?}", old_pixel_data_type),
        };

        self.pixels = new_pixels;
        self.pixel_data_type = new_pixel_data_type;
    }

    fn convert_into_f16_data(&mut self) {
        let old_pixel_data_type = self.pixel_data_type;
        let new_pixel_data_type = PixelDataType::Half16;
        assert!(old_pixel_data_type != new_pixel_data_type);

        let pixel_slice = self.pixels.as_slice();
        let mut new_pixels = Vec::<f32>::new();
        match old_pixel_data_type {
            // PixelDataType::UInt8 => {}
            // PixelDataType::UInt16 => {}
            PixelDataType::Float32 => {
                let num_items_per_f32 = std::mem::size_of::<f32>() / std::mem::size_of::<f16>();
                let capacity = pixel_slice.len() / num_items_per_f32;
                new_pixels.reserve_exact(capacity);
                for v in pixel_slice.chunks_exact(num_items_per_f32) {
                    let a_bytes = f16::from_f32(v[0]).to_ne_bytes();
                    let b_bytes = f16::from_f32(v[1]).to_ne_bytes();
                    let bytes: [u8; 4] = [a_bytes[0], a_bytes[1], b_bytes[0], b_bytes[1]];
                    let x = f32::from_ne_bytes(bytes);
                    new_pixels.push(x);
                }
            }
            _ => panic!("Unsupported pixel data type: {:#?}", old_pixel_data_type),
        };

        self.pixels = new_pixels;
        self.pixel_data_type = new_pixel_data_type;
    }

    fn convert_into_f32_data(&mut self) {
        let old_pixel_data_type = self.pixel_data_type;
        let new_pixel_data_type = PixelDataType::Float32;
        assert!(old_pixel_data_type != new_pixel_data_type);

        let pixel_slice = self.pixels.as_slice();
        let mut new_pixels = Vec::<f32>::new();
        match old_pixel_data_type {
            PixelDataType::UInt8 => {
                let f = |x: u8| (x as f32) / (u8::max_value() as f32);
                convert_pixel_data_to_f32(pixel_slice, &mut new_pixels, f);
            }
            PixelDataType::UInt16 => {
                let f = |x: u16| (x as f32) / (u16::max_value() as f32);
                convert_pixel_data_to_f32(pixel_slice, &mut new_pixels, f);
            }
            PixelDataType::Half16 => {
                let f = |x: f16| x.to_f32();
                convert_pixel_data_to_f32(pixel_slice, &mut new_pixels, f);
            }
            _ => panic!("Unsupported pixel data type: {:#?}", old_pixel_data_type),
        };

        self.pixels = new_pixels;
        self.pixel_data_type = new_pixel_data_type;
    }

    pub fn convert_into_pixel_data_type(&mut self, out_pixel_data_type: PixelDataType) {
        let old_pixel_data_type = self.pixel_data_type;
        let new_pixel_data_type = out_pixel_data_type;
        debug!(
            "Convert Pixel Data Type from {:#?} to {:#?}",
            old_pixel_data_type, new_pixel_data_type
        );
        if old_pixel_data_type == new_pixel_data_type {
            return;
        }

        let pixel_values_num_before = self.pixels.as_slice().len();
        match new_pixel_data_type {
            PixelDataType::Float32 => self.convert_into_f32_data(),
            PixelDataType::Half16 => self.convert_into_f16_data(),
            PixelDataType::UInt8 => self.convert_into_u8_data(),
            PixelDataType::UInt16 => {
                panic!("Convert to {:#?} is not supported", PixelDataType::UInt16)
            }
            _ => panic!("Convert invalid data type is not supported"),
        }

        let pixel_values_num_after = self.pixels.as_slice().len();
        assert_ne!(pixel_values_num_before, pixel_values_num_after);
    }

    pub fn as_slice(&self) -> &[f32] {
        &self.pixels[..]
    }

    pub fn as_slice_mut(&mut self) -> &mut [f32] {
        &mut self.pixels[..]
    }

    pub fn set_pixels(&mut self, pixels: Vec<f32>) {
        self.pixels = pixels;
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn num_channels(&self) -> i32 {
        self.num_channels
    }

    pub fn pixel_data_type(&self) -> PixelDataType {
        self.pixel_data_type
    }

    pub fn size(&self) -> usize {
        pixel_block_size(
            self.width,
            self.height,
            self.num_channels,
            self.pixel_data_type,
        )
    }

    pub fn size_bytes(&self) -> usize {
        pixel_block_size_bytes(
            self.width,
            self.height,
            self.num_channels,
            self.pixel_data_type,
        )
    }

    pub fn get_pixel_index(&self, x: i32, y: i32) -> isize {
        let x_stride = self.num_channels;
        let y_stride = self.width * x_stride;
        get_pixel_index(self.width, self.height, x_stride, y_stride, x, y)
    }

    pub fn data_resize(
        &mut self,
        width: i32,
        height: i32,
        num_channels: i32,
        pixel_data_type: PixelDataType,
    ) {
        self.width = width;
        self.height = height;
        self.num_channels = num_channels;
        self.pixel_data_type = pixel_data_type;

        let new_capacity = pixel_block_size(width, height, num_channels, pixel_data_type) as usize;
        let new_capacity_elements = new_capacity
            / (channel_size_bytes(PixelDataType::Float32) / channel_size_bytes(pixel_data_type));

        // Resize with all values defaulting to "1.0" in the native
        // pixel data type.
        let fill_value = convert_value_to_f32_bytes(1.0, pixel_data_type);
        self.pixels.resize(new_capacity_elements, fill_value);
    }
}

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

    let pixel_data_type = PixelDataType::Float32;
    PixelBlock {
        width,
        height,
        num_channels,
        pixel_data_type,
        pixels,
    }
}

impl Hash for PixelBlock {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.width.hash(state);
        self.height.hash(state);
        self.num_channels.hash(state);
        self.pixel_data_type.hash(state);
        // Note: Skipping the 'pixels' data.
    }
}

impl fmt::Debug for PixelBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PixelBlock")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("num_channels", &self.num_channels)
            .field("pixel_data_type", &self.pixel_data_type)
            .finish()
    }
}
