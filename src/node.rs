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

use log::{debug, error, info, warn};
use std::rc::Rc;

use crate::attrblock::AttrBlock;
use crate::cache::CacheImpl;
use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::NodeType;
use crate::cxxbridge::ffi::StreamDataState;
use crate::data::EdgeIdx;
use crate::data::EdgeWeight;
use crate::data::GraphIdx;
use crate::data::HashValue;
use crate::data::Identifier;
use crate::data::NodeIdx;
use crate::data::NodeWeight;
use crate::pixelblock::PixelBlock;
use crate::stream::StreamDataImpl;

pub mod grade;
pub mod lens_distort;
pub mod merge_image;
pub mod null;
pub mod read_image;
pub mod traits;
pub mod transform;
pub mod write_image;

#[derive(Debug)]
pub struct NodeImpl {
    node_type: NodeType,
    id: Identifier,
    status: NodeStatus,
    attr_block: Box<dyn AttrBlock>,
    compute: Box<dyn traits::Operation>,
}

impl NodeImpl {
    pub fn get_id(&self) -> Identifier {
        self.id
    }

    pub fn get_node_type(&self) -> NodeType {
        debug!("Node.get_node_type() -> {:?}", self.node_type);
        self.node_type
    }

    pub fn get_node_type_id(&self) -> u8 {
        debug!("Node.get_node_type_id() -> {}", self.node_type.repr);
        self.node_type.repr
    }

    pub fn get_status(&self) -> NodeStatus {
        debug!("Node.get_status() -> {:?}", self.status);
        self.status
    }

    pub fn get_status_id(&self) -> u8 {
        debug!("Node.get_status_id() -> {}", self.status.repr);
        self.status.repr
    }

    // This method is used to determine "has this node changed?
    // If I re-compute this Node, do I expect a different value?"
    pub fn hash(&self, frame: i32, inputs: &Vec<Rc<StreamDataImpl>>) -> HashValue {
        let node_type_id = self.get_node_type_id();
        let value = self
            .compute
            .cache_hash(frame, node_type_id, &self.attr_block, inputs);
        debug!("Node.hash(): id={} hash={}", self.id, value);
        value
    }

    pub fn compute(
        &mut self,
        frame: i32,
        inputs: &Vec<Rc<StreamDataImpl>>,
        output: &mut Rc<StreamDataImpl>,
        cache: &mut Box<CacheImpl>,
    ) -> NodeStatus {
        let node_type_id = self.get_node_type_id();
        let status =
            self.compute
                .compute(frame, node_type_id, &self.attr_block, inputs, output, cache);
        self.status = status;
        status
    }

    pub fn attr_exists(&self, name: &str) -> AttrState {
        self.attr_block.attr_exists(name)
    }

    pub fn data_debug_string(&self) -> String {
        format!("{:?}", self.attr_block)
    }

    pub fn get_attr_str(&self, name: &str) -> &str {
        match self.attr_block.attr_exists(name) {
            AttrState::Exists => self.attr_block.get_attr_str(name),
            AttrState::Missing => {
                warn!("Missing attribute: {}", name);
                ""
            }
            _ => panic!("Incorrect AttrState"),
        }
    }

    pub fn set_attr_str(&mut self, name: &str, value: &str) {
        match self.attr_block.attr_exists(name) {
            AttrState::Exists => self.attr_block.set_attr_str(name, value),
            AttrState::Missing => warn!("Missing attribute: {}", name),
            _ => panic!("Incorrect AttrState"),
        }
    }

    pub fn get_attr_i32(&self, name: &str) -> i32 {
        match self.attr_block.attr_exists(name) {
            AttrState::Exists => self.attr_block.get_attr_i32(name),
            AttrState::Missing => {
                warn!("Missing attribute: {}", name);
                0
            }
            _ => panic!("Incorrect AttrState"),
        }
    }

    pub fn set_attr_i32(&mut self, name: &str, value: i32) {
        match self.attr_block.attr_exists(name) {
            AttrState::Exists => self.attr_block.set_attr_i32(name, value),
            AttrState::Missing => warn!("Missing attribute: {}", name),
            _ => panic!("Incorrect AttrState"),
        }
    }

    pub fn get_attr_f32(&self, name: &str) -> f32 {
        match self.attr_block.attr_exists(name) {
            AttrState::Exists => self.attr_block.get_attr_f32(name),
            AttrState::Missing => {
                warn!("Missing attribute: {}", name);
                0.0
            }
            _ => panic!("Incorrect AttrState"),
        }
    }

    pub fn set_attr_f32(&mut self, name: &str, value: f32) {
        match self.attr_block.attr_exists(name) {
            AttrState::Exists => self.attr_block.set_attr_f32(name, value),
            AttrState::Missing => warn!("Missing attribute: {}", name),
            _ => panic!("Incorrect AttrState"),
        }
    }
}

pub fn create_node(node_type: NodeType, id: Identifier) -> NodeImpl {
    debug!("create_node(id={:?}, node_type={:?})", id, node_type);
    match node_type {
        NodeType::ReadImage => read_image::new(id),
        NodeType::WriteImage => write_image::new(id),
        NodeType::LensDistort => lens_distort::new(id),
        NodeType::Null => null::new(id),
        NodeType::Grade => grade::new(id),
        NodeType::MergeImage => merge_image::new(id),
        NodeType::Transform => transform::new(id),
        _ => panic!("Invalid NodeType: {:?}", node_type),
    }
}

pub fn create_node_box_with_id(node_type: NodeType, id: Identifier) -> Box<NodeImpl> {
    debug!("create_node_box(node_type={:?}, id={:?})", node_type, id);
    Box::new(create_node(node_type, id))
}
