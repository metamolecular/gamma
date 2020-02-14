# GraphCore

A graph library for Rust.

GraphCore provides primitives and traversals for working with [graphs](https://en.wikipedia.org/wiki/Graph_theory).

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
graphcore = "0.1"
```

## Examples

`HashGraph` is the reference `Graph` implementation.

```rust
use graphcore::graph::Graph;
use graphcore::graph::HashGraph;

fn main() {
  let mut graph = HashGraph::build(vec![ 0, 1, 2 ], vec![
      (&0, &1, "a"),
      (&1, &2, "b")
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

Depth-first traversal is implemented as an `Iterator`.

```rust
use graphcore::graph::Graph;
use graphcore::graph::HashGraph;
use graphcore::traversal::depth_first;

fn main() {
  let graph = HashGraph::build(vec![ 0, 1, 2 ], vec![
      (&0, &1, ()),
      (&1, &2, ()),
      (&2, &0, ()),
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
use graphcore::graph::Graph;
use graphcore::graph::HashGraph;
use graphcore::traversal::breadth_first;

fn main() {
  let graph = HashGraph::build(vec![ 0, 1, 2 ], vec![
      (&0, &1, ()),
      (&1, &2, ()),
      (&2, &0, ()),
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

GraphCore is not yet stable, but care is taken to limit breaking changes
and warn with deprecation whenever possible. Patch version never introduce
breaking changes.

# License

GraphCore is distributed under the terms of the MIT License. See
[LICENSE-MIT](LICENSE-MIT) and [COPYRIGHT](COPYRIGHT) for details.