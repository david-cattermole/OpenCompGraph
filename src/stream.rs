use image;

use crate::cxxbridge::ffi::StreamDataState;
use crate::data::HashValue;
use crate::data::{BoundingBox2D, Matrix4, PixelBlock};

#[derive(Debug, Clone, Hash)]
pub struct StreamDataImpl {
    state: StreamDataState,
    hash: HashValue,
    bbox: Box<BoundingBox2D>,
    pixel_block: Box<PixelBlock>,
    color_matrix: Box<Matrix4>,
    transform_matrix: Box<Matrix4>,
}

impl StreamDataImpl {
    pub fn new() -> StreamDataImpl {
        let state = StreamDataState::Invalid;
        let hash = 0;
        let width = 2;
        let height = 2;
        let num_channels = 3;
        let bbox = Box::new(BoundingBox2D::new());
        let pixel_block = Box::new(PixelBlock::new(width, height, num_channels));
        let color_matrix = Box::new(Matrix4::new());
        let transform_matrix = Box::new(Matrix4::new());
        StreamDataImpl {
            state,
            hash,
            bbox,
            pixel_block,
            color_matrix,
            transform_matrix,
        }
    }

    pub fn get_state(&self) -> StreamDataState {
        self.state
    }

    pub fn get_hash(&self) -> HashValue {
        self.hash
    }

    pub fn get_bounding_box(&self) -> &Box<BoundingBox2D> {
        &self.bbox
    }

    pub fn get_pixel_block(&self) -> &Box<PixelBlock> {
        &self.pixel_block
    }

    pub fn get_pixel_block_as_mut(&mut self) -> &mut Box<PixelBlock> {
        &mut self.pixel_block
    }

    pub fn set_pixel_block(&mut self, pixel_block: Box<PixelBlock>) {
        self.pixel_block = pixel_block;
    }

    pub fn get_color_matrix(&self) -> &Box<Matrix4> {
        &self.color_matrix
    }

    pub fn get_transform_matrix(&self) -> &Box<Matrix4> {
        &self.transform_matrix
    }
}
