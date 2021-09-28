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

pub mod index;
pub mod row;

use half::f16;
use log::debug;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::time::Instant;

use crate::cxxbridge::ffi::BBox2Di;
use crate::cxxbridge::ffi::BlockSize;
use crate::cxxbridge::ffi::DataType;
use crate::data::COLOR_BARS;
use crate::data::COLOR_BARS_HEIGHT;
use crate::data::COLOR_BARS_NUM_CHANNELS;
use crate::data::COLOR_BARS_WIDTH;
use crate::pixelblock::datablock::DataBlock;
use crate::pixelblock::dataslice::DataSlice;
use crate::pixelblock::dataslice::DataSliceMut;
use crate::pixelblock::pixelblock::row::RowFloat32;
use crate::pixelblock::pixelblock::row::RowsFloat32;
use crate::pixelblock::pixelblock::row::RowsFloat32Iterator;
use crate::pixelblock::utils::transmute_slice_f16_to_u16;
use crate::pixelblock::utils::transmute_slice_mut_f16_to_u16;

#[derive(Clone)]
pub struct PixelBlock {
    blocksize: BlockSize,
    datablock: DataBlock,
}

impl PixelBlock {
    pub fn new(blocksize: BlockSize, pixel_data_type: DataType) -> PixelBlock {
        let size = blocksize.size();
        let datablock = DataBlock::new(size, pixel_data_type);
        PixelBlock {
            blocksize,
            datablock,
        }
    }

    pub fn empty(pixel_data_type: DataType) -> PixelBlock {
        let blocksize = BlockSize::empty();
        let datablock = DataBlock::empty(pixel_data_type);
        PixelBlock {
            blocksize,
            datablock,
        }
    }

    pub fn new_constant_pixel_rgba_f32(r: f32, g: f32, b: f32, a: f32) -> PixelBlock {
        let width = 1;
        let height = 1;
        let num_channels = 4;
        let blocksize = BlockSize::new(width, height, num_channels);

        let single_pixel = [r, g, b, a];
        let datablock = DataBlock::from_slice_f32(&single_pixel);

        PixelBlock {
            blocksize,
            datablock,
        }
    }

    pub fn new_color_bars() -> PixelBlock {
        let width = COLOR_BARS_WIDTH;
        let height = COLOR_BARS_HEIGHT;
        let num_channels = COLOR_BARS_NUM_CHANNELS;
        let pixel_slice = COLOR_BARS;

        let blocksize = BlockSize::new(width, height, num_channels);
        let datablock = DataBlock::from_slice_f32(pixel_slice);
        PixelBlock {
            blocksize,
            datablock,
        }
    }

    pub fn from_datablock(blocksize: BlockSize, datablock: DataBlock) -> PixelBlock {
        let block_size = blocksize.size();
        let data_len = datablock.len();
        assert_eq!(block_size, data_len);
        PixelBlock {
            blocksize,
            datablock,
        }
    }

