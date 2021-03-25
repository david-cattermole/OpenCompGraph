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
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::rc::Rc;

use crate::attrblock::AttrBlock;
use crate::cache::CacheImpl;
use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::NodeType;
use crate::data::Identifier;
use crate::node::traits::Operation;
use crate::node::NodeImpl;
use crate::stream::StreamDataImpl;

pub fn new(id: Identifier) -> NodeImpl {
    NodeImpl {
        node_type: NodeType::Null,
        id,
        status: NodeStatus::Uninitialized,
        compute: Box::new(NullOperation::new()),
        attr_block: Box::new(NullAttrs::new()),
    }
}

#[derive(Debug, Clone, Default)]
pub struct NullOperation {}

#[derive(Debug, Clone, Default, Hash)]
pub struct NullAttrs {}

impl NullOperation {
    pub fn new() -> NullOperation {
        NullOperation {}
    }
}

impl NullAttrs {
    pub fn new() -> NullAttrs {
        NullAttrs {}
    }
}

impl Operation for NullOperation {
    fn compute(
        &mut self,
        _frame: i32,
        _node_type_id: u8,
        _attr_block: &Box<dyn AttrBlock>,
        inputs: &Vec<Rc<StreamDataImpl>>,
        output: &mut Rc<StreamDataImpl>,
        _cache: &mut Box<CacheImpl>,
    ) -> NodeStatus {
        debug!("NullOperation.compute()");
        // debug!("AttrBlock: {:?}", _attr_block);
        // debug!("Inputs: {:?}", inputs);
        // debug!("Output: {:?}", output);
        match inputs.len() {
            0 => NodeStatus::Error,
            _ => {
                *output = inputs[0].clone();
                NodeStatus::Valid
            }
        }
    }
}

impl AttrBlock for NullAttrs {
    fn attr_hash(&self, frame: i32, state: &mut DefaultHasher) {
        // TODO: Should we use "frame" to hash the value?
        self.hash(state)
    }

    fn attr_exists(&self, _name: &str) -> AttrState {
        AttrState::Missing
    }

    fn get_attr_str(&self, _name: &str) -> &str {
        ""
    }

    fn set_attr_str(&mut self, _name: &str, _value: &str) {
        ()
    }

    fn get_attr_i32(&self, _name: &str) -> i32 {
        0
    }

    fn set_attr_i32(&mut self, _name: &str, _value: i32) {
        ()
    }

    fn get_attr_f32(&self, _name: &str) -> f32 {
        0.0
    }

    fn set_attr_f32(&mut self, _name: &str, _value: f32) {
        ()
    }
}
