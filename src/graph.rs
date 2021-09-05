/*
 * Copyright (C) 2020, 2021 David Cattermole.
 *
 * This file is part of OpenCompGraph.
 *
 * OpenCompGraph is free software: you can redistribute it and/or modify it
 * under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * OpenCompGraph is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with OpenCompGraph.  If not, see <https://www.gnu.org/licenses/>.
 * ====================================================================
 *
 */

use anyhow::Result;
use log::{debug, error, warn};
use petgraph;
use petgraph::dot::{Config, Dot};
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use rustc_hash::FxHashMap;
use std::collections::VecDeque;
use std::rc::Rc;
use std::time::Instant;

use crate::cache::CacheImpl;
use crate::cxxbridge::ffi::AttrState;
use crate::cxxbridge::ffi::ExecuteStatus;
use crate::cxxbridge::ffi::GraphState;
use crate::cxxbridge::ffi::NodeStatus;
use crate::cxxbridge::ffi::StreamDataImplShared;
use crate::data::EdgeWeight;
use crate::data::ErrorCode;
use crate::data::FrameValue;
use crate::data::GraphIdx;
use crate::data::HashValue;
use crate::data::Identifier;
use crate::data::NodeComputeMode;
use crate::data::NodeIdx;
use crate::data::NodeWeight;
use crate::graphiter::UpstreamEvalSearch;
use crate::node::NodeImpl;
use crate::stream::StreamDataImpl;
use crate::stream::StreamDataImplRc;

type NodeGraph =
    petgraph::stable_graph::StableGraph<NodeWeight, EdgeWeight, petgraph::Directed, GraphIdx>;

#[derive(Debug)]
pub struct GraphImpl {
    nodes: Vec<Box<NodeImpl>>,
    ids: Vec<Identifier>,
    graph: NodeGraph,
    output: Rc<StreamDataImpl>,
    state: GraphState,
    status: ExecuteStatus,
}

