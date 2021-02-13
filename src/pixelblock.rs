use image;
use image::GenericImageView;
use image::RgbaImage;
use log::{debug, error, info, log_enabled, warn, Level};
use std::cmp;
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash;
use std::hash::{Hash, Hasher};

use crate::attrblock::AttrBlock;
use crate::colorutils::convert_linear_to_srgb;
use crate::colorutils::convert_srgb_to_linear;
use crate::cxxbridge::ffi::AttrState;
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

#[derive(Clone)]
pub struct PixelBlock {
    pub width: i32,
    pub height: i32,
    pub num_channels: i32,
    pub pixel_data_type: PixelDataType,
    pub pixels: Vec<f32>,
}

impl PixelBlock {
    pub fn new(
        width: i32,
        height: i32,
        num_channels: i32,
        pixel_data_type: PixelDataType,
    ) -> PixelBlock {
        let size = (width * height * num_channels) as usize;
        let pixels = vec![0.0 as f32; size];
        let pixel_data_type = PixelDataType::Float32;
        PixelBlock {
            width,
            height,
            num_channels,
            pixel_data_type,
            pixels,
        }
    }

    pub fn new_constant_pixel_rgba(r: f32, g: f32, b: f32, a: f32) -> PixelBlock {
        let width = 1;
        let height = 1;
        let num_channels = 4;
        let pixels = vec![r, g, b, a];
        let pixel_data_type = PixelDataType::Float32;
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
        let size = (width * height * num_channels) as usize;
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

    /// Consumes a image::DynamicImage and produces a PixelBlock.
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

    pub fn to_image_buffer_rgb_u8(&self) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let width = self.width;
        let height = self.height;
        // let num_channels = self.num_channels;
        // let pixel_data_type = self.pixel_data_type;
        let pixels = &self.pixels;

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

    pub fn to_image_buffer_rgba_u8(&self) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
        let width = self.width;
        let height = self.height;
        // let num_channels = self.num_channels;
        // let pixel_data_type = self.pixel_data_type;
        let pixels = &self.pixels;

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
            self.num_channels,
            pixels_u8.len()
        );
        match image::ImageBuffer::from_raw(width as u32, height as u32, pixels_u8) {
            Some(data) => data,
            _ => panic!("invalid image."),
        }
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

        // Reserve ("exactly") enough memory to hold our pixels. We do
        // not expect needing to append more pixels later in the
        // future and we don't want to waste memory with unneeded
        // pixels allocated that will never be used.
        let new_capacity = (width * height * num_channels) as usize;
        let additional_num_elements = cmp::max(0, new_capacity - self.pixels.capacity());
        self.pixels.reserve_exact(additional_num_elements);

        // SAFTEY: The memory is reserved at least enough memory to
        // use, but may point to uninitialized data. It is expected
        // the user either except "garbage" pixel values after calling
        // this function, or the user is expecting to write over the
        // pixel data immediately.
        unsafe { self.pixels.set_len(new_capacity) };
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

    pub fn size_bytes(&self) -> usize {
        let num_bytes = match self.pixel_data_type {
            PixelDataType::Float32 => std::mem::size_of::<f32>(),
            PixelDataType::Half16 => std::mem::size_of::<i16>(), // TODO: Support "half" properly.
            PixelDataType::UInt8 => std::mem::size_of::<u8>(),
            PixelDataType::UInt16 => std::mem::size_of::<u16>(),
            _ => {
                error!("Invalid PixelDataType: {:?}", self.pixel_data_type);
                0
            }
        };
        num_bytes * (self.width * self.height * self.num_channels) as usize
    }
}

impl Hash for PixelBlock {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.width.hash(state);
        self.height.hash(state);
        self.num_channels.hash(state);
        // Note: Skipping the 'pixels' data.
    }
}

impl fmt::Debug for PixelBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PixelBlock")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("num_channels", &self.num_channels)
            .finish()
    }
}
