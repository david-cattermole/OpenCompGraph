use petgraph;
use petgraph::dot::{Config, Dot};
use petgraph::Direction;
use std::hash::{Hash, Hasher};

use crate::cache::CacheImpl;
use crate::cxxbridge::create_stream_data_shared;
use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::CacheImplShared;
use crate::cxxbridge::ffi::ExecuteStatus;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::StreamDataImplShared;
use crate::data::BoundingBox2D;
use crate::data::EdgeIdx;
use crate::data::EdgeWeight;
use crate::data::GraphIdx;
use crate::data::Identifier;
use crate::data::NodeIdx;
use crate::data::NodeWeight;
use crate::graphiter::UpstreamEvalSearch;
use crate::node::NodeImpl;
use crate::stream::StreamDataImpl;

type InnerGraph =
    petgraph::stable_graph::StableGraph<NodeWeight, EdgeWeight, petgraph::Directed, GraphIdx>;

#[derive(Debug)]
pub struct GraphImpl {
    nodes: Vec<Box<NodeImpl>>,
    graph: InnerGraph,
    output: StreamDataImplShared,
    status: ExecuteStatus,
}

impl GraphImpl {
    pub fn new() -> GraphImpl {
        let nodes = Vec::new();
        let graph = InnerGraph::with_capacity(0, 0);
        let output = create_stream_data_shared();
        let status = ExecuteStatus::Error;
        GraphImpl {
            nodes,
            graph,
            output,
            status,
        }
    }

    // Add, Remove and Modify
    pub fn add_node(&mut self, node_box: Box<NodeImpl>) -> usize {
        let id = node_box.get_id();
        let nodes_index = self.nodes.len();
        self.nodes.push(node_box);

        let index = self.graph.add_node(id).index();
        println!("Add Node id={} index={}", id, index);
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

    /// Compute the graph!
    // TODO: Add an "executor" variable to this method to explain how
    // we wish to execute the graph.
    pub fn execute(
        &mut self,
        start_node_index: usize,
        cache: &mut Box<CacheImpl>,
    ) -> ExecuteStatus {
        println!("Execute: {}", start_node_index);
        // println!(
        //     "{:?}",
        //     Dot::with_config(&self.graph, &[Config::EdgeNoLabel])
        // );

        let mut sorted_node_indexes = Vec::<NodeIdx>::new();

        // Get the stack of indices to be computed, going upstream
        // from the starting index.
        let start = NodeIdx::new(start_node_index);
        let mut walker = UpstreamEvalSearch::new(&self.graph, start);
        while let Some(nx) = walker.next(&self.graph) {
            let index = nx.index();
            let node_weight = self.graph[nx];
            let node = &self.nodes[index];
            sorted_node_indexes.push(nx);
            assert_eq!(node_weight, node.get_id());
            // println!("walk index: {}", index);

            // // We can access `graph` mutably here still
            // self.graph[nx] += 1;  // Modify the node weight.
        }

        let mut parent_inputs = Vec::<StreamDataImplShared>::new();
        for nx in sorted_node_indexes.iter().rev() {
            // println!("Compute Node Index: {:?}", nx);

            let mut inputs = Vec::<StreamDataImplShared>::new();
            let parents = self.graph.neighbors_directed(*nx, Direction::Incoming);
            for parent_node_index in parents {
                let parent_index = parent_node_index.index();
                // println!("parent index: {}", parent_index);
                let parent_node = &self.nodes[parent_index];
                let parent_hash = parent_node.hash(&parent_inputs);

                match cache.get(&parent_hash) {
                    Some(value) => {
                        // println!("Got hash: {}", parent_hash);
                        let mut stream_data = create_stream_data_shared();
                        stream_data.inner = value.inner.clone();
                        inputs.push(stream_data);
                    }
                    _ => println!("Missing from cache: {}", parent_hash),
                }
            }
            parent_inputs = inputs.to_vec();

            let node_index = nx.index();
            let node = &mut self.nodes[node_index];
            // println!("node: {:#?}", node);
            let node_hash = node.hash(&inputs);

            // Compute the node
            let mut cached_output = cache.get(&node_hash);
            match cached_output {
                Some(value) => {
                    // println!("Reuse Hash: {}", node_hash);
                }
                None => {
                    let mut output = create_stream_data_shared();
                    match node.compute(&inputs, &mut output) {
                        NodeStatus::Valid => cache.insert(node_hash, output.clone()),
                        NodeStatus::Uninitialized => {
                            println!("Node is uninitialized: node_index={}", node_index);
                            break;
                        }
                        NodeStatus::Error => {
                            println!("Failed to compute node: node_index={}", node_index);
                            break;
                        }
                        _ => {
                            println!("Unknown error: node_index={}", node_index);
                            break;
                        }
                    }
                    self.output = output;
                }
            }
        }

        self.status = ExecuteStatus::Success;
        self.status
    }

    pub fn output_stream(&self) -> StreamDataImplShared {
        println!("Query Stream Output...");
        assert_eq!(self.status, ExecuteStatus::Success);
        self.output.clone()
    }
}
