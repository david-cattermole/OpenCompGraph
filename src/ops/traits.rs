#[allow(unused_imports)]
use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::OperationStatus;
use crate::cxxbridge::Output;
use crate::data::Identifier;
use crate::ops::OperationImpl;

pub trait AttrBlock: std::fmt::Debug {
    fn attr_exists(&self, name: &str) -> AttrState;
    fn get_attr_string(&self, name: &str) -> &str;
    fn set_attr_string(&mut self, name: &str, value: &str);
}

pub trait Compute: std::fmt::Debug {
    fn hash(
        &mut self,
        id: Identifier,
        op_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        inputs: &Vec<Output>,
    ) -> usize;

    fn compute(
        &mut self,
        attr_block: &Box<dyn AttrBlock>,
        inputs: &Vec<Output>,
        output: &mut Box<Output>,
    ) -> OperationStatus;
}
