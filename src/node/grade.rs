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
use crate::colorop::colorgrade::ColorOpGrade;
use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::NodeType;
use crate::data::FrameValue;
use crate::data::HashValue;
use crate::data::Identifier;
use crate::data::NodeComputeMode;
use crate::hashutils::HashableF32;
use crate::node::traits::Operation;
use crate::node::traits::Validate;
use crate::node::NodeImpl;
use crate::stream::StreamDataImpl;

pub fn new(id: Identifier) -> NodeImpl {
    NodeImpl {
        node_type: NodeType::Grade,
        id,
        status: NodeStatus::Uninitialized,
        compute: Box::new(GradeOperation::new()),
        validate: Box::new(GradeValidate::new()),
        attr_block: Box::new(GradeAttrs::new()),
    }
}

#[derive(Debug, Clone, Default)]
pub struct GradeOperation {}

#[derive(Debug, Clone, Default)]
pub struct GradeAttrs {
    pub enable: i32,

    pub process_r: i32,
    pub process_g: i32,
    pub process_b: i32,
    pub process_a: i32,

    pub black_point_r: f32,
    pub black_point_g: f32,
    pub black_point_b: f32,
    pub black_point_a: f32,

    pub white_point_r: f32,
    pub white_point_g: f32,
    pub white_point_b: f32,
    pub white_point_a: f32,

    pub lift_r: f32,
    pub lift_g: f32,
    pub lift_b: f32,
    pub lift_a: f32,

    pub gain_r: f32,
    pub gain_g: f32,
    pub gain_b: f32,
    pub gain_a: f32,

    pub multiply_r: f32,
    pub multiply_g: f32,
    pub multiply_b: f32,
    pub multiply_a: f32,

    pub offset_r: f32,
    pub offset_g: f32,
    pub offset_b: f32,
    pub offset_a: f32,

    pub gamma_r: f32,
    pub gamma_g: f32,
    pub gamma_b: f32,
    pub gamma_a: f32,

    pub reverse: i32,
    pub clamp_black: i32,
    pub clamp_white: i32,
    pub premult: i32,
    pub mix: f32,
}

impl hash::Hash for GradeAttrs {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.enable.hash(state);
        if self.enable == 0 {
            return;
        }

        self.process_r.hash(state);
        self.process_g.hash(state);
        self.process_b.hash(state);
        self.process_a.hash(state);

        if self.process_r != 0 {
            HashableF32::new(self.black_point_r).hash(state);
            HashableF32::new(self.white_point_r).hash(state);
            HashableF32::new(self.lift_r).hash(state);
            HashableF32::new(self.gain_r).hash(state);
            HashableF32::new(self.multiply_r).hash(state);
            HashableF32::new(self.offset_r).hash(state);
            HashableF32::new(self.gamma_r).hash(state);
        }
        if self.process_g != 0 {
            HashableF32::new(self.black_point_g).hash(state);
            HashableF32::new(self.white_point_g).hash(state);
            HashableF32::new(self.lift_g).hash(state);
            HashableF32::new(self.gain_g).hash(state);
            HashableF32::new(self.multiply_g).hash(state);
            HashableF32::new(self.offset_g).hash(state);
            HashableF32::new(self.gamma_g).hash(state);
        }
        if self.process_b != 0 {
            HashableF32::new(self.black_point_b).hash(state);
            HashableF32::new(self.white_point_b).hash(state);
            HashableF32::new(self.lift_b).hash(state);
            HashableF32::new(self.gain_b).hash(state);
            HashableF32::new(self.multiply_b).hash(state);
            HashableF32::new(self.offset_b).hash(state);
            HashableF32::new(self.gamma_b).hash(state);
        }
        if self.process_a != 0 {
            HashableF32::new(self.black_point_a).hash(state);
            HashableF32::new(self.white_point_a).hash(state);
            HashableF32::new(self.lift_a).hash(state);
            HashableF32::new(self.gain_a).hash(state);
            HashableF32::new(self.multiply_a).hash(state);
            HashableF32::new(self.offset_a).hash(state);
            HashableF32::new(self.gamma_a).hash(state);
        }

        if (self.process_r != 0)
            || (self.process_g != 0)
            || (self.process_b != 0)
            || (self.process_a != 0)
        {
            self.reverse.hash(state);
            self.clamp_black.hash(state);
            self.clamp_white.hash(state);
            self.premult.hash(state);
            HashableF32::new(self.mix).hash(state);
        }
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

            process_r: 1,
            process_g: 1,
            process_b: 1,
            // Do not adjust alpha channel by default.
            process_a: 0,

