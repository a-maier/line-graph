# line-graph

Construct the line graph of an undirected graph

This crate provides a single function that takes an undirected
[petgraph](https://github.com/petgraph/petgraph) graph and
constructs the corresponding
[line graph](https://en.wikipedia.org/wiki/Line_graph).
Node weights are turned into edge weights and vice versa.

## Example

The triangle graph is the same as its line graph.

```rust
use line_graph::line_graph;
use petgraph::{
   algo::is_isomorphic,
   graph::UnGraph
};

let g = UnGraph::<(), ()>::from_edges([(0, 1), (1, 2), (2, 0)]);
let g_line = line_graph(&g);
assert!(is_isomorphic(&g, &g_line));
```

## Caveats

If edges are connected by two vertices, the corresponding vertices
in the line graph will also be connected by two edges.


License: MIT OR Apache-2.0
