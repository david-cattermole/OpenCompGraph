use anyhow::Result;
use log::{debug, error, info, warn};
use petgraph;
use petgraph::dot::{Config, Dot};
use petgraph::Direction;
use std::hash::{Hash, Hasher};

use crate::cache::CacheImpl;
use crate::cxxbridge::create_stream_data_shared;
use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::CacheImplShared;
use crate::cxxbridge::ffi::ExecuteStatus;
use crate::cxxbridge::ffi::GraphState;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::StreamDataImplShared;
use crate::data::EdgeIdx;
use crate::data::EdgeWeight;
use crate::data::ErrorCode;
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
    state: GraphState,
    status: ExecuteStatus,
}

impl GraphImpl {
    /// Create a brand new empty graph.
    pub fn new() -> GraphImpl {
        let nodes = Vec::new();
        let ids = Vec::new();
        let graph = NodeGraph::with_capacity(0, 0);
        let output = create_stream_data_shared();
        let state = GraphState::Uninitialized;
        let status = ExecuteStatus::Uninitialized;
        GraphImpl {
            nodes,
            ids,
            graph,
            output,
            state,
            status,
        }
    }

    /// What state is the graph in?
    pub fn state(&self) -> GraphState {
        self.state
    }

    /// Return the last execution status.
    pub fn execute_status(&self) -> ExecuteStatus {
        self.status
    }

    /// Add a new node to the graph.
    pub fn add_node(&mut self, node_box: Box<NodeImpl>) -> usize {
        let id = node_box.get_id();
        let nodes_index = self.nodes.len();
        self.nodes.push(node_box);
        self.ids.push(id);

        self.state = GraphState::Dirty;
        let index = self.graph.add_node(id).index();
        debug!("Add Node index={} id={}", index, id);
        assert_eq!(index, nodes_index);
        index
    }

    /// Remove node from the graph.
    pub fn remove_node(&mut self, node_id: u64) -> bool {
        debug!("Remove Node id={}", node_id);
        let node_idx = match self.find_node_index_from_id(node_id) {
            Some(value) => value,
            None => {
                warn!("Node id not found: id={}", node_id);
                return false; // TODO: Return an error code?
            }
        };
        // TODO: Zero out the now empty fields.
        let node_index = petgraph::graph::NodeIndex::new(node_idx);
        match self.graph.remove_node(node_index) {
            Some(value) => {
                self.state = GraphState::Dirty;
                true
            }
            None => false,
        }
    }

    pub fn node_attr_exists(&self, node_id: Identifier, name: &str) -> AttrState {
        let node_box = match self.find_node_index_from_id(node_id) {
            Some(value) => &self.nodes[value],
            None => {
                warn!("Node id not found: id={}", node_id);
                return AttrState::Missing;
            }
        };
        node_box.attr_exists(name)
    }

    pub fn get_node_attr_str(&self, node_id: Identifier, name: &str) -> &str {
        let node_box = match self.find_node_index_from_id(node_id) {
            Some(value) => &self.nodes[value],
            None => {
                warn!("Node id not found: id={}", node_id);
                return "";
            }
        };
        node_box.get_attr_str(name)
    }

    pub fn set_node_attr_str(&mut self, node_id: Identifier, name: &str, value: &str) {
        let mut node_box = match self.find_node_index_from_id(node_id) {
            Some(value) => &mut self.nodes[value],
            None => {
                warn!("Node id not found: id={}", node_id);
                return; // TODO: Add error code?
            }
        };
        // TODO: Only set dirty state if setting the value was
        // successful (there is no way to confirm this currently.)
        self.state = GraphState::Dirty;
        node_box.set_attr_str(name, value);
    }

    pub fn get_node_attr_i32(&self, node_id: Identifier, name: &str) -> i32 {
        let node_box = match self.find_node_index_from_id(node_id) {
            Some(value) => &self.nodes[value],
            None => {
                warn!("Node id not found: id={}", node_id);
                return 0;
            }
        };
        node_box.get_attr_i32(name)
    }

    pub fn set_node_attr_i32(&mut self, node_id: Identifier, name: &str, value: i32) {
        let mut node_box = match self.find_node_index_from_id(node_id) {
            Some(value) => &mut self.nodes[value],
            None => {
                warn!("Node id not found: id={}", node_id);
                return; // TODO: Add error code?
            }
        };
        // TODO: Only set dirty state if setting the value was
        // successful (there is no way to confirm this currently.)
        self.state = GraphState::Dirty;
        node_box.set_attr_i32(name, value);
    }

    pub fn get_node_attr_f32(&self, node_id: Identifier, name: &str) -> f32 {
        let node_box = match self.find_node_index_from_id(node_id) {
            Some(value) => &self.nodes[value],
            None => {
                warn!("Node id not found: id={}", node_id);
                return 0.0;
            }
        };
        node_box.get_attr_f32(name)
    }