impl GraphImpl {
    /// Create a brand new empty graph.
    pub fn new() -> GraphImpl {
        let nodes = Vec::new();
        let ids = Vec::new();
        let graph = NodeGraph::with_capacity(0, 0);
        let output = Rc::new(StreamDataImpl::new());
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
            Some(_value) => {
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
        let node_box = match self.find_node_index_from_id(node_id) {
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
        let node_box = match self.find_node_index_from_id(node_id) {
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
        let node_box = match self.find_node_index_from_id(node_id) {
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
            Some(_value) => true,
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
        let edges_existing: Vec<_> = incoming_edges
            .into_iter()
            .take_while(|x| *x.weight() == input_num)
            .map(|x| (x.source(), x.target()))
            .collect();

        // Remove the existing edges first.
        for (edge_src_index, edge_dst_index) in edges_existing {
            if let Some(edge_index) = self.graph.find_edge(edge_src_index, edge_dst_index) {
                debug!(
                    "Remove edge: src_index={:#?} dst_index={:#?} edge_index={:#?}",
                    edge_src_index, edge_dst_index, edge_index
                );
                self.graph.remove_edge(edge_index);
                ()
            }
        }

        self.state = GraphState::Dirty;
        self.graph.update_edge(src_index, dst_index, input_num);
    }

    // Get the stack of indices to be computed, going upstream
    // from the starting index.
    fn find_all_upstream_nodes(&self, start_node_idx: GraphIdx) -> Vec<NodeIdx> {
        let mut node_indexes = Vec::<NodeIdx>::new();
        let start_index = NodeIdx::new(start_node_idx);
        let mut walker = UpstreamEvalSearch::new(&self.graph, start_index);

        while let Some((node, _depth)) = walker.next(&self.graph) {
            node_indexes.push(node);
        }
        node_indexes
    }

    // Get the stack of indices to be computed, going upstream
    // from the starting index.
    fn find_direct_upstream_nodes(&self, start_node_idx: GraphIdx) -> Vec<NodeIdx> {
        let mut node_indexes = Vec::<NodeIdx>::new();
        let start_index = NodeIdx::new(start_node_idx);
        let mut walker = UpstreamEvalSearch::new(&self.graph, start_index);
        while let Some((node, depth)) = walker.next(&self.graph) {
            if depth == 0 {
                // Skip the current level.
                continue;
            }
            if depth > 1 {
                break;
            }
            node_indexes.push(node);
        }
        node_indexes
    }

    /// Get upstream parent inputs (so we can calculate the node
    /// hash).
    fn compute_node_input_stream_data(
        &self,
        node_idx: NodeIdx,
        stream_data_cache: &FxHashMap<GraphIdx, Rc<StreamDataImpl>>,
    ) -> Result<Vec<Rc<StreamDataImpl>>, ErrorCode> {
        let mut inputs = Vec::<Rc<StreamDataImpl>>::new();

        let parent_node_indexes = self.find_direct_upstream_nodes(node_idx.index());
        debug!("Parent input count: {}", parent_node_indexes.len());
        for parent_node_index in parent_node_indexes {
            let parent_index = parent_node_index.index();
            debug!("parent index: {}", parent_index);

            if let Some(stream_data) = stream_data_cache.get(&parent_index) {
                inputs.push(stream_data.clone());
            } else {
                panic!("Parent node index is missing: {}", parent_index);
            }
        }
        Ok(inputs)
    }

    /// Get upstream parent inputs (so we can calculate the node
    /// hash).
    fn compute_node_input_hash_values(
        &self,
        node_idx: NodeIdx,
        hash_cache: &FxHashMap<GraphIdx, HashValue>,
    ) -> Vec<HashValue> {
        let mut input_hash_values = Vec::<HashValue>::new();

        let parent_node_indexes = self.find_direct_upstream_nodes(node_idx.index());
        debug!("Parent input count: {}", parent_node_indexes.len());
        for parent_node_index in parent_node_indexes {
            let parent_index = parent_node_index.index();
            debug!("parent index: {}", parent_index);

            if let Some(hash_value) = hash_cache.get(&parent_index) {
                input_hash_values.push(*hash_value);
            } else {
                panic!("Parent node index is missing: {}", parent_index);
            }
        }
        input_hash_values
    }

    /// Compute the node.
    fn compute_node_output(
        &mut self,
        inputs: &Vec<Rc<StreamDataImpl>>,
        node_index: GraphIdx,
        // Frame is floating point so we can evaluate sub-frames and
        // frame blending.
        frame: FrameValue,
        node_compute_mode: NodeComputeMode,
        cache: &mut Box<CacheImpl>,
        stream_data_cache: &mut FxHashMap<GraphIdx, Rc<StreamDataImpl>>,
    ) -> Result<(), ErrorCode> {
        let node = &mut self.nodes[node_index];

        // TODO: Store the amount of time the node took to compute,
        // and how much (new) memory it requires. Then use the
        // meta-information to weight the importance of the item in
        // the cache.

        // TODO: The node compute should be split into multiple
        // methods.
        //
        // - A) a method to initalised and validate the node's input.
        // - B) a method to calculate the pixel values.
        // - C) a method to calculate the bounding box only.
        //
        // 'A' is single threaded and is used to initalise data
        // structures and cache data for the future computations.
        //
        // 'B' is run on each row of the image and may be distributed
        // across many threads.
        //
        // 'C' is used to provide bounding-box information so we can
        // test if our render region intersects with the bounding box.
        //
        // Both 'B' and 'C' expect 'A' to have already been called so
        // that any data structures are valid and up-to-date.

        match node.compute(frame, node_compute_mode, &inputs, &mut self.output, cache) {
            NodeStatus::Valid | NodeStatus::Warning => {
                stream_data_cache.insert(node_index, self.output.clone());
                Ok(())
            }
            NodeStatus::Uninitialized => {
                error!("Node is uninitialized: node_index={}", node_index);
                Err(ErrorCode::Uninitialized)
            }
            NodeStatus::Error => {
                error!("Failed to compute node: node_index={}", node_index);
                Err(ErrorCode::Failure)
            }
            _ => {
                error!("Unknown error: node_index={}", node_index);
                Err(ErrorCode::Failure)
            }
        }
    }

    fn execute_frame(
        &mut self,
        node_indexes: &Vec<NodeIdx>,
        frame: FrameValue,
        cache: &mut Box<CacheImpl>,
    ) -> Result<(), ErrorCode> {
        debug!("Execute Frame Context: {}", frame);
        let start = Instant::now();

        // Compute all hash values for each node in the entire
        // connected graph.
        //
        // Start at upstream nodes and move down toward main node.
        let mut hash_cache = FxHashMap::<GraphIdx, HashValue>::default();
        let mut hash_values = Vec::new();
        for node_index in node_indexes.iter().rev() {
            debug!("Compute Node Hash: {:?}", node_index);
            let node = &self.nodes[node_index.index()];
            let input_hash_values =
                self.compute_node_input_hash_values(*node_index, &mut hash_cache);
            let hash_value = node.hash(frame, &input_hash_values);
            hash_cache.insert(node_index.index(), hash_value);
            hash_values.push(hash_value);
        }

        // Validate node, start at node and walk up the graph.
        let mut validated_node_indexes = Vec::new();
        let mut node_stack = VecDeque::<(NodeIdx, NodeComputeMode)>::new();
        let start_node_index = node_indexes[0];
        let start_node_compute_mode = NodeComputeMode::ALL;
        node_stack.push_front((start_node_index, start_node_compute_mode));
        validated_node_indexes.push((start_node_index, start_node_compute_mode));
        while let Some((node_index, node_compute_mode)) = node_stack.pop_front() {
            debug!("Validate Node: {:?}", node_index);
            let input_node_indexes = self.find_direct_upstream_nodes(node_index.index());
            let mut input_nodes = Vec::<&Box<NodeImpl>>::new();
            for up_node_index in &input_node_indexes {
                debug!("up_node_index: {:?}", up_node_index);
                let up_node = &self.nodes[up_node_index.index()];
                input_nodes.push(up_node);
            }

            let node = &self.nodes[node_index.index()];
            let hash_value = hash_cache.get(&node_index.index()).unwrap();
            let node_compute_modes =
                node.validate_inputs(node_compute_mode, *hash_value, &input_nodes);

            for (up_node_index, up_node_compute_mode) in
                input_node_indexes.iter().zip(node_compute_modes.iter())
            {
                debug!("Validate Up Node: {:?}", up_node_index);
                node_stack.push_front((*up_node_index, *up_node_compute_mode));
                validated_node_indexes.push((*up_node_index, *up_node_compute_mode));
            }
        }

        // Start at upstream nodes to compute first.
        let mut stream_data_cache = FxHashMap::<GraphIdx, Rc<StreamDataImpl>>::default();
        for (node_index, node_compute_mode) in validated_node_indexes.iter().rev() {
            debug!("Compute Node: {:?}", node_index);

            let node_inputs =
                self.compute_node_input_stream_data(*node_index, &stream_data_cache)?;
            self.compute_node_output(
                &node_inputs,
                node_index.index(),
                frame,
                *node_compute_mode,
                cache,
                &mut stream_data_cache,
            )?;
        }
        let duration = start.elapsed();
        debug!("Frame Execute {} total time: {:?}", frame, duration);

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
        frames: &[FrameValue],
        cache: &mut Box<CacheImpl>,
    ) -> ExecuteStatus {
        debug!("Execute: {}", start_node_id);
        let start = Instant::now();

        let start_node_idx = match self.find_node_index_from_id(start_node_id) {
            Some(value) => value,
            None => {
                self.status = ExecuteStatus::Error;
                warn!("Node id not found: id={}", start_node_id);
                return self.status;
            }
        };
        let node_indexes = self.find_all_upstream_nodes(start_node_idx);

        for frame in frames {
            match self.execute_frame(&node_indexes, *frame, cache) {
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
        let duration = start.elapsed();
        debug!(
            "Execute {} Frames | total time: {:?}",
            frames.len(),
            duration
        );

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
        StreamDataImplShared {
            inner: Box::new(StreamDataImplRc::from_rc_data(self.output.clone())),
        }
    }
}

pub fn create_graph_box() -> Box<GraphImpl> {
    debug!("create_graph_box()");
    Box::new(GraphImpl::new())
}
