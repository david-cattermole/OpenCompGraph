#[allow(unused_imports)]
use petgraph;
use petgraph::dot::{Config, Dot};
use std::hash::{Hash, Hasher};

use crate::cxxbridge::create_stream_data_shared;
use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::ExecuteStatus;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::StreamDataImplShared;
use crate::data::{BoundingBox2D, Matrix4, PixelBlock};
use crate::data::{EdgeIdx, EdgeWeight, GraphIdx, Identifier, NodeIdx, NodeWeight};
use crate::graphiter::UpstreamEvalSearch;
use crate::node::NodeImpl;

type InnerGraph =
    petgraph::stable_graph::StableGraph<NodeWeight, EdgeWeight, petgraph::Directed, GraphIdx>;

#[derive(Debug)]
pub struct GraphImpl {
    nodes: Vec<Box<NodeImpl>>,
    graph: InnerGraph,
}

impl GraphImpl {
    // Add, Remove and Modify
    pub fn add_node(&mut self, op_box: Box<NodeImpl>) -> usize {
        let id = op_box.get_id();
        let nodes_index = self.nodes.len();
        self.nodes.push(op_box);

        let index = self.graph.add_node(id).index();
        println!("Add Op id={} index={}", id, index);
        assert_eq!(index, nodes_index);
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
        // println!(
        //     "{:?}",
        //     Dot::with_config(&self.graph, &[Config::EdgeNoLabel])
        // );

        let mut sorted_node_indexes = Vec::<usize>::new();

        let start = petgraph::graph::NodeIndex::new(start_index);
        let mut walker = UpstreamEvalSearch::new(&self.graph, start);
        while let Some(nx) = walker.next(&self.graph) {
            let index = nx.index();
            let node_weight = self.graph[nx];
            let op = &self.nodes[index];
            sorted_node_indexes.push(index);
            assert_eq!(node_weight, op.get_id());
            // println!("walk index: {}", index);

            // // We can access `graph` mutably here still
            // self.graph[nx] += 1;  // Modify the node weight.
        }

        let inputs = Vec::<StreamDataImplShared>::new();
        let mut output = create_stream_data_shared();
        for op_index in sorted_node_indexes.iter().rev() {
            println!("Compute Node Index: {:?}", op_index);

            let op = &mut self.nodes[*op_index];
            // println!("op: {:#?}", op);
            op.compute(&inputs, &mut output);
        }

        ExecuteStatus::Success
    }
}

pub fn create_graph() -> GraphImpl {
    let nodes = Vec::new();
    let graph = InnerGraph::with_capacity(0, 0);
    GraphImpl { nodes, graph }
}
