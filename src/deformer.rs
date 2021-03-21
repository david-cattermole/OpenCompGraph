use log::{debug, error, info, warn};
use std::collections::hash_map::DefaultHasher;
use std::hash;
use std::hash::Hash;

use crate::attrblock::AttrBlock;
use crate::cxxbridge::ffi::BBox2Df;
use crate::mathutils;

pub mod ldpk_utils;
pub mod brownian;
pub mod tde4_classic;

/// Helper trait to clone a deformer.
pub trait DeformerClone {
    fn clone_box(&self) -> Box<dyn Deformer>;
}

impl<T> DeformerClone for T
where
    T: 'static + Deformer + Clone,
{
    fn clone_box(&self) -> Box<dyn Deformer> {
        Box::new(self.clone())
    }
}

/// Clone a deformer.
impl Clone for Box<dyn Deformer> {
    fn clone(&self) -> Box<dyn Deformer> {
        let mut obj_box = self.clone_box();
        // HACK: Re-commit the underlying data. This is a HACK to make
        // sure the internal C++ LDPK plugin has it's parameters set.
        obj_box.commit_data().unwrap();
        obj_box
    }
}

/// Hash a deformer.
impl Hash for Box<dyn Deformer> {
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        self.hash_deformer().hash(state)
    }
}

/// Deformer Trait.
pub trait Deformer: AttrBlock + DeformerClone {
    fn hash_deformer(&self) -> u64;

    fn commit_data(&mut self) -> Result<(), String>;

    fn apply_forward(&self, x: f32, y: f32) -> (f32, f32);

    fn apply_backward(&self, x: f32, y: f32) -> (f32, f32);

    fn apply_forward_bounding_box(&self, bbox: BBox2Df, image_window: BBox2Df) -> BBox2Df {
        bbox
    }

    fn apply_backward_bounding_box(&self, bbox: BBox2Df, image_window: BBox2Df) -> BBox2Df {
        bbox
    }

    /// Given a slice of values, every 'stride' number of buffer values,
    /// the X and Y are deformed.
    fn apply_forward_slice_in_place(
        &self,
        buffer: &mut [f32],
        image_window: BBox2Df,
        stride: usize,
    ) {

        let min_x = image_window.min_x;
        let min_y = image_window.min_y;
        let max_x = image_window.max_x;
        let max_y = image_window.max_y;

        let count = buffer.len() / stride;
        for i in 0..count {
            let index = i * stride;

            let x = buffer[index + 0];
            let y = buffer[index + 1];
            let x_remap = mathutils::remap(min_x, max_x, 0.0, 1.0, x);
            let y_remap = mathutils::remap(min_y, max_y, 0.0, 1.0, y);

            let (xu, yu) = self.apply_forward(x_remap, y_remap);

            let xu_remap = mathutils::remap(0.0, 1.0, min_x, max_x, xu);
            let yu_remap = mathutils::remap(0.0, 1.0, min_y, max_y, yu);
            buffer[index + 0] = xu_remap;
            buffer[index + 1] = yu_remap;
        }
    }

    /// Given a slice of values, every 'stride' number of buffer values,
    /// the X and Y are deformed.
    fn apply_backward_slice_in_place(
        &self,
        buffer: &mut [f32],
        image_window: BBox2Df,
        stride: usize,
    ) {
        warn!(
            "deform_backward_slice_in_place: len={} stride={} deformer={:#?}",
            buffer.len(),
            stride,
            self
        );

        let min_x = image_window.min_x;
        let min_y = image_window.min_y;
        let max_x = image_window.max_x;
        let max_y = image_window.max_y;

        let count = buffer.len() / stride;
        for i in 0..count {
            let index = i * stride;

            let x = buffer[index + 0];
            let y = buffer[index + 1];
            let x_remap = mathutils::remap(min_x, max_x, 0.0, 1.0, x);
            let y_remap = mathutils::remap(min_y, max_y, 0.0, 1.0, y);

            let (xu, yu) = self.apply_backward(x_remap, y_remap);

            let xu_remap = mathutils::remap(0.0, 1.0, min_x, max_x, xu);
            let yu_remap = mathutils::remap(0.0, 1.0, min_y, max_y, yu);
            buffer[index + 0] = xu_remap;
            buffer[index + 1] = yu_remap;
        }
    }

    fn data_debug_string(&self) -> String {
        format!("{:?}", self)
    }
}
