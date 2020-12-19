use std::string::String;

use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::OperationStatus;
use crate::cxxbridge::ffi::OperationType;
use crate::cxxbridge::ffi::StreamDataImplShared;
use crate::data::Identifier;
use crate::ops::traits::{AttrBlock, Compute};
use crate::ops::OperationImpl;

pub fn new(id: Identifier) -> OperationImpl {
    OperationImpl {
        op_type: OperationType::ReadImage,
        id,
        op_status: OperationStatus::Uninitialized,
        compute: Box::new(ReadImageCompute {}),
        attr_block: Box::new(ReadImageAttrs {
            file_path: "".to_string(),
        }),
    }
}

#[derive(Debug, Clone, Default)]
pub struct ReadImageCompute {}

#[derive(Debug, Clone, Default)]
pub struct ReadImageAttrs {
    pub file_path: String,
}

impl Compute for ReadImageCompute {
    fn hash(
        &mut self,
        id: Identifier,
        op_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        inputs: &Vec<StreamDataImplShared>,
    ) -> usize {
        // virtual const Hash hash() {
        //     return this->id * this->type * 123456789;
        // };

        0
    }

    fn compute(
        &mut self,
        attr_block: &Box<dyn AttrBlock>,
        inputs: &Vec<StreamDataImplShared>,
        output: &mut StreamDataImplShared,
    ) -> OperationStatus {
        println!("ReadImageCompute.compute()");
        println!("AttrBlock: {:?}", attr_block);
        println!("Inputs: {:?}", inputs);
        println!("Output: {:?}", output);
        OperationStatus::Valid
    }
}

impl AttrBlock for ReadImageAttrs {
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
