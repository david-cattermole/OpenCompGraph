use image;
use image::GenericImageView;
use log::{debug, error, info, log_enabled, warn, Level};
use petgraph;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem;

use crate::colorutils::convert_srgb_to_linear;

pub type GraphIdx = usize;
pub type NodeWeight = u64;
pub type EdgeWeight = u8;
pub type NodeIdx = petgraph::graph::NodeIndex<GraphIdx>;
pub type EdgeIdx = petgraph::graph::EdgeIndex<GraphIdx>;
pub type HashValue = u64;
pub type Identifier = u64;

#[derive(Clone)]
pub struct PixelBlock {
    pub width: u32,
    pub height: u32,
    pub num_channels: u8,
    pub pixels: Vec<f32>,
}

impl PixelBlock {
    pub fn new(width: u32, height: u32, num_channels: u8) -> PixelBlock {
        let size = (width * height * num_channels as u32) as usize;
        let pixels = vec![0.5 as f32; size];
        PixelBlock {
            width,
            height,
            num_channels,
            pixels,
        }
    }

    pub fn from_image(img: image::DynamicImage) -> PixelBlock {
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

        PixelBlock {
            width,
            height,
            num_channels,
            pixels,
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
