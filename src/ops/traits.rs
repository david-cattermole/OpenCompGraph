use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::OperationStatus;
use crate::cxxbridge::Output;
// use crate::data::{BoundingBox2D, Hash, Matrix4, PixelBlock};
use crate::ops::OperationImpl;

// pub trait Input {
//     fn get_input(&self, num: usize) -> Operation;
//     fn set_input(&self, num: usize, op: Operation);
//     fn get_num_input(&self) -> usize;
// }

pub trait AttrBlock: std::fmt::Debug {
    fn attr_exists(&self, name: &str) -> AttrState;
    fn get_attr_string(&self, name: &str) -> &str;
    fn set_attr_string(&mut self, name: &str, value: &str);
}

pub trait Compute: std::fmt::Debug {
    fn hash(
        &mut self,
        id: usize,
        op_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        // inputs: &Vec<Option<&Operation>>,
    ) -> usize;

    fn compute(
        &mut self,
        // inputs: &[Input],
        attr_block: &Box<dyn AttrBlock>,
        output: &mut Box<Output>,
    ) -> OperationStatus;
}
