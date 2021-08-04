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

use log::debug;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::rc::Rc;

use crate::attrblock::AttrBlock;
use crate::cache::CacheImpl;
use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::BBox2Di;
use crate::cxxbridge::ffi::ImageShared;
use crate::cxxbridge::ffi::ImageSpec;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::NodeType;
use crate::data::Identifier;
use crate::node::traits::Operation;
use crate::node::NodeImpl;
use crate::ops;
use crate::pixelblock::PixelBlock;
use crate::stream::StreamDataImpl;

pub fn new(id: Identifier) -> NodeImpl {
    NodeImpl {
        node_type: NodeType::CropImage,
        id,
        status: NodeStatus::Uninitialized,
        compute: Box::new(CropImageOperation::new()),
        attr_block: Box::new(CropImageAttrs::new()),
    }
}

#[derive(Debug, Clone, Default)]
pub struct CropImageOperation {}

#[derive(Debug, Clone, Default)]
pub struct CropImageAttrs {
    pub enable: i32,
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

impl Operation for CropImageOperation {
    fn compute(
        &mut self,
        frame: i32,
        node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        inputs: &Vec<Rc<StreamDataImpl>>,
        output: &mut Rc<StreamDataImpl>,
        _cache: &mut Box<CacheImpl>,
    ) -> NodeStatus {
        debug!("CropImageOperation.compute()");
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
        if inputs.len() != 1 {
            let hash_value = self.cache_hash(frame, node_type_id, &attr_block, inputs);
            let mut stream_data = StreamDataImpl::new();
            stream_data.set_hash(hash_value);
            *output = std::rc::Rc::new(stream_data);
            return NodeStatus::Error;
        }

        let mut stream_data = StreamDataImpl::new();
        let hash_value = self.cache_hash(frame, node_type_id, &attr_block, inputs);

        // Cache the results of the crop. If the input values do not
        // change we can easily look up the pixels again.
        let window_min_x = attr_block.get_attr_i32("window_min_x");
        let window_min_y = attr_block.get_attr_i32("window_min_y");
        let window_max_x = attr_block.get_attr_i32("window_max_x");
        let window_max_y = attr_block.get_attr_i32("window_max_y");
        let crop_window = BBox2Di::new(window_min_x, window_min_y, window_max_x, window_max_y);

        debug_assert!(inputs.len() == 1);

        let input = &inputs[0].clone();
        let mut input_pixel_block = input.clone_pixel_block();

        // TODO: 'from' should come from the input stream, and 'to'
        // should be a common value in the graph.
        // let metadata = input.metadata();
        // let from_color_space = metadata.color_space();
        let from_color_space = "sRGB".to_string();
        let to_color_space = "ACES - ACEScg".to_string();

        let display_window = input.display_window();
        let mut data_window = input.data_window();
        ops::bake::do_process(
            &mut input_pixel_block,
            display_window,
            &mut data_window,
            &input.deformers(),
            input.color_matrix(),
            from_color_space,
            to_color_space,
        );

        let mut img = ImageShared {
            pixel_block: Box::new(input_pixel_block),
            display_window: display_window,
            data_window: data_window,
            spec: ImageSpec::new(),
        };

        let reformat = attr_block.get_attr_i32("reformat") == 1;
        let black_outside = attr_block.get_attr_i32("black_outside") == 1;
        let intersect = attr_block.get_attr_i32("intersect") == 1;
        let ok = ops::imagecrop::crop_image_in_place(
            &mut img,
            crop_window,
            reformat,
            black_outside,
            intersect,
        );

        let pixel_block_rc = Rc::new(*img.pixel_block);
        let (pixel_block, data_window, display_window) =
            (pixel_block_rc.clone(), img.data_window, img.display_window);

        stream_data.set_data_window(data_window);
        stream_data.set_display_window(display_window);
        stream_data.set_hash(hash_value);
        stream_data.set_pixel_block(pixel_block);

        *output = std::rc::Rc::new(stream_data);
        NodeStatus::Valid
    }
}

impl AttrBlock for CropImageAttrs {
    fn attr_hash(&self, _frame: i32, state: &mut DefaultHasher) {
        if self.enable == 1 {
            self.enable.hash(state);
            self.window_min_x.hash(state);
            self.window_min_y.hash(state);
            self.window_max_x.hash(state);
            self.window_max_y.hash(state);
            self.reformat.hash(state);
            self.black_outside.hash(state);
            self.intersect.hash(state);
        }
    }

    fn attr_exists(&self, name: &str) -> AttrState {
        match name {
            "enable" => AttrState::Exists,
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
