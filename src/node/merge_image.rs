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
use std::rc::Rc;

use crate::attrblock::AttrBlock;
use crate::cache::CacheImpl;
use crate::cache::CachedImage;
use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::BBox2Di;
use crate::cxxbridge::ffi::BakeOption;
use crate::cxxbridge::ffi::ImageShared;
use crate::cxxbridge::ffi::ImageSpec;
use crate::cxxbridge::ffi::MergeImageMode;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::NodeType;
use crate::cxxbridge::ffi::PixelDataType;
use crate::data::HashValue;
use crate::data::Identifier;
use crate::data::NodeComputeMode;
use crate::data::COLOR_SPACE_NAME_LINEAR;
use crate::hashutils::HashableF32;
use crate::node::traits::Operation;
use crate::node::traits::Validate;
use crate::node::NodeImpl;
use crate::ops::bake;
use crate::ops::imagemerge;
use crate::pixelblock::PixelBlock;
use crate::stream::StreamDataImpl;

pub fn new(id: Identifier) -> NodeImpl {
    NodeImpl {
        node_type: NodeType::MergeImage,
        id,
        status: NodeStatus::Uninitialized,
        compute: Box::new(MergeImageOperation::new()),
        validate: Box::new(MergeImageValidate::new()),
        attr_block: Box::new(MergeImageAttrs::new()),
    }
}

#[derive(Debug, Clone, Default)]
pub struct MergeImageOperation {}

#[derive(Debug, Clone, Default)]
pub struct MergeImageAttrs {
    pub enable: i32,
    pub mode: i32, // index for a MergeImageMode.
    pub mix: f32,
}

impl MergeImageOperation {
    pub fn new() -> MergeImageOperation {
        MergeImageOperation {}
    }
}

impl MergeImageAttrs {
    pub fn new() -> MergeImageAttrs {
        MergeImageAttrs {
            enable: 1,
            mode: 1, // 1 = 'A Over B'
            mix: 1.0,
        }
    }
}

fn do_image_process(
    inputs: &Vec<Rc<StreamDataImpl>>,
    merge_mode: MergeImageMode,
    mix: f32,
) -> ImageShared {
    debug_assert!(inputs.len() == 2);
    let input_a = &inputs[0].clone();
    let src_a_pixel_block = input_a.pixel_block();
    debug!(
        "Merge Source A: {} x {} : {}",
        src_a_pixel_block.width(),
        src_a_pixel_block.height(),
        src_a_pixel_block.num_channels()
    );

    let input_b = &inputs[1].clone();
    let src_b_pixel_block = input_b.pixel_block();
    debug!(
        "Merge Source B: {} x {} : {}",
        src_b_pixel_block.width(),
        src_b_pixel_block.height(),
        src_b_pixel_block.num_channels()
    );

    // Copy input data
    let mut copy_a = &mut (**input_a).clone();
    let mut copy_b = &mut (**input_b).clone();
    let mut pixel_block_a = copy_a.clone_pixel_block();
    let mut pixel_block_b = copy_b.clone_pixel_block();
    let mut image_spec_a = copy_a.clone_image_spec();
    let mut image_spec_b = copy_b.clone_image_spec();

    let bake_option = BakeOption::All;
    let from_color_space_a = &image_spec_a.color_space();
    let from_color_space_b = &image_spec_b.color_space();
    let to_color_space = COLOR_SPACE_NAME_LINEAR.to_string();

    // Stream A
    let display_window_a = copy_a.display_window();
    let mut data_window_a = copy_a.data_window();
    bake::do_process(
        bake_option,
        &mut pixel_block_a,
        display_window_a,
        &mut data_window_a,
        &mut image_spec_a,
        &mut copy_a,
        &from_color_space_a,
        &to_color_space,
        PixelDataType::Float32,
    );
    let pixel_block_a_box = Box::new(pixel_block_a);
    let image_a = ImageShared {
        pixel_block: pixel_block_a_box,
        display_window: display_window_a,
        data_window: data_window_a,
        spec: image_spec_a,
    };

    // Stream B
    let display_window_b = copy_b.display_window();
    let mut data_window_b = copy_b.data_window();
    bake::do_process(
        bake_option,
        &mut pixel_block_b,
        display_window_b,
        &mut data_window_b,
        &mut image_spec_b,
        &mut copy_b,
        &from_color_space_b,
        &to_color_space,
        PixelDataType::Float32,
    );
    let pixel_block_b_box = Box::new(pixel_block_b);
    let image_b = ImageShared {
        pixel_block: pixel_block_b_box,
        display_window: input_b.display_window(),
        data_window: input_b.data_window(),
        spec: image_spec_b,
    };

    // Merge images
    let pixel_block_out = PixelBlock::new_constant_pixel_rgba_f32(0.0, 0.0, 0.0, 1.0);
    let mut image_out = ImageShared {
        pixel_block: Box::new(pixel_block_out),
        display_window: BBox2Di::new(0, 0, 1, 1),
        data_window: BBox2Di::new(0, 0, 1, 1),
        spec: ImageSpec::new(),
    };
    let ok = imagemerge::merge(merge_mode, &image_a, &image_b, mix, &mut image_out);
    debug!("merge_image Succcess: {}", ok);

    image_out
}

