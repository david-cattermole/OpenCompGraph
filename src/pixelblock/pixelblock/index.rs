// use std::ops::{Add, Sub};

use crate::cxxbridge::ffi::BlockSize;
// use crate::pixelblock::blocksize::BlockSize;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Ord, PartialOrd)]
pub struct BlockIndex {
    index: usize,
    width: usize,
    height: usize,
    num_channels: usize,
}

impl BlockIndex {
    pub fn new(blocksize: BlockSize) -> BlockIndex {
        BlockIndex {
            index: 0,
            width: blocksize.width() as usize,
            height: blocksize.height() as usize,
            num_channels: blocksize.num_channels() as usize,
        }
    }

    pub fn from_index(blocksize: BlockSize, index: usize) -> BlockIndex {
        BlockIndex {
            index,
            width: blocksize.width() as usize,
            height: blocksize.height() as usize,
            num_channels: blocksize.num_channels() as usize,
        }
    }

    pub fn from_xy(blocksize: BlockSize, x: usize, y: usize) -> BlockIndex {
        let width = blocksize.width() as usize;
        let height = blocksize.height() as usize;

        if x >= width {
            panic!("Column out of bounds");
        }

        if y >= height {
            panic!("Row out of bounds");
        }

        let num_channels = blocksize.num_channels() as usize;
        let index = y * (width * num_channels) + (x * num_channels);
        BlockIndex {
            index,
            width,
            height,
            num_channels,
        }
    }

    pub fn column(&self) -> usize {
        self.index % (self.width * self.num_channels)
    }

    pub fn row(&self) -> usize {
        self.index / (self.height * self.num_channels)
    }
}

// Some type casting traits.
impl From<BlockIndex> for usize {
    fn from(ix: BlockIndex) -> Self {
        ix.index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let width = 1;
        let height = 1;
        let num_channels = 4;
        let blocksize = BlockSize::new(width, height, num_channels);
        let index_num: usize = BlockIndex::new(blocksize).into();
        assert_eq!(0usize, index_num);
    }

    #[test]
    fn from_index() {
        let width = 1;
        let height = 1;
        let num_channels = 3;
        let blocksize = BlockSize::new(width, height, num_channels);
        let index_num: usize = BlockIndex::from_index(blocksize, 3).into();
        assert_eq!(3usize, index_num);
    }
}
