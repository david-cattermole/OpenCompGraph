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
use crate::deformer::Deformer;
use crate::hashutils::HashableF32;

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

impl Deformer for DeformerBrownian {
    fn hash_deformer(&self) -> u64 {
        let mut state = DefaultHasher::default();
        self.hash(&mut state);
        state.finish()
    }

    fn commit_data(&mut self) -> Result<(), String> {
        Ok(())
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
    fn apply_forward(&self, xd: f32, yd: f32) -> (f32, f32) {
        let xc = self.xc;
        let yc = self.yc;
        let k1 = self.k1;
        let k2 = self.k2;

        let r: f32 = ((xd - xc).powi(2) + (yd - yc).powi(2)).sqrt();
        let r2: f32 = r.powi(2);
        let r4: f32 = r.powi(4) * 2.0;

        let xu = xd + ((xd - xc) * ((k1 * r2) + (k2 * r4)));
        let yu = yd + ((yd - yc) * ((k1 * r2) + (k2 * r4)));

        (xu, yu)
    }

    fn apply_backward(&self, xd: f32, yd: f32) -> (f32, f32) {
        let xc = self.xc;
        let yc = self.yc;
        let k1 = self.k1;
        let k2 = self.k2;

        let r: f32 = ((xd - xc).powi(2) + (yd - yc).powi(2)).sqrt();
        let r2: f32 = r.powi(2);
        let r4: f32 = r.powi(4) * 2.0;

        let xu = xd - ((xd - xc) * ((k1 * r2) + (k2 * r4)));
        let yu = yd - ((yd - yc) * ((k1 * r2) + (k2 * r4)));

        (xu, yu)
    }
}

impl AttrBlock for DeformerBrownian {
    fn attr_hash(&self, _frame: i32, state: &mut DefaultHasher) {
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
