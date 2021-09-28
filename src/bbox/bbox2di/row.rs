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

use crate::cxxbridge::ffi::BBox2Di;

/// Represents a single row of a BBox2Di.
pub struct BBox2DRow<'a> {
    pub inner: &'a BBox2Di,
    pub row_index: usize,
}

impl<'a> BBox2DRow<'a> {
    pub fn new(inner: &'a BBox2Di, row: usize) -> Self {
        BBox2DRow {
            inner,
            row_index: row,
        }
    }

    #[allow(dead_code)]
    pub fn min(&self) -> i32 {
        self.inner.min_x as i32
    }

    pub fn max(&self) -> i32 {
        self.inner.max_x as i32
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.inner.width() as usize
    }

    #[allow(dead_code)]
    pub fn has_next(&self) -> bool {
        self.row_index < self.len()
    }
}
