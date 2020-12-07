use std::string::String;

use crate::cxxbridge::ffi::{AttrState, ComputeStatus};
use crate::cxxbridge::Output;
use crate::ops::traits::{AttrBlock, Compute};

#[derive(Debug, Clone, Default)]
pub struct ReadImageCompute {}

#[derive(Debug, Clone, Default)]
pub struct ReadImageAttrs {
    pub file_path: String,
}

impl Compute for ReadImageCompute {
    fn hash(
        &mut self,
        id: usize,
        op_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
    ) -> usize {
        0
    }

    fn compute(&mut self, attr_block: &Box<dyn AttrBlock>, output: &Box<Output>) -> ComputeStatus {
        println!("ReadImageCompute.compute()");
        println!("AttrBlock: {:?}", attr_block);
        ComputeStatus::Success
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
