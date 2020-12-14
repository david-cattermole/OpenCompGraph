#[allow(unused_imports)]
use petgraph;
use std::hash::{Hash, Hasher};

use crate::cxxbridge::ffi::OperationType;
use crate::cxxbridge::ffi::{AttrState, OperationStatus};
use crate::cxxbridge::Output;
use crate::data::{BoundingBox2D, Matrix4, PixelBlock};
use crate::data::{EdgeIdx, EdgeWeight, GraphIdx, Identifier, NodeIdx, NodeWeight};
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

    pub fn connect(&mut self, src_index: usize, dst_index: usize) {
        println!("Connect {} to {}", src_index, dst_index);
        let src = petgraph::graph::NodeIndex::new(src_index);
        let dst = petgraph::graph::NodeIndex::new(dst_index);
        let index = self.graph.add_edge(src, dst, ()).index();
    }

    // Compute The graph
    pub fn execute(&mut self, start_index: usize) -> ExecuteStatus {
        println!("Execute: {}", start_index);
        let start = petgraph::graph::NodeIndex::new(start_index);
        let mut walker = Bfs::new(&self.graph, start);
        while let Some(nx) = walker.next(&self.graph) {
            // we can access `graph` mutably here still
            println!("walk nx: {}", nx.index());
            self.graph[nx] += 1;
        }
        ExecuteStatus::Success
    }
}

pub fn create_graph() -> GraphImpl {
    let ops = Vec::new();
    let graph = InnerGraph::with_capacity(0, 0);
    GraphImpl { ops, graph }
}
