use image;
use log::{debug, error, info, warn};
use std::rc::Rc;

use crate::bbox::BBox2D;
use crate::cxxbridge::ffi::StreamDataState;
use crate::data::HashValue;
use crate::matrix::Matrix4;
use crate::pixelblock::PixelBlock;

#[derive(Debug, Clone, Hash)]
pub struct StreamDataImpl {
    state: StreamDataState,
    hash: HashValue,
    display_window: Box<BBox2D>,
    data_window: Box<BBox2D>,
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

        let display_window = Box::new(BBox2D::new(0.0, 0.0, 1.0, 1.0));
        let data_window = Box::new(BBox2D::new(0.0, 0.0, 1.0, 1.0));
        let color_matrix = Box::new(Matrix4::new());
        let transform_matrix = Box::new(Matrix4::new());

        StreamDataImpl {
            state,
            hash,
            display_window,
            data_window,
            color_matrix,
            transform_matrix,
            pixel_block,
        }
    }

    pub fn state(&self) -> StreamDataState {
        self.state
    }

    pub fn state_id(&self) -> u8 {
        self.state.repr
    }

    pub fn set_state(&mut self, value: StreamDataState) {
        self.state = value;
    }

    pub fn hash(&self) -> HashValue {
        self.hash
    }

    pub fn set_hash(&mut self, value: HashValue) {
        self.hash = value;
    }

    pub fn display_window(&self) -> &Box<BBox2D> {
        &self.display_window
    }

    pub fn set_display_window(&mut self, value: Box<BBox2D>) {
        self.display_window = value;
    }

    pub fn data_window(&self) -> &Box<BBox2D> {
        &self.data_window
    }

    pub fn set_data_window(&mut self, value: Box<BBox2D>) {
        self.data_window = value;
    }

    pub fn pixel_block(&self) -> &PixelBlock {
        &self.pixel_block
    }

    pub fn pixel_block_as_mut(&mut self) -> &mut PixelBlock {
        Rc::make_mut(&mut self.pixel_block)
    }

    pub fn set_pixel_block(&mut self, pixel_block: PixelBlock) {
        // when the "old" Rc goes out of scope it will be cleaned up
        // if there are no more references to the underlying
        // allocation.
        let old_data = Rc::clone(&self.pixel_block);
        self.pixel_block = Rc::new(pixel_block);
    }

    pub fn pixel_buffer(&self) -> &[f32] {
        &self.pixel_block.pixels.as_slice()
    }

    pub fn pixel_width(&self) -> u32 {
        self.pixel_block.width
    }

    pub fn pixel_height(&self) -> u32 {
        self.pixel_block.height
    }

    pub fn pixel_num_channels(&self) -> u8 {
        self.pixel_block.num_channels
    }

    pub fn color_matrix(&self) -> &Box<Matrix4> {
        &self.color_matrix
    }

    pub fn transform_matrix(&self) -> &Box<Matrix4> {
        &self.transform_matrix
    }
}
