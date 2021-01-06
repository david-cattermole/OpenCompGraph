use image::RgbaImage;
use log::{debug, error, info, log_enabled, warn, Level};
use std::collections::hash_map::DefaultHasher;
use std::hash;
use std::hash::Hash;
use std::path::Path;
use std::string::String;

use crate::attrblock::AttrBlock;
use crate::colorutils;
use crate::colorutils::convert_linear_to_srgb;
use crate::colorxform;
use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::NodeType;
use crate::cxxbridge::ffi::StreamDataImplShared;
use crate::data::HashValue;
use crate::data::Identifier;
use crate::node::traits::Operation;
use crate::node::NodeImpl;

pub fn new(id: Identifier) -> NodeImpl {
    NodeImpl {
        node_type: NodeType::WriteImage,
        id,
        status: NodeStatus::Uninitialized,
        compute: Box::new(WriteImageOperation::new()),
        attr_block: Box::new(WriteImageAttrs::new()),
    }
}

#[derive(Debug, Clone, Default)]
pub struct WriteImageOperation {}

#[derive(Debug, Clone, Default, Hash)]
pub struct WriteImageAttrs {
    pub enable: i32,
    pub file_path: String,
}

impl WriteImageOperation {
    pub fn new() -> WriteImageOperation {
        WriteImageOperation {}
    }
}

impl WriteImageAttrs {
    pub fn new() -> WriteImageAttrs {
        WriteImageAttrs {
            enable: 1,
            file_path: "".to_string(),
        }
    }
}

impl Operation for WriteImageOperation {
    fn compute(
        &mut self,
        node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        inputs: &Vec<StreamDataImplShared>,
        output: &mut StreamDataImplShared,
    ) -> NodeStatus {
        debug!("WriteImageOperation.compute()");
        // debug!("AttrBlock: {:?}", attr_block);
        // debug!("Inputs: {:?}", inputs);
        // debug!("Output: {:?}", output);
        let enable = attr_block.get_attr_i32("enable");
        if enable != 1 {
            return NodeStatus::Error;
        }

        match inputs.len() {
            0 => NodeStatus::Error,
            _ => {
                let input = &inputs[0].inner;

                // Copy input data
                let mut copy = &mut input.clone();
                let num_channels = copy.pixel_num_channels();

                // Apply Color Matrix
                let color_matrix = copy.color_matrix().to_na_matrix();
                let pixel_block = copy.pixel_block_as_mut();
                let mut pixels = &mut pixel_block.pixels;
                colorutils::apply_color_matrix_inplace(pixels, num_channels, color_matrix);

                let img = pixel_block.to_image_buffer_rgb_u8();

                let file_path = attr_block.get_attr_str("file_path");
                debug!("Writing... {:?}", file_path);
                let ok = match img.save(file_path) {
                    Ok(value) => true,
                    Err(_) => false,
                };
                debug!("Succcess: {}", ok);
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
            "enable" => AttrState::Exists,
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
        match name {
            "enable" => self.enable,
            _ => 0,
        }
    }

    fn set_attr_i32(&mut self, name: &str, value: i32) {
        match name {
            "enable" => self.enable = value,
            _ => (),
        };
    }

    fn get_attr_f32(&self, name: &str) -> f32 {
        0.0
    }

    fn set_attr_f32(&mut self, name: &str, value: f32) {
        ()
    }
}
