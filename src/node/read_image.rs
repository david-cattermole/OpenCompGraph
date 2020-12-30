use fastapprox::faster;
use image;
use image::GenericImageView;
use image::ImageBuffer;
use rustc_hash::FxHasher;
use std::collections::hash_map::DefaultHasher;
use std::hash;
use std::hash::Hash;
use std::path::Path;
use std::string::String;

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
        op_status: NodeStatus::Uninitialized,
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

/// https://www.excamera.com/sphinx/article-srgb.html
fn convert_srgb_to_linear(x: f32) -> f32 {
    let a: f32 = 0.055;
    if x <= 0.04045 {
        x * (1.0 / 12.92)
    } else {
        faster::pow((x + a) * (1.0 / (1.0 + a)), 2.4)
    }
}

impl Compute for ReadImageCompute {
    fn compute(
        &mut self,
        attr_block: &Box<dyn AttrBlock>,
        inputs: &Vec<StreamDataImplShared>,
        output: &mut StreamDataImplShared,
    ) -> NodeStatus {
        println!("ReadImageCompute.compute()");
        // println!("AttrBlock: {:?}", attr_block);
        // println!("Inputs: {:?}", inputs);
        // println!("Output: {:?}", output);

        let file_path = attr_block.get_attr_str("file_path");
        // println!("file_path {:?}", file_path);

        let path = match Path::new(&file_path).canonicalize() {
            Ok(full_path) => full_path,
            Err(_) => return NodeStatus::Error,
        };
        println!("Opening... {:?}", path);
        if path.is_file() == true {
            let img = image::open(path).unwrap();
            let (width, height) = img.dimensions();
            let color_type = img.color();
            println!("Resolution: {:?}x{:?}", width, height);
            println!("Color Type: {:?}", color_type);
            let num_channels = match color_type {
                image::ColorType::Rgb8 => 3,
                image::ColorType::Rgba8 => 3,
                _ => 0,
            };
            println!("Num Channels: {:?}", num_channels);

            // Convert the image to f32 values
            let rgba_img = img.into_rgb8();
            let flat_samples = rgba_img.into_flat_samples();
            let pixels: Vec<f32> = flat_samples
                .as_slice()
                .into_iter()
                .map(|x| convert_srgb_to_linear((*x as f32) / (u8::max_value() as f32)))
                .collect();

            // Get pixel statistics
            let min = pixels.iter().fold(f32::INFINITY, |a, &b| a.min(b));
            let max = pixels.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
            let avg = pixels.iter().sum::<f32>() / (pixels.len() as f32);
            println!("Min value: {}", min);
            println!("Max value: {}", max);
            println!("Avg value: {}", avg);

            let mut pixel_block = PixelBlock::new(width, height, num_channels);
            let pixels = pixel_block.set_pixels(pixels);
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
