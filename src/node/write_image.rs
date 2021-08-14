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

use log::{debug, warn};
use std::collections::hash_map::DefaultHasher;
use std::hash;
use std::hash::Hash;
use std::rc::Rc;
use std::string::String;

use crate::attrblock::AttrBlock;
use crate::cache::CacheImpl;
use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::BakeOption;
use crate::cxxbridge::ffi::ImageShared;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::NodeType;
use crate::data::Identifier;
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
    // Crop the input image the display window before writing?
    //
    // '-1' = Auto.
    // '0' = disabled.
    // '1' = enabled.
    //
    // When 'auto' is enabled, if the image format to be writen does
    // not support a data windows, the image is cropped, otherwise it
    // is not.
    pub crop_on_write: i32,
    // TODO: Add attributes:
    //
    // - Pre-multiply (?)
    // - Output Channels
    // - Output colour space.
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
            crop_on_write: -1,
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

                let file_path = attr_block.get_attr_str("file_path");
                let path_expanded = pathutils::expand_string(file_path.to_string(), frame);
                // debug!("file_path: {:?}", file_path);
                // debug!("path_expanded: {:?}", path_expanded);

                // Copy input data
                let mut copy = &mut (**input).clone();
                let display_window = copy.display_window();
                let mut data_window = copy.data_window();
                let mut pixel_block = copy.clone_pixel_block();
                let mut image_spec = copy.clone_image_spec();
                // debug!("ImageSpec: {:?}", image_spec);

                let from_color_space = &image_spec.color_space();

                // The color space to write to.
                let to_color_space = if path_expanded.ends_with(".jpg")
                    || path_expanded.ends_with(".png")
                    || path_expanded.ends_with(".jpeg")
                    || path_expanded.ends_with(".jpe")
                    || path_expanded.ends_with(".jif")
                    || path_expanded.ends_with(".jfif")
                    || path_expanded.ends_with(".jfi")
                {
                    "sRGB".to_string()
                } else {
                    "Linear".to_string()
                };

                let bake_option = BakeOption::All;
                ops::bake::do_process(
                    bake_option,
                    &mut pixel_block,
                    display_window,
                    &mut data_window,
                    &mut image_spec,
                    &mut copy,
                    &from_color_space,
                    &to_color_space,
                );

                // Write pixels
                *output = inputs[0].clone();
                let pixel_block_box = Box::new(pixel_block);
                let image = ImageShared {
                    pixel_block: pixel_block_box,
                    display_window,
                    data_window,
                    spec: image_spec,
                };
                let num_threads = 0;
                let crop_on_write = attr_block.get_attr_i32("crop_on_write");

                // Should the image be cropped to the display window
                // before writing?
                let do_crop = match crop_on_write {
                    0 => false, // Disabled
                    1 => true,  // Enabled
                    -1 => {
                        // Auto.
                        //
                        // When If the image format does not support data
                        // windows, then crop off the extra information outside
                        // the display window.
                        let format_supports_data_window = path_expanded.ends_with(".exr");
                        !format_supports_data_window
                    }
                    _ => panic!("Invalid crop_on_write value: {}", crop_on_write),
                };

                let ok = imageio::write_image(&image, &path_expanded, num_threads, do_crop);
                if ok == false {
                    warn!("Failed to write image: status={}", ok);
                }
                debug!("Success: {}", ok);
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
            "crop_on_write" => AttrState::Exists,
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
            "crop_on_write" => self.crop_on_write,
            _ => 0,
        }
    }

    fn set_attr_i32(&mut self, name: &str, value: i32) {
        match name {
            "enable" => self.enable = value,
            "crop_on_write" => self.crop_on_write = value,
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
