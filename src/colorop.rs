/*
 * Copyright (C) 2021 David Cattermole.
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

use std::collections::hash_map::DefaultHasher;
use std::hash;
use std::hash::Hash;
use std::hash::Hasher;

use crate::attrblock::AttrBlock;
use crate::data::HashValue;

pub mod colorgrade;

/// Helper trait to clone a ColorOp.
pub trait ColorOpClone {
    fn clone_box(&self) -> Box<dyn ColorOp>;
}

impl<T> ColorOpClone for T
where
    T: 'static + ColorOp + Clone,
{
    fn clone_box(&self) -> Box<dyn ColorOp> {
        Box::new(self.clone())
    }
}

/// Clone a ColorOp.
impl Clone for Box<dyn ColorOp> {
    fn clone(&self) -> Box<dyn ColorOp> {
        let obj_box = self.clone_box();
        obj_box
    }
}

/// Hash a ColorOp.
impl Hash for Box<dyn ColorOp> {
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        self.hash_color_op().hash(state)
    }
}

/// ColorOp Trait.
pub trait ColorOp: AttrBlock + ColorOpClone {
    fn hash_color_op(&self) -> HashValue;

    /// Given a slice of pixel values (buffer) and the number of
    /// channels in each pixel, the RGBA values will be adjusted by
    /// the ColorOp.
    fn apply_slice_in_place(&self, buffer: &mut [f32], num_channels: i32);
}

/// Calcluate hash for a Vec of ColorOp instances.
pub fn color_ops_hash(color_ops: &Vec<Box<dyn ColorOp>>) -> HashValue {
    let mut state = DefaultHasher::default();
    for color_op in color_ops {
        color_op.hash(&mut state);
    }
    state.finish()
}
