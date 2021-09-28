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

use crate::cxxbridge::ffi::DataType;

impl DataType {
    #[inline]
    pub fn size_bytes(&self) -> usize {
        let num_bytes = match *self {
            DataType::UInt8 => std::mem::size_of::<u8>(),
            DataType::Half16 => std::mem::size_of::<f16>(),
            DataType::Float32 => std::mem::size_of::<f32>(),
            DataType::UInt16 => std::mem::size_of::<u16>(),
            _ => {
                panic!("Invalid DataType: {:?}", self);
            }
        };
        num_bytes
    }
}
