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

#[derive(Debug, Clone)]
pub struct DeformerBrownian {
    /// Enable/disable the deformer.
    enable: i32,

    /// Center Offset X
    xc: f32,

    /// Center Offset Y
    yc: f32,

    /// Distortion Coefficient Degree 2
    k1: f32,

    /// Distortion Coefficient Degree 4
    k2: f32,
}

impl hash::Hash for DeformerBrownian {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.enable.hash(state);
        if self.enable == 0 {
            return;
        }

        HashableF32::new(self.k1).hash(state);
        HashableF32::new(self.k2).hash(state);
        HashableF32::new(self.xc).hash(state);
        HashableF32::new(self.yc).hash(state);
    }
}

impl Default for DeformerBrownian {
    fn default() -> Self {
        Self {
            enable: 1,
            xc: 0.0,
            yc: 0.0,
            k1: 0.0,
            k2: 0.0,
        }
    }
}

/// Brownian lens distortion model.
///
/// xu = xd + ((xd - xc) * ((k1 * r2) + (k2 * r4)));
/// yu = yd + ((yd - yc) * ((k1 * r2) + (k2 * r4)));
///
/// where:
///   xu = undistorted image point
///   xd = distorted image point
///   xc = distortion center
///   k1, k2, etc = Nth radial distortion coefficent
///   p1, p2, etc = Nth tangential distortion coefficent
///   r = sqrt(pow(xd - xc, 2) + pow(yd - yc, 2))
///
fn _apply(
    obj: &DeformerBrownian,
    xd: f32,
    yd: f32,
    image_window: BBox2Df,
    inverse: bool,
) -> (f32, f32) {
    let min_x = image_window.min_x;
    let min_y = image_window.min_y;
    let max_x = image_window.max_x;
    let max_y = image_window.max_y;

    let xd_remap = interp::remap(min_x, max_x, 0.0, 1.0, xd);
    let yd_remap = interp::remap(min_y, max_y, 0.0, 1.0, yd);

    let xc = obj.xc;
    let yc = obj.yc;
    let k1 = obj.k1;
    let k2 = obj.k2;

    let r: f32 = ((xd - xc).powi(2) + (yd - yc).powi(2)).sqrt();
    let r2: f32 = r.powi(2);
    let r4: f32 = r.powi(4) * 2.0;

    let (xu, yu) = match inverse {
        false => (
            xd_remap + ((xd_remap - xc) * ((k1 * r2) + (k2 * r4))),
            yd_remap + ((yd_remap - yc) * ((k1 * r2) + (k2 * r4))),
        ),
        true => (
            xd_remap - ((xd_remap - xc) * ((k1 * r2) + (k2 * r4))),
            yd_remap - ((yd_remap - yc) * ((k1 * r2) + (k2 * r4))),
        ),
    };

    let xu_remap = interp::remap(0.0, 1.0, min_x, max_x, xu);
    let yu_remap = interp::remap(0.0, 1.0, min_y, max_y, yu);

    (xu_remap, yu_remap)
}

impl Deformer for DeformerBrownian {
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

impl AttrBlock for DeformerBrownian {
    fn attr_hash(&self, _frame: FrameValue, state: &mut DefaultHasher) {
        self.hash(state)
    }

    fn attr_exists(&self, name: &str) -> AttrState {
        match name {
            "enable" => AttrState::Exists,
            "k1" => AttrState::Exists,
            "k2" => AttrState::Exists,
            "xc" => AttrState::Exists,
            "yc" => AttrState::Exists,
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
            "xc" => self.xc,
            "yc" => self.yc,
            _ => 0.0,
        }
    }

    fn set_attr_f32(&mut self, name: &str, value: f32) {
        match name {
            "k1" => self.k1 = value,
            "k2" => self.k2 = value,
            "xc" => self.xc = value,
            "yc" => self.yc = value,
            _ => (),
        }
    }
}
