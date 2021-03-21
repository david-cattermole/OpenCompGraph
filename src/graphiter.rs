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
