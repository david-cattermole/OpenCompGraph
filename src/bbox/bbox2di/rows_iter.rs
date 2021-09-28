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

use crate::bbox::bbox2di::row::BBox2DRow;
use crate::cxxbridge::ffi::BBox2Di;

/// Iterator over rows.
pub struct BBox2DRowsIterator<'a> {
    inner: &'a BBox2Di,
    next_row: usize,
}

impl<'a> BBox2DRowsIterator<'a> {
    pub fn new(inner: &'a BBox2Di) -> Self {
        BBox2DRowsIterator { inner, next_row: 0 }
    }
}

/// Implement a Iterator for Rows.
impl<'a> Iterator for BBox2DRowsIterator<'a> {
    type Item = BBox2DRow<'a>;

    fn next(&mut self) -> Option<BBox2DRow<'a>> {
        if self.next_row >= (self.inner.height() as usize) {
            return None;
        }

        let current = self.next_row;
        self.next_row += 1;
        Some(BBox2DRow {
            inner: self.inner,
            row_index: current,
        })
    }
}