    pub fn set_node_attr_f32(&mut self, node_id: Identifier, name: &str, value: f32) {
        let mut node_box = match self.find_node_index_from_id(node_id) {
            Some(value) => &mut self.nodes[value],
            None => {
                warn!("Node id not found: id={}", node_id);
                return; // TODO: Add error code?
            }
        };
        // TODO: Only set dirty state if setting the value was
        // successful (there is no way to confirm this currently.)
        self.state = GraphState::Dirty;
        node_box.set_attr_f32(name, value);
    }

    pub fn node_status(&self, node_id: Identifier) -> NodeStatus {
        let node_box = match self.find_node_index_from_id(node_id) {
            Some(value) => &self.nodes[value],
            None => {
                warn!("Node id not found: id={}", node_id);
                return NodeStatus::NonExistent;
            }
        };
        node_box.get_status()
    }

    fn node_attrs_data_debug_string(&self, node_id: Identifier) -> String {
        let node_box = match self.find_node_index_from_id(node_id) {
            Some(value) => &self.nodes[value],
            None => {
                warn!("Node id not found: id={}", node_id);
                return "could not find node attrs".to_string();
            }
        };
        node_box.data_debug_string()
    }

    /// Check if a node with the given hash exists in the graph.
    pub fn node_exists(&self, node_id: u64) -> bool {
        let node_idx = self.find_node_index_from_id(node_id);
        let found = match node_idx {
            Some(value) => true,
            None => false,
        };
        debug!("Node with hash {} exists={}", node_id, found);
        found
    }

    /// Return a node index of the node with the given hash.
    fn find_node_index_from_id(&self, node_id: u64) -> Option<usize> {
        debug!("find_node_index_from_id {:?}.", node_id);
        let maybe_node_idx = self.ids.iter().position(|&v| v == node_id);
        match maybe_node_idx {
            Some(value) => debug!("Node found: id={} index={}", node_id, value),
            None => warn!("Node NOT found: id={}", node_id),
            // _ => (),
        }
        maybe_node_idx
    }

    /// Connect the source node (src_node_id) to destination node
    /// (dst_node_id), with the input number (input_num).
    ///
    /// Each node may have multiple input nodes. For example a node
    /// combining two images will have two inputs.
    ///
    /// If the edge between the source and destination nodes already
    /// exist and the same input number is used, no new connection is
    /// made.
    pub fn connect(&mut self, src_node_id: Identifier, dst_node_id: Identifier, input_num: u8) {
        debug!("Connect {} to {}:{}", src_node_id, dst_node_id, input_num);
        let src_node_idx = match self.find_node_index_from_id(src_node_id) {
            Some(value) => value,
            None => {
                warn!("Source node id not found: id={}", src_node_id);
                return; // TODO: Return an error code?
            }
        };
        let dst_node_idx = match self.find_node_index_from_id(dst_node_id) {
            Some(value) => value,
            None => {
                warn!("Destination node id not found: id={}", dst_node_id);
                return; // TODO: Return an error code?
            }
        };

        let src_index = petgraph::graph::NodeIndex::new(src_node_idx);
        let dst_index = petgraph::graph::NodeIndex::new(dst_node_idx);

        // Check there is no other edge from src to dst, with
        // the same input_num value.
        let incoming_edges = self.graph.edges_directed(dst_index, Direction::Incoming);
        let edges_existing = incoming_edges
            .into_iter()
            .fold(0, |acc, x| acc + (*x.weight() == input_num) as usize);

        if edges_existing == 0 {
            self.state = GraphState::Dirty;
            self.graph.add_edge(src_index, dst_index, input_num);
        }
    }

    // Get the stack of indices to be computed, going upstream
    // from the starting index.
    fn find_upstream_indexes(&self, start_node_idx: usize) -> Vec<NodeIdx> {
        let mut sorted_node_indexes = Vec::<NodeIdx>::new();
        let start_index = NodeIdx::new(start_node_idx);
        let mut walker = UpstreamEvalSearch::new(&self.graph, start_index);
        while let Some(nx) = walker.next(&self.graph) {
            let index = nx.index();
            let node = &self.nodes[index];
            sorted_node_indexes.push(nx);
            debug!("walk index: {}", index);
        }
        sorted_node_indexes
    }

    /// Get upstream parent inputs (so we can calculate the node hash)
    fn compute_node_metadata(
        &mut self,
        nx: NodeIdx,
        frame: i32,
        parent_inputs: &mut Vec<StreamDataImplShared>,
        cache: &mut Box<CacheImpl>,
    ) -> Result<(GraphIdx, Vec<StreamDataImplShared>), ErrorCode> {
        let mut inputs = Vec::<StreamDataImplShared>::new();
        let parents = self.graph.neighbors_directed(nx, Direction::Incoming);
        for parent_node_index in parents {
            let parent_index = parent_node_index.index();
            debug!("parent index: {}", parent_index);
            let parent_node = &self.nodes[parent_index];
            let parent_hash = parent_node.hash(frame, &parent_inputs);

            match cache.get(&parent_hash) {
                Some(value) => {
                    debug!("Got hash: {}", parent_hash);
                    let mut stream_data = create_stream_data_shared();
                    stream_data.inner = value.inner.clone();
                    inputs.push(stream_data);
                }
                _ => warn!("Missing from cache: {}", parent_hash),
            }
        }
        let node_index = nx.index();
        Ok((node_index, inputs))
    }

