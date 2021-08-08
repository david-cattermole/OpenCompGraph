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

use crate::cxxbridge::ffi::Vector4i32;

impl Vector4i32 {
    pub fn new(x: i32, y: i32, z: i32, w: i32) -> Vector4i32 {
        Vector4i32 { x, y, z, w }
    }

    pub fn identity() -> Vector4i32 {
        Vector4i32 {
            x: 0,
            y: 0,
            z: 0,
            w: 0,
        }
    }

    pub fn to_na_vector(&self) -> na::Vector4<i32> {
        na::Vector4::<i32>::new(self.x, self.y, self.z, self.w)
    }

    pub fn from_na_vector(value: na::Vector4<i32>) -> Vector4i32 {
        Vector4i32 {
            x: value[(0)],
            y: value[(1)],
            z: value[(2)],
            w: value[(3)],
        }
    }
}