            black_point_r: 0.0,
            black_point_g: 0.0,
            black_point_b: 0.0,
            black_point_a: 0.0,

            white_point_r: 1.0,
            white_point_g: 1.0,
            white_point_b: 1.0,
            white_point_a: 1.0,

            lift_r: 0.0,
            lift_g: 0.0,
            lift_b: 0.0,
            lift_a: 0.0,

            gain_r: 1.0,
            gain_g: 1.0,
            gain_b: 1.0,
            gain_a: 1.0,

            multiply_r: 1.0,
            multiply_g: 1.0,
            multiply_b: 1.0,
            multiply_a: 1.0,

            offset_r: 0.0,
            offset_g: 0.0,
            offset_b: 0.0,
            offset_a: 0.0,

            gamma_r: 1.0,
            gamma_g: 1.0,
            gamma_b: 1.0,
            gamma_a: 1.0,

            reverse: 0,
            clamp_black: 1,
            clamp_white: 0,
            premult: 0,

            mix: 1.0,
        }
    }
}

impl Operation for GradeOperation {
    fn compute(
        &mut self,
        _frame: FrameValue,
        _node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        hash_value: HashValue,
        node_compute_mode: NodeComputeMode,
        inputs: &Vec<Rc<StreamDataImpl>>,
        output: &mut Rc<StreamDataImpl>,
        _cache: &mut Box<CacheImpl>,
    ) -> NodeStatus {
        debug!("GradeOperation.compute()");
        debug!("GradeOperation NodeComputeMode={:#?}", node_compute_mode);
        // debug!("AttrBlock: {:?}", attr_block);
        // debug!("Inputs: {:?}", inputs);
        // debug!("Output: {:?}", output);

        let mut status = NodeStatus::Valid;
        let mut stream_data = match inputs.len() {
            0 => {
                // No input given, return an empty default stream.
                status = NodeStatus::Warning;
                StreamDataImpl::new()
            }
            _ => {
                let input = &inputs[0].clone();
                (**input).clone()
            }
        };

        let enable = attr_block.get_attr_i32("enable");
        let process_r = attr_block.get_attr_i32("process_r");
        let process_g = attr_block.get_attr_i32("process_g");
        let process_b = attr_block.get_attr_i32("process_b");
        let process_a = attr_block.get_attr_i32("process_a");
        let has_work_to_do = enable == 1
            && ((process_r != 0) || (process_g != 0) || (process_b != 0) || (process_a != 0));

        if has_work_to_do == false {
            // Set Output data
            *output = std::rc::Rc::new(stream_data);
            return status;
        }

        let mut color_op = ColorOpGrade::default();
        // No need to set 'enable' attribute, because we ensure
        // there's some work to do before creating the ColorOpGrade.
        color_op.set_attr_i32("process_r", process_r);
        color_op.set_attr_i32("process_g", process_g);
        color_op.set_attr_i32("process_b", process_b);
        color_op.set_attr_i32("process_a", process_a);

        // Attributes that change are non-linear.
        let gamma_r = attr_block.get_attr_f32("gamma_r");
        let gamma_g = attr_block.get_attr_f32("gamma_g");
        let gamma_b = attr_block.get_attr_f32("gamma_b");
        let gamma_a = attr_block.get_attr_f32("gamma_a");
        color_op.set_attr_f32("gamma_r", gamma_r);
        color_op.set_attr_f32("gamma_g", gamma_g);
        color_op.set_attr_f32("gamma_b", gamma_b);
        color_op.set_attr_f32("gamma_a", gamma_a);

        let reverse = attr_block.get_attr_i32("reverse");
        let clamp_black = attr_block.get_attr_i32("clamp_black");
        let clamp_white = attr_block.get_attr_i32("clamp_white");
        let premult = attr_block.get_attr_i32("premult");
        let mix = attr_block.get_attr_f32("mix");
        color_op.set_attr_i32("reverse", reverse);
        color_op.set_attr_i32("clamp_black", clamp_black);
        color_op.set_attr_i32("clamp_white", clamp_white);
        color_op.set_attr_i32("premult", premult);
        color_op.set_attr_f32("mix", mix);

        // Attributes making linear changes to the colour.
        let black_point_r = attr_block.get_attr_f32("black_point_r");
        let black_point_g = attr_block.get_attr_f32("black_point_g");
        let black_point_b = attr_block.get_attr_f32("black_point_b");
        let black_point_a = attr_block.get_attr_f32("black_point_a");
        color_op.set_attr_f32("black_point_r", black_point_r);
        color_op.set_attr_f32("black_point_g", black_point_g);
        color_op.set_attr_f32("black_point_b", black_point_b);
        color_op.set_attr_f32("black_point_a", black_point_a);

        let white_point_r = attr_block.get_attr_f32("white_point_r");
        let white_point_g = attr_block.get_attr_f32("white_point_g");
        let white_point_b = attr_block.get_attr_f32("white_point_b");
        let white_point_a = attr_block.get_attr_f32("white_point_a");
        color_op.set_attr_f32("white_point_r", white_point_r);
        color_op.set_attr_f32("white_point_g", white_point_g);
        color_op.set_attr_f32("white_point_b", white_point_b);
        color_op.set_attr_f32("white_point_a", white_point_a);

        let lift_r = attr_block.get_attr_f32("lift_r");
        let lift_g = attr_block.get_attr_f32("lift_g");
        let lift_b = attr_block.get_attr_f32("lift_b");
        let lift_a = attr_block.get_attr_f32("lift_a");
        color_op.set_attr_f32("lift_r", lift_r);
        color_op.set_attr_f32("lift_g", lift_g);
        color_op.set_attr_f32("lift_b", lift_b);
        color_op.set_attr_f32("lift_a", lift_a);

        let gain_r = attr_block.get_attr_f32("gain_r");
        let gain_g = attr_block.get_attr_f32("gain_g");
        let gain_b = attr_block.get_attr_f32("gain_b");
        let gain_a = attr_block.get_attr_f32("gain_a");
        color_op.set_attr_f32("gain_r", gain_r);
        color_op.set_attr_f32("gain_g", gain_g);
        color_op.set_attr_f32("gain_b", gain_b);
        color_op.set_attr_f32("gain_a", gain_a);

        let multiply_r = attr_block.get_attr_f32("multiply_r");
        let multiply_g = attr_block.get_attr_f32("multiply_g");
        let multiply_b = attr_block.get_attr_f32("multiply_b");
        let multiply_a = attr_block.get_attr_f32("multiply_a");
        color_op.set_attr_f32("multiply_r", multiply_r);
        color_op.set_attr_f32("multiply_g", multiply_g);
        color_op.set_attr_f32("multiply_b", multiply_b);
        color_op.set_attr_f32("multiply_a", multiply_a);

        let offset_r = attr_block.get_attr_f32("offset_r");
        let offset_g = attr_block.get_attr_f32("offset_g");
        let offset_b = attr_block.get_attr_f32("offset_b");
        let offset_a = attr_block.get_attr_f32("offset_a");
        color_op.set_attr_f32("offset_r", offset_r);
        color_op.set_attr_f32("offset_g", offset_g);
        color_op.set_attr_f32("offset_b", offset_b);
        color_op.set_attr_f32("offset_a", offset_a);

        stream_data.push_color_op(Box::new(color_op));

        // Set Output data
        stream_data.set_hash(hash_value);
        *output = Rc::new(stream_data);
        status
    }
}

