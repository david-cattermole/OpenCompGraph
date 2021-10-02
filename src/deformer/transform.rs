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

// use log::warn;
use nalgebra as na;
use std::collections::hash_map::DefaultHasher;
use std::hash;
use std::hash::Hash;
use std::hash::Hasher;

use crate::attrblock::AttrBlock;
use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::BBox2Df;
use crate::data::FrameValue;
use crate::deformer::Deformer;
use crate::hashutils::HashableF32;
use crate::math::interp;
use crate::math::xform;

#[derive(Debug, Clone)]
pub struct DeformerTransform {
    pub enable: i32,
    pub invert: i32,
    pub translate_x: f32,
    pub translate_y: f32,
    pub rotate: f32,
    pub rotate_center_x: f32,
    pub rotate_center_y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub pivot_x: f32,
    pub pivot_y: f32,
}

impl hash::Hash for DeformerTransform {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.enable.hash(state);
        if self.enable == 0 {
            return;
        }

        self.invert.hash(state);
        HashableF32::new(self.translate_x).hash(state);
        HashableF32::new(self.translate_y).hash(state);
        HashableF32::new(self.rotate).hash(state);
        HashableF32::new(self.rotate_center_x).hash(state);
        HashableF32::new(self.rotate_center_y).hash(state);
        HashableF32::new(self.scale_x).hash(state);
        HashableF32::new(self.scale_y).hash(state);
        HashableF32::new(self.pivot_x).hash(state);
        HashableF32::new(self.pivot_y).hash(state);
    }
}

impl Default for DeformerTransform {
    fn default() -> Self {
        Self {
            enable: 1,
            invert: 0,
            translate_x: 0.0,
            translate_y: 0.0,
            rotate: 0.0,
            rotate_center_x: 0.0,
            rotate_center_y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            pivot_x: 0.5,
            pivot_y: 0.5,
        }
    }
}

/// 2D Transform deformer.
fn _apply(
    obj: &DeformerTransform,
    xd: f32,
    yd: f32,
    image_window: BBox2Df,
    inverse: bool,
) -> (f32, f32) {
    // Normalise the translate values between the image window.
    let tx = interp::inverse_lerp(image_window.min_x, image_window.max_x, obj.translate_x);
    let ty = interp::inverse_lerp(image_window.min_x, image_window.max_x, obj.translate_y);
    let mut out_matrix = xform::create_transform_trs_2d(
        tx,
        ty,
        obj.rotate_center_x,
        obj.rotate_center_y,
        obj.rotate,
        obj.scale_x,
        obj.scale_y,
    );

    // Apply matrix invert.
    let mut do_invert = false;
    if inverse == false {
        if obj.invert > 0 {
            do_invert = true;
        }
    } else {
        if obj.invert == 0 {
            do_invert = true;
        }
    }
    if do_invert == true {
        let inverse_matrix = out_matrix.try_inverse();
        match inverse_matrix {
            Some(value) => out_matrix = value,
            None => (),
        }
    }

    let px = obj.pivot_x;
    let py = obj.pivot_y;

    let v = na::Point4::new(xd - px, yd - py, 0.0, 1.0);
    let r = out_matrix * v;
    (r.x + px, r.y + py)
}

impl Deformer for DeformerTransform {
    fn hash_deformer(&self) -> u64 {
        let mut state = DefaultHasher::default();
        self.hash(&mut state);
        state.finish()
    }

    fn commit_data(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn apply_slice_in_place(
        &self,
        buffer: &mut [f32],
        image_window: BBox2Df,
        inverse: bool,
        stride: usize,
    ) {
        let count = buffer.len() / stride;
        for i in 0..count {
            let index = i * stride;

            let x = buffer[index + 0];
            let y = buffer[index + 1];

            let (xu, yu) = _apply(self, x, y, image_window, inverse);

            buffer[index + 0] = xu;
            buffer[index + 1] = yu;
        }
    }
}

impl AttrBlock for DeformerTransform {
    fn attr_hash(&self, _frame: FrameValue, state: &mut DefaultHasher) {
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
            "pivot_x" => AttrState::Exists,
            "pivot_y" => AttrState::Exists,
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
            "pivot_x" => self.pivot_x,
            "pivot_y" => self.pivot_y,
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
            "pivot_x" => self.pivot_x = value,
            "pivot_y" => self.pivot_y = value,
            _ => (),
        }
    }
}
