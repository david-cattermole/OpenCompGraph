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

/// Iterator over row items.
pub struct BBox2DRowIterator<'a> {
    inner: &'a BBox2DRow<'a>,
    index: i32,
}

impl<'a> BBox2DRowIterator<'a> {
    #[allow(dead_code)]
    pub fn new(inner: &'a BBox2DRow) -> Self {
        BBox2DRowIterator {
            inner,
            index: inner.min(),
        }
    }
}

/// Implement a Iterator for Rows.
impl<'a> Iterator for BBox2DRowIterator<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.index >= self.inner.max() {
            return None;
        }

        let current = self.index;
        self.index += 1;
        Some(current)
    }
}