impl AttrBlock for GradeAttrs {
    fn attr_hash(&self, _frame: FrameValue, state: &mut DefaultHasher) {
        self.hash(state)
    }

    fn attr_exists(&self, name: &str) -> AttrState {
        match name {
            "enable" => AttrState::Exists,

            "process_r" => AttrState::Exists,
            "process_g" => AttrState::Exists,
            "process_b" => AttrState::Exists,
            "process_a" => AttrState::Exists,

            "black_point_r" => AttrState::Exists,
            "black_point_g" => AttrState::Exists,
            "black_point_b" => AttrState::Exists,
            "black_point_a" => AttrState::Exists,

            "white_point_r" => AttrState::Exists,
            "white_point_g" => AttrState::Exists,
            "white_point_b" => AttrState::Exists,
            "white_point_a" => AttrState::Exists,

            "lift_r" => AttrState::Exists,
            "lift_g" => AttrState::Exists,
            "lift_b" => AttrState::Exists,
            "lift_a" => AttrState::Exists,

            "gain_r" => AttrState::Exists,
            "gain_g" => AttrState::Exists,
            "gain_b" => AttrState::Exists,
            "gain_a" => AttrState::Exists,

            "multiply_r" => AttrState::Exists,
            "multiply_g" => AttrState::Exists,
            "multiply_b" => AttrState::Exists,
            "multiply_a" => AttrState::Exists,

            "offset_r" => AttrState::Exists,
            "offset_g" => AttrState::Exists,
            "offset_b" => AttrState::Exists,
            "offset_a" => AttrState::Exists,

            "gamma_r" => AttrState::Exists,
            "gamma_g" => AttrState::Exists,
            "gamma_b" => AttrState::Exists,
            "gamma_a" => AttrState::Exists,

            "reverse" => AttrState::Exists,
            "clamp_black" => AttrState::Exists,
            "clamp_white" => AttrState::Exists,
            "premult" => AttrState::Exists,
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

            "process_r" => self.process_r,
            "process_g" => self.process_g,
            "process_b" => self.process_b,
            "process_a" => self.process_a,

            "reverse" => self.reverse,
            "clamp_black" => self.clamp_black,
            "clamp_white" => self.clamp_white,
            "premult" => self.premult,

            _ => 0,
        }
    }