impl Operation for MergeImageOperation {
    fn compute(
        &mut self,
        _frame: i32,
        _node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        hash_value: HashValue,
        _node_compute_mode: NodeComputeMode,
        inputs: &Vec<Rc<StreamDataImpl>>,
        output: &mut Rc<StreamDataImpl>,
        cache: &mut Box<CacheImpl>,
    ) -> NodeStatus {
        debug!("MergeImageOperation.compute()");
        // debug!("AttrBlock: {:?}", attr_block);
        // debug!("Inputs: {:?}", inputs);
        // debug!("Output: {:?}", output);

        let enable = attr_block.get_attr_i32("enable");
        if enable != 1 {
            let mut stream_data = StreamDataImpl::new();
            stream_data.set_hash(hash_value);
            *output = std::rc::Rc::new(stream_data);
            return NodeStatus::Warning;
        }
        if inputs.len() != 2 {
            let mut stream_data = StreamDataImpl::new();
            stream_data.set_hash(hash_value);
            *output = std::rc::Rc::new(stream_data);
            return NodeStatus::Error;
        }

        // Cache the results of the merge. If the input values do not
        // change we can easily look up the pixels again.
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
                debug!("Cache Miss");
                let merge_mode = MergeImageMode::from(attr_block.get_attr_i32("mode"));
                let mix = attr_block.get_attr_f32("mix");
                let img = do_image_process(inputs, merge_mode, mix);

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
        };

        debug!(
            "pixel_block: {:?} x {:?} x {:?}",
            pixel_block.width(),
            pixel_block.height(),
            pixel_block.num_channels()
        );

        debug!(
            "data_window: {},{} to {},{} | {}x{}",
            data_window.min_x,
            data_window.min_y,
            data_window.max_x,
            data_window.max_y,
            data_window.width(),
            data_window.height(),
        );

        debug!(
            "display_window: {},{} to {},{} | {}x{}",
            display_window.min_x,
            display_window.min_y,
            display_window.max_x,
            display_window.max_y,
            display_window.width(),
            display_window.height(),
        );

        let mut stream_data = StreamDataImpl::new();
        stream_data.set_data_window(data_window);
        stream_data.set_display_window(display_window);
        stream_data.set_hash(hash_value);
        stream_data.set_pixel_block(pixel_block);

        *output = std::rc::Rc::new(stream_data);
        NodeStatus::Valid
    }
}

impl AttrBlock for MergeImageAttrs {
    fn attr_hash(&self, _frame: i32, state: &mut DefaultHasher) {
        if self.enable == 1 {
            self.enable.hash(state);
            self.mode.hash(state);
            HashableF32::new(self.mix).hash(state);
        }
    }

    fn attr_exists(&self, name: &str) -> AttrState {
        match name {
            "enable" => AttrState::Exists,
            "mode" => AttrState::Exists,
            "mix" => AttrState::Exists,
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
            "mode" => self.mode,
            _ => 0,
        }
    }

    fn set_attr_i32(&mut self, name: &str, value: i32) {
        match name {
            "enable" => self.enable = value,
            "mode" => self.mode = value,
            _ => (),
        };
    }

    fn get_attr_f32(&self, name: &str) -> f32 {
        match name {
            "mix" => self.mix,
            _ => 0.0,
        }
    }

    fn set_attr_f32(&mut self, name: &str, value: f32) {
        match name {
            "mix" => self.mix = value,
            _ => (),
        };
    }
}

#[derive(Debug, Clone, Default)]
pub struct MergeImageValidate {}

impl MergeImageValidate {
    pub fn new() -> MergeImageValidate {
        MergeImageValidate {}
    }
}

impl Validate for MergeImageValidate {
    fn validate_inputs(
        &self,
        _node_type_id: u8,
        _attr_block: &Box<dyn AttrBlock>,
        hash_value: HashValue,
        node_compute_mode: NodeComputeMode,
        input_nodes: &Vec<&Box<NodeImpl>>,
    ) -> Vec<NodeComputeMode> {
        debug!(
            "MergeImageValidate::validate_inputs(): NodeComputeMode={:#?} HashValue={:#?}",
            node_compute_mode, hash_value
        );
        let mut node_compute_modes = Vec::new();
        if input_nodes.len() > 0 {
            node_compute_modes.push(node_compute_mode & NodeComputeMode::ALL);
            if input_nodes.len() >= 2 {
                node_compute_modes.push(node_compute_mode & NodeComputeMode::ALL);
                for _ in input_nodes.iter().skip(2) {
                    node_compute_modes.push(node_compute_mode & NodeComputeMode::NONE);
                }
            }
        }
        node_compute_modes
    }
}
