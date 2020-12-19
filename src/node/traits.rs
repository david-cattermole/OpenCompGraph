#[allow(unused_imports)]
use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::StreamDataImplShared;
use crate::data::Identifier;
use crate::node::NodeImpl;

pub trait AttrBlock: std::fmt::Debug {
    fn attr_exists(&self, name: &str) -> AttrState;
    fn get_attr_string(&self, name: &str) -> &str;
    fn set_attr_string(&mut self, name: &str, value: &str);
}

pub trait Compute: std::fmt::Debug {
    fn hash(
        &mut self,
        id: Identifier,
        node_type_id: u8,
        attr_block: &Box<dyn AttrBlock>,
        inputs: &Vec<StreamDataImplShared>,
    ) -> usize;

    fn compute(
        &mut self,
        attr_block: &Box<dyn AttrBlock>,
        inputs: &Vec<StreamDataImplShared>,
        output: &mut StreamDataImplShared,
    ) -> NodeStatus;
}
