use std::path::Path;
use std::string::String;

use image;
use image::GenericImageView;

use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::NodeType;
use crate::cxxbridge::ffi::StreamDataImplShared;
use crate::data::Identifier;
use crate::node::traits::{AttrBlock, Compute};
use crate::node::NodeImpl;

pub fn new(id: Identifier) -> NodeImpl {
    NodeImpl {
        node_type: NodeType::ReadImage,
        id,
        op_status: NodeStatus::Uninitialized,
        compute: Box::new(ReadImageCompute {}),
        attr_block: Box::new(ReadImageAttrs {
            file_path: "".to_string(),
        }),
    }
}

#[derive(Debug, Clone, Default)]
pub struct ReadImageCompute {}

#[derive(Debug, Clone, Default)]
pub struct ReadImageAttrs {
    pub file_path: String,
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
        println!("AttrBlock: {:?}", attr_block);
        println!("Inputs: {:?}", inputs);
        println!("Output: {:?}", output);

        let file_path = attr_block.get_attr_string("file_path");
        println!("file_path {:?}", file_path);

        let path = match Path::new(&file_path).canonicalize() {
            Ok(full_path) => full_path,
            Err(_) => return NodeStatus::Error,
        };
        println!("Opening... {:?}", path);
        if path.is_file() == true {
            let img = image::open(path).unwrap();

            // The dimensions method returns the images width and height.
            println!("dimensions {:?}", img.dimensions());

            // The color method returns the image's `ColorType`.
            println!("{:?}", img.color());
        }

        // virtual void compute() {
        //     // This is the fallback node output.
        //     Result res = ocg::NodeHelper::createEmptyResult();
        //
        //     // Read Image here.
        //     if (m_path.find(".jpg") != -1) {
        //         int image_width = 0;
        //         int image_height = 0;
        //         void *data = jpeg::read_file(
        //             m_path.c_str(),
        //             image_width,
        //             image_height);
        //
        //         PixelBlock pixels;
        //         pixels.data = data;
        //
        //         BoundingBox displayBox;
        //         displayBox.min.x = 0;
        //         displayBox.min.y = 0;
        //         displayBox.max.x = image_width;
        //         displayBox.max.y = image_height;
        //
        //         Image img;
        //         img.pixelBlock = pixels;
        //         img.displayWindow = displayBox;
        //
        //         m_result.setImage(img);
        //     }
        // };

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
