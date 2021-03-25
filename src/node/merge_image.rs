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
use crate::cxxbridge::ffi::ImageShared;
use crate::cxxbridge::ffi::MergeImageMode;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::NodeType;
use crate::cxxbridge::ffi::PixelDataType;
use crate::data::HashValue;
use crate::data::Identifier;
use crate::hashutils::HashableF32;
use crate::node::traits::Operation;
use crate::node::NodeImpl;
use crate::ops::imagemerge;
use crate::pathutils;
use crate::pixelblock::PixelBlock;
use crate::stream::StreamDataImpl;
use crate::stream::StreamDataImplRc;

pub fn new(id: Identifier) -> NodeImpl {
    NodeImpl {
        node_type: NodeType::MergeImage,
        id,
        status: NodeStatus::Uninitialized,
        compute: Box::new(MergeImageOperation::new()),
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
    let mut copy_b = &mut (**input_b).clone();
    let num_channels = copy_b.pixel_num_channels();
    let width = copy_b.pixel_width();
    let height = copy_b.pixel_height();
    let display_window = copy_b.display_window();
    let transform_matrix = copy_b.transform_matrix().to_na_matrix();
    let src_data_window = copy_b.data_window();
    let mut data_window = copy_b.data_window();
    let mut pixel_block_a = input_a.clone_pixel_block();
    let mut pixel_block_b = copy_b.clone_pixel_block();

    debug!("Merge Convert");
    pixel_block_a.convert_into_f32_data();
    pixel_block_b.convert_into_f32_data();

    let pixel_block_a_box = Box::new(pixel_block_a);
    let image_a = ImageShared {
        pixel_block: pixel_block_a_box,
        display_window: input_a.display_window(),
        data_window: input_a.data_window(),
    };

    let pixel_block_b_box = Box::new(pixel_block_b);
    let image_b = ImageShared {
        pixel_block: pixel_block_b_box,
        display_window: input_b.display_window(),
        data_window: input_b.data_window(),
    };

    let pixel_block_out_box = Box::new(PixelBlock::new_constant_pixel_rgba_f32(0.0, 0.0, 0.0, 1.0));
    let mut image_out = ImageShared {
        pixel_block: pixel_block_out_box,
        display_window: BBox2Di::new(0, 0, 1, 1),
        data_window: BBox2Di::new(0, 0, 1, 1),
    };

    let ok = imagemerge::merge(merge_mode, &image_a, &image_b, mix, &mut image_out);
    debug!("Succcess: {}", ok);

    image_out
}

impl Operation for MergeImageOperation {
    fn compute(
        &mut self,
        frame: i32,
        node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
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
            let hash_value = self.cache_hash(frame, node_type_id, &attr_block, inputs);
            let mut stream_data = StreamDataImpl::new();
            stream_data.set_hash(hash_value);
            *output = std::rc::Rc::new(stream_data);
            return NodeStatus::Warning;
        }
        if inputs.len() != 2 {
            let hash_value = self.cache_hash(frame, node_type_id, &attr_block, inputs);
            let mut stream_data = StreamDataImpl::new();
            stream_data.set_hash(hash_value);
            *output = std::rc::Rc::new(stream_data);
            return NodeStatus::Error;
        }

        let mut stream_data = StreamDataImpl::new();
        let hash_value = self.cache_hash(frame, node_type_id, &attr_block, inputs);

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

        stream_data.set_data_window(data_window);
        stream_data.set_display_window(display_window);
        stream_data.set_hash(hash_value);
        stream_data.set_pixel_block(pixel_block);

        *output = std::rc::Rc::new(stream_data);
        NodeStatus::Valid
    }
}

impl AttrBlock for MergeImageAttrs {
    fn attr_hash(&self, frame: i32, state: &mut DefaultHasher) {
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

    fn get_attr_str(&self, name: &str) -> &str {
        ""
    }

    fn set_attr_str(&mut self, name: &str, value: &str) {
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
