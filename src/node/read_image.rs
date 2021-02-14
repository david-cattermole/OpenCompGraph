use log::{debug, error, info, warn};
use rustc_hash::FxHasher;
use std::collections::hash_map::DefaultHasher;
use std::hash;
use std::hash::Hash;
use std::path::Path;
use std::rc::Rc;
use std::string::String;

use crate::attrblock::AttrBlock;
use crate::cache::CacheImpl;
use crate::cache::CachedImage;
use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::BBox2Df;
use crate::cxxbridge::ffi::BBox2Di;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::NodeType;
use crate::data::HashValue;
use crate::data::Identifier;
use crate::imageio;
use crate::node::traits::Operation;
use crate::node::NodeImpl;
use crate::pathutils;
use crate::pixelblock::PixelBlock;
use crate::stream::StreamDataImpl;
use crate::stream::StreamDataImplRc;

pub fn new(id: Identifier) -> NodeImpl {
    NodeImpl {
        node_type: NodeType::ReadImage,
        id,
        status: NodeStatus::Uninitialized,
        compute: Box::new(ReadImageOperation::new()),
        attr_block: Box::new(ReadImageAttrs::new()),
    }
}

#[derive(Debug, Clone, Default)]
pub struct ReadImageOperation {}

#[derive(Debug, Clone, Default)]
pub struct ReadImageAttrs {
    pub enable: i32,
    pub file_path: String,
}

impl ReadImageOperation {
    pub fn new() -> ReadImageOperation {
        ReadImageOperation {}
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

impl Operation for ReadImageOperation {
    fn compute(
        &mut self,
        frame: i32,
        node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        inputs: &Vec<Rc<StreamDataImpl>>,
        output: &mut Rc<StreamDataImpl>,
        cache: &mut Box<CacheImpl>,
    ) -> NodeStatus {
        debug!("ReadImageOperation.compute()");
        // debug!("AttrBlock: {:?}", attr_block);
        // debug!("Inputs: {:?}", inputs);
        // debug!("Output: {:?}", output);
        let enable = attr_block.get_attr_i32("enable");
        if enable != 1 {
            let hash_value = self.cache_hash(frame, node_type_id, &attr_block, inputs);
            let mut stream_data = StreamDataImpl::new();
            stream_data.set_hash(hash_value);
            *output = std::rc::Rc::new(stream_data);
            return NodeStatus::Warning;
        }
        let file_path = attr_block.get_attr_str("file_path");
        let path_expanded = pathutils::expand_string(file_path.to_string(), frame);
        debug!("file_path: {:?}", file_path);
        debug!("path_expanded: {:?}", path_expanded);

        let path = match Path::new(&path_expanded).canonicalize() {
            Ok(full_path) => full_path,
            Err(_) => {
                let hash_value = self.cache_hash(frame, node_type_id, &attr_block, inputs);

                let mut stream_data = StreamDataImpl::new();
                stream_data.set_hash(hash_value);
                *output = std::rc::Rc::new(stream_data);
                return NodeStatus::Warning;
            }
        };

        debug!("Opening... {:?}", path);
        if path.is_file() == true {
            let mut stream_data = StreamDataImpl::new();

            let hash_value = self.cache_hash(frame, node_type_id, &attr_block, inputs);
            debug!("hash_value: {:?}", hash_value);

            let (pixel_block, data_window, display_window) = match cache.get(&hash_value) {
                Some(cached_img) => {
                    debug!("Cache Hit");
                    (
                        cached_img.pixel_block.clone(),
                        cached_img.data_window,
                        cached_img.display_window,
                    )
                }
                _ => {
                    warn!("Cache Miss");
                    let img = imageio::read_image(&path_expanded);
                    let pixel_block_rc = Rc::new(*img.pixel_block);
                    let cached_img = CachedImage {
                        pixel_block: pixel_block_rc.clone(),
                        data_window: img.data_window,
                        display_window: img.display_window,
                    };
                    cache.insert(hash_value, cached_img);
                    (pixel_block_rc.clone(), img.data_window, img.display_window)
                }
            };

            debug!(
                "pixel_block: {:?} x {:?} x {:?}",
                pixel_block.width(),
                pixel_block.height(),
                pixel_block.num_channels()
            );

            stream_data.set_data_window(data_window);
            stream_data.set_display_window(display_window);
            stream_data.set_hash(hash_value);
            stream_data.set_pixel_block(pixel_block);

            *output = std::rc::Rc::new(stream_data);
        }
        NodeStatus::Valid
    }
}

impl AttrBlock for ReadImageAttrs {
    fn attr_hash(&self, frame: i32, state: &mut DefaultHasher) {
        if self.enable == 1 {
            self.enable.hash(state);
            let path_expanded = pathutils::expand_string(self.file_path.to_string(), frame);
            path_expanded.hash(state);
        }
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
