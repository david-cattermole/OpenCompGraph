use log::{debug, error, info, warn};
use std::collections::hash_map::DefaultHasher;
use std::hash;
use std::hash::Hash;
use std::string::String;

use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::NodeType;
use crate::cxxbridge::ffi::StreamDataImplShared;
use crate::data::HashValue;
use crate::data::Identifier;
use crate::hashutils::HashableF32;
use crate::node::traits::AttrBlock;
use crate::node::traits::Compute;
use crate::node::NodeImpl;

pub fn new(id: Identifier) -> NodeImpl {
    NodeImpl {
        node_type: NodeType::Grade,
        id,
        status: NodeStatus::Uninitialized,
        compute: Box::new(GradeCompute::new()),
        attr_block: Box::new(GradeAttrs::new()),
    }
}

#[derive(Debug, Clone, Default)]
pub struct GradeCompute {}

#[derive(Debug, Clone, Default)]
pub struct GradeAttrs {
    pub enable: i32,
    pub multiply: f32,
}

impl hash::Hash for GradeAttrs {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        HashableF32::new(self.multiply).hash(state);
    }
}

impl GradeCompute {
    pub fn new() -> GradeCompute {
        GradeCompute {}
    }
}

impl GradeAttrs {
    pub fn new() -> GradeAttrs {
        GradeAttrs {
            enable: 1,
            multiply: 1.0,
        }
    }
}

impl Compute for GradeCompute {
    fn compute(
        &mut self,
        node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        inputs: &Vec<StreamDataImplShared>,
        output: &mut StreamDataImplShared,
    ) -> NodeStatus {
        debug!("GradeCompute.compute()");
        // debug!("AttrBlock: {:?}", attr_block);
        // debug!("Inputs: {:?}", inputs);
        // debug!("Output: {:?}", output);
        let enable = attr_block.get_attr_i32("enable");
        if enable != 1 {
            return NodeStatus::Error;
        }

        match inputs.len() {
            0 => NodeStatus::Error,
            _ => {
                let mut copy = inputs[0].inner.clone();

                let multiply = attr_block.get_attr_f32("multiply");
                let pixel_block = copy.pixel_block_as_mut();
                // let mut i = 0;
                for v in &mut pixel_block.pixels {
                    // if i < 5 {
                    //     debug!("a={}", *v);
                    // }
                    *v *= multiply;
                    // if i < 5 {
                    //     debug!("b={}", *v);
                    // }
                    // i += 1;
                }

                let hash_value = self.cache_hash(node_type_id, &attr_block, inputs);
                output.inner = copy;
                output.inner.set_hash(hash_value);
                NodeStatus::Valid
            }
        }
    }
}

impl AttrBlock for GradeAttrs {
    fn attr_hash(&self, state: &mut DefaultHasher) {
        self.hash(state)
    }

    fn attr_exists(&self, name: &str) -> AttrState {
        match name {
            "enable" => AttrState::Exists,
            "multiply" => AttrState::Exists,
            _ => AttrState::Missing,
        }
    }

    fn get_attr_str(&self, name: &str) -> &str {
        ""
    }

    fn set_attr_str(&mut self, name: &str, value: &str) {
        ()
    }

    fn get_attr_i32(&self, name: &str) -> i32 {
        match name {
            "enable" => self.enable,
            _ => 0,
        }
    }

    fn set_attr_i32(&mut self, name: &str, value: i32) {
        match name {
            "enable" => self.enable = value,
            _ => (),
        };
    }

    fn get_attr_f32(&self, name: &str) -> f32 {
        match name {
            "multiply" => self.multiply,
            _ => 0.0,
        }
    }

    fn set_attr_f32(&mut self, name: &str, value: f32) {
        match name {
            "multiply" => self.multiply = value,
            _ => (),
        }
    }
}
