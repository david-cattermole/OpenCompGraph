use image;
use image::GenericImageView;
use image::ImageBuffer;
use log::{debug, error, info, log_enabled, warn, Level};
use rustc_hash::FxHasher;
use std::collections::hash_map::DefaultHasher;
use std::hash;
use std::hash::Hash;
use std::path::Path;
use std::string::String;

use crate::colorutils;
use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::NodeType;
use crate::cxxbridge::ffi::StreamDataImplShared;
use crate::data::HashValue;
use crate::data::Identifier;
use crate::data::PixelBlock;
use crate::node::traits::AttrBlock;
use crate::node::traits::Compute;
use crate::node::NodeImpl;

pub fn new(id: Identifier) -> NodeImpl {
    NodeImpl {
        node_type: NodeType::ReadImage,
        id,
        status: NodeStatus::Uninitialized,
        compute: Box::new(ReadImageCompute::new()),
        attr_block: Box::new(ReadImageAttrs::new()),
    }
}

#[derive(Debug, Clone, Default)]
pub struct ReadImageCompute {}

#[derive(Debug, Clone, Default, hash::Hash)]
pub struct ReadImageAttrs {
    pub file_path: String,
}

impl ReadImageCompute {
    pub fn new() -> ReadImageCompute {
        ReadImageCompute {}
    }
}

impl ReadImageAttrs {
    pub fn new() -> ReadImageAttrs {
        ReadImageAttrs {
            file_path: "".to_string(),
        }
    }
}

impl Compute for ReadImageCompute {
    fn compute(
        &mut self,
        node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        inputs: &Vec<StreamDataImplShared>,
        output: &mut StreamDataImplShared,
    ) -> NodeStatus {
        debug!("ReadImageCompute.compute()");
        // debug!("AttrBlock: {:?}", attr_block);
        // debug!("Inputs: {:?}", inputs);
        // debug!("Output: {:?}", output);

        let file_path = attr_block.get_attr_str("file_path");
        // debug!("file_path {:?}", file_path);

        let path = match Path::new(&file_path).canonicalize() {
            Ok(full_path) => full_path,
            Err(_) => return NodeStatus::Error,
        };
        debug!("Opening... {:?}", path);
        if path.is_file() == true {
            let img = image::open(path).unwrap();
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
                .map(|x| colorutils::convert_srgb_to_linear((*x as f32) / (u8::max_value() as f32)))
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

            let mut pixel_block = PixelBlock::new(width, height, num_channels);
            let pixels = pixel_block.set_pixels(pixels);
            let hash_value = self.cache_hash(node_type_id, &attr_block, inputs);
            output.inner.set_hash(hash_value);
            output.inner.set_pixel_block(pixel_block);
        }

        // BoundingBox displayBox;
        // displayBox.min.x = 0;
        // displayBox.min.y = 0;
        // displayBox.max.x = image_width;
        // displayBox.max.y = image_height;

        // Image img;
        // img.pixelBlock = pixels;
        // img.displayWindow = displayBox;

        NodeStatus::Valid
    }
}

impl AttrBlock for ReadImageAttrs {
    fn attr_hash(&self, state: &mut DefaultHasher) {
        self.hash(state)
    }

    fn attr_exists(&self, name: &str) -> AttrState {
        match name {
            "file_path" => AttrState::Exists,
            _ => AttrState::Missing,
        }
    }

    fn get_attr_str(&self, name: &str) -> &str {
        match name {
            "file_path" => &self.file_path,
            _ => "",
        }
    }

    fn set_attr_str(&mut self, name: &str, value: &str) {
        match name {
            "file_path" => self.file_path = value.to_string(),
            _ => (),
        };
    }

    fn get_attr_i32(&self, name: &str) -> i32 {
        0
    }

    fn set_attr_i32(&mut self, name: &str, value: i32) {
        ()
    }

    fn get_attr_f32(&self, name: &str) -> f32 {
        0.0
    }

    fn set_attr_f32(&mut self, name: &str, value: f32) {
        ()
    }
}
