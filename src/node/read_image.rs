use image;
use image::GenericImageView;
use image::ImageBuffer;
use log::{debug, error, info, warn};
use rustc_hash::FxHasher;
use std::collections::hash_map::DefaultHasher;
use std::hash;
use std::hash::Hash;
use std::path::Path;
use std::string::String;

use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::BBox2D;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::NodeType;
use crate::cxxbridge::ffi::StreamDataImplShared;
use crate::data::HashValue;
use crate::data::Identifier;
use crate::node::traits::AttrBlock;
use crate::node::traits::Compute;
use crate::node::NodeImpl;
use crate::pixelblock::PixelBlock;

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
    pub enable: i32,
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
            enable: 1,
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
        let enable = attr_block.get_attr_i32("enable");
        if enable != 1 {
            return NodeStatus::Error;
        }

        let file_path = attr_block.get_attr_str("file_path");
        // debug!("file_path {:?}", file_path);

        let path = match Path::new(&file_path).canonicalize() {
            Ok(full_path) => full_path,
            Err(_) => return NodeStatus::Error,
        };
        debug!("Opening... {:?}", path);
        if path.is_file() == true {
            // Read image
            let img = image::open(path).unwrap();
            let pixel_block = PixelBlock::from_dynamic_image(img);

            let display_window = BBox2D::new(
                0.0,
                0.0,
                (pixel_block.width - 1) as f32,
                (pixel_block.height - 1) as f32,
            );
            let data_window = display_window.clone();

            let hash_value = self.cache_hash(node_type_id, &attr_block, inputs);
            output.inner.set_display_window(display_window);
            output.inner.set_data_window(data_window);
            output.inner.set_hash(hash_value);
            output.inner.set_pixel_block(pixel_block);
        }

        NodeStatus::Valid
    }
}

impl AttrBlock for ReadImageAttrs {
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
