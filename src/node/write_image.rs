use std::path::Path;
use std::string::String;

use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::NodeType;
use crate::cxxbridge::ffi::StreamDataImplShared;
use crate::data::HashValue;
use crate::data::Identifier;
use crate::node::traits::AttrBlock;
use crate::node::traits::Compute;
use crate::node::NodeImpl;
use image::RgbaImage;
use std::collections::hash_map::DefaultHasher;
use std::hash;
use std::hash::Hash;

pub fn new(id: Identifier) -> NodeImpl {
    NodeImpl {
        node_type: NodeType::WriteImage,
        id,
        op_status: NodeStatus::Uninitialized,
        compute: Box::new(WriteImageCompute::new()),
        attr_block: Box::new(WriteImageAttrs::new()),
    }
}

#[derive(Debug, Clone, Default)]
pub struct WriteImageCompute {}

#[derive(Debug, Clone, Default, Hash)]
pub struct WriteImageAttrs {
    pub file_path: String,
}

impl WriteImageCompute {
    pub fn new() -> WriteImageCompute {
        WriteImageCompute {}
    }
}

impl WriteImageAttrs {
    pub fn new() -> WriteImageAttrs {
        WriteImageAttrs {
            file_path: "".to_string(),
        }
    }
}

impl Compute for WriteImageCompute {
    fn compute(
        &mut self,
        attr_block: &Box<dyn AttrBlock>,
        inputs: &Vec<StreamDataImplShared>,
        output: &mut StreamDataImplShared,
    ) -> NodeStatus {
        println!("WriteImageCompute.compute()");
        // println!("AttrBlock: {:?}", attr_block);
        // println!("Inputs: {:?}", inputs);
        // println!("Output: {:?}", output);
        match inputs.len() {
            0 => NodeStatus::Error,
            _ => {
                let input = &inputs[0].inner;

                let file_path = attr_block.get_attr_str("file_path");
                // println!("file_path {:?}", file_path);

                let pixel_block = input.get_pixel_block();
                let width = pixel_block.width;
                let height = pixel_block.height;
                let pixels = &pixel_block.pixels;

                // Convert f32 pixel image to u8 ImageBuffer.
                let pixels_u8: Vec<u8> = pixels
                    .iter()
                    .map(|x| ((*x as f32) * (u8::max_value() as f32)) as u8)
                    .collect();
                let img: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
                    match image::ImageBuffer::from_raw(width, height, pixels_u8) {
                        Some(data) => data,
                        _ => panic!("invalid image."),
                    };

                println!("Writing... {:?}", file_path);
                let ok = match img.save(file_path) {
                    Ok(value) => true,
                    Err(_) => false,
                };
                println!("Succcess: {}", ok);
                NodeStatus::Valid
            }
        }
    }
}

impl AttrBlock for WriteImageAttrs {
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
