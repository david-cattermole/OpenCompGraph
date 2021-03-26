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

use crate::attrblock::AttrBlock;
use crate::cache::CacheImpl;
use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::NodeType;
use crate::data::Identifier;
use crate::deformer::tde4_classic::DeformerTde4Classic;
use crate::deformer::Deformer;
use crate::hashutils::HashableF32;
use crate::node::traits::Operation;
use crate::node::NodeImpl;
use crate::stream::StreamDataImpl;

pub fn new(id: Identifier) -> NodeImpl {
    NodeImpl {
        node_type: NodeType::LensDistort,
        id,
        status: NodeStatus::Uninitialized,
        compute: Box::new(LensDistortOperation::new()),
        attr_block: Box::new(LensDistortAttrs::new()),
    }
}

#[derive(Debug, Clone, Default)]
pub struct LensDistortOperation {}

#[derive(Debug, Clone, Default)]
pub struct LensDistortAttrs {
    pub enable: i32,
    pub k1: f32,
    pub k2: f32,
    pub center_x: f32,
    pub center_y: f32,
}

impl hash::Hash for LensDistortAttrs {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.enable.hash(state);
        HashableF32::new(self.k1).hash(state);
        HashableF32::new(self.k2).hash(state);
        HashableF32::new(self.center_x).hash(state);
        HashableF32::new(self.center_y).hash(state);
    }
}

impl LensDistortOperation {
    pub fn new() -> LensDistortOperation {
        LensDistortOperation {}
    }
}

impl LensDistortAttrs {
    pub fn new() -> LensDistortAttrs {
        LensDistortAttrs {
            enable: 1,
            k1: 0.0,
            k2: 0.0,
            center_x: 0.0,
            center_y: 0.0,
        }
    }
}

impl Operation for LensDistortOperation {
    fn compute(
        &mut self,
        frame: i32,
        node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        inputs: &Vec<Rc<StreamDataImpl>>,
        output: &mut Rc<StreamDataImpl>,
        _cache: &mut Box<CacheImpl>,
    ) -> NodeStatus {
        debug!("LensDistortOperation.compute()");
        // debug!("AttrBlock: {:?}", attr_block);
        // debug!("Inputs: {:?}", inputs);
        // debug!("Output: {:?}", output);

        match inputs.len() {
            0 => NodeStatus::Error,
            _ => {
                let input = &inputs[0].clone();
                let mut copy = (**input).clone();

                let enable = attr_block.get_attr_i32("enable");
                if enable == 1 {
                    let mut deformer = DeformerTde4Classic::default();
                    deformer.set_attr_f32("distortion", attr_block.get_attr_f32("k1"));
                    deformer.set_attr_f32("quartic_distortion", attr_block.get_attr_f32("k2"));
                    deformer
                        .set_attr_f32("lens_center_offset_x", attr_block.get_attr_f32("center_x"));
                    deformer
                        .set_attr_f32("lens_center_offset_y", attr_block.get_attr_f32("center_y"));
                    deformer.commit_data().unwrap();
                    copy.push_deformer(Box::new(deformer));
                }

                // Set Output data
                let hash_value = self.cache_hash(frame, node_type_id, &attr_block, inputs);
                copy.set_hash(hash_value);
                *output = Rc::new(copy);
                NodeStatus::Valid
            }
        }
    }
}

impl AttrBlock for LensDistortAttrs {
    fn attr_hash(&self, _frame: i32, state: &mut DefaultHasher) {
        self.hash(state)
    }

    fn attr_exists(&self, name: &str) -> AttrState {
        match name {
            "enable" => AttrState::Exists,
            "k1" => AttrState::Exists,
            "k2" => AttrState::Exists,
            "center_x" => AttrState::Exists,
            "center_y" => AttrState::Exists,
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
            _ => 0,
        }
    }

    fn set_attr_i32(&mut self, name: &str, value: i32) {
        match name {
            "enable" => self.enable = value,
            _ => (),
        };
    }

    fn get_attr_f32(&self, name: &str) -> f32 {
        match name {
            "k1" => self.k1,
            "k2" => self.k2,
            "center_x" => self.center_x,
            "center_y" => self.center_y,
            _ => 0.0,
        }
    }

    fn set_attr_f32(&mut self, name: &str, value: f32) {
        match name {
            "k1" => self.k1 = value,
            "k2" => self.k2 = value,
            "center_x" => self.center_x = value,
            "center_y" => self.center_y = value,
            _ => (),
        }
    }
}