    /// Construct a pixel block from an existing PixelBlock, with a
    /// new data window.
    ///
    /// The pixels inside the data window are copied.
    pub fn from_pixel_block(
        pixel_block: &PixelBlock,
        data_window: BBox2Di,
        crop_window: BBox2Di,
    ) -> PixelBlock {
        let start = Instant::now();
        assert!(pixel_block.width() == data_window.width());
        assert!(pixel_block.height() == data_window.height());

        let diff_window = BBox2Di::intersection(data_window, crop_window);

        let width = crop_window.width();
        let height = crop_window.height();
        let num_channels = pixel_block.num_channels();

        let data_type = pixel_block.data_type();
        let blocksize = BlockSize::new(width, height, num_channels);
        let mut new_pixel_block = PixelBlock::new(blocksize, data_type);

        // Detect if there are any pixels to copy over. If not, simply
        // return the transparent black pixels for the crop window.
        if diff_window.area() == 0 {
            return new_pixel_block;
        }

        // Copy over pixels, scanline by scanline.
        //
        // Each scanline may live inside or outside the original data
        // window.
        let dst_offset_y = crop_window.min_y;
        let dst_offset_x = diff_window.min_x - crop_window.min_x;
        let src_offset_y = data_window.min_y;
        let src_offset_start_x = (diff_window.min_x - data_window.min_x) * num_channels;
        let src_offset_end_x = src_offset_start_x + (diff_window.width()) * num_channels;

        // Copy each source row to the destination.
        let start_copy = Instant::now();

        for y in diff_window.min_y..diff_window.max_y {
            // Destination indexes into the cropped output image.
            let dst_row = y - dst_offset_y;
            let dst_start_index =
                (((dst_row * crop_window.width()) + dst_offset_x) * num_channels) as usize;
            let dst_end_index = dst_start_index + (diff_window.width() * num_channels) as usize;

            // Source indexes into the input image.
            let src_row = y - src_offset_y;
            let src_row_index = (src_row * data_window.width() * num_channels) as usize;
            let src_start_index = src_row_index + src_offset_start_x as usize;
            let src_end_index = src_row_index + src_offset_end_x as usize;

            let mut dst_dataslice = new_pixel_block.as_mut_slice();
            match &mut dst_dataslice {
                DataSliceMut::Float32(dst) => {
                    let dst_data = &mut dst[dst_start_index..dst_end_index];
                    let src_data = &pixel_block.as_slice_f32()[src_start_index..src_end_index];
                    dst_data.copy_from_slice(src_data);
                }

                DataSliceMut::UInt8(dst) => {
                    let dst_data = &mut dst[dst_start_index..dst_end_index];
                    let src_data = &pixel_block.as_slice_u8()[src_start_index..src_end_index];
                    dst_data.copy_from_slice(src_data);
                }
                DataSliceMut::Half16(dst) => {
                    let dst_data = &mut dst[dst_start_index..dst_end_index];
                    let src_data = &pixel_block.as_slice_f16()[src_start_index..src_end_index];
                    dst_data.copy_from_slice(src_data);
                }
                DataSliceMut::UInt16(dst) => {
                    let dst_data = &mut dst[dst_start_index..dst_end_index];
                    let src_data = &pixel_block.as_slice_u16()[src_start_index..src_end_index];
                    dst_data.copy_from_slice(src_data);
                }
            }
        }

        let duration_copy = start_copy.elapsed();
        let duration = start.elapsed();
        debug!("from_pixel_block copy time: {:?}", duration_copy);
        debug!("from_pixel_block total time: {:?}", duration);

        new_pixel_block
    }

    pub fn as_slice(&self) -> DataSlice {
        self.datablock.as_slice()
    }

    pub fn as_slice_f32(&self) -> &[f32] {
        self.datablock.as_slice_f32()
    }

    pub fn as_slice_f16(&self) -> &[f16] {
        self.datablock.as_slice_f16()
    }

    pub fn as_slice_f16_fake(&self) -> &[u16] {
        let slice_f16 = self.as_slice_f16();
        transmute_slice_f16_to_u16(slice_f16)
    }

    pub fn as_slice_u16(&self) -> &[u16] {
        self.datablock.as_slice_u16()
    }

    pub fn as_slice_u8(&self) -> &[u8] {
        self.datablock.as_slice_u8()
    }

    pub fn as_mut_slice(&mut self) -> DataSliceMut {
        self.datablock.as_mut_slice()
    }

    pub fn as_mut_slice_f32(&mut self) -> &mut [f32] {
        self.datablock.as_mut_slice_f32()
    }

    pub fn as_mut_slice_f16(&mut self) -> &mut [f16] {
        self.datablock.as_mut_slice_f16()
    }

    pub fn as_mut_slice_f16_fake(&mut self) -> &mut [u16] {
        let slice_f16 = self.as_mut_slice_f16();
        transmute_slice_mut_f16_to_u16(slice_f16)
    }

    pub fn as_mut_slice_u16(&mut self) -> &mut [u16] {
        self.datablock.as_mut_slice_u16()
    }

    pub fn as_mut_slice_u8(&mut self) -> &mut [u8] {
        self.datablock.as_mut_slice_u8()
    }

    pub fn data_type(&self) -> DataType {
        self.datablock.data_type()
    }

