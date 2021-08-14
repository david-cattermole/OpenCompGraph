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
 * Viewer node to be used to apply last-minute changes to the image.
 */

use log::{debug, warn};
use std::collections::hash_map::DefaultHasher;
use std::hash;
use std::hash::Hash;
use std::rc::Rc;
use std::string::String;

use crate::attrblock::AttrBlock;
use crate::cache::CacheImpl;
use crate::cache::CachedImage;
use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::BakeOption;
use crate::cxxbridge::ffi::ImageShared;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::NodeType;
use crate::data::Identifier;
use crate::imageio;
use crate::node::traits::Operation;
use crate::node::NodeImpl;
use crate::ops::bake;
use crate::ops::imagecrop;
use crate::pathutils;
use crate::stream::StreamDataImpl;

pub fn new(id: Identifier) -> NodeImpl {
    NodeImpl {
        node_type: NodeType::Viewer,
        id,
        status: NodeStatus::Uninitialized,
        compute: Box::new(ViewerOperation::new()),
        attr_block: Box::new(ViewerAttrs::new()),
    }
}

#[derive(Debug, Clone, Default)]
pub struct ViewerOperation {}

#[derive(Debug, Clone, Default, hash::Hash)]
pub struct ViewerAttrs {
    pub enable: i32,
    pub bake_option: i32, // index for a BakeOption.
    pub crop_to_format: i32,
    pub disk_cache: i32,
    pub file_path: String,
}

impl ViewerOperation {
    pub fn new() -> ViewerOperation {
        ViewerOperation {}
    }
}

impl ViewerAttrs {
    pub fn new() -> ViewerAttrs {
        ViewerAttrs {
            enable: 1,
            bake_option: 1,
            crop_to_format: 0,
            disk_cache: 0,
            file_path: "".to_string(),
        }
    }
}

impl Operation for ViewerOperation {
    fn compute(
        &mut self,
        frame: i32,
        node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        inputs: &Vec<Rc<StreamDataImpl>>,
        output: &mut Rc<StreamDataImpl>,
        cache: &mut Box<CacheImpl>,
    ) -> NodeStatus {
        debug!("ViewerOperation.compute()");
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
                let mut copy = (**input).clone();

                let crop_to_format = attr_block.get_attr_i32("crop_to_format") != 0;
                let bake_option_num = if crop_to_format {
                    3 as i32 // 3 == Bake All.
                } else {
                    attr_block.get_attr_i32("bake_option")
                };
                let bake_option = BakeOption::from(bake_option_num);

                // TODO: Determine if we really need to change the
                // pixels or not. For example, Check if the color
                // space is already set correctly, check if there are
                // any deformers, check if the color matrix is not
                // identity.
                let change_pixels = (bake_option != BakeOption::Nothing) || crop_to_format;

                if change_pixels {
                    let hash_value = self.cache_hash(frame, node_type_id, &attr_block, inputs);
                    let (pixel_block, image_spec, data_window, display_window) =
                        match cache.get(&hash_value) {
                            Some(cached_img) => {
                                // debug!("Cache Hit");

                                // If the cached image was generated
                                // while baking deformations to
                                // pixels, then we must remove
                                // deformers from the stream so that
                                // we do not get a double-deformation
                                // effect when reading from the cache.
                                if bake_option == BakeOption::All {
                                    copy.clear_deformers();
                                }

                                (
                                    cached_img.pixel_block.clone(),
                                    cached_img.spec.clone(),
                                    cached_img.data_window,
                                    cached_img.display_window,
                                )
                            }
                            _ => {
                                // debug!("Cache Miss");

                                // Copy input data
                                let mut display_window = copy.display_window();
                                let mut data_window = copy.data_window();
                                let mut pixel_block = copy.clone_pixel_block();
                                let mut image_spec = copy.clone_image_spec();

                                let from_color_space = &image_spec.color_space();
                                let to_color_space = "Linear".to_string();
                                bake::do_process(
                                    bake_option,
                                    &mut pixel_block,
                                    display_window,
                                    &mut data_window,
                                    &mut image_spec,
                                    &mut copy,
                                    &from_color_space,
                                    &to_color_space,
                                );

                                if crop_to_format {
                                    let mut img = ImageShared {
                                        pixel_block: Box::new(pixel_block),
                                        display_window: display_window,
                                        data_window: data_window,
                                        spec: image_spec,
                                    };

                                    let reformat = true;
                                    let black_outside = false;
                                    let intersect = false;
                                    let _ok = imagecrop::crop_image_in_place(
                                        &mut img,
                                        display_window,
                                        reformat,
                                        black_outside,
                                        intersect,
                                    );

                                    pixel_block = *img.pixel_block;
                                    display_window = img.display_window;
                                    data_window = img.data_window;
                                    image_spec = img.spec;
                                }

                                let pixel_block_rc = Rc::new(pixel_block);
                                let cached_img = CachedImage {
                                    pixel_block: pixel_block_rc.clone(),
                                    spec: image_spec.clone(),
                                    data_window: data_window,
                                    display_window: display_window,
                                };
                                cache.insert(hash_value, cached_img);
                                (
                                    pixel_block_rc.clone(),
                                    image_spec,
                                    data_window,
                                    display_window,
                                )
                            }
                        };

                    copy.set_data_window(data_window);
                    copy.set_display_window(display_window);
                    copy.set_hash(hash_value);
                    copy.set_pixel_block(pixel_block);
                    copy.set_image_spec(image_spec);
                    *output = std::rc::Rc::new(copy);
                }

                NodeStatus::Valid
            }
        }
    }
}

