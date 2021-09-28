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

use crate::cxxbridge::ffi::BlockSize;
pub use crate::pixelblock::pixelblock::row::RowsFloat32;

/// Get the pixel index into a packed data structure of RGB(A) pixel
/// data.
///
/// Returns the index if the x,y coordinates are with-in bounds,
/// otherwise -1 is returned.
#[inline]
fn get_index(width: i32, height: i32, x_stride: i32, y_stride: i32, x: i32, y: i32) -> isize {
    let index_x = x * x_stride;
    let index_y = y * y_stride;
    let max_x = (width - 1) * x_stride;
    let max_y = (height - 1) * y_stride;
    let mut index = index_x + index_y;
    if (index_x < 0) || (index_x > max_x) || (index_y < 0) || (index_y > max_y) {
        index = -1;
    }
    return index as isize;
}

// #[derive(Clone, Copy, Debug, Hash, Default, Eq, PartialEq, Ord, PartialOrd)]
// pub struct BlockSize {
//     width: i32,
//     height: i32,
//     num_channels: i32,
// }

impl BlockSize {
    pub fn new(width: i32, height: i32, num_channels: i32) -> BlockSize {
        BlockSize {
            width,
            height,
            num_channels,
        }
    }

    pub fn empty() -> BlockSize {
        let width = 0;
        let height = 0;
        let num_channels = 0;
        BlockSize::new(width, height, num_channels)
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn num_channels(&self) -> i32 {
        self.num_channels
    }

    pub fn size(&self) -> usize {
        (self.width * self.height * self.num_channels) as usize
    }

    pub fn get_index(&self, x: i32, y: i32) -> isize {
        let x_stride = self.num_channels;
        let y_stride = self.width * x_stride;
        get_index(self.width, self.height, x_stride, y_stride, x, y)
    }

    pub fn get_xy(&self, ix: usize) -> (i32, i32) {
        let row_stride = (self.width() * self.num_channels()) as usize;
        let row_ix = ix % row_stride;
        let x = row_ix / (self.num_channels() as usize);
        let y = ix / row_stride as usize;
        (x as i32, y as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixelblock::pixelblock::PixelBlock;

    #[test]
    fn new() {
        let width = 3;
        let height = 3;
        let num_channels = 3;
        let blocksize = BlockSize::new(width, height, num_channels);
        assert_eq!(blocksize.size(), (width * height * num_channels) as usize);
    }

    #[test]
    fn empty() {
        let blocksize = BlockSize::empty();
        assert_eq!(blocksize.size(), 0usize);
    }

    #[test]
    fn get_index() {
        let width = 3;
        let height = 2;
        let num_channels = 3;
        let blocksize = BlockSize::new(width, height, num_channels);
        let a = blocksize.get_index(0, 0);
        let b = blocksize.get_index(1, 0);
        let c = blocksize.get_index(0, 1);
        let d = blocksize.get_index(1, 1);
        assert_eq!(a, 0isize);
        assert_eq!(b, num_channels as isize);
        assert_eq!(c, (width * num_channels) as isize);
        assert_eq!(d, ((width * num_channels) + num_channels) as isize);
    }

    #[test]
    fn get_xy() {
        let width = 3;
        let height = 2;
        let num_channels = 4;
        let blocksize = BlockSize::new(width, height, num_channels);
        let (ax, ay) = blocksize.get_xy(0);
        assert_eq!(ax, 0i32);
        assert_eq!(ay, 0i32);

        let ix = num_channels as usize;
        let (bx, by) = blocksize.get_xy(ix);
        assert_eq!(bx, 1i32);
        assert_eq!(by, 0i32);

        let ix = (width * num_channels) as usize;
        let (cx, cy) = blocksize.get_xy(ix);
        assert_eq!(cx, 0i32);
        assert_eq!(cy, 1i32);

        let ix = ((width * num_channels) + num_channels) as usize;
        let (dx, dy) = blocksize.get_xy(ix);
        assert_eq!(dx, 1i32);
        assert_eq!(dy, 1i32);
    }
}
