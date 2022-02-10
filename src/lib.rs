//! Construct the line graph of an undirected graph
//!
//! This crate provides a single function that takes an undirected
//! [petgraph](https://github.com/petgraph/petgraph) graph and
//! constructs the corresponding
//! [line graph](https://en.wikipedia.org/wiki/Line_graph).
//! Node weights are turned into edge weights and vice versa.
//!
//! # Example
//!
//! The triangle graph is the same as its line graph.
//!
//! ```rust
//! use line_graph::line_graph;
//! use petgraph::{
//!    algo::is_isomorphic,
//!    graph::UnGraph
//! };
//!
//! let g = UnGraph::<(), ()>::from_edges([(0, 1), (1, 2), (2, 0)]);
//! let g_line = line_graph(&g);
//! assert!(is_isomorphic(&g, &g_line));
//! ```
//!
//! # Caveats
//!
//! If edges are connected by two vertices, the corresponding vertices
//! in the line graph will also be connected by two edges.
//!
use petgraph::{
    graph::{DefaultIx, IndexType, UnGraph},
    visit::{EdgeRef, IntoNodeReferences},
};
use std::default::Default;

/// Construct the line graph for `g`
pub fn line_graph<N, E, Ix>(g: &UnGraph<N, E, Ix>) -> UnGraph<E, N, DefaultIx>
where
    N: Clone,
    E: Clone + Default,
    Ix: IndexType,
{
    let mut line_graph = UnGraph::with_capacity(g.edge_count(), 0);
    for _ in 0..g.edge_count() {
        line_graph.add_node(Default::default());
    }
    for (nidx, nwt) in g.node_references() {
        for (s, e1) in g.edges(nidx).enumerate() {
            for e2 in g.edges(nidx).skip(s + 1) {
                let (v1, v2) = {
                    use petgraph::visit::EdgeIndexable;
                    (g.to_index(e1.id()), g.to_index(e2.id()))
                };
                let (v1, v2) = {
                    use petgraph::visit::NodeIndexable;
                    (line_graph.from_index(v1), line_graph.from_index(v2))
                };
                line_graph.add_edge(v1, v2, nwt.clone());
            }
        }
    }

    for node in g.edge_references() {
        let id = {
            use petgraph::visit::EdgeIndexable;
            g.to_index(node.id())
        };
        let id = {
            use petgraph::visit::NodeIndexable;
            line_graph.from_index(id)
        };

        let weight = line_graph.node_weight_mut(id).unwrap();
        *weight = node.weight().clone();
    }
    line_graph
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::algo::is_isomorphic;

    #[test]
    fn dipole() {
        let orig = UnGraph::<(), ()>::from_edges([(0, 1), (0, 1), (0, 1)]);
        let target = UnGraph::<(), ()>::from_edges([
            (0, 1),
            (0, 1),
            (0, 2),
            (0, 2),
            (1, 2),
            (1, 2),
        ]);
        assert!(is_isomorphic(&target, &line_graph(&orig)));
    }

    // wikipedia example, indices shifted by -1
    #[test]
    fn simple() {
        let orig = UnGraph::<(), ()>::from_edges([
            (0, 1),
            (0, 2),
            (0, 3),
            (1, 4),
            (2, 3),
            (3, 4),
        ]);
        let target = UnGraph::<(), ()>::from_edges([
            (0, 1),
            (0, 2),
            (0, 3),
            (1, 2),
            (1, 4),
            (2, 4),
            (2, 5),
            (3, 5),
            (4, 5),
        ]);
        assert!(is_isomorphic(&target, &line_graph(&orig)));
    }
}
