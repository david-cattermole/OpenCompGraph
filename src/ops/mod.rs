use crate::cxxbridge::ffi::OperationType;
use crate::cxxbridge::ffi::{AttrState, OperationStatus};
use crate::cxxbridge::Output;
use crate::data::{BoundingBox2D, Matrix4, PixelBlock};

pub mod null;
pub mod read_image;
pub mod traits;
pub mod write_image;

#[derive(Debug)]
pub struct OperationImpl {
    op_type: OperationType,
    id: usize,
    op_status: OperationStatus,
    attr_block: Box<dyn traits::AttrBlock>,
    compute: Box<dyn traits::Compute>,
    output: Box<Output>,
}

impl OperationImpl {
    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_op_type(&self) -> OperationType {
        println!("Operation.get_op_type() -> {:?}", self.op_type);
        self.op_type
    }

    pub fn get_op_type_id(&self) -> u8 {
        println!("Operation.get_op_type_id() -> {}", self.op_type.repr);
        self.op_type.repr
    }

    pub fn get_status(&self) -> OperationStatus {
        println!("Operation.get_status() -> {:?}", self.op_status);
        self.op_status
    }

    pub fn get_status_id(&self) -> u8 {
        println!("Operation.get_status_id() -> {}", self.op_status.repr);
        self.op_status.repr
    }

    // This method is used to determine "has this operation changed?
    // If I re-compute this Operation, do I expect a different value?"
    pub fn hash(&mut self) -> usize {
        println!("Operation.hash() -> {}", self.id);
        let id = self.get_id();
        let op_type_id = self.get_op_type_id();
        self.compute.hash(id, op_type_id, &self.attr_block)
    }


    pub fn compute(&mut self) -> OperationStatus {
        self.compute.compute(&self.attr_block, &mut self.output)
    }

    pub fn attr_exists(&self, name: &str) -> AttrState {
        self.attr_block.attr_exists(name)
    }

    pub fn get_attr_string(&self, name: &str) -> &str {
        self.attr_block.get_attr_string(name)
    }

    pub fn set_attr_string(&mut self, name: &str, value: &str) {
        self.attr_block.set_attr_string(name, value);
    }
}

pub fn create_operation(id: usize, op_type: OperationType) -> OperationImpl {
    println!("create_operation(id={:?}, op_type={:?})", id, op_type);
    match op_type {
        OperationType::ReadImage => read_image::new(id),
        OperationType::WriteImage => write_image::new(id),
        OperationType::Null => null::new(id),
        _ => panic!("Invalid OperationType: {:?}", op_type),
    }
}
