use half::f16;
use image;
use image::GenericImageView;
use log::{debug, error, info, log_enabled, warn, Level};
use std::cmp;
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::attrblock::AttrBlock;
use crate::colorutils::convert_linear_to_srgb;
use crate::colorutils::convert_srgb_to_linear;
use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::BBox2Di;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::NodeType;
use crate::cxxbridge::ffi::PixelDataType;
use crate::cxxbridge::ffi::StreamDataImplShared;
use crate::data::HashValue;
use crate::data::Identifier;
use crate::data::COLOR_BARS;
use crate::data::COLOR_BARS_HEIGHT;
use crate::data::COLOR_BARS_NUM_CHANNELS;
use crate::data::COLOR_BARS_WIDTH;
use crate::node::traits::Operation;
use crate::node::NodeImpl;

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
    // num_channels as usize
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
    let mut index = index_x + index_y;
    let max_x = (width - 1) * x_stride;
    let max_y = (height - 1) * y_stride;
    let max_index = max_x + max_y;
    if (index < 0) || (index > max_index) {
        index = -1;
    }
    return index as isize;
}

#[derive(Clone)]
pub struct PixelBlock {
    width: i32,
    height: i32,
    num_channels: i32,
    pixel_data_type: PixelDataType,

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

    pub fn convert_into_f32_data(&mut self) {
        if self.pixel_data_type == PixelDataType::Float32 {
            return;
        } else {
            let old_pixel_data_type = self.pixel_data_type;
            let old_size_bytes = self.size_bytes();
            let size = self.size();

            let new_pixel_data_type = PixelDataType::Float32;
            let new_size_bytes = pixel_block_size_bytes(
                self.width,
                self.height,
                self.num_channels,
                new_pixel_data_type,
            );

            let pixel_slice = self.pixels.as_slice();
            let new_pixels: Vec<f32> = match old_pixel_data_type {
                PixelDataType::UInt8 => {
                    // SAFETY: It is safe to convert the pixel data
                    // from 'f32' to 'u8' because there can be no
                    // alignment issues. f32 is 4-byte aligned. u8 is
                    // 1-byte aligned which fits into 4-bytes evenly.
                    let pixels_u8 = unsafe { std::mem::transmute::<&[f32], &[u8]>(pixel_slice) };
                    pixels_u8
                        .into_iter()
                        .map(|x| (*x as f32) / (u8::max_value() as f32))
                        .collect()
                }
                PixelDataType::UInt16 => {
                    // SAFETY: It is safe to convert the pixel data
                    // from 'f32' to 'u16' because there can be no
                    // alignment issues. f32 is 4-byte aligned. u16 is
                    // 2-byte aligned, which fits into 4-bytes evenly.
                    let pixels_u16 = unsafe { std::mem::transmute::<&[f32], &[u16]>(pixel_slice) };
                    pixels_u16
                        .into_iter()
                        .map(|x| (*x as f32) / (u16::max_value() as f32))
                        .collect()
                }
                PixelDataType::Half16 => {
                    // SAFETY: It is safe to convert the pixel data
                    // from 'f32' to 'f16' because there can be no
                    // alignment issues. f32 is 4-byte aligned. f16 is
                    // 2-byte aligned, which fits into 4-bytes evenly.
                    let pixels_f16 = unsafe { std::mem::transmute::<&[f32], &[f16]>(pixel_slice) };
                    pixels_f16.into_iter().map(|x| (*x).to_f32()).collect()
                }
                _ => panic!("Unsupported pixel data type: {:#?}", old_pixel_data_type),
            };

            self.pixels = new_pixels;
            self.pixel_data_type = new_pixel_data_type;
        }
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
        let y_stride = self.width * self.num_channels;
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

        // Resize with all values defaulting to "1.0" in the native
        // pixel data type.
        let fill_value = convert_value_to_f32_bytes(1.0, pixel_data_type);
        self.pixels.resize(new_capacity, fill_value);
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
    let data_window = BBox2Di::new(0, 0, width, height);
    let display_window = BBox2Di::new(0, 0, width, height);
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
