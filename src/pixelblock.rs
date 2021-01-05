use image;
use image::GenericImageView;
use image::RgbaImage;
use log::{debug, error, info, log_enabled, warn, Level};
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash;
use std::hash::{Hash, Hasher};

use crate::colorutils::convert_linear_to_srgb;
use crate::colorutils::convert_srgb_to_linear;
use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::NodeType;
use crate::cxxbridge::ffi::PixelDataType;
use crate::cxxbridge::ffi::StreamDataImplShared;
use crate::data::HashValue;
use crate::data::Identifier;
use crate::node::traits::AttrBlock;
use crate::node::traits::Compute;
use crate::node::NodeImpl;

#[derive(Clone)]
pub struct PixelBlock {
    pub width: u32,
    pub height: u32,
    pub num_channels: u8,
    pub pixel_data_type: PixelDataType,
    pub pixels: Vec<f32>,
}

impl PixelBlock {
    pub fn new(width: u32, height: u32, num_channels: u8) -> PixelBlock {
        let size = (width * height * num_channels as u32) as usize;
        let pixels = vec![0.5 as f32; size];
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
        let (width, height) = img.dimensions();
        let color_type = img.color();
        debug!("Resolution: {:?}x{:?}", width, height);
        debug!("Color Type: {:?}", color_type);
        let num_channels = match color_type {
            image::ColorType::Rgb8 => 3,
            image::ColorType::Rgba8 => 3,
            _ => 0,
        };
        debug!("Num Channels: {:?}", num_channels);

        // Convert the image to f32 values
        //
        // NOTE: We strip off the alpha channel here, this
        // functionality will be implemented later.
        let rgba_img = img.into_rgb8();
        let flat_samples = rgba_img.into_flat_samples();
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

        match image::ImageBuffer::from_raw(width, height, pixels_u8) {
            Some(data) => data,
            _ => panic!("invalid image."),
        }
    }

    pub fn set_pixels(&mut self, pixels: Vec<f32>) {
        self.pixels = pixels;
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
