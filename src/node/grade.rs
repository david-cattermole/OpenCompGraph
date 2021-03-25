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
use crate::cxxbridge::ffi::Matrix4;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::NodeType;
use crate::data::Identifier;
use crate::hashutils::HashableF32;
use crate::math::colorxform;
use crate::node::traits::Operation;
use crate::node::NodeImpl;
use crate::stream::StreamDataImpl;

pub fn new(id: Identifier) -> NodeImpl {
    NodeImpl {
        node_type: NodeType::Grade,
        id,
        status: NodeStatus::Uninitialized,
        compute: Box::new(GradeOperation::new()),
        attr_block: Box::new(GradeAttrs::new()),
    }
}

#[derive(Debug, Clone, Default)]
pub struct GradeOperation {}

#[derive(Debug, Clone, Default)]
pub struct GradeAttrs {
    pub enable: i32,
    pub multiply_r: f32,
    pub multiply_g: f32,
    pub multiply_b: f32,
    pub multiply_a: f32,
}

impl hash::Hash for GradeAttrs {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        // TODO: The hash should change based on the values actually
        // used.
        //
        // For example, when enable=0, all attributes are not used and
        // should therefore not be used to compute the hash.
        self.enable.hash(state);
        HashableF32::new(self.multiply_r).hash(state);
        HashableF32::new(self.multiply_g).hash(state);
        HashableF32::new(self.multiply_b).hash(state);
        HashableF32::new(self.multiply_a).hash(state);
    }
}

impl GradeOperation {
    pub fn new() -> GradeOperation {
        GradeOperation {}
    }
}

impl GradeAttrs {
    pub fn new() -> GradeAttrs {
        GradeAttrs {
            enable: 1,
            multiply_r: 1.0,
            multiply_g: 1.0,
            multiply_b: 1.0,
            multiply_a: 1.0,
        }
    }
}

impl Operation for GradeOperation {
    fn compute(
        &mut self,
        frame: i32,
        node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        inputs: &Vec<Rc<StreamDataImpl>>,
        output: &mut Rc<StreamDataImpl>,
        _cache: &mut Box<CacheImpl>,
    ) -> NodeStatus {
        debug!("GradeOperation.compute()");
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
                    // Calculate Color Matrix
                    let in_matrix = input.color_matrix().to_na_matrix();
                    let r_multiply = attr_block.get_attr_f32("multiply_r");
                    let g_multiply = attr_block.get_attr_f32("multiply_g");
                    let b_multiply = attr_block.get_attr_f32("multiply_b");
                    let a_multiply = attr_block.get_attr_f32("multiply_a");
                    let out_matrix = colorxform::apply_scale_rgba(
                        in_matrix, r_multiply, g_multiply, b_multiply, a_multiply,
                    );
                    copy.set_color_matrix(Matrix4::from_na_matrix(out_matrix));
                }

                // Set Output data
                let hash_value = self.cache_hash(frame, node_type_id, &attr_block, inputs);
                copy.set_hash(hash_value);
                *output = std::rc::Rc::new(copy);
                NodeStatus::Valid
            }
        }
    }
}

impl AttrBlock for GradeAttrs {
    fn attr_hash(&self, frame: i32, state: &mut DefaultHasher) {
        // TODO: Should we use "frame" to hash the value?
        self.hash(state)
    }

    fn attr_exists(&self, name: &str) -> AttrState {
        match name {
            "enable" => AttrState::Exists,
            "multiply_r" => AttrState::Exists,
            "multiply_g" => AttrState::Exists,
            "multiply_b" => AttrState::Exists,
            "multiply_a" => AttrState::Exists,
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
            "multiply_r" => self.multiply_r,
            "multiply_g" => self.multiply_g,
            "multiply_b" => self.multiply_b,
            "multiply_a" => self.multiply_a,
            _ => 0.0,
        }
    }

    fn set_attr_f32(&mut self, name: &str, value: f32) {
        match name {
            "multiply_r" => self.multiply_r = value,
            "multiply_g" => self.multiply_g = value,
            "multiply_b" => self.multiply_b = value,
            "multiply_a" => self.multiply_a = value,
            _ => (),
        }
    }
}
