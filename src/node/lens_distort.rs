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
    pub direction: i32,
    pub lens_center_offset_x: f32,
    pub lens_center_offset_y: f32,
    pub distortion: f32,
    pub anamorphic_squeeze: f32,
    pub curvature_x: f32,
    pub curvature_y: f32,
    pub quartic_distortion: f32,
}

impl hash::Hash for LensDistortAttrs {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.enable.hash(state);
        HashableF32::new(self.lens_center_offset_x).hash(state);
        HashableF32::new(self.lens_center_offset_y).hash(state);
        HashableF32::new(self.distortion).hash(state);
        HashableF32::new(self.anamorphic_squeeze).hash(state);
        HashableF32::new(self.curvature_x).hash(state);
        HashableF32::new(self.curvature_y).hash(state);
        HashableF32::new(self.quartic_distortion).hash(state);
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
            direction: 1, // 0 = 'undistort'
            lens_center_offset_x: 0.0,
            lens_center_offset_y: 0.0,
            distortion: 0.0,
            anamorphic_squeeze: 1.0,
            curvature_x: 0.0,
            curvature_y: 0.0,
            quartic_distortion: 0.0,
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

                    let direction = attr_block.get_attr_i32("direction");
                    let lco_x = attr_block.get_attr_f32("lens_center_offset_x");
                    let lco_y = attr_block.get_attr_f32("lens_center_offset_y");
                    let distortion = attr_block.get_attr_f32("distortion");
                    let anamorphic_squeeze = attr_block.get_attr_f32("anamorphic_squeeze");
                    let curvature_x = attr_block.get_attr_f32("curvature_x");
                    let curvature_y = attr_block.get_attr_f32("curvature_y");
                    let quartic_distortion = attr_block.get_attr_f32("quartic_distortion");
                    deformer.set_attr_i32("direction", direction);
                    deformer.set_attr_f32("lens_center_offset_x", lco_x);
                    deformer.set_attr_f32("lens_center_offset_y", lco_y);
                    deformer.set_attr_f32("distortion", distortion);
                    deformer.set_attr_f32("anamorphic_squeeze", anamorphic_squeeze);
                    deformer.set_attr_f32("curvature_x", curvature_x);
                    deformer.set_attr_f32("curvature_y", curvature_y);
                    deformer.set_attr_f32("quartic_distortion", quartic_distortion);
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
            "direction" => AttrState::Exists,
            "lens_center_offset_x" => AttrState::Exists,
            "lens_center_offset_y" => AttrState::Exists,
            "distortion" => AttrState::Exists,
            "anamorphic_squeeze" => AttrState::Exists,
            "curvature_x" => AttrState::Exists,
            "curvature_y" => AttrState::Exists,
            "quartic_distortion" => AttrState::Exists,
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
            "direction" => self.direction,
            _ => 0,
        }
    }

    fn set_attr_i32(&mut self, name: &str, value: i32) {
        match name {
            "enable" => self.enable = value,
            "direction" => self.direction = value,
            _ => (),
        };
    }

    fn get_attr_f32(&self, name: &str) -> f32 {
        match name {
            "lens_center_offset_x" => self.lens_center_offset_x,
            "lens_center_offset_y" => self.lens_center_offset_y,
            "distortion" => self.distortion,
            "anamorphic_squeeze" => self.anamorphic_squeeze,
            "curvature_x" => self.curvature_x,
            "curvature_y" => self.curvature_y,
            "quartic_distortion" => self.quartic_distortion,
            _ => 0.0,
        }
    }

    fn set_attr_f32(&mut self, name: &str, value: f32) {
        match name {
            "lens_center_offset_x" => self.lens_center_offset_x = value,
            "lens_center_offset_y" => self.lens_center_offset_y = value,
            "distortion" => self.distortion = value,
            "anamorphic_squeeze" => self.anamorphic_squeeze = value,
            "curvature_x" => self.curvature_x = value,
            "curvature_y" => self.curvature_y = value,
            "quartic_distortion" => self.quartic_distortion = value,
            _ => (),
        }
    }
}