    /// Compute the node.
    fn compute_node_output(
        &mut self,
        inputs: &Vec<StreamDataImplShared>,
        node_index: GraphIdx,
        frame: i32,
        cache: &mut Box<CacheImpl>,
    ) -> Result<(), ErrorCode> {
        let node = &mut self.nodes[node_index];
        let node_hash = node.hash(frame, &inputs);
        let mut cached_output = cache.get(&node_hash);
        match cached_output {
            Some(value) => {
                debug!("Reuse Hash: {}", node_hash);
                self.output = value.clone();
                Ok(())
            }
            None => {
                debug!("Cache Miss: {}", node_hash);
                self.output = create_stream_data_shared();
                match node.compute(frame, &inputs, &mut self.output) {
                    NodeStatus::Valid | NodeStatus::Warning => {
                        cache.insert(node_hash, self.output.clone());
                        Ok(())
                    }
                    NodeStatus::Uninitialized => {
                        error!("Node is uninitialized: node_index={}", node_index);
                        return Err(ErrorCode::Uninitialized);
                    }
                    NodeStatus::Error => {
                        error!("Failed to compute node: node_index={}", node_index);
                        return Err(ErrorCode::Failure);
                    }
                    _ => {
                        error!("Unknown error: node_index={}", node_index);
                        return Err(ErrorCode::Failure);
                    }
                }
            }
        }
    }

    fn execute_frame(
        &mut self,
        sorted_node_indexes: &Vec<NodeIdx>,
        frame: i32,
        cache: &mut Box<CacheImpl>,
    ) -> Result<(), ErrorCode> {
        info!("Execute Frame Context: {}", frame);

        let mut parent_inputs = Vec::<StreamDataImplShared>::new();
        for nx in sorted_node_indexes.iter().rev() {
            debug!("Compute Node Index: {:?}", nx);
            let (node_index, node_inputs) =
                self.compute_node_metadata(*nx, frame, &mut parent_inputs, cache)?;
            self.compute_node_output(&node_inputs, node_index, frame, cache)?;
            parent_inputs = node_inputs.to_vec();
        }
        Ok(())
    }

    /// Compute the graph!
    //
    // TODO: Add an "executor" variable to this method to as the
    // manager of the (CPU and/or GPU) resources. This is to avoid two
    // different graphs trying to evaluate at the same time and
    // stealing resources or causing resource contention.
    //
    // TODO: Add an "evaluation context" variable to this method to
    // explain how we wish to execute the graph; Single threaded,
    // multi-threaded, or background/asynchronously?
    //
    pub fn execute(
        &mut self,
        start_node_id: u64,
        frames: &[i32],
        cache: &mut Box<CacheImpl>,
    ) -> ExecuteStatus {
        info!("Execute: {}", start_node_id);
        let start_node_idx = match self.find_node_index_from_id(start_node_id) {
            Some(value) => value,
            None => {
                self.status = ExecuteStatus::Error;
                warn!("Node id not found: id={}", start_node_id);
                return self.status;
            }
        };
        let sorted_node_indexes = self.find_upstream_indexes(start_node_idx);

        for frame in frames {
            match self.execute_frame(&sorted_node_indexes, *frame, cache) {
                Err(e) => {
                    match e {
                        ErrorCode::Failure => {
                            self.status = ExecuteStatus::Error;
                        }
                        ErrorCode::Invalid => {
                            self.status = ExecuteStatus::Error;
                        }
                        ErrorCode::Uninitialized => {
                            self.status = ExecuteStatus::Uninitialized;
                        }
                    }
                    return self.status;
                }
                _ => (),
            }
        }

        self.state = GraphState::Clean;
        self.status = ExecuteStatus::Success;
        self.status
    }

    /// Convert the graph into a human-readable string, for debug
    /// printing.
    pub fn data_debug_string(&self) -> String {
        debug!("Graph Debug");
        let mut string = format!(
            "{:#?}",
            Dot::with_config(&self.graph, &[Config::GraphContentOnly])
        );
        for (i, id) in self.ids.iter().enumerate() {
            let node_status = self.node_status(*id);
            let attrs = self.node_attrs_data_debug_string(*id);
            let line = format!(
                "index={} id={} status={:?} attrs={}\n",
                i, *id, node_status, attrs
            );
            string.push_str(line.as_str());
        }
        string
    }

    /// Get the output stream from the last executed graph node.
    pub fn output_stream(&self) -> StreamDataImplShared {
        debug!("Query Stream Output...");
        self.output.clone()
    }
}

pub fn create_graph_box() -> Box<GraphImpl> {
    debug!("create_graph_box()");
    Box::new(GraphImpl::new())
}
