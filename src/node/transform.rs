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
use crate::data::HashValue;
use crate::data::Identifier;
use crate::data::NodeComputeMode;
use crate::deformer::transform::DeformerTransform;
use crate::deformer::Deformer;
use crate::hashutils::HashableF32;
use crate::node::traits::Operation;
use crate::node::traits::Validate;
use crate::node::NodeImpl;
use crate::stream::StreamDataImpl;

pub fn new(id: Identifier) -> NodeImpl {
    NodeImpl {
        node_type: NodeType::Transform,
        id,
        status: NodeStatus::Uninitialized,
        compute: Box::new(TransformOperation::new()),
        validate: Box::new(TransformValidate::new()),
        attr_block: Box::new(TransformAttrs::new()),
    }
}

#[derive(Debug, Clone, Default)]
pub struct TransformOperation {}

#[derive(Debug, Clone, Default)]
pub struct TransformAttrs {
    pub enable: i32,
    pub invert: i32,
    pub translate_x: f32,
    pub translate_y: f32,
    pub rotate: f32,
    pub rotate_center_x: f32,
    pub rotate_center_y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    // TODO: Add pivot point.
}

impl hash::Hash for TransformAttrs {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.enable.hash(state);
        self.invert.hash(state);
        HashableF32::new(self.translate_x).hash(state);
        HashableF32::new(self.translate_y).hash(state);
        HashableF32::new(self.rotate).hash(state);
        HashableF32::new(self.rotate_center_x).hash(state);
        HashableF32::new(self.rotate_center_y).hash(state);
        HashableF32::new(self.scale_x).hash(state);
        HashableF32::new(self.scale_y).hash(state);
    }
}

impl TransformOperation {
    pub fn new() -> TransformOperation {
        TransformOperation {}
    }
}

impl TransformAttrs {
    pub fn new() -> TransformAttrs {
        TransformAttrs {
            enable: 1,
            invert: 0,
            translate_x: 0.0,
            translate_y: 0.0,
            rotate: 0.0,
            rotate_center_x: 0.0,
            rotate_center_y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
        }
    }
}

impl Operation for TransformOperation {
    fn compute(
        &mut self,
        _frame: i32,
        _node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        hash_value: HashValue,
        node_compute_mode: NodeComputeMode,
        inputs: &Vec<Rc<StreamDataImpl>>,
        output: &mut Rc<StreamDataImpl>,
        _cache: &mut Box<CacheImpl>,
    ) -> NodeStatus {
        debug!("TransformOperation.compute()");
        debug!(
            "TransformOperation NodeComputeMode={:#?}",
            node_compute_mode
        );
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
                    let mut deformer = DeformerTransform::default();

                    deformer.set_attr_i32("invert", attr_block.get_attr_i32("invert"));
                    deformer.set_attr_f32("translate_x", attr_block.get_attr_f32("translate_x"));
                    deformer.set_attr_f32("translate_y", attr_block.get_attr_f32("translate_y"));
                    deformer.set_attr_f32("rotate", attr_block.get_attr_f32("rotate"));
                    deformer.set_attr_f32(
                        "rotate_center_x",
                        attr_block.get_attr_f32("rotate_center_x"),
                    );
                    deformer.set_attr_f32(
                        "rotate_center_y",
                        attr_block.get_attr_f32("rotate_center_y"),
                    );
                    deformer.set_attr_f32("scale_x", attr_block.get_attr_f32("scale_x"));
                    deformer.set_attr_f32("scale_y", attr_block.get_attr_f32("scale_y"));

                    deformer.commit_data().unwrap();
                    copy.push_deformer(Box::new(deformer));
                }

                // Set Output data
                copy.set_hash(hash_value);
                *output = Rc::new(copy);
                NodeStatus::Valid
            }
        }
    }
}

impl AttrBlock for TransformAttrs {
    fn attr_hash(&self, _frame: i32, state: &mut DefaultHasher) {
        self.hash(state)
    }

    fn attr_exists(&self, name: &str) -> AttrState {
        match name {
            "enable" => AttrState::Exists,
            "invert" => AttrState::Exists,
            "translate_x" => AttrState::Exists,
            "translate_y" => AttrState::Exists,
            "rotate" => AttrState::Exists,
            "rotate_center_x" => AttrState::Exists,
            "rotate_center_y" => AttrState::Exists,
            "scale_x" => AttrState::Exists,
            "scale_y" => AttrState::Exists,
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
            "invert" => self.invert,
            _ => 0,
        }
    }

    fn set_attr_i32(&mut self, name: &str, value: i32) {
        match name {
            "enable" => self.enable = value,
            "invert" => self.invert = value,
            _ => (),
        };
    }

    fn get_attr_f32(&self, name: &str) -> f32 {
        match name {
            "translate_x" => self.translate_x,
            "translate_y" => self.translate_y,
            "rotate" => self.rotate,
            "rotate_center_x" => self.rotate_center_x,
            "rotate_center_y" => self.rotate_center_y,
            "scale_x" => self.scale_x,
            "scale_y" => self.scale_y,
            _ => 0.0,
        }
    }

    fn set_attr_f32(&mut self, name: &str, value: f32) {
        match name {
            "translate_x" => self.translate_x = value,
            "translate_y" => self.translate_y = value,
            "rotate" => self.rotate = value,
            "rotate_center_x" => self.rotate_center_x = value,
            "rotate_center_y" => self.rotate_center_y = value,
            "scale_x" => self.scale_x = value,
            "scale_y" => self.scale_y = value,
            _ => (),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct TransformValidate {}

impl TransformValidate {
    pub fn new() -> TransformValidate {
        TransformValidate {}
    }
}

impl Validate for TransformValidate {
    fn validate_inputs(
        &self,
        _node_type_id: u8,
        _attr_block: &Box<dyn AttrBlock>,
        _hash_value: HashValue,
        node_compute_mode: NodeComputeMode,
        input_nodes: &Vec<&Box<NodeImpl>>,
    ) -> Vec<NodeComputeMode> {
        debug!("TransformValidate::validate_inputs()");
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
