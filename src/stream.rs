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

use log::debug;
use std::rc::Rc;

use crate::cxxbridge::ffi::BBox2Df;
use crate::cxxbridge::ffi::BBox2Di;
use crate::cxxbridge::ffi::DeformerDirection;
use crate::cxxbridge::ffi::Matrix4;
use crate::cxxbridge::ffi::PixelDataType;
use crate::cxxbridge::ffi::StreamDataState;
use crate::data::HashValue;
use crate::deformer::Deformer;
use crate::deformutils;
use crate::pixelblock::PixelBlock;

#[derive(Debug, Clone, Hash)]
pub struct StreamDataImpl {
    state: StreamDataState,
    hash: HashValue,
    display_window: BBox2Di,
    data_window: BBox2Di,
    color_matrix: Matrix4,
    transform_matrix: Matrix4,
    pixel_block: Rc<PixelBlock>,
    deformers: Vec<Box<dyn Deformer>>,
}

impl StreamDataImpl {
    pub fn new() -> StreamDataImpl {
        let state = StreamDataState::Invalid;
        let hash = 0;

        let pixel_block = Rc::new(PixelBlock::new_color_bars());
        let bbox_max_width = pixel_block.width();
        let bbox_max_height = pixel_block.height();
        let display_window = BBox2Di::new(0, 0, bbox_max_width, bbox_max_height);
        let data_window = BBox2Di::new(0, 0, bbox_max_width, bbox_max_height);
        let color_matrix = Matrix4::identity();
        let transform_matrix = Matrix4::identity();
        let deformers = Vec::new();

        StreamDataImpl {
            state,
            hash,
            display_window,
            data_window,
            color_matrix,
            transform_matrix,
            pixel_block,
            deformers,
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

    pub fn size_bytes(&self) -> usize {
        self.pixel_block.size_bytes()
    }

    pub fn hash(&self) -> HashValue {
        self.hash
    }

    pub fn set_hash(&mut self, value: HashValue) {
        self.hash = value;
    }

    pub fn display_window(&self) -> BBox2Di {
        self.display_window
    }

    pub fn set_display_window(&mut self, value: BBox2Di) {
        self.display_window = value;
    }

    pub fn data_window(&self) -> BBox2Di {
        self.data_window
    }

    pub fn set_data_window(&mut self, value: BBox2Di) {
        self.data_window = value;
    }

    pub fn color_matrix(&self) -> Matrix4 {
        self.color_matrix
    }

    pub fn set_color_matrix(&mut self, value: Matrix4) {
        self.color_matrix = value;
    }

    pub fn transform_matrix(&self) -> Matrix4 {
        self.transform_matrix
    }

    pub fn set_transform_matrix(&mut self, value: Matrix4) {
        self.transform_matrix = value;
    }

    pub fn apply_deformers(
        &self,
        buffer: &mut [f32],
        image_window: BBox2Df,
        direction: DeformerDirection,
    ) {
        debug!("StreamData.apply_deformers...");
        deformutils::apply_deformers_to_positions(&self.deformers, direction, image_window, buffer);
    }

    pub fn deformers(&self) -> &Vec<Box<dyn Deformer>> {
        &self.deformers
    }

    pub fn deformers_len(&self) -> usize {
        self.deformers.len()
    }

    pub fn push_deformer(&mut self, value: Box<dyn Deformer>) {
        self.deformers.push(value);
    }

    pub fn clone_pixel_block(&self) -> PixelBlock {
        (*self.pixel_block).clone()
    }

    pub fn pixel_block(&self) -> Rc<PixelBlock> {
        self.pixel_block.clone()
    }

    pub fn pixel_block_as_mut(&mut self) -> &mut PixelBlock {
        Rc::make_mut(&mut self.pixel_block)
    }

    pub fn set_pixel_block(&mut self, pixel_block: Rc<PixelBlock>) {
        // when the "old" Rc goes out of scope it will be cleaned up
        // if there are no more references to the underlying
        // allocation.
        let old_data = Rc::clone(&self.pixel_block);
        self.pixel_block = pixel_block.clone();
    }

    pub fn pixel_buffer(&self) -> &[f32] {
        &self.pixel_block.as_slice()
    }

    pub fn pixel_width(&self) -> i32 {
        self.pixel_block.width()
    }

    pub fn pixel_height(&self) -> i32 {
        self.pixel_block.height()
    }

    pub fn pixel_num_channels(&self) -> i32 {
        self.pixel_block.num_channels()
    }

    pub fn pixel_data_type(&self) -> PixelDataType {
        self.pixel_block.pixel_data_type()
    }
}

pub fn create_stream_data_box() -> Box<StreamDataImpl> {
    debug!("create_stream_data_box()");
    Box::new(StreamDataImpl::new())
}

#[derive(Debug, Hash, Clone)]
pub struct StreamDataImplRc {
    inner: Rc<StreamDataImpl>,
}

impl StreamDataImplRc {
    pub fn new() -> StreamDataImplRc {
        let inner = Rc::new(StreamDataImpl::new());
        StreamDataImplRc { inner }
    }

    pub fn from_data(stream_data: StreamDataImpl) -> StreamDataImplRc {
        let inner = Rc::new(stream_data);
        StreamDataImplRc { inner }
    }

    pub fn from_rc_data(rc_stream_data: Rc<StreamDataImpl>) -> StreamDataImplRc {
        StreamDataImplRc {
            inner: rc_stream_data,
        }
    }

    pub fn inner(&self) -> Rc<StreamDataImpl> {
        self.inner.clone()
    }

    pub fn set_inner(&mut self, inner: Rc<StreamDataImpl>) {
        self.inner = inner;
    }

    pub fn state(&self) -> StreamDataState {
        self.inner.state()
    }

    pub fn state_id(&self) -> u8 {
        self.inner.state_id()
    }

    pub fn size_bytes(&self) -> usize {
        self.inner.size_bytes()
    }

    pub fn hash(&self) -> HashValue {
        self.inner.hash()
    }

    pub fn display_window(&self) -> BBox2Di {
        self.inner.display_window()
    }

    pub fn data_window(&self) -> BBox2Di {
        self.inner.data_window()
    }

    pub fn color_matrix(&self) -> Matrix4 {
        self.inner.color_matrix()
    }

    pub fn transform_matrix(&self) -> Matrix4 {
        self.inner.transform_matrix()
    }

    pub fn apply_deformers(
        &self,
        buffer: &mut [f32],
        image_window: BBox2Df,
        direction: DeformerDirection,
    ) {
        self.inner.apply_deformers(buffer, image_window, direction);
    }

    pub fn deformers(&self) -> &Vec<Box<dyn Deformer>> {
        self.inner.deformers()
    }

    pub fn deformers_len(&self) -> usize {
        self.inner.deformers_len()
    }

    pub fn pixel_block(&self) -> Rc<PixelBlock> {
        self.inner.pixel_block()
    }

    pub fn pixel_buffer(&self) -> &[f32] {
        self.inner.pixel_buffer()
    }

    pub fn pixel_width(&self) -> i32 {
        self.inner.pixel_width()
    }

    pub fn pixel_height(&self) -> i32 {
        self.inner.pixel_height()
    }

    pub fn pixel_num_channels(&self) -> i32 {
        self.inner.pixel_num_channels()
    }

    pub fn pixel_data_type(&self) -> PixelDataType {
        self.inner.pixel_data_type()
    }
}

pub fn create_stream_data_box_rc() -> Box<StreamDataImplRc> {
    debug!("create_stream_data_rc()");
    Box::new(StreamDataImplRc::new())
}
