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

use petgraph;
use petgraph::data::DataMap;
use petgraph::stable_graph::EdgeReference;
use petgraph::visit::{
    EdgeRef, GraphRef, IntoEdgeReferences, IntoEdgesDirected, IntoNeighborsDirected, VisitMap,
    Visitable,
};
use petgraph::{Direction, Incoming};
use std::collections::VecDeque;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct UpstreamEvalSearch<N, VM> {
    pub stack: VecDeque<(N, u32)>, // The queue of nodes to visit
    pub discovered: VM,            // The map of discovered nodes
}

impl<N, VM> UpstreamEvalSearch<N, VM>
where
    N: Copy + PartialEq,
    VM: VisitMap<N>,
{
    // Create a new **UpstreamEvalSearch**, using the graph's visitor
    // map, and put **start** in the stack of nodes to visit.
    pub fn new<G>(graph: G, start: N) -> Self
    where
        G: GraphRef + Visitable<NodeId = N, Map = VM>,
    {
        let mut discovered = graph.visit_map();
        discovered.visit(start);
        let mut stack = VecDeque::new();
        stack.push_front((start, 0));
        UpstreamEvalSearch { stack, discovered }
    }

    // Return the next node in the breath-first-search, or **None** if
    // the traversal is done.
    pub fn next<G>(&mut self, graph: G) -> Option<(N, u32)>
    where
        G: IntoEdgesDirected<NodeId = N> + DataMap,
        G::EdgeWeight: Ord,
    {
        let dir = Direction::Incoming;
        if let Some((node, depth)) = self.stack.pop_front() {
            let mut edges: Vec<_> = graph
                .edges_directed(node, dir)
                .map(|e_ref| (e_ref.id(), graph.edge_weight(e_ref.id()), e_ref.source()))
                .collect();
            edges.sort_unstable_by_key(|ewn| ewn.1);

            for edge in edges {
                let source_node = edge.2;
                if self.discovered.visit(source_node) {
                    self.stack.push_back((source_node, depth + 1));
                }
            }
            return Some((node, depth));
        }
        None
    }
}
