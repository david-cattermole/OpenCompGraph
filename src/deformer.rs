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

use std::hash;
use std::hash::Hash;

use crate::attrblock::AttrBlock;
use crate::cxxbridge::ffi::BBox2Df;

pub mod brownian;
pub mod ldpk_utils;
pub mod tde4_classic;
pub mod transform;

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

    /// Default implementation does not change the input bbox.
    fn apply_bounding_box(&self, bbox: BBox2Df, _image_window: BBox2Df) -> BBox2Df {
        bbox
    }

    /// Given a slice of values, every 'stride' number of buffer values,
    /// the X and Y are deformed.
    fn apply_slice_in_place(
        &self,
        buffer: &mut [f32],
        image_window: BBox2Df,
        inverse: bool,
        stride: usize,
    );

    fn data_debug_string(&self) -> String {
        format!("{:?}", self)
    }
}
