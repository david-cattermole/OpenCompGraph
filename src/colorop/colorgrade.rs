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
 */

use std::collections::hash_map::DefaultHasher;
use std::hash;
use std::hash::Hash;
use std::hash::Hasher;

use crate::attrblock::AttrBlock;
use crate::colorop::ColorOp;
use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::Vector4f32;
use crate::cxxbridge::ffi::Vector4i32;
use crate::data::FrameValue;
use crate::data::HashValue;
use crate::hashutils::HashableF32;
use crate::ops;

#[derive(Debug, Clone)]
pub struct ColorOpGrade {
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

impl hash::Hash for ColorOpGrade {
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

impl Default for ColorOpGrade {
    fn default() -> Self {
        Self {
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

impl ColorOp for ColorOpGrade {
    fn hash_color_op(&self) -> HashValue {
        let mut state = DefaultHasher::default();
        self.hash(&mut state);
        state.finish()
    }

    fn apply_slice_in_place(&self, buffer: &mut [f32], num_channels: i32) {
        let enable = self.enable;
        let process = Vector4i32::new(
            self.process_r,
            self.process_g,
            self.process_b,
            self.process_a,
        );
        let has_work_to_do = enable == 1
            && ((process.x != 0) || (process.y != 0) || (process.z != 0) || (process.w != 0));

        if has_work_to_do == false {
            // Set Output data
            return;
        }

        // Attributes that change are non-linear.
        let gamma = Vector4f32::new(self.gamma_r, self.gamma_g, self.gamma_b, self.gamma_a);
        let reverse = self.reverse != 0;
        let clamp_black = self.clamp_black != 0;
        let clamp_white = self.clamp_white != 0;
        let premult = self.premult != 0;
        let mix = self.mix;

        // Attributes making linear changes to the colour.
        let black_point = Vector4f32::new(
            self.black_point_r,
            self.black_point_g,
            self.black_point_b,
            self.black_point_a,
        );
        let white_point = Vector4f32::new(
            self.white_point_r,
            self.white_point_g,
            self.white_point_b,
            self.white_point_a,
        );
        let lift = Vector4f32::new(self.lift_r, self.lift_g, self.lift_b, self.lift_a);
        let gain = Vector4f32::new(self.gain_r, self.gain_g, self.gain_b, self.gain_a);
        let multiply = Vector4f32::new(
            self.multiply_r,
            self.multiply_g,
            self.multiply_b,
            self.multiply_a,
        );
        let offset = Vector4f32::new(self.offset_r, self.offset_g, self.offset_b, self.offset_a);

        ops::colorgrade::apply_color_grade_inplace(
            buffer,
            num_channels,
            process,
            black_point,
            white_point,
            lift,
            gain,
            multiply,
            offset,
            gamma,
            reverse,
            clamp_black,
            clamp_white,
            premult,
            mix,
        )
    }
}

impl AttrBlock for ColorOpGrade {
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
