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
use crate::data::FrameValue;
use crate::data::HashValue;
use crate::data::NodeComputeMode;
use crate::node::NodeImpl;
use crate::stream::StreamDataImpl;

pub trait Validate: std::fmt::Debug {
    fn validate_inputs(
        &self,
        node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        hash_value: HashValue,
        node_compute_mode: NodeComputeMode,
        input_nodes: &Vec<&Box<NodeImpl>>,
    ) -> Vec<NodeComputeMode>;
}

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
        frame: FrameValue,
        node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        inputs_hash: &Vec<HashValue>,
    ) -> HashValue {
        let mut state = DefaultHasher::new();
        node_type_id.hash(&mut state);
        attr_block.attr_hash(frame, &mut state);
        for input_hash in inputs_hash {
            input_hash.hash(&mut state);
        }
        state.finish()
    }

    // TODO: Operations should have a method to initialise and check
    // the operation has all required data before doing the main
    // "compute" function.

    // TOOD: Create a method that must be overridden by implementations.
    // that are responsible for returning the bounding box.

    fn compute(
        &mut self,
        frame: FrameValue,
        node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        hash_value: HashValue,
        node_compute_mode: NodeComputeMode,
        inputs: &Vec<Rc<StreamDataImpl>>,
        output: &mut Rc<StreamDataImpl>,
        cache: &mut Box<CacheImpl>,
    ) -> NodeStatus;
}
