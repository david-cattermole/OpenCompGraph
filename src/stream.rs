use image;
use log::{debug, error, info, warn};
use std::rc::Rc;

use crate::cxxbridge::ffi::BBox2Df;
use crate::cxxbridge::ffi::Matrix4;
use crate::cxxbridge::ffi::StreamDataState;
use crate::data::HashValue;
use crate::pixelblock::PixelBlock;

#[derive(Debug, Clone, Hash)]
pub struct StreamDataImpl {
    state: StreamDataState,
    hash: HashValue,
    display_window: BBox2Df,
    data_window: BBox2Df,
    color_matrix: Matrix4,
    transform_matrix: Matrix4,
    pixel_block: Rc<PixelBlock>,
}

impl StreamDataImpl {
    pub fn new() -> StreamDataImpl {
        let state = StreamDataState::Invalid;
        let hash = 0;

        let pixel_block = Rc::new(PixelBlock::new_color_bars());
        let bbox_max_width = (pixel_block.width - 1) as f32;
        let bbox_max_height = (pixel_block.height - 1) as f32;
        let display_window = BBox2Df::new(0.0, 0.0, bbox_max_width, bbox_max_height);
        let data_window = BBox2Df::new(0.0, 0.0, bbox_max_width, bbox_max_height);
        let color_matrix = Matrix4::identity();
        let transform_matrix = Matrix4::identity();

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

    pub fn display_window(&self) -> BBox2Df {
        self.display_window
    }

    pub fn set_display_window(&mut self, value: BBox2Df) {
        self.display_window = value;
    }

    pub fn data_window(&self) -> BBox2Df {
        self.data_window
    }

    pub fn set_data_window(&mut self, value: BBox2Df) {
        self.data_window = value;
    }

    pub fn color_matrix(&self) -> Matrix4 {
        debug!("StreamDataImpl.color_matrix: {:#?}", self.color_matrix);
        self.color_matrix
    }

    pub fn set_color_matrix(&mut self, value: Matrix4) {
        debug!("StreamDataImpl.set_color_matrix: {:#?}", value);
        self.color_matrix = value;
    }

    pub fn transform_matrix(&self) -> Matrix4 {
        self.transform_matrix
    }

    pub fn set_transform_matrix(&mut self, value: Matrix4) {
        self.transform_matrix = value;
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
}
