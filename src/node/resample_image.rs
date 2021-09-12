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
use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::BBox2Di;
use crate::cxxbridge::ffi::ImageShared;
use crate::cxxbridge::ffi::ImageSpec;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::NodeType;
use crate::cxxbridge::ffi::PixelDataType;
use crate::data::FrameValue;
use crate::data::HashValue;
use crate::data::Identifier;
use crate::data::NodeComputeMode;
use crate::node::traits::Operation;
use crate::node::traits::Validate;
use crate::node::NodeImpl;
use crate::ops::imageresample;
use crate::pixelblock::PixelBlock;
use crate::stream::StreamDataImpl;

pub fn new(id: Identifier) -> NodeImpl {
    NodeImpl {
        node_type: NodeType::ResampleImage,
        id,
        status: NodeStatus::Uninitialized,
        compute: Box::new(ResampleImageOperation::new()),
        validate: Box::new(ResampleImageValidate::new()),
        attr_block: Box::new(ResampleImageAttrs::new()),
    }
}

#[derive(Debug, Clone, Default)]
pub struct ResampleImageOperation {}

#[derive(Debug, Clone, Default)]
pub struct ResampleImageAttrs {
    pub enable: i32,
    pub factor: i32,
    pub interpolate: i32, // 0 = off, 1 = on.
}

impl ResampleImageOperation {
    pub fn new() -> ResampleImageOperation {
        ResampleImageOperation {}
    }
}

impl ResampleImageAttrs {
    pub fn new() -> ResampleImageAttrs {
        ResampleImageAttrs {
            enable: 1,
            factor: 0,      // 0 = no change, negative is down-res, positive is up-res.
            interpolate: 0, // 0 = do not interpolate.
        }
    }
}

impl Operation for ResampleImageOperation {
    fn compute(
        &mut self,
        _frame: FrameValue,
        _node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        hash_value: HashValue,
        node_compute_mode: NodeComputeMode,
        inputs: &Vec<Rc<StreamDataImpl>>,
        output: &mut Rc<StreamDataImpl>,
        _cache: &mut Box<CacheImpl>,
    ) -> NodeStatus {
        debug!("ResampleImageOperation.compute()");
        debug!(
            "ResampleImageOperation NodeComputeMode={:#?}",
            node_compute_mode
        );
        debug!("AttrBlock: {:?}", attr_block);
        debug!("Inputs: {:?}", inputs);
        debug!("Output: {:?}", output);

        if inputs.len() == 0 {
            // No input given, return an empty default stream.
            let stream_data = StreamDataImpl::new();
            *output = std::rc::Rc::new(stream_data);
            return NodeStatus::Warning;
        }

        let enable = attr_block.get_attr_i32("enable");
        if enable != 1 {
            let stream_data = StreamDataImpl::new();
            *output = std::rc::Rc::new(stream_data);
            return NodeStatus::Valid;
        }

        // TODO: Use the given cache to search for an already existing
        // image.

        let mut status = NodeStatus::Valid;

        debug_assert!(inputs.len() == 1);
        let input = &inputs[0].clone();
        let mut stream_data = (**input).clone();
        stream_data.set_hash(hash_value);

        // Source image.
        let src_pixel_block = stream_data.clone_pixel_block();
        let src_image_spec = stream_data.clone_image_spec();
        let mut src_img = ImageShared {
            pixel_block: Box::new(src_pixel_block),
            display_window: stream_data.display_window(),
            data_window: stream_data.data_window(),
            spec: src_image_spec,
        };

        // Destination image.
        let mut dst_img = ImageShared {
            pixel_block: Box::new(PixelBlock::empty(PixelDataType::Float32)),
            display_window: BBox2Di::new(0, 0, 0, 0),
            data_window: BBox2Di::new(0, 0, 0, 0),
            spec: ImageSpec::new(),
        };

        let factor = attr_block.get_attr_i32("factor");
        if factor != 0 {
            let interpolate = attr_block.get_attr_i32("interpolate") != 0;
            let ok = imageresample::image_resample(&mut src_img, &mut dst_img, factor, interpolate);
            if ok == false {
                error!("ResampleImage failed!");
                status = NodeStatus::Error;
            }
        }

        let pixel_block = Rc::new(*dst_img.pixel_block);
        let data_window = dst_img.data_window;
        let display_window = dst_img.display_window;

        stream_data.set_data_window(data_window);
        stream_data.set_display_window(display_window);
        stream_data.set_hash(hash_value);
        stream_data.set_pixel_block(pixel_block);

        *output = std::rc::Rc::new(stream_data);
        status
    }
}

impl AttrBlock for ResampleImageAttrs {
    fn attr_hash(&self, _frame: FrameValue, state: &mut DefaultHasher) {
        if self.enable == 1 {
            self.enable.hash(state);
            if self.factor != 0 {
                self.factor.hash(state);
                self.interpolate.hash(state);
            }
        }
    }

    fn attr_exists(&self, name: &str) -> AttrState {
        match name {
            "enable" => AttrState::Exists,
            "factor" => AttrState::Exists,
            "interpolate" => AttrState::Exists,
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
            "factor" => self.factor,
            "interpolate" => self.interpolate,
            _ => 0,
        }
    }

    fn set_attr_i32(&mut self, name: &str, value: i32) {
        match name {
            "enable" => self.enable = value,
            "factor" => self.factor = value,
            "interpolate" => self.interpolate = value,
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
pub struct ResampleImageValidate {}

impl ResampleImageValidate {
    pub fn new() -> ResampleImageValidate {
        ResampleImageValidate {}
    }
}

impl Validate for ResampleImageValidate {
    fn validate_inputs(
        &self,
        _node_type_id: u8,
        _attr_block: &Box<dyn AttrBlock>,
        hash_value: HashValue,
        node_compute_mode: NodeComputeMode,
        input_nodes: &Vec<&Box<NodeImpl>>,
    ) -> Vec<NodeComputeMode> {
        debug!(
            "ResampleImageValidate::validate_inputs(): NodeComputeMode={:#?} HashValue={:#?}",
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
