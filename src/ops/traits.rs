use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::ComputeStatus;
use crate::cxxbridge::Output;
// use crate::data::{BoundingBox2D, Hash, Matrix4, PixelBlock};
use crate::ops::Operation;

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

// pub trait Output {
//     fn get_hash(&self) -> Hash {}
//     fn get_bounding_box(&self) -> BoundingBox2D {}
//     fn get_pixel_block(&self) -> PixelBlock {}
//     fn get_color_matrix(&self) -> Matrix4 {}
//     fn get_transform_matrix(&self) -> Matrix4 {}
// }

pub trait Compute {
    // fn get_output(&self) -> Box<OperationResult>;
    fn hash(
        &mut self,
        id: usize,
        op_type_id: u8,
        // inputs: &[Input],
        attr_block: &Box<dyn AttrBlock>,
    ) -> usize;

    fn compute(
        &mut self,
        // inputs: &[Input],
        attr_block: &Box<dyn AttrBlock>,
        output: &Box<Output>,
    ) -> ComputeStatus;
}
