use image;
use image::GenericImageView;
use image::ImageBuffer;
use std::path::Path;
use std::string::String;

use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::NodeType;
use crate::cxxbridge::ffi::StreamDataImplShared;
use crate::data::Identifier;
use crate::data::PixelBlock;
use crate::node::traits::{AttrBlock, Compute};
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

#[derive(Debug, Clone, Default)]
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
    fn hash(
        &mut self,
        id: Identifier,
        node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        inputs: &Vec<StreamDataImplShared>,
    ) -> usize {
        // virtual const Hash hash() {
        //     return this->id * this->type * 123456789;
        // };

        0
    }

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
            // pixels: &image::DynamicImage
            // let pixel_block = &mut output.inner.get_pixel_block();
            let mut pixel_block =
                Box::<PixelBlock>::new(PixelBlock::new(width, height, num_channels));
            pixel_block.set_pixels(img);
            output.inner.set_pixel_block(pixel_block);
        }

        // PixelBlock pixels;
        // pixels.data = data;

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