    fn set_attr_i32(&mut self, name: &str, value: i32) {
        match name {
            "enable" => self.enable = value,

            "process_r" => self.process_r = value,
            "process_g" => self.process_g = value,
            "process_b" => self.process_b = value,
            "process_a" => self.process_a = value,

            "reverse" => self.reverse = value,
            "clamp_black" => self.clamp_black = value,
            "clamp_white" => self.clamp_white = value,
            "premult" => self.premult = value,

            _ => (),
        };
    }

    fn get_attr_f32(&self, name: &str) -> f32 {
        match name {
            "black_point_r" => self.black_point_r,
            "black_point_g" => self.black_point_g,
            "black_point_b" => self.black_point_b,
            "black_point_a" => self.black_point_a,

            "white_point_r" => self.white_point_r,
            "white_point_g" => self.white_point_g,
            "white_point_b" => self.white_point_b,
            "white_point_a" => self.white_point_a,

            "lift_r" => self.lift_r,
            "lift_g" => self.lift_g,
            "lift_b" => self.lift_b,
            "lift_a" => self.lift_a,

            "gain_r" => self.gain_r,
            "gain_g" => self.gain_g,
            "gain_b" => self.gain_b,
            "gain_a" => self.gain_a,

            "multiply_r" => self.multiply_r,
            "multiply_g" => self.multiply_g,
            "multiply_b" => self.multiply_b,
            "multiply_a" => self.multiply_a,

            "offset_r" => self.offset_r,
            "offset_g" => self.offset_g,
            "offset_b" => self.offset_b,
            "offset_a" => self.offset_a,

            "gamma_r" => self.gamma_r,
            "gamma_g" => self.gamma_g,
            "gamma_b" => self.gamma_b,
            "gamma_a" => self.gamma_a,

            "mix" => self.mix,

            _ => 0.0,
        }
    }

    fn set_attr_f32(&mut self, name: &str, value: f32) {
        match name {
            "black_point_r" => self.black_point_r = value,
            "black_point_g" => self.black_point_g = value,
            "black_point_b" => self.black_point_b = value,
            "black_point_a" => self.black_point_a = value,

            "white_point_r" => self.white_point_r = value,
            "white_point_g" => self.white_point_g = value,
            "white_point_b" => self.white_point_b = value,
            "white_point_a" => self.white_point_a = value,

            "lift_r" => self.lift_r = value,
            "lift_g" => self.lift_g = value,
            "lift_b" => self.lift_b = value,
            "lift_a" => self.lift_a = value,

            "gain_r" => self.gain_r = value,
            "gain_g" => self.gain_g = value,
            "gain_b" => self.gain_b = value,
            "gain_a" => self.gain_a = value,

            "multiply_r" => self.multiply_r = value,
            "multiply_g" => self.multiply_g = value,
            "multiply_b" => self.multiply_b = value,
            "multiply_a" => self.multiply_a = value,

            "offset_r" => self.offset_r = value,
            "offset_g" => self.offset_g = value,
            "offset_b" => self.offset_b = value,
            "offset_a" => self.offset_a = value,

            "gamma_r" => self.gamma_r = value,
            "gamma_g" => self.gamma_g = value,
            "gamma_b" => self.gamma_b = value,
            "gamma_a" => self.gamma_a = value,

            "mix" => self.mix = value,

            _ => (),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct GradeValidate {}

impl GradeValidate {
    pub fn new() -> GradeValidate {
        GradeValidate {}
    }
}

impl Validate for GradeValidate {
    fn validate_inputs(
        &self,
        _node_type_id: u8,
        _attr_block: &Box<dyn AttrBlock>,
        hash_value: HashValue,
        node_compute_mode: NodeComputeMode,
        input_nodes: &Vec<&Box<NodeImpl>>,
    ) -> Vec<NodeComputeMode> {
        debug!(
            "GradeValidate::validate_inputs(): NodeComputeMode={:#?} HashValue={:#?}",
            node_compute_mode, hash_value
        );
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
