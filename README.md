# Gamma

A graph library for Rust.

Gamma provides primitives and traversals for working with [graphs](https://en.wikipedia.org/wiki/Graph_theory). It is based on ideas presented in *[A Minimal Graph API](https://depth-first.com/articles/2020/01/06/a-minimal-graph-api/)*.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
gamma = "0.2"
```

## Examples

`StableGraph` is the reference `Graph` implementation. Node, edge, and
edge iteration order are stable and determined by the `build` function.

```rust
use gamma::graph::{ Graph, StableGraph };

fn main() {
  let graph = StableGraph::build(vec![ 0, 1, 2 ], vec![
      (0, 1, "a"),
      (1, 2, "b")
  ]
  ).unwrap();
  
  assert_eq!(graph.is_empty(), false);
  assert_eq!(graph.order(), 3);
  assert_eq!(graph.size(), 2);
  assert_eq!(graph.nodes().collect::<Vec<_>>(), vec![ &0, &1, &2 ]);
  assert_eq!(graph.has_node(&0), true);
  assert_eq!(graph.neighbors(&1).unwrap().collect::<Vec<_>>(), vec![ &0, &2 ]);
  assert_eq!(graph.degree(&1).unwrap(), 2);
  assert_eq!(graph.edges().collect::<Vec<_>>(), vec![
      (&0, &1), (&1, &2)
  ]);
  assert_eq!(graph.has_edge(&0, &1).unwrap(), true);
  assert_eq!(graph.has_edge(&1, &0).unwrap(), true);
  assert_eq!(graph.has_edge(&0, &2).unwrap(), false);
}
```

`IndexGraph` also features stable node, neighbor, and
edge iteration order. However, it is backed purely by `Vec`s, uses no reference
counting, and does not implement `WeightedGraph`. IndexGraph` also allows
neighbor iteration order on each terminal of an edge, which can be useful for
algorithm debugging.

```rust
use gamma::graph::{ Graph, IndexGraph };
 
let mut graph = IndexGraph::build(vec![
    vec![ 1 ],
    vec![ 0, 2 ],
    vec![ 1 ]
]).unwrap();

assert_eq!(graph.degree(&1), Ok(2));
```

Depth-first traversal is implemented as an `Iterator`.

```rust
use gamma::graph::{ Graph, StableGraph };
use gamma::traversal::depth_first;

fn main() {
  let graph = StableGraph::build(vec![ 0, 1, 2 ], vec![
      (0, 1, ()),
      (1, 2, ()),
      (2, 0, ()),
  ]).unwrap();
  let traversal = depth_first(&graph, &0).unwrap();
  
  assert_eq!(traversal.collect::<Vec<_>>(), vec![
      (&0, &1, false),
      (&1, &2, false),
      (&2, &0, true)
  ]);
}
```

Breadth-first traversal is also implemented as an `Iterator`.

```rust
use gamma::graph::{ Graph, StableGraph };
use gamma::traversal::breadth_first;

fn main() {
  let graph = StableGraph::build(vec![ 0, 1, 2 ], vec![
      (0, 1, ()),
      (1, 2, ()),
      (2, 0, ()),
  ]).unwrap();
  let traversal = breadth_first(&graph, &0).unwrap();
  
  assert_eq!(traversal.collect::<Vec<_>>(), vec![
      (&0, &1, false),
      (&0, &2, false),
      (&1, &2, true)
  ]);
}
```

## Versions

Gamma is not yet stable, but care is taken to limit breaking changes whenever
possible. Patch versions never introduce breaking changes.

# License

Gamma is distributed under the terms of the MIT License. See
[LICENSE-MIT](LICENSE-MIT) and [COPYRIGHT](COPYRIGHT) for details.