//#[allow(unused_imports)]
//use petgraph;
use std::string::String;

// use crate::cxxbridge::ffi::Operation;
use crate::cxxbridge::ffi::{AttrState, OperationStatus, OperationType};
use crate::cxxbridge::Output;
use crate::ops::traits::{AttrBlock, Compute};
use crate::ops::OperationImpl;

pub fn new(id: usize) -> OperationImpl {
    // // Only 1 input is allowed, and it's uninitialized at first.
    // let mut inputs = Vec::new();
    // inputs.push(None);
    OperationImpl {
        op_type: OperationType::WriteImage,
        id,
        //        node_id: petgraph::graph::NodeIndex::new(0),
        op_status: OperationStatus::Uninitialized,
        compute: Box::new(WriteImageCompute {}),
        attr_block: Box::new(WriteImageAttrs {
            file_path: "".to_string(),
        }),
        // inputs,
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
    fn hash(
        &mut self,
        id: usize,
        op_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
    ) -> usize {
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
