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
use shellexpand;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash;
use std::hash::Hash;
use std::path::Path;
use std::path::MAIN_SEPARATOR;
use std::rc::Rc;
use std::string::String;

use crate::attrblock::AttrBlock;
use crate::cache::CacheImpl;
use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::BakeOption;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::NodeType;
use crate::data::HashValue;
use crate::data::Identifier;
use crate::data::NodeComputeMode;
use crate::node::traits::Operation;
use crate::node::traits::Validate;
use crate::node::NodeImpl;
use crate::stream::StreamDataImpl;

pub fn new(id: Identifier) -> NodeImpl {
    NodeImpl {
        node_type: NodeType::Viewer,
        id,
        status: NodeStatus::Uninitialized,
        compute: Box::new(ViewerOperation::new()),
        validate: Box::new(ViewerValidate::new()),
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
    pub disk_cache_dir: String,
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
            disk_cache_dir: "".to_string(),
        }
    }
}

fn expand_directory(directory: &str) -> String {
    let expanded_dir = match shellexpand::full(directory) {
        Ok(v) => (*v).to_string(),
        Err(e) => {
            warn!(
                "Environment variable unknown: name={} cause={}",
                e.var_name, e.cause
            );
            directory.to_string()
        }
    };
    expanded_dir.to_string()
}

fn compute_file_path(directory: &str, hash_value: HashValue, file_ext: &str) -> String {
    let expanded_dir = expand_directory(directory);
    let format_path = format!(
        "{}{}ocg_disk_cache_{}.{}",
        expanded_dir, MAIN_SEPARATOR, hash_value, file_ext
    );
    let file_path = match fs::canonicalize(&format_path) {
        Ok(v) => match v.to_str() {
            Some(s) => s.to_string(),
            None => format_path.to_string(),
        },
        Err(_) => format_path.to_string(),
    };
    file_path
}

impl Operation for ViewerOperation {
    fn compute(
        &mut self,
        _frame: i32,
        _node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        _hash_value: HashValue,
        node_compute_mode: NodeComputeMode,
        inputs: &Vec<Rc<StreamDataImpl>>,
        output: &mut Rc<StreamDataImpl>,
        _cache: &mut Box<CacheImpl>,
    ) -> NodeStatus {
        debug!("ViewerOperation.compute()");
        debug!("ViewerOperation NodeComputeMode={:#?}", node_compute_mode);
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
                let copy = (**input).clone();
                *output = std::rc::Rc::new(copy);
                NodeStatus::Valid
            }
        }
    }
}

impl AttrBlock for ViewerAttrs {
    fn attr_hash(&self, _frame: i32, state: &mut DefaultHasher) {
        self.enable.hash(state);
        if self.enable == 1 {
            self.bake_option.hash(state);
            self.crop_to_format.hash(state);
            if self.disk_cache == 1 {
                self.disk_cache.hash(state);
                self.disk_cache_dir.hash(state);
                // TODO: Look up the file path and use existance as hash?
            }
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
            "disk_cache_dir" => AttrState::Exists,
            _ => AttrState::Missing,
        }
    }

    fn get_attr_str(&self, name: &str) -> &str {
        match name {
            "disk_cache_dir" => &self.disk_cache_dir,
            _ => "",
        }
    }

    fn set_attr_str(&mut self, name: &str, value: &str) {
        match name {
            "disk_cache_dir" => self.disk_cache_dir = value.to_string(),
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

#[derive(Debug, Clone, Default)]
pub struct ViewerValidate {}

impl ViewerValidate {
    pub fn new() -> ViewerValidate {
        ViewerValidate {}
    }
}

impl Validate for ViewerValidate {
    fn validate_inputs(
        &self,
        _node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        hash_value: HashValue,
        node_compute_mode: NodeComputeMode,
        input_nodes: &Vec<&Box<NodeImpl>>,
    ) -> Vec<NodeComputeMode> {
        debug!(
            "ViewerValidate::validate_inputs(): NodeComputeMode={:#?} HashValue={:#?}",
            node_compute_mode, hash_value
        );
        let mut node_compute_modes = Vec::new();

        if input_nodes.len() > 0 {
            let enable = attr_block.get_attr_i32("enable") != 0;
            let disk_cache = attr_block.get_attr_i32("disk_cache") != 0;
            let crop_to_format = attr_block.get_attr_i32("crop_to_format") != 0;
            let bake_option_num = if crop_to_format {
                3 as i32 // 3 == Bake All.
            } else {
                attr_block.get_attr_i32("bake_option")
            };
            let bake_option = BakeOption::from(bake_option_num);

            if enable == false {
                node_compute_modes.push(NodeComputeMode::ALL);
            } else {
                if disk_cache == true {
                    let disk_cache_dir = attr_block.get_attr_str("disk_cache_dir");
                    let file_ext = "jpg"; // JPEG - lossy fast playback.
                    let file_path = compute_file_path(disk_cache_dir, hash_value, file_ext);

                    // Try to read file path.
                    let file_exists = Path::new(&file_path).exists();
                    if file_exists == false {
                        node_compute_modes.push(node_compute_mode & NodeComputeMode::ALL)
                    } else {
                        match bake_option {
                            BakeOption::Nothing => {
                                node_compute_modes.push(node_compute_mode & NodeComputeMode::ALL)
                            }
                            BakeOption::ColorSpace => node_compute_modes.push(
                                node_compute_mode
                                    & (NodeComputeMode::ALL - NodeComputeMode::COLOR_SPACE),
                            ),
                            BakeOption::ColorSpaceAndGrade => node_compute_modes.push(
                                node_compute_mode
                                    & (NodeComputeMode::ALL
                                        - NodeComputeMode::COLOR_SPACE
                                        - NodeComputeMode::COLOR),
                            ),
                            BakeOption::All => node_compute_modes.push(
                                node_compute_mode
                                    & (NodeComputeMode::ALL
                                        - NodeComputeMode::COLOR_SPACE
                                        - NodeComputeMode::COLOR
                                        - NodeComputeMode::DEFORMER
                                        - NodeComputeMode::PIXEL),
                            ),
                            _ => panic!("BakeOption is invalid value: {:#?}", bake_option),
                        }
                    }
                } else {
                    node_compute_modes.push(NodeComputeMode::ALL);
                }
            }
            for _ in input_nodes.iter().skip(1) {
                node_compute_modes.push(node_compute_mode & NodeComputeMode::NONE);
            }
        }
        node_compute_modes
    }
}
