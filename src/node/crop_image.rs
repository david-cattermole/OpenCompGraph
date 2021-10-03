/*
 * Copyright (C) 2021 David Cattermole.
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

use log::{debug, error};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::rc::Rc;

use crate::attrblock::AttrBlock;
use crate::cache::CacheImpl;
use crate::cache::CachedImage;
use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::BBox2Di;
use crate::cxxbridge::ffi::BakeOption;
use crate::cxxbridge::ffi::DataType;
use crate::cxxbridge::ffi::ImageShared;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::NodeType;
use crate::data::FrameValue;
use crate::data::HashValue;
use crate::data::Identifier;
use crate::data::NodeComputeMode;
use crate::data::COLOR_SPACE_NAME_LINEAR;
use crate::node::traits::Operation;
use crate::node::traits::Validate;
use crate::node::NodeImpl;
use crate::ops::bake;
use crate::ops::imagecrop;
use crate::stream::StreamDataImpl;

pub fn new(id: Identifier) -> NodeImpl {
    NodeImpl {
        node_type: NodeType::CropImage,
        id,
        status: NodeStatus::Uninitialized,
        compute: Box::new(CropImageOperation::new()),
        validate: Box::new(CropImageValidate::new()),
        attr_block: Box::new(CropImageAttrs::new()),
    }
}

#[derive(Debug, Clone, Default)]
pub struct CropImageOperation {}

#[derive(Debug, Clone, Default)]
pub struct CropImageAttrs {
    pub enable: i32,
    pub use_cache: i32,
    pub window_min_x: i32,
    pub window_min_y: i32,
    pub window_max_x: i32,
    pub window_max_y: i32,
    pub reformat: i32,      // 0 = off, 1 = on.
    pub black_outside: i32, // 0 = off, 1 = on.
    pub intersect: i32,     // 0 = off, 1 = on.
}

impl CropImageOperation {
    pub fn new() -> CropImageOperation {
        CropImageOperation {}
    }
}

impl CropImageAttrs {
    pub fn new() -> CropImageAttrs {
        CropImageAttrs {
            enable: 1,
            use_cache: 0,
            window_min_x: 0,
            window_min_y: 0,
            window_max_x: 0,
            window_max_y: 0,
            reformat: 0,
            black_outside: 0,
            intersect: 0,
        }
    }
}

fn do_image_process(
    stream_data: &mut StreamDataImpl,
    crop_window: BBox2Di,
    reformat: bool,
    black_outside: bool,
    intersect: bool,
) -> (bool, ImageShared) {
    // Source image.
    let mut pixel_block = stream_data.clone_pixel_block();
    let mut image_spec = stream_data.clone_image_spec();

    // 'from' comes from the input stream, and 'to' is a common
    // value in the graph.
    let from_color_space = image_spec.color_space.clone();
    let to_color_space = COLOR_SPACE_NAME_LINEAR.to_string();

    let display_window = stream_data.display_window();
    let mut data_window = stream_data.data_window();
    let bake_option = BakeOption::All;
    bake::do_process(
        bake_option,
        &mut pixel_block,
        display_window,
        &mut data_window,
        &mut image_spec,
        stream_data,
        &from_color_space,
        &to_color_space,
        DataType::Float32,
    );

    let mut dst_img = ImageShared {
        pixel_block: Box::new(pixel_block),
        display_window: display_window,
        data_window: data_window,
        spec: image_spec,
    };

    let ok = imagecrop::crop_image_in_place(
        &mut dst_img,
        crop_window,
        reformat,
        black_outside,
        intersect,
    );
    (ok, dst_img)
}

impl Operation for CropImageOperation {
    fn compute(
        &mut self,
        _frame: FrameValue,
        _node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        hash_value: HashValue,
        node_compute_mode: NodeComputeMode,
        inputs: &Vec<Rc<StreamDataImpl>>,
        output: &mut Rc<StreamDataImpl>,
        cache: &mut Box<CacheImpl>,
    ) -> NodeStatus {
        debug!("CropImageOperation.compute()");
        debug!(
            "CropImageOperation NodeComputeMode={:#?}",
            node_compute_mode
        );
        // debug!("AttrBlock: {:?}", attr_block);
        // debug!("Inputs: {:?}", inputs);
        // debug!("Output: {:?}", output);

        if inputs.len() == 0 {
            // No input given, return an empty default stream.
            let stream_data = StreamDataImpl::new();
            *output = std::rc::Rc::new(stream_data);
            return NodeStatus::Warning;
        }

        let input = &inputs[0].clone();
        let mut stream_data = (**input).clone();
        let enable = attr_block.get_attr_i32("enable");
        if enable != 1 {
            *output = std::rc::Rc::new(stream_data);
            return NodeStatus::Valid;
        }

        let window_min_x = attr_block.get_attr_i32("window_min_x");
        let window_min_y = attr_block.get_attr_i32("window_min_y");
        let window_max_x = attr_block.get_attr_i32("window_max_x");
        let window_max_y = attr_block.get_attr_i32("window_max_y");
        let crop_window = BBox2Di::new(window_min_x, window_min_y, window_max_x, window_max_y);

        let reformat = attr_block.get_attr_i32("reformat") == 1;
        let black_outside = attr_block.get_attr_i32("black_outside") == 1;
        let intersect = attr_block.get_attr_i32("intersect") == 1;

        let mut status = NodeStatus::Valid;
        let use_cache = attr_block.get_attr_i32("use_cache") != 0;
        let (pixel_block, data_window, display_window) = match use_cache {
            true => match cache.get(&hash_value) {
                Some(cached_img) => {
                    debug!("Cache Hit");
                    (
                        cached_img.pixel_block.clone(),
                        cached_img.data_window,
                        cached_img.display_window,
                    )
                }
                _ => {
                    debug!("Cache Miss");

                    // Cache the results of the crop. If the input values do not
                    // change we can easily look up the pixels again.
                    let (ok, img) = do_image_process(
                        &mut stream_data,
                        crop_window,
                        reformat,
                        black_outside,
                        intersect,
                    );
                    if ok == false {
                        error!("CropImage failed!");
                        status = NodeStatus::Error;
                    }

                    let pixel_block_rc = Rc::new(*img.pixel_block);
                    let cached_img = CachedImage {
                        pixel_block: pixel_block_rc.clone(),
                        spec: img.spec,
                        data_window: img.data_window,
                        display_window: img.display_window,
                    };
                    cache.insert(hash_value, cached_img);
                    (pixel_block_rc.clone(), img.data_window, img.display_window)
                }
            },
            false => {
                let (ok, img) = do_image_process(
                    &mut stream_data,
                    crop_window,
                    reformat,
                    black_outside,
                    intersect,
                );
                if ok == false {
                    error!("CropImage failed!");
                    status = NodeStatus::Error;
                }

                let pixel_block_rc = Rc::new(*img.pixel_block);
                (pixel_block_rc.clone(), img.data_window, img.display_window)
            }
        };

        stream_data.set_hash(hash_value);
        stream_data.set_data_window(data_window);
        stream_data.set_display_window(display_window);
        stream_data.set_pixel_block(pixel_block);

        *output = std::rc::Rc::new(stream_data);
        status
    }
}

impl AttrBlock for CropImageAttrs {
    fn attr_hash(&self, _frame: FrameValue, state: &mut DefaultHasher) {
        self.enable.hash(state);
        if self.enable == 0 {
            return;
        }
        self.window_min_x.hash(state);
        self.window_min_y.hash(state);
        self.window_max_x.hash(state);
        self.window_max_y.hash(state);
        self.reformat.hash(state);
        self.black_outside.hash(state);
        self.intersect.hash(state);
    }

    fn attr_exists(&self, name: &str) -> AttrState {
        match name {
            "enable" => AttrState::Exists,
            "use_cache" => AttrState::Exists,
            "window_min_x" => AttrState::Exists,
            "window_min_y" => AttrState::Exists,
            "window_max_x" => AttrState::Exists,
            "window_max_y" => AttrState::Exists,
            "reformat" => AttrState::Exists,
            "black_outside" => AttrState::Exists,
            "intersect" => AttrState::Exists,
            _ => AttrState::Missing,
        }
    }

    fn get_attr_str(&self, _name: &str) -> &str {
        ""
    }

    fn set_attr_str(&mut self, _name: &str, _value: &str) {
        ()
    }

    fn get_attr_i32(&self, name: &str) -> i32 {
        match name {
            "enable" => self.enable,
            "use_cache" => self.use_cache,
            "window_min_x" => self.window_min_x,
            "window_min_y" => self.window_min_y,
            "window_max_x" => self.window_max_x,
            "window_max_y" => self.window_max_y,
            "reformat" => self.reformat,
            "intersect" => self.intersect,
            "black_outside" => self.black_outside,
            _ => 0,
        }
    }

    fn set_attr_i32(&mut self, name: &str, value: i32) {
        match name {
            "enable" => self.enable = value,
            "use_cache" => self.use_cache = value,
            "window_min_x" => self.window_min_x = value,
            "window_min_y" => self.window_min_y = value,
            "window_max_x" => self.window_max_x = value,
            "window_max_y" => self.window_max_y = value,
            "reformat" => self.reformat = value,
            "black_outside" => self.black_outside = value,
            "intersect" => self.intersect = value,
            _ => (),
        };
    }

    fn get_attr_f32(&self, name: &str) -> f32 {
        match name {
            _ => 0.0,
        }
    }

    fn set_attr_f32(&mut self, name: &str, _value: f32) {
        match name {
            _ => (),
        };
    }
}

#[derive(Debug, Clone, Default)]
pub struct CropImageValidate {}

impl CropImageValidate {
    pub fn new() -> CropImageValidate {
        CropImageValidate {}
    }
}

impl Validate for CropImageValidate {
    fn validate_inputs(
        &self,
        _node_type_id: u8,
        _attr_block: &Box<dyn AttrBlock>,
        hash_value: HashValue,
        node_compute_mode: NodeComputeMode,
        input_nodes: &Vec<&Box<NodeImpl>>,
    ) -> Vec<NodeComputeMode> {
        debug!(
            "CropImageValidate::validate_inputs(): NodeComputeMode={:#?} HashValue={:#?}",
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
