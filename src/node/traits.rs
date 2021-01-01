use log::{debug, error, info, warn};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::StreamDataImplShared;
use crate::data::HashValue;
use crate::data::Identifier;
use crate::node::NodeImpl;

pub trait AttrBlock: std::fmt::Debug {
    fn attr_hash(&self, state: &mut DefaultHasher);

    fn attr_exists(&self, name: &str) -> AttrState;

    fn get_attr_str(&self, name: &str) -> &str;
    fn set_attr_str(&mut self, name: &str, value: &str);

    fn get_attr_i32(&self, name: &str) -> i32;
    fn set_attr_i32(&mut self, name: &str, value: i32);

    fn get_attr_f32(&self, name: &str) -> f32;
    fn set_attr_f32(&mut self, name: &str, value: f32);
}

pub trait Compute: std::fmt::Debug {
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
