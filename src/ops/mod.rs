use crate::cxxbridge::ffi::OperationType;
use crate::cxxbridge::ffi::{AttrState, ComputeStatus};
use crate::cxxbridge::Output;
use crate::data::{BoundingBox2D, Matrix4, PixelBlock};

pub mod read_image;
pub mod traits;
pub mod write_image;

pub struct Operation {
    op_type: OperationType,
    id: usize,
    attr_block: Box<dyn traits::AttrBlock>,
    compute: Box<dyn traits::Compute>,
    output: Box<Output>,
}

impl Operation {
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

    pub fn hash(&mut self) -> usize {
        println!("Operation.hash() -> {}", self.id);
        let id = self.get_id();
        let op_type_id = self.get_op_type_id();
        self.compute.hash(id, op_type_id, &self.attr_block)
    }

    pub fn compute(&mut self) -> ComputeStatus {
        self.compute.compute(&self.attr_block, &self.output)
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

pub fn create_operation(id: usize, op_type: OperationType) -> Box<Operation> {
    println!("create_operation(id={:?}, op_type={:?})", id, op_type);
    match op_type {
        OperationType::ReadImage => Box::new(Operation {
            op_type,
            id,
            compute: Box::new(read_image::ReadImageCompute {}),
            attr_block: Box::new(read_image::ReadImageAttrs {
                file_path: "".to_string(),
            }),
            output: Box::new(Output::new()),
        }),
        OperationType::WriteImage => Box::new(Operation {
            op_type,
            id,
            compute: Box::new(write_image::WriteImageCompute {}),
            attr_block: Box::new(write_image::WriteImageAttrs {
                file_path: "".to_string(),
            }),
            output: Box::new(Output::new()),
        }),
        _ => panic!("Invalid OperationType: {:?}", op_type),
    }
}
