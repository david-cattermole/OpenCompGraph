use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::NodeType;
use crate::cxxbridge::ffi::StreamDataImplShared;
use crate::cxxbridge::ffi::StreamDataState;
use crate::data::BoundingBox2D;
use crate::data::EdgeIdx;
use crate::data::EdgeWeight;
use crate::data::GraphIdx;
use crate::data::HashValue;
use crate::data::Identifier;
use crate::data::NodeIdx;
use crate::data::NodeWeight;
use crate::data::PixelBlock;

pub mod grade;
pub mod null;
pub mod read_image;
pub mod traits;
pub mod write_image;

#[derive(Debug)]
pub struct NodeImpl {
    node_type: NodeType,
    id: Identifier,
    op_status: NodeStatus,
    attr_block: Box<dyn traits::AttrBlock>,
    compute: Box<dyn traits::Compute>,
}

impl NodeImpl {
    pub fn get_id(&self) -> Identifier {
        self.id
    }

    pub fn get_node_type(&self) -> NodeType {
        // println!("Node.get_node_type() -> {:?}", self.node_type);
        self.node_type
    }

    pub fn get_node_type_id(&self) -> u8 {
        // println!("Node.get_node_type_id() -> {}", self.node_type.repr);
        self.node_type.repr
    }

    pub fn get_status(&self) -> NodeStatus {
        // println!("Node.get_status() -> {:?}", self.op_status);
        self.op_status
    }

    pub fn get_status_id(&self) -> u8 {
        // println!("Node.get_status_id() -> {}", self.op_status.repr);
        self.op_status.repr
    }

    // This method is used to determine "has this node changed?
    // If I re-compute this Node, do I expect a different value?"
    pub fn hash(&mut self, inputs: &Vec<StreamDataImplShared>) -> HashValue {
        // println!("Node.hash() -> {}", self.id);
        let id = self.get_id();
        let node_type_id = self.get_node_type_id();
        self.compute
            .hash(id, node_type_id, &self.attr_block, inputs)
    }

    pub fn compute(
        &mut self,
        inputs: &Vec<StreamDataImplShared>,
        output: &mut StreamDataImplShared,
    ) -> NodeStatus {
        self.compute.compute(&self.attr_block, inputs, output)
    }

    pub fn attr_exists(&self, name: &str) -> AttrState {
        self.attr_block.attr_exists(name)
    }

    pub fn get_attr_str(&self, name: &str) -> &str {
        match self.attr_block.attr_exists(name) {
            AttrState::Exists => self.attr_block.get_attr_str(name),
            AttrState::Missing => {
                println!("Missing attribute: {}", name);
                ""
            }
            _ => panic!("Incorrect AttrState"),
        }
    }

    pub fn set_attr_str(&mut self, name: &str, value: &str) {
        match self.attr_block.attr_exists(name) {
            AttrState::Exists => self.attr_block.set_attr_str(name, value),
            AttrState::Missing => println!("Missing attribute: {}", name),
            _ => panic!("Incorrect AttrState"),
        }
    }

    pub fn get_attr_i32(&self, name: &str) -> i32 {
        match self.attr_block.attr_exists(name) {
            AttrState::Exists => self.attr_block.get_attr_i32(name),
            AttrState::Missing => {
                println!("Missing attribute: {}", name);
                0
            }
            _ => panic!("Incorrect AttrState"),
        }
    }

    pub fn set_attr_i32(&mut self, name: &str, value: i32) {
        match self.attr_block.attr_exists(name) {
            AttrState::Exists => self.attr_block.set_attr_i32(name, value),
            AttrState::Missing => println!("Missing attribute: {}", name),
            _ => panic!("Incorrect AttrState"),
        }
    }

    pub fn get_attr_f32(&self, name: &str) -> f32 {
        match self.attr_block.attr_exists(name) {
            AttrState::Exists => self.attr_block.get_attr_f32(name),
            AttrState::Missing => {
                println!("Missing attribute: {}", name);
                0.0
            }
            _ => panic!("Incorrect AttrState"),
        }
    }

    pub fn set_attr_f32(&mut self, name: &str, value: f32) {
        match self.attr_block.attr_exists(name) {
            AttrState::Exists => self.attr_block.set_attr_f32(name, value),
            AttrState::Missing => println!("Missing attribute: {}", name),
            _ => panic!("Incorrect AttrState"),
        }
    }
}

pub fn create_node(node_type: NodeType, id: Identifier) -> NodeImpl {
    // println!("create_node(id={:?}, node_type={:?})", id, node_type);
    match node_type {
        NodeType::ReadImage => read_image::new(id),
        NodeType::WriteImage => write_image::new(id),
        NodeType::Null => null::new(id),
        NodeType::Grade => grade::new(id),
        _ => panic!("Invalid NodeType: {:?}", node_type),
    }
}
