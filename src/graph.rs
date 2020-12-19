#[allow(unused_imports)]
use petgraph;
use petgraph::dot::{Config, Dot};
use std::hash::{Hash, Hasher};

use crate::cxxbridge::ffi::ExecuteStatus;
use crate::cxxbridge::ffi::{AttrState, OperationStatus};
use crate::cxxbridge::Output;
use crate::data::{BoundingBox2D, Matrix4, PixelBlock};
use crate::data::{EdgeIdx, EdgeWeight, GraphIdx, Identifier, NodeIdx, NodeWeight};
use crate::graphiter::UpstreamEvalSearch;
use crate::ops::OperationImpl;

type InnerGraph =
    petgraph::stable_graph::StableGraph<NodeWeight, EdgeWeight, petgraph::Directed, GraphIdx>;

#[derive(Debug)]
pub struct GraphImpl {
    ops: Vec<Box<OperationImpl>>,
    graph: InnerGraph,
}

impl GraphImpl {
    // Add, Remove and Modify
    pub fn add_op(&mut self, op_box: Box<OperationImpl>) -> usize {
        let id = op_box.get_id();
        let ops_index = self.ops.len();
        self.ops.push(op_box);

        let index = self.graph.add_node(id).index();
        println!("Add Op id={} index={}", id, index);
        assert_eq!(index, ops_index);
        index
    }

    pub fn connect(&mut self, src_index: usize, dst_index: usize, input_num: u8) {
        println!("Connect {} to {}:{}", src_index, dst_index, input_num);
        let src = petgraph::graph::NodeIndex::new(src_index);
        let dst = petgraph::graph::NodeIndex::new(dst_index);
        // TODO: Check there is no other edge from src to dst, with
        // the same input_num value.
        let index = self.graph.add_edge(src, dst, input_num).index();
    }

    // Compute The graph
    pub fn execute(&mut self, start_index: usize) -> ExecuteStatus {
        println!("Execute: {}", start_index);
        println!(
            "{:?}",
            Dot::with_config(&self.graph, &[Config::EdgeNoLabel])
        );

        let inputs = Vec::<Output>::new();
        let start = petgraph::graph::NodeIndex::new(start_index);
        let mut walker = UpstreamEvalSearch::new(&self.graph, start);
        while let Some(nx) = walker.next(&self.graph) {
            // we can access `graph` mutably here still
            let index = nx.index();
            let node_weight = self.graph[nx];
            let op = &mut self.ops[index];
            assert_eq!(node_weight, op.get_id());
            println!("walk index: {}", index);
            // println!("op: {:?}", op);
            op.compute(&inputs);
            // self.graph[nx] += 1;  // Modify the node weight.
        }
        ExecuteStatus::Success
    }
}

pub fn create_graph() -> GraphImpl {
    let ops = Vec::new();
    let graph = InnerGraph::with_capacity(0, 0);
    GraphImpl { ops, graph }
}
