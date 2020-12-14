use std::string::String;

use crate::cxxbridge::ffi::{AttrState, OperationStatus, OperationType};
use crate::cxxbridge::Output;
use crate::data::Identifier;
use crate::ops::traits::{AttrBlock, Compute};
use crate::ops::OperationImpl;

pub fn new(id: Identifier) -> OperationImpl {
    OperationImpl {
        op_type: OperationType::WriteImage,
        id,
        op_status: OperationStatus::Uninitialized,
        compute: Box::new(WriteImageCompute {}),
        attr_block: Box::new(WriteImageAttrs {
            file_path: "".to_string(),
        }),
        output: Box::new(Output::new()),
    }
}

#[derive(Debug, Clone, Default)]
pub struct WriteImageCompute {}

#[derive(Debug, Clone, Default)]
pub struct WriteImageAttrs {
    pub file_path: String,
}

impl Compute for WriteImageCompute {
    fn hash(&mut self, id: Identifier, op_type_id: u8, attr_block: &Box<dyn AttrBlock>) -> usize {
        0
    }

    fn compute(
        &mut self,
        attr_block: &Box<dyn AttrBlock>,
        output: &mut Box<Output>,
    ) -> OperationStatus {
        println!("WriteImageCompute.compute()");
        println!("AttrBlock: {:?}", attr_block);
        println!("Output: {:?}", output);
        println!("Output: {:?}", output);
        OperationStatus::Valid
    }
}

impl AttrBlock for WriteImageAttrs {
    fn attr_exists(&self, name: &str) -> AttrState {
        match name {
            "file_path" => AttrState::Exists,
            _ => AttrState::Missing,
        }
    }

    fn get_attr_string(&self, name: &str) -> &str {
        match name {
            "file_path" => &self.file_path,
            _ => "",
        }
    }

    fn set_attr_string(&mut self, name: &str, value: &str) {
        match name {
            "file_path" => self.file_path = value.to_string(),
            _ => (),
        };
    }
}