    pub fn data_resize(&mut self, blocksize: BlockSize, data_type: DataType) {
        let new_length = blocksize.size();
        self.datablock.data_resize(new_length, data_type);
        self.blocksize = blocksize;
    }

    pub fn convert_into_data_type(&mut self, to_data_type: DataType) {
        self.datablock.convert_into_data_type(to_data_type);
    }

    pub fn size_bytes(&self) -> usize {
        self.datablock.size_bytes()
    }

    pub fn get_pixel_index(&self, x: i32, y: i32) -> isize {
        self.blocksize.get_index(x, y)
    }

    pub fn width(&self) -> i32 {
        self.blocksize.width()
    }

    pub fn height(&self) -> i32 {
        self.blocksize.height()
    }

    pub fn num_channels(&self) -> i32 {
        self.blocksize.num_channels()
    }

    pub fn size(&self) -> usize {
        self.blocksize.size()
    }
}

impl Hash for PixelBlock {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.blocksize.hash(state);
        // Note: Skipping the 'pixels' data.
    }
}

impl fmt::Debug for PixelBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PixelBlock")
            .field("blocksize", &self.blocksize)
            .finish()
    }
}

impl<'a> RowsFloat32<'a> for PixelBlock {
    fn row(&self, row: usize) -> RowFloat32 {
        RowFloat32::new(self, row)
    }

    fn rows(&'a self) -> Box<dyn Iterator<Item = RowFloat32> + 'a> {
        Box::new(RowsFloat32Iterator::new(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_f32() {
        let width = 3;
        let height = 5;
        let num_channels = 4;
        let blocksize = BlockSize::new(width, height, num_channels);
        let data_type = DataType::Float32;
        let pixelblock = PixelBlock::new(blocksize, data_type);
        assert_eq!(
            pixelblock.size_bytes(),
            (width * height * num_channels * 4) as usize
        );
    }

    #[test]
    fn new_u16() {
        let width = 2;
        let height = 4;
        let num_channels = 3;
        let blocksize = BlockSize::new(width, height, num_channels);
        let data_type = DataType::UInt16;
        let pixelblock = PixelBlock::new(blocksize, data_type);
        assert_eq!(
            pixelblock.size_bytes(),
            (width * height * num_channels * 2) as usize
        );
    }

    #[test]
    fn new_u8() {
        let width = 2;
        let height = 2;
        let num_channels = 3;
        let blocksize = BlockSize::new(width, height, num_channels);
        let data_type = DataType::UInt8;
        let pixelblock = PixelBlock::new(blocksize, data_type);
        assert_eq!(
            pixelblock.size_bytes(),
            (width * height * num_channels) as usize
        );
    }

    #[test]
    fn get_index() {
        let width = 3;
        let height = 5;
        let num_channels = 4;
        let blocksize = BlockSize::new(width, height, num_channels);
        let data_type = DataType::Float32;
        let pixelblock = PixelBlock::new(blocksize, data_type);

        let index_num: usize = BlockIndex::new(blocksize).into();
        assert_eq!(0usize, index_num);

        let index_num: usize = BlockIndex::from_index(blocksize, 3).into();
        assert_eq!(3usize, index_num);

        let index_num: usize = BlockIndex::from_index(blocksize, 6).into();
        assert_eq!(6usize, index_num);

        let index_num: usize = BlockIndex::from_xy(blocksize, 2, 0).into();
        assert_eq!((2 * num_channels) as usize, index_num);

        let index_num: usize = BlockIndex::from_xy(blocksize, 2, 1).into();
        assert_eq!(
            ((width * num_channels) + (2 * num_channels)) as usize,
            index_num,
        );
    }

    #[test]
    fn get_row() {
        let width = 3;
        let height = 5;
        let num_channels = 4;
        let blocksize = BlockSize::new(width, height, num_channels);
        let data_type = DataType::Float32;
        let pixelblock = PixelBlock::new(blocksize, data_type);

        match pixelblock.data_type() {
            DataType::Float32 => {
                let row = pixelblock.row(1);
                let rs = row.as_slice();
                assert_eq!(rs.len(), (width * num_channels) as usize);
            }
            _ => panic!("unsupported."),
        }
    }
}
