use std::string::String;

use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::NodeType;
use crate::cxxbridge::ffi::StreamDataImplShared;
use crate::data::Identifier;
use crate::node::traits::{AttrBlock, Compute};
use crate::node::NodeImpl;

pub fn new(id: Identifier) -> NodeImpl {
    NodeImpl {
        node_type: NodeType::Null,
        id,
        op_status: NodeStatus::Uninitialized,
        compute: Box::new(NullCompute {}),
        attr_block: Box::new(NullAttrs {}),
    }
}

#[derive(Debug, Clone, Default)]
pub struct NullCompute {}

#[derive(Debug, Clone, Default)]
pub struct NullAttrs {}

impl Compute for NullCompute {
    fn hash(
        &mut self,
        id: Identifier,
        node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        inputs: &Vec<StreamDataImplShared>,
    ) -> usize {
        0
    }

    fn compute(
        &mut self,
        attr_block: &Box<dyn AttrBlock>,
        inputs: &Vec<StreamDataImplShared>,
        output: &mut StreamDataImplShared,
    ) -> NodeStatus {
        println!("NullCompute.compute()");
        println!("AttrBlock: {:?}", attr_block);
        println!("Inputs: {:?}", inputs);
        println!("Output: {:?}", output);
        NodeStatus::Valid
    }
}

impl AttrBlock for NullAttrs {
    fn attr_exists(&self, name: &str) -> AttrState {
        AttrState::Missing
    }

    fn get_attr_string(&self, name: &str) -> &str {
        ""
    }

    fn set_attr_string(&mut self, name: &str, value: &str) {
        ()
    }
}
