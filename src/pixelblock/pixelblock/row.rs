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

use std::ops::Index;

use crate::pixelblock::pixelblock::PixelBlock;

type RowValue = f32;

/// Represents a single row of pixels as f32 values.
pub struct RowFloat32<'a> {
    inner: &'a PixelBlock,
    pub row_index: usize,
}

impl<'a> RowFloat32<'a> {
    pub fn new(inner: &'a PixelBlock, row: usize) -> Self {
        RowFloat32 {
            inner,
            row_index: row,
        }
    }

    pub fn as_slice(&self) -> &[RowValue] {
        let width = self.inner.width() as usize;
        let num_channels = self.inner.num_channels() as usize;
        let stride = width * num_channels;
        let begin = self.row_index * stride;
        let end = (self.row_index + 1) * stride;
        &self.inner.as_slice_f32()[begin..end]
    }

    pub fn has_next(&self) -> bool {
        self.row_index < (self.inner.height() as usize)
    }
}

/// Index overloading for Row.
impl<'a> Index<usize> for RowFloat32<'a> {
    type Output = RowValue;

    fn index(&self, ix: usize) -> &Self::Output {
        &self.inner.as_slice_f32()[(self.row_index * self.inner.width() as usize) + ix]
    }
}

/// Iterator over rows.
pub struct RowsFloat32Iterator<'a> {
    inner: &'a PixelBlock,
    next_row: usize,
}

impl<'a> RowsFloat32Iterator<'a> {
    pub fn new(inner: &'a PixelBlock) -> Self {
        RowsFloat32Iterator { inner, next_row: 0 }
    }
}

/// Implement a Iterator for Rows.
impl<'a> Iterator for RowsFloat32Iterator<'a> {
    type Item = RowFloat32<'a>;

    fn next(&mut self) -> Option<RowFloat32<'a>> {
        if self.next_row >= (self.inner.height() as usize) {
            return None;
        }

        let current = self.next_row;
        self.next_row += 1;
        Some(RowFloat32 {
            inner: self.inner,
            row_index: current,
        })
    }
}

/// List of rows.
pub trait RowsFloat32<'a> {
    fn row(&self, row: usize) -> RowFloat32;

    fn rows(&'a self) -> Box<dyn Iterator<Item = RowFloat32> + 'a>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixelblock::pixelblock::PixelBlock;

    #[test]
    fn slice_row() {
        let width = 3;
        let height = 3;
        let num_channels = 3;
        let pixel_data_type = DataType::Float32;
        let blocksize = BlockSize::new(width, height, num_channels);
        let b = PixelBlock::new(blocksize, pixel_data_type);
        let row = b.row(1);
        let rs = row.as_slice();
        assert_eq!(rs.len(), (width * num_channels) as usize);
        assert_eq!(rs[0], 0.0);
    }

    #[test]
    fn row_iterator() {
        let width = 3;
        let height = 3;
        let num_channels = 3;
        let pixel_data_type = DataType::Float32;
        let blocksize = BlockSize::new(width, height, num_channels);
        let b = PixelBlock::new(blocksize, pixel_data_type);
        assert_eq!(height as usize, b.rows().count());
    }
}
