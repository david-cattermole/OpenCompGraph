use log::{debug, error, info, warn};
use std::collections::hash_map::DefaultHasher;
use std::hash;
use std::hash::Hash;
use std::string::String;

use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::NodeType;
use crate::cxxbridge::ffi::StreamDataImplShared;
use crate::data::HashValue;
use crate::data::Identifier;
use crate::node::traits::AttrBlock;
use crate::node::traits::Compute;
use crate::node::NodeImpl;

pub fn new(id: Identifier) -> NodeImpl {
    NodeImpl {
        node_type: NodeType::Null,
        id,
        status: NodeStatus::Uninitialized,
        compute: Box::new(NullCompute::new()),
        attr_block: Box::new(NullAttrs::new()),
    }
}

#[derive(Debug, Clone, Default)]
pub struct NullCompute {}

#[derive(Debug, Clone, Default, Hash)]
pub struct NullAttrs {}

impl NullCompute {
    pub fn new() -> NullCompute {
        NullCompute {}
    }
}

impl NullAttrs {
    pub fn new() -> NullAttrs {
        NullAttrs {}
    }
}

impl Compute for NullCompute {
    fn compute(
        &mut self,
        node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        inputs: &Vec<StreamDataImplShared>,
        output: &mut StreamDataImplShared,
    ) -> NodeStatus {
        debug!("NullCompute.compute()");
        // debug!("AttrBlock: {:?}", attr_block);
        // debug!("Inputs: {:?}", inputs);
        // debug!("Output: {:?}", output);
        match inputs.len() {
            0 => NodeStatus::Error,
            _ => {
                let hash_value = self.cache_hash(node_type_id, &attr_block, inputs);
                let mut copy = inputs[0].inner.clone();
                copy.set_hash(hash_value);
                output.inner = copy;
                NodeStatus::Valid
            }
        }
    }
}

impl AttrBlock for NullAttrs {
    fn attr_hash(&self, state: &mut DefaultHasher) {
        self.hash(state)
    }

    fn attr_exists(&self, name: &str) -> AttrState {
        AttrState::Missing
    }

    fn get_attr_str(&self, name: &str) -> &str {
        ""
    }

    fn set_attr_str(&mut self, name: &str, value: &str) {
        ()
    }

    fn get_attr_i32(&self, name: &str) -> i32 {
        0
    }

    fn set_attr_i32(&mut self, name: &str, value: i32) {
        ()
    }

    fn get_attr_f32(&self, name: &str) -> f32 {
        0.0
    }

    fn set_attr_f32(&mut self, name: &str, value: f32) {
        ()
    }
}
