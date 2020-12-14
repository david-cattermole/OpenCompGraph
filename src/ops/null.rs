use std::string::String;

use crate::cxxbridge::ffi::{AttrState, OperationStatus, OperationType};
use crate::cxxbridge::Output;
use crate::data::Identifier;
use crate::ops::traits::{AttrBlock, Compute};
use crate::ops::OperationImpl;

pub fn new(id: Identifier) -> OperationImpl {
    OperationImpl {
        op_type: OperationType::Null,
        id,
        op_status: OperationStatus::Uninitialized,
        compute: Box::new(NullCompute {}),
        attr_block: Box::new(NullAttrs {}),
        output: Box::new(Output::new()),
    }
}

#[derive(Debug, Clone, Default)]
pub struct NullCompute {}

#[derive(Debug, Clone, Default)]
pub struct NullAttrs {}

impl Compute for NullCompute {
    fn hash(&mut self, id: Identifier, op_type_id: u8, attr_block: &Box<dyn AttrBlock>) -> usize {
        0
    }

    fn compute(
        &mut self,
        attr_block: &Box<dyn AttrBlock>,
        output: &mut Box<Output>,
    ) -> OperationStatus {
        println!("NullCompute.compute()");
        println!("AttrBlock: {:?}", attr_block);
        OperationStatus::Valid
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
