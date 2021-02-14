use log::{debug, error, info, log_enabled, warn, Level};
use std::collections::hash_map::DefaultHasher;
use std::hash;
use std::hash::Hash;
use std::path::Path;
use std::rc::Rc;
use std::string::String;

use crate::attrblock::AttrBlock;
use crate::cache::CacheImpl;
use crate::colorutils;
use crate::colorutils::convert_linear_to_srgb;
use crate::colorxform;
use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::ImageShared;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::NodeType;
use crate::data::HashValue;
use crate::data::Identifier;
use crate::deformutils;
use crate::imageio;
use crate::node::traits::Operation;
use crate::node::NodeImpl;
use crate::pathutils;
use crate::pixelblock::PixelBlock;
use crate::stream::StreamDataImpl;

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

#[derive(Debug, Clone, Default, hash::Hash)]
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
        frame: i32,
        node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        inputs: &Vec<Rc<StreamDataImpl>>,
        output: &mut Rc<StreamDataImpl>,
        cache: &mut Box<CacheImpl>,
    ) -> NodeStatus {
        debug!("WriteImageOperation.compute()");
        // debug!("AttrBlock: {:?}", attr_block);
        // debug!("Inputs: {:?}", inputs);
        // debug!("Output: {:?}", output);
        let enable = attr_block.get_attr_i32("enable");
        if enable != 1 {
            return NodeStatus::Valid;
        }

        match inputs.len() {
            0 => NodeStatus::Error,
            _ => {
                let input = &inputs[0].clone();
                let src_pixel_block = input.pixel_block();

                // Copy input data
                let mut copy = &mut (**input).clone();
                let num_channels = copy.pixel_num_channels();
                let width = copy.pixel_width();
                let height = copy.pixel_height();
                let display_window = copy.display_window();
                let data_window = copy.data_window();
                let transform_matrix = copy.transform_matrix().to_na_matrix();
                let mut pixel_block = copy.clone_pixel_block();

                {
                    // Apply Deformations.
                    //
                    // TODO: Apply 'transform_matrix' as part of the
                    // deformation, so we only resample the image once.
                    //
                    // TODO: Determine the destination size and
                    // pre-allocate memory for it.
                    let src_pixels = src_pixel_block.as_slice();
                    let mut pixels = &mut pixel_block.as_slice_mut();
                    deformutils::apply_deformers_to_pixels(
                        &copy.deformers(),
                        // display_window,
                        // data_window,
                        width,
                        height,
                        num_channels,
                        &src_pixels,
                        width,
                        height,
                        num_channels,
                        &mut pixels[..],
                    );
                }

                // Apply Color Matrix
                {
                    let color_matrix = copy.color_matrix().to_na_matrix();
                    let mut pixels = &mut pixel_block.as_slice_mut();
                    colorutils::apply_color_matrix_inplace(pixels, num_channels, color_matrix);
                }

                let file_path = attr_block.get_attr_str("file_path");
                let path_expanded = pathutils::expand_string(file_path.to_string(), frame);
                debug!("file_path: {:?}", file_path);
                debug!("path_expanded: {:?}", path_expanded);

                // Write pixels
                *output = inputs[0].clone();
                let pixel_block_box = Box::new(pixel_block);
                let image = ImageShared {
                    pixel_block: pixel_block_box,
                    display_window,
                    data_window,
                };
                let ok = imageio::write_image(&image, &path_expanded);
                debug!("Succcess: {}", ok);
                NodeStatus::Valid
            }
        }
    }
}

impl AttrBlock for WriteImageAttrs {
    fn attr_hash(&self, frame: i32, state: &mut DefaultHasher) {
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
