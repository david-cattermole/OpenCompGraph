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

use nalgebra as na;
use std::hash::{Hash, Hasher};

use crate::cxxbridge::ffi::Vector4f32;
use crate::hashutils;

impl Vector4f32 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vector4f32 {
        Vector4f32 { x, y, z, w }
    }

    pub fn identity() -> Vector4f32 {
        Vector4f32 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    pub fn to_na_vector(&self) -> na::Vector4<f32> {
        na::Vector4::<f32>::new(self.x, self.y, self.z, self.w)
    }

    pub fn from_na_vector(value: na::Vector4<f32>) -> Vector4f32 {
        Vector4f32 {
            x: value[(0)],
            y: value[(1)],
            z: value[(2)],
            w: value[(3)],
        }
    }
}

impl Hash for Vector4f32 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        hashutils::HashableF32::new(self.x).hash(state);
        hashutils::HashableF32::new(self.y).hash(state);
        hashutils::HashableF32::new(self.z).hash(state);
        hashutils::HashableF32::new(self.w).hash(state);
    }
}
