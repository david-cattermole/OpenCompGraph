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

use crate::cxxbridge::ffi::Matrix4;
use crate::hashutils;

impl Matrix4 {
    pub fn from_diagonal(x: f32, y: f32, z: f32, w: f32) -> Matrix4 {
        Matrix4 {
            m00: x,
            m01: 0.0,
            m02: 0.0,
            m03: 0.0,
            //
            m10: 0.0,
            m11: y,
            m12: 0.0,
            m13: 0.0,
            //
            m20: 0.0,
            m21: 0.0,
            m22: z,
            m23: 0.0,
            //
            m30: 0.0,
            m31: 0.0,
            m32: 0.0,
            m33: w,
        }
    }

    pub fn identity() -> Matrix4 {
        Matrix4 {
            m00: 1.0,
            m01: 0.0,
            m02: 0.0,
            m03: 0.0,
            //
            m10: 0.0,
            m11: 1.0,
            m12: 0.0,
            m13: 0.0,
            //
            m20: 0.0,
            m21: 0.0,
            m22: 1.0,
            m23: 0.0,
            //
            m30: 0.0,
            m31: 0.0,
            m32: 0.0,
            m33: 1.0,
        }
    }

    pub fn to_na_matrix(&self) -> na::Matrix4<f32> {
        na::Matrix4::<f32>::new(
            self.m00, self.m01, self.m02, self.m03, //
            self.m10, self.m11, self.m12, self.m13, //
            self.m20, self.m21, self.m22, self.m23, //
            self.m30, self.m31, self.m32, self.m33, //
        )
    }

    pub fn from_na_matrix(value: na::Matrix4<f32>) -> Matrix4 {
        Matrix4 {
            m00: value[(0, 0)],
            m01: value[(0, 1)],
            m02: value[(0, 2)],
            m03: value[(0, 3)],
            //
            m10: value[(1, 0)],
            m11: value[(1, 1)],
            m12: value[(1, 2)],
            m13: value[(1, 3)],
            //
            m20: value[(2, 0)],
            m21: value[(2, 1)],
            m22: value[(2, 2)],
            m23: value[(2, 3)],
            //
            m30: value[(3, 0)],
            m31: value[(3, 1)],
            m32: value[(3, 2)],
            m33: value[(3, 3)],
        }
    }
}

impl Hash for Matrix4 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        hashutils::HashableF32::new(self.m00).hash(state);
        hashutils::HashableF32::new(self.m01).hash(state);
        hashutils::HashableF32::new(self.m02).hash(state);
        hashutils::HashableF32::new(self.m03).hash(state);
        //
        hashutils::HashableF32::new(self.m10).hash(state);
        hashutils::HashableF32::new(self.m11).hash(state);
        hashutils::HashableF32::new(self.m12).hash(state);
        hashutils::HashableF32::new(self.m13).hash(state);
        //
        hashutils::HashableF32::new(self.m20).hash(state);
        hashutils::HashableF32::new(self.m21).hash(state);
        hashutils::HashableF32::new(self.m22).hash(state);
        hashutils::HashableF32::new(self.m23).hash(state);
        //
        hashutils::HashableF32::new(self.m30).hash(state);
        hashutils::HashableF32::new(self.m31).hash(state);
        hashutils::HashableF32::new(self.m32).hash(state);
        hashutils::HashableF32::new(self.m33).hash(state);
    }
}
