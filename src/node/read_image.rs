/*
 * Copyright (C) 2020, 2021 David Cattermole.
 *
 * This file is part of OpenCompGraph.
 *
 * OpenCompGraph is free software: you can redistribute it and/or modify it
 * under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * OpenCompGraph is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with OpenCompGraph.  If not, see <https://www.gnu.org/licenses/>.
 * ====================================================================
 *
 */

use log::debug;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::path::Path;
use std::rc::Rc;
use std::string::String;

use crate::attrblock::AttrBlock;
use crate::cache::CacheImpl;
use crate::cache::CachedImage;
use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::NodeType;
use crate::data::FrameValue;
use crate::data::HashValue;
use crate::data::Identifier;
use crate::data::NodeComputeMode;
use crate::imageio;
use crate::node::traits::Operation;
use crate::node::traits::Validate;
use crate::node::NodeImpl;
use crate::pathutils;
use crate::stream::StreamDataImpl;

pub fn new(id: Identifier) -> NodeImpl {
    NodeImpl {
        node_type: NodeType::ReadImage,
        id,
        status: NodeStatus::Uninitialized,
        compute: Box::new(ReadImageOperation::new()),
        validate: Box::new(ReadImageValidate::new()),
        attr_block: Box::new(ReadImageAttrs::new()),
    }
}

#[derive(Debug, Clone, Default)]
pub struct ReadImageOperation {}

#[derive(Debug, Clone, Default)]
pub struct ReadImageAttrs {
    pub enable: i32,
    pub file_path: String,
    // TODO: Add more attributes:
    //
    // - Start frame - The first frame number to use.
    // - End frame - the last frame number to use.
    // - Missing frame behavior - What to do when an image is not found.
    //
    // - Pre-multiply
    // - Un-premultiply
    // - Input Color Space
    // - Output channels.
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
        frame: FrameValue,
        _node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        hash_value: HashValue,
        node_compute_mode: NodeComputeMode,
        _inputs: &Vec<Rc<StreamDataImpl>>,
        output: &mut Rc<StreamDataImpl>,
        cache: &mut Box<CacheImpl>,
    ) -> NodeStatus {
        debug!("ReadImageOperation.compute()");
        debug!(
            "ReadImageOperation NodeComputeMode={:#?}",
            node_compute_mode
        );
        // debug!("AttrBlock: {:?}", attr_block);
        // debug!("Inputs: {:?}", inputs);
        // debug!("Output: {:?}", output);

        if node_compute_mode.contains(NodeComputeMode::PIXEL) {
            let enable = attr_block.get_attr_i32("enable");
            if enable != 1 {
                let mut stream_data = StreamDataImpl::new();
                stream_data.set_hash(hash_value);
                *output = std::rc::Rc::new(stream_data);
                return NodeStatus::Warning;
            }
            let file_path = attr_block.get_attr_str("file_path");
            let frame_num = frame.round().trunc() as i32;
            let path_expanded = pathutils::expand_string(file_path.to_string(), frame_num);

            let path = match Path::new(&path_expanded).canonicalize() {
                Ok(full_path) => full_path,
                Err(_) => {
                    // The path could not be canonicalised, probably
                    // meaning the path does not exist.
                    let mut stream_data = StreamDataImpl::new();
                    *output = std::rc::Rc::new(stream_data);
                    return NodeStatus::Warning;
                }
            };

            debug!("Opening... {:?}", path);
            if path.is_file() == true {
                let mut stream_data = StreamDataImpl::new();

                let (pixel_block, image_spec, data_window, display_window) =
                    match cache.get(&hash_value) {
                        Some(cached_img) => {
                            // debug!("Cache Hit");
                            (
                                cached_img.pixel_block.clone(),
                                cached_img.spec.clone(),
                                cached_img.data_window,
                                cached_img.display_window,
                            )
                        }
                        _ => {
                            // debug!("Cache Miss");
                            let num_threads = 0;
                            let img = imageio::read_image(&path_expanded, num_threads);
                            let pixel_block_rc = Rc::new(*img.pixel_block);
                            let cached_img = CachedImage {
                                pixel_block: pixel_block_rc.clone(),
                                spec: img.spec.clone(),
                                data_window: img.data_window,
                                display_window: img.display_window,
                            };
                            cache.insert(hash_value, cached_img);
                            (
                                pixel_block_rc.clone(),
                                img.spec.clone(),
                                img.data_window,
                                img.display_window,
                            )
                        }
                    };

                // debug!(
                //     "pixel_block: {:?} x {:?} x {:?}",
                //     pixel_block.width(),
                //     pixel_block.height(),
                //     pixel_block.num_channels()
                // );

                // debug!("spec: {:?}", image_spec);

                // debug!(
                //     "data_window: {},{} to {},{} | {}x{}",
                //     data_window.min_x,
                //     data_window.min_y,
                //     data_window.max_x,
                //     data_window.max_y,
                //     data_window.width(),
                //     data_window.height(),
                // );

                // debug!(
                //     "display_window: {},{} to {},{} | {}x{}",
                //     display_window.min_x,
                //     display_window.min_y,
                //     display_window.max_x,
                //     display_window.max_y,
                //     display_window.width(),
                //     display_window.height(),
                // );

                stream_data.set_data_window(data_window);
                stream_data.set_display_window(display_window);
                stream_data.set_hash(hash_value);
                stream_data.set_pixel_block(pixel_block);
                stream_data.set_image_spec(image_spec);

                *output = std::rc::Rc::new(stream_data);
            } else {
                let enable = attr_block.get_attr_i32("enable");
                if enable != 1 {
                    let mut stream_data = StreamDataImpl::new();
                    stream_data.set_hash(hash_value);
                    *output = std::rc::Rc::new(stream_data);
                    return NodeStatus::Valid;
                }
            }
        }
        NodeStatus::Valid
    }
}

impl AttrBlock for ReadImageAttrs {
    fn attr_hash(&self, frame: FrameValue, state: &mut DefaultHasher) {
        self.enable.hash(state);
        if self.enable == 1 {
            let frame_num = frame.round().trunc() as i32;
            let path_expanded = pathutils::expand_string(self.file_path.to_string(), frame_num);
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

    fn get_attr_f32(&self, _name: &str) -> f32 {
        0.0
    }

    fn set_attr_f32(&mut self, _name: &str, _value: f32) {
        ()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ReadImageValidate {}

impl ReadImageValidate {
    pub fn new() -> ReadImageValidate {
        ReadImageValidate {}
    }
}

impl Validate for ReadImageValidate {
    fn validate_inputs(
        &self,
        _node_type_id: u8,
        _attr_block: &Box<dyn AttrBlock>,
        hash_value: HashValue,
        node_compute_mode: NodeComputeMode,
        input_nodes: &Vec<&Box<NodeImpl>>,
    ) -> Vec<NodeComputeMode> {
        debug!(
            "ReadImageValidate::validate_inputs(): NodeComputeMode={:#?} HashValue={:#?}",
            node_compute_mode, hash_value
        );
        let mut node_compute_modes = Vec::new();
        if input_nodes.len() > 0 {
            node_compute_modes.push(node_compute_mode & NodeComputeMode::ALL);
            for _ in input_nodes.iter().skip(1) {
                node_compute_modes.push(node_compute_mode & NodeComputeMode::NONE);
            }
        }
        node_compute_modes
    }
}
