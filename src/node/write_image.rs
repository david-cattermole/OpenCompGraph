use std::path::Path;
use std::string::String;

use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::NodeType;
use crate::cxxbridge::ffi::StreamDataImplShared;
use crate::data::Identifier;
use crate::node::traits::{AttrBlock, Compute};
use crate::node::NodeImpl;
use image::RgbaImage;

pub fn new(id: Identifier) -> NodeImpl {
    NodeImpl {
        node_type: NodeType::WriteImage,
        id,
        op_status: NodeStatus::Uninitialized,
        compute: Box::new(WriteImageCompute {}),
        attr_block: Box::new(WriteImageAttrs {
            file_path: "".to_string(),
        }),
    }
}

#[derive(Debug, Clone, Default)]
pub struct WriteImageCompute {}

#[derive(Debug, Clone, Default)]
pub struct WriteImageAttrs {
    pub file_path: String,
}

impl Compute for WriteImageCompute {
    fn hash(
        &mut self,
        id: Identifier,
        node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        inputs: &Vec<StreamDataImplShared>,
    ) -> usize {
        0
    }

    fn compute(
        &mut self,
        attr_block: &Box<dyn AttrBlock>,
        inputs: &Vec<StreamDataImplShared>,
        output: &mut StreamDataImplShared,
    ) -> NodeStatus {
        println!("WriteImageCompute.compute()");
        println!("AttrBlock: {:?}", attr_block);
        println!("Inputs: {:?}", inputs);
        println!("Output: {:?}", output);

        let file_path = attr_block.get_attr_string("file_path");
        println!("file_path {:?}", file_path);

        let img = image::DynamicImage::ImageRgba8(RgbaImage::new(8, 8));
        let path = match Path::new(&file_path).canonicalize() {
            Ok(full_path) => full_path,
            Err(_) => return NodeStatus::Error,
        };
        println!("Writing... {:?}", path);
        let ok = match img.save(path) {
            Ok(value) => true,
            Err(_) => false,
        };
        println!("Succcess: {}", ok);
        NodeStatus::Valid
    }
}

impl AttrBlock for WriteImageAttrs {
    fn attr_exists(&self, name: &str) -> AttrState {
        match name {
            "file_path" => AttrState::Exists,
            _ => AttrState::Missing,
        }
    }

    fn get_attr_string(&self, name: &str) -> &str {
        match name {
            "file_path" => &self.file_path,
            _ => "",
        }
    }

    fn set_attr_string(&mut self, name: &str, value: &str) {
        match name {
            "file_path" => self.file_path = value.to_string(),
            _ => (),
        };
    }
}
