use log::{debug, error, info, warn};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use crate::attrblock::AttrBlock;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::StreamDataImplShared;
use crate::data::HashValue;
use crate::data::Identifier;
use crate::node::NodeImpl;

pub trait Operation: std::fmt::Debug {
    fn cache_hash(
        &self,
        node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        inputs: &Vec<StreamDataImplShared>,
    ) -> HashValue {
        let mut state = DefaultHasher::new();
        node_type_id.hash(&mut state);
        attr_block.attr_hash(&mut state);
        for input in inputs {
            input.inner.hash(&mut state);
        }
        state.finish()
    }

    fn compute(
        &mut self,
        node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        inputs: &Vec<StreamDataImplShared>,
        output: &mut StreamDataImplShared,
    ) -> NodeStatus;
}
