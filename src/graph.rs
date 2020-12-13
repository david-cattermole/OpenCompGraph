#[allow(unused_imports)]
use petgraph;
use std::hash::{Hash, Hasher};

use crate::cxxbridge::ffi::OperationType;
use crate::cxxbridge::ffi::{AttrState, OperationStatus};
use crate::cxxbridge::Output;
use crate::data::{
    BoundingBox2D, EdgeIdx, EdgeWeight, GraphIdx, Matrix4, NodeIdx, NodeWeight, PixelBlock,
};
use crate::ops::OperationImpl;

type InnerGraph =
    petgraph::stable_graph::StableGraph<NodeWeight, EdgeWeight, petgraph::Directed, GraphIdx>;

#[derive(Debug)]
pub struct GraphImpl {
    ops: Vec<Box<OperationImpl>>,
    graph: InnerGraph,
}

impl GraphImpl {
    pub fn add_op(&mut self, op_box: Box<OperationImpl>) -> usize {
        let id = op_box.get_id();
        self.ops.push(op_box);
        let index = self.graph.add_node(id).index();
        println!("Add Op id={} index={}", id, index);
        index
    }

    pub fn connect(&mut self, src_op_id: usize, dst_op_id: usize) {
        println!("Connect {} to {}", src_op_id, dst_op_id);
    }
}

pub fn create_graph() -> GraphImpl {
    let ops = Vec::new();
    let graph = InnerGraph::with_capacity(0, 0);
    GraphImpl { ops, graph }
}
