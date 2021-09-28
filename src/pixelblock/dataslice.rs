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

use half::f16;

#[repr(u8)]
#[derive(Clone, Debug)]
pub enum DataSlice<'a> {
    Float32(&'a [f32]),
    UInt8(&'a [u8]),
    Half16(&'a [f16]),
    UInt16(&'a [u16]),
}

#[repr(u8)]
#[derive(Debug)]
pub enum DataSliceMut<'a> {
    Float32(&'a mut [f32]),
    UInt8(&'a mut [u8]),
    Half16(&'a mut [f16]),
    UInt16(&'a mut [u16]),
}

impl<'a> DataSlice<'a> {
    pub fn len(&self) -> usize {
        match self {
            DataSlice::Float32(slice) => slice.len(),
            DataSlice::UInt8(slice) => slice.len(),
            DataSlice::Half16(slice) => slice.len(),
            DataSlice::UInt16(slice) => slice.len(),
        }
    }
}
