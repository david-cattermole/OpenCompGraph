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
use std::hash;
use std::hash::Hash;
use std::rc::Rc;
use std::string::String;

use crate::attrblock::AttrBlock;
use crate::cache::CacheImpl;
use crate::colorspace;
use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::ImageShared;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::NodeType;
use crate::data::Identifier;
use crate::deformutils;
use crate::imageio;
use crate::node::traits::Operation;
use crate::node::NodeImpl;
use crate::ops;
use crate::pathutils;
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
        _node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        inputs: &Vec<Rc<StreamDataImpl>>,
        output: &mut Rc<StreamDataImpl>,
        _cache: &mut Box<CacheImpl>,
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

                // Copy input data
                let copy = &mut (**input).clone();
                let display_window = copy.display_window();
                let src_data_window = copy.data_window();
                let mut data_window = copy.data_window();
                let mut pixel_block = copy.clone_pixel_block();

                // Convert to f32 data and convert to linear color space.
                //
                // Before applying any changes to the pixels we
                // must ensure we work in 32-bit floating-point linear
                // color space.
                pixel_block.convert_into_f32_data();

                let width = pixel_block.width();
                let height = pixel_block.height();
                let num_channels = pixel_block.num_channels();
                colorspace::color_convert_inplace(
                    &mut pixel_block.as_slice_mut(),
                    width,
                    height,
                    num_channels,
                    &"Utility - sRGB - Texture".to_string(),
                    &"ACES - ACEScg".to_string(),
                );

                {
                    // Apply Deformations.
                    let ref_pixel_block = pixel_block.clone();
                    deformutils::apply_deformers_to_pixels(
                        &copy.deformers(),
                        display_window,
                        &ref_pixel_block,
                        src_data_window,
                        &mut pixel_block,
                        &mut data_window,
                    );
                }

                // Apply Color Matrix
                {
                    let color_matrix = copy.color_matrix().to_na_matrix();
                    let pixels = &mut pixel_block.as_slice_mut();
                    ops::xformcolor::apply_color_matrix_inplace(pixels, num_channels, color_matrix);
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
        self.enable.hash(state);
        if self.enable == 1 {
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

    fn get_attr_f32(&self, _name: &str) -> f32 {
        0.0
    }

    fn set_attr_f32(&mut self, _name: &str, _value: f32) {
        ()
    }
}
