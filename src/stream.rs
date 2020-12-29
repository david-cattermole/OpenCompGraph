use image;
use std::rc::Rc;

use crate::cxxbridge::ffi::StreamDataState;
use crate::data::BoundingBox2D;
use crate::data::HashValue;
use crate::data::Matrix4;
use crate::data::PixelBlock;

#[derive(Debug, Clone, Hash)]
pub struct StreamDataImpl {
    state: StreamDataState,
    hash: HashValue,
    bbox: Box<BoundingBox2D>,
    color_matrix: Box<Matrix4>,
    transform_matrix: Box<Matrix4>,
    pixel_block: Rc<PixelBlock>,
}

impl StreamDataImpl {
    pub fn new() -> StreamDataImpl {
        let state = StreamDataState::Invalid;
        let hash = 0;

        let width = 2;
        let height = 2;
        let num_channels = 3;
        let pixel_block = Rc::new(PixelBlock::new(width, height, num_channels));

        let bbox = Box::new(BoundingBox2D::new());
        let color_matrix = Box::new(Matrix4::new());
        let transform_matrix = Box::new(Matrix4::new());

        StreamDataImpl {
            state,
            hash,
            bbox,
            color_matrix,
            transform_matrix,
            pixel_block,
        }
    }

    pub fn get_state(&self) -> StreamDataState {
        self.state
    }

    pub fn get_state_id(&self) -> u8 {
        self.state.repr
    }

    pub fn get_hash(&self) -> HashValue {
        self.hash
    }

    pub fn get_bounding_box(&self) -> &Box<BoundingBox2D> {
        &self.bbox
    }

    pub fn get_pixel_block(&self) -> &PixelBlock {
        &self.pixel_block
    }

    pub fn get_pixel_block_as_mut(&mut self) -> &mut PixelBlock {
        Rc::make_mut(&mut self.pixel_block)
    }

    pub fn set_pixel_block(&mut self, pixel_block: PixelBlock) {
        // when the "old" Rc goes out of scope it will be cleaned up
        // if there are no more references to the underlying
        // allocation.
        let old_data = Rc::clone(&self.pixel_block);
        self.pixel_block = Rc::new(pixel_block);
    }

    pub fn get_pixel_buffer(&self) -> &[f32] {
        &self.pixel_block.pixels.as_slice()
    }

    pub fn get_pixel_width(&self) -> u32 {
        self.pixel_block.width
    }

    pub fn get_pixel_height(&self) -> u32 {
        self.pixel_block.height
    }

    pub fn get_pixel_num_channels(&self) -> u8 {
        self.pixel_block.num_channels
    }

    pub fn get_color_matrix(&self) -> &Box<Matrix4> {
        &self.color_matrix
    }

    pub fn get_transform_matrix(&self) -> &Box<Matrix4> {
        &self.transform_matrix
    }
}
