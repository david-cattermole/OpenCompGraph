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

use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::rc::Rc;

use crate::attrblock::AttrBlock;
use crate::cache::CacheImpl;
use crate::cxxbridge::ffi::NodeStatus;
use crate::data::HashValue;
use crate::stream::StreamDataImpl;


pub trait Operation: std::fmt::Debug {
    // TODO: Add a "OperationCacheType". This will allow us to
    // categorize the Operation in terms of cache.
    //
    // For example we could have a "Trivial" OperationCacheType, which
    // would indicate that the operation is trivial to compute (ie. do
    // not store this in the cache because we can easily re-compute
    // it).
    //
    // Another example type might be "BoundByIO", which would indicate
    // that the operation can be asynconously computed.
    //
    // Another example type might be "BoundByCPU", which would
    // indicate that the operation should be multi-threaded to be
    // computed in the shortest time possible.

    fn cache_hash(
        &self,
        frame: i32,
        node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        inputs: &Vec<Rc<StreamDataImpl>>,
    ) -> HashValue {
        let mut state = DefaultHasher::new();
        node_type_id.hash(&mut state);
        attr_block.attr_hash(frame, &mut state);
        for input in inputs {
            (*input).hash(&mut state);
        }
        state.finish()
    }

    fn compute(
        &mut self,
        frame: i32,
        node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        inputs: &Vec<Rc<StreamDataImpl>>,
        output: &mut Rc<StreamDataImpl>,
        cache: &mut Box<CacheImpl>,
    ) -> NodeStatus;
}
