# Gamma

A graph library for Rust.

Gamma provides primitives and traversals for working with [graphs](https://en.wikipedia.org/wiki/Graph_theory). It is based on ideas presented in *[A Minimal Graph API](https://depth-first.com/articles/2020/01/06/a-minimal-graph-api/)*.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
gamma = "0.5"
```

## Examples

`StableGraph` is the reference `Graph` implementation. Node, neighbor, and
edge iteration order are stable and determined by the `build` function.

```rust
use gamma::graph::{ Graph, StableGraph, Error };

fn main() -> Result<(), Error> {
    let graph = StableGraph::build(vec![ 0, 1, 2 ], vec![
        (0, 1, "a"),
        (1, 2, "b")
    ])?;
    
    assert_eq!(graph.is_empty(), false);
    assert_eq!(graph.order(), 3);
    assert_eq!(graph.size(), 2);
    assert_eq!(graph.nodes().collect::<Vec<_>>(), vec![ &0, &1, &2 ]);
    assert_eq!(graph.has_node(&0), true);
    assert_eq!(graph.neighbors(&1)?.collect::<Vec<_>>(), vec![ &0, &2 ]);
    assert_eq!(graph.degree(&1)?, 2);
    assert_eq!(graph.edges().collect::<Vec<_>>(), vec![
        (&0, &1), (&1, &2)
    ]);
    assert_eq!(graph.has_edge(&0, &1)?, true);
    assert_eq!(graph.has_edge(&1, &0)?, true);
    assert_eq!(graph.has_edge(&0, &2)?, false);

    Ok(())
}
```

`IndexGraph` also features stable node, neighbor, and
edge iteration order. However, it is backed purely by `Vec`s, uses no reference
counting, and does not implement `WeightedGraph`. `IndexGraph` also allows
neighbor iteration order on each terminal of an edge to be set, which can be
useful for algorithm debugging.

```rust
use gamma::graph::{ Graph, IndexGraph };

fn main() -> Result<(), Error> {
    let mut graph = IndexGraph::build(vec![
        vec![ 1 ],
        vec![ 0, 2 ],
        vec![ 1 ]
    ])?;
    
    assert_eq!(graph.degree(&1), Ok(2));

    Ok(())
}
 
```

In `HashGraph`, node and edge iterator order are unstable. Use this
implementation when you don't need edge weights or stable iteration order.

```rust
use std::collections::HashMap;
use gamma::graph::{ Graph, HashGraph };

fn main() -> Result<(), Error> {
    let mut adjacency = HashMap::new();

    adjacency.insert('A', vec![ 'B' ]);
    adjacency.insert('B', vec![ 'A' ]);

    let mut graph = HashGraph::build(adjacency)?;

    assert_eq!(graph.degree(&'A'), Ok(1));

    Ok(())
}
```

Depth-first traversal is implemented as an `Iterator`.

```rust
use gamma::graph::{ Graph, StableGraph, Error };
use gamma::traversal::{ depth_first, Step };

fn main() -> Result<(), Error> {
    let graph = StableGraph::build(vec![ 0, 1, 2 ], vec![
        (0, 1, ()),
        (1, 2, ()),
        (2, 0, ()),
    ])?;
    let traversal = depth_first(&graph, &0)?;
    
    assert_eq!(traversal.collect::<Vec<_>>(), vec![
        Step::new(&0, &1, false),
        Step::new(&1, &2, false),
        Step::new(&2, &0, true)
    ]);
}
```

Breadth-first traversal is also implemented as an `Iterator`.

```rust
use gamma::graph::{ Graph, StableGraph, Error };
use gamma::traversal::{ breadth_first, Step };

fn main() -> Result<(), Error> {
    let graph = StableGraph::build(vec![ 0, 1, 2 ], vec![
        (0, 1, ()),
        (1, 2, ()),
        (2, 0, ()),
    ])?;
    let traversal = breadth_first(&graph, &0)?;
    
    assert_eq!(traversal.collect::<Vec<_>>(), vec![
        Step::new(&0, &1, false),
        Step::new(&0, &2, false),
        Step::new(&1, &2, true)
    ]);

    Ok(())
}
```

A greedy matching algorithm is provided, with the result returned as a
`Graph`.

```rust
use std::collections::HashSet;
use gamma::graph::{Graph, IndexGraph, Error};
use gamma::matching::greedy;

fn main() -> Result<(), Error> {
    let graph = IndexGraph::build(vec![
        vec![ 1, 5 ],
        vec![ 0, 2 ],
        vec![ 1, 3 ],
        vec![ 2, 4 ],
        vec![ 3, 5 ],
        vec![ 4, 0 ]
    ])?;
    let matching = greedy(&graph);
    let mut edges = HashSet::new();

    edges.insert((&0, &1));
    edges.insert((&2, &3));
    edges.insert((&4, &5));

    assert_eq!(matching.edges().collect::<HashSet<_>>(), edges);

    Ok(())
}
```

A `Graph`'s connected components are also reported as an Iterator.

```rust
use gamma::graph::{ Graph, IndexGraph };
use gamma::selection::components;

fn main() -> Result<(), Error> {
    let mut graph = IndexGraph::build(vec![
        vec![ 1 ],
        vec![ 0 ],
        vec![ ]
    ])?;
    let components = components(&graph).collect::<Vec<_>>();

    assert_eq!(components.len(), 2);

    Ok(())
}
```

## Versions

Gamma is not yet stable, but care is taken to limit breaking changes whenever
possible. Patch versions never introduce breaking changes.

# License

Gamma is distributed under the terms of the MIT License. See
[LICENSE-MIT](LICENSE-MIT) and [COPYRIGHT](COPYRIGHT) for details.