impl AttrBlock for ViewerAttrs {
    fn attr_hash(&self, frame: i32, state: &mut DefaultHasher) {
        self.enable.hash(state);
        if self.enable == 1 {
            self.bake_option.hash(state);
            self.crop_to_format.hash(state);
            self.disk_cache.hash(state);
            self.file_path.hash(state);
        }
    }

    fn attr_exists(&self, name: &str) -> AttrState {
        /*
         * Add attributes:
         *
         * - Image Exposure
         * - Image Gamma
         * - Image Soft-clip
         * - Crop image to the image display window.
         * - Down-res image
         */
        match name {
            "enable" => AttrState::Exists,

            // "color_exposure" => AttrState::Exists,
            // "color_gamma" => AttrState::Exists,
            // "color_soft_clip" => AttrState::Exists,

            // "down_res" => AttrState::Exists,
            "crop_to_format" => AttrState::Exists,

            "bake_option" => AttrState::Exists,
            "disk_cache" => AttrState::Exists,
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
            "crop_to_format" => self.crop_to_format,
            "bake_option" => self.bake_option,
            "disk_cache" => self.disk_cache,
            _ => 0,
        }
    }

    fn set_attr_i32(&mut self, name: &str, value: i32) {
        match name {
            "enable" => self.enable = value,
            "crop_to_format" => self.crop_to_format = value,
            "bake_option" => self.bake_option = value,
            "disk_cache" => self.disk_cache = value,
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

// let disk_cache = attr_block.get_attr_i32("disk_cache");
// if disk_cache {
//     // 1) Compute file path

//     // let file_path = attr_block.get_attr_str("file_path");
//     // let path_expanded = pathutils::expand_string(file_path.to_string(), frame);
//     // // debug!("file_path: {:?}", file_path);
//     // // debug!("path_expanded: {:?}", path_expanded);

//     //
//     // 2) Try to read file path.

//     // let num_threads = 0;
//     // let img = imageio::read_image(&path_expanded, num_threads);
//     // let pixel_block_rc = Rc::new(*img.pixel_block);

//     //
//     // 3) If file read worked...
//     // 3a) load image into cache
//     // 3b) exit.
//     //
//     // 4) If file read failed..
//     // 4a) Compute image
//     // 4b) load image into cache
//     // 4c) write image to file path

// let ok = imageio::write_image(&image, &path_expanded, num_threads, do_crop);
// if ok == false {
//     warn!("Failed to write image: status={}", ok);
// }
// debug!("Success: {}", ok);

//     // 4d) exit.
// } else {
