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

type NodeGraph =
    petgraph::stable_graph::StableGraph<NodeWeight, EdgeWeight, petgraph::Directed, GraphIdx>;

#[derive(Debug)]
pub struct GraphImpl {
    nodes: Vec<Box<NodeImpl>>,
    ids: Vec<Identifier>,
    graph: NodeGraph,
    output: StreamDataImplShared,
    status: ExecuteStatus,
}

impl GraphImpl {
    /// Create a brand new empty graph.
    pub fn new() -> GraphImpl {
        let nodes = Vec::new();
        let ids = Vec::new();
        let graph = NodeGraph::with_capacity(0, 0);
        let output = create_stream_data_shared();
        let status = ExecuteStatus::Error;
        GraphImpl {
            nodes,
            ids,
            graph,
            output,
            status,
        }
    }

    /// Add a new node to the graph.
    pub fn add_node(&mut self, node_box: Box<NodeImpl>) -> usize {
        let id = node_box.get_id();
        let nodes_index = self.nodes.len();
        self.nodes.push(node_box);
        self.ids.push(id);

        // TODO: Remove the "id" from the graph (node weight).
        let index = self.graph.add_node(id).index();
        println!("Add Node index={} id={}", index, id);
        assert_eq!(index, nodes_index);
        index
    }

    /// Remove node from the graph.
    pub fn remove_node(&mut self, node_id: u64) -> bool {
        println!("Remove Node id={}", node_id);
        let node_idx = match self.find_node_index_from_id(node_id) {
            Some(value) => value,
            None => {
                println!("Node id not found: id={}", node_id);
                return false; // TODO: Return an error code?
            }
        };
        // TODO: Zero out the now empty fields.
        let node_index = petgraph::graph::NodeIndex::new(node_idx);
        match self.graph.remove_node(node_index) {
            Some(value) => true,
            None => false,
        }
    }

    pub fn node_attr_exists(&self, node_id: Identifier, name: &str) -> AttrState {
        let node_box = match self.find_node_index_from_id(node_id) {
            Some(value) => &self.nodes[value],
            None => {
                println!("Node id not found: id={}", node_id);
                return AttrState::Missing;
            }
        };
        node_box.attr_exists(name)
    }

    pub fn get_node_attr_str(&self, node_id: Identifier, name: &str) -> &str {
        let node_box = match self.find_node_index_from_id(node_id) {
            Some(value) => &self.nodes[value],
            None => {
                println!("Node id not found: id={}", node_id);
                return "";
            }
        };
        node_box.get_attr_str(name)
    }

    pub fn set_node_attr_str(&mut self, node_id: Identifier, name: &str, value: &str) {
        let mut node_box = match self.find_node_index_from_id(node_id) {
            Some(value) => &mut self.nodes[value],
            None => {
                println!("Node id not found: id={}", node_id);
                return; // Add error code?
            }
        };
        node_box.set_attr_str(name, value);
    }

    pub fn get_node_attr_i32(&self, node_id: Identifier, name: &str) -> i32 {
        let node_box = match self.find_node_index_from_id(node_id) {
            Some(value) => &self.nodes[value],
            None => {
                println!("Node id not found: id={}", node_id);
                return 0;
            }
        };
        node_box.get_attr_i32(name)
    }

    pub fn set_node_attr_i32(&mut self, node_id: Identifier, name: &str, value: i32) {
        let mut node_box = match self.find_node_index_from_id(node_id) {
            Some(value) => &mut self.nodes[value],
            None => {
                println!("Node id not found: id={}", node_id);
                return; // Add error code?
            }
        };
        node_box.set_attr_i32(name, value);
    }

    pub fn get_node_attr_f32(&self, node_id: Identifier, name: &str) -> f32 {
        let node_box = match self.find_node_index_from_id(node_id) {
            Some(value) => &self.nodes[value],
            None => {
                println!("Node id not found: id={}", node_id);
                return 0.0;
            }
        };
        node_box.get_attr_f32(name)
    }

    pub fn set_node_attr_f32(&mut self, node_id: Identifier, name: &str, value: f32) {
        let mut node_box = match self.find_node_index_from_id(node_id) {
            Some(value) => &mut self.nodes[value],
            None => {
                println!("Node id not found: id={}", node_id);
                return; // Add error code?
            }
        };
        node_box.set_attr_f32(name, value);
    }

    /// Check if a node with the given hash exists in the graph.
    pub fn node_exists(&mut self, node_id: u64) -> bool {
        let node_idx = self.find_node_index_from_id(node_id);
        let found = match node_idx {
            Some(value) => true,
            None => false,
        };
        println!("Node with hash {} exists={}", node_id, found);
        found
    }

    /// Return a node index of the node with the given hash.
    fn find_node_index_from_id(&self, node_id: u64) -> Option<usize> {
        // println!("find_node_index_from_id {:?}.", node_id);
        let maybe_node_idx = self.ids.iter().position(|&v| v == node_id);
        match maybe_node_idx {
            // Some(value) => println!("Node found: id={} index={}", node_id, value),
            None => println!("Node NOT found: id={}", node_id),
            _ => (),
        }
        maybe_node_idx
    }

    /// Connect the source node (src_node_id) to destination node
    /// (dst_node_id).
    pub fn connect(&mut self, src_node_id: Identifier, dst_node_id: Identifier, input_num: u8) {
        println!("Connect {} to {}:{}", src_node_id, dst_node_id, input_num);
        let src_node_idx = match self.find_node_index_from_id(src_node_id) {
            Some(value) => value,
            None => {
                println!("Source node id not found: id={}", src_node_id);
                return; // TODO: Return an error code?
            }
        };
        let dst_node_idx = match self.find_node_index_from_id(dst_node_id) {
            Some(value) => value,
            None => {
                println!("Destination node id not found: id={}", dst_node_id);
                return; // TODO: Return an error code?
            }
        };

        let src_index = petgraph::graph::NodeIndex::new(src_node_idx);
        let dst_index = petgraph::graph::NodeIndex::new(dst_node_idx);
        // TODO: Check there is no other edge from src to dst, with
        // the same input_num value.
        let index = self.graph.add_edge(src_index, dst_index, input_num).index();
    }

    /// Compute the graph!
    //
    // TODO: Add an "executor" variable to this method to explain how
    // we wish to execute the graph.
    pub fn execute(&mut self, start_node_id: u64, cache: &mut Box<CacheImpl>) -> ExecuteStatus {
        println!("Execute: {}", start_node_id);
        let start_node_idx = match self.find_node_index_from_id(start_node_id) {
            Some(value) => value,
            None => {
                self.status = ExecuteStatus::Error;
                println!("Node id not found: id={}", start_node_id);
                return self.status;
            }
        };

        // println!(
        //     "{:?}",
        //     Dot::with_config(&self.graph, &[Config::EdgeNoLabel])
        // );

        let mut sorted_node_indexes = Vec::<NodeIdx>::new();

        // Get the stack of indices to be computed, going upstream
        // from the starting index.
        let start_index = NodeIdx::new(start_node_idx);
        let mut walker = UpstreamEvalSearch::new(&self.graph, start_index);
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
                    self.output = value.clone();
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

    /// Get the output stream from the last executed graph node.
    pub fn output_stream(&self) -> StreamDataImplShared {
        // println!("Query Stream Output...");
        assert_eq!(self.status, ExecuteStatus::Success);
        self.output.clone()
    }
}
