use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::OperationType;
use crate::traits::AttrBlock;
use crate::traits::Compute;

pub mod read_image;
pub mod write_image;

pub struct Operation {
    op_type: OperationType,
    id: usize,
    compute: Box<dyn Compute>,
    attr_block: Box<dyn AttrBlock>,
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

    pub fn compute(&mut self) -> Result<bool, &'static str> {
        self.compute.compute(&self.attr_block)
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
        }),
        OperationType::WriteImage => Box::new(Operation {
            op_type,
            id,
            compute: Box::new(write_image::WriteImageCompute {}),
            attr_block: Box::new(write_image::WriteImageAttrs {
                file_path: "".to_string(),
            }),
        }),
        _ => panic!("Invalid OperationType: {:?}", op_type),
    }
}
