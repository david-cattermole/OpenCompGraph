use petgraph;
use petgraph::visit::{GraphRef, IntoEdgeReferences, IntoNeighborsDirected, VisitMap, Visitable};
use petgraph::{Direction, Incoming};
use std::collections::VecDeque;

#[derive(Clone)]
pub struct MyBfs<N, VM> {
    pub stack: VecDeque<N>, // The queue of nodes to visit
    pub discovered: VM,     // The map of discovered nodes
}

impl<N, VM> Default for MyBfs<N, VM>
where
    VM: Default,
{
    fn default() -> Self {
        MyBfs {
            stack: VecDeque::new(),
            discovered: VM::default(),
        }
    }
}

impl<N, VM> MyBfs<N, VM>
where
    N: Copy + PartialEq,
    VM: VisitMap<N>,
{
    // Create a new **MyBfs**, using the graph's visitor map, and put
    // **start** in the stack of nodes to visit.
    pub fn new<G>(graph: G, start: N) -> Self
    where
        G: GraphRef + Visitable<NodeId = N, Map = VM>,
    {
        let mut discovered = graph.visit_map();
        discovered.visit(start);
        let mut stack = VecDeque::new();
        stack.push_front(start);
        MyBfs { stack, discovered }
    }

    // Return the next node in the bfs, or **None** if the traversal
    // is done.
    pub fn next<G>(&mut self, graph: G) -> Option<N>
    where
        G: IntoNeighborsDirected<NodeId = N>,
    {
        let dir = Direction::Incoming;
        if let Some(node) = self.stack.pop_front() {
            for succ in graph.neighbors_directed(node, dir) {
                if self.discovered.visit(succ) {
                    self.stack.push_back(succ);
                }
            }
            return Some(node);
        }
        None
    }
}
