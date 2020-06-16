use std::collections::HashMap;
use std::hash::Hash;

use super::{ Graph, WeightedGraph, Error };

/// Reference implementation of Graph and WeightedGraph.Implements an
/// undirected, edge-weighted, graph. Node, neighbor, and edge
/// iteration order are set by the build function and will remain stable.
/// 
/// ```rust
/// use gamma::graph::{ Graph, StableGraph, Error };
/// 
/// fn main() -> Result<(), Error> {
///     let mut graph = StableGraph::build(vec![ 0, 1, 2 ], vec![
///         (0, 1, "a"),
///         (1, 2, "b")
///     ])?;
/// 
///     assert_eq!(graph.is_empty(), false);
///     assert_eq!(graph.order(), 3);
///     assert_eq!(graph.size(), 2);
///     assert_eq!(graph.nodes().collect::<Vec<_>>(), vec![ &0, &1, &2 ]);
///     assert_eq!(graph.has_node(&0), true);
///     assert_eq!(graph.neighbors(&1)?.collect::<Vec<_>>(), vec![ &0, &2 ]);
///     assert_eq!(graph.degree(&1)?, 2);
///     assert_eq!(graph.edges().collect::<Vec<_>>(), vec![
///        (&0, &1), (&1, &2)
///     ]);
///     assert_eq!(graph.has_edge(&0, &1)?, true);
///     assert_eq!(graph.has_edge(&1, &0)?, true);
///     assert_eq!(graph.has_edge(&0, &2)?, false);
/// 
///     Ok(())
/// }
/// ```
/// 
/// Although nodes and edges must implement Clone, the can be done in a
/// lightweight manner using reference counting (std::rc::Rc). Likewise,
/// references implement Clone (efficiently), so they can be used as
/// nodes if the resulting StableGraph is not moved.
pub struct StableGraph<N, E> {
    nodes: Vec<N>,
    adjacency: HashMap<N, Vec<(N, E)>>,
    edges: Vec<(N, N)>
}

impl<N: Eq+Hash+Clone, E: Clone> StableGraph<N, E> {
    pub fn build(
        node_list: Vec<N>,
        edge_list: Vec<(N, N, E)>
    ) -> Result<Self, Error> {
        let mut adjacency = HashMap::new();
        let mut nodes = vec![ ];
        let mut edges = vec![ ];

        for node in node_list {
            if adjacency.contains_key(&node) {
                return Err(Error::DuplicateNode);
            }

            adjacency.insert(node.clone(), Vec::new());
            nodes.push(node);
        }

        for (source, target, weight) in edge_list {
            match adjacency.get_mut(&source) {
                Some(outs) => {
                    for (mate, _) in outs.iter() {
                        if mate == &target {
                            return Err(Error::DuplicateEdge);
                        }
                    }

                    outs.push((target.clone(), weight.clone()));
                },
                None => return Err(Error::UnknownNode)
            }

            match adjacency.get_mut(&target) {
                Some(outs) => {
                    for (mate, _) in outs.iter() {
                        if mate == &source {
                            return Err(Error::DuplicateEdge);
                        }
                    }

                    outs.push((source.clone(), weight));
                },
                None => return Err(Error::UnknownNode)
            }

            edges.push((source, target));
        }

        Ok(Self { nodes, adjacency, edges })
    }
}

impl<'a, N: 'a+Hash+Eq, E: 'a> Graph<'a, N> for StableGraph<N, E> {
    type NodeIterator = std::slice::Iter<'a, N>;
    type NeighborIterator = NeighborIterator<'a, N, E>;
    type EdgeIterator = EdgeIterator<'a, N>;

    fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    fn order(&self) -> usize {
        self.nodes.len()
    }

    fn size(&self) -> usize {
        self.edges.len()
    }

    fn nodes(&'a self) -> Self::NodeIterator {
        self.nodes.iter()
    }

    fn has_node(&self, node: &N) -> bool {
        self.adjacency.contains_key(node)
    }

    fn neighbors(
        &'a self, node: &N
    ) -> Result<Self::NeighborIterator, Error> {
        match self.adjacency.get(node) {
            Some(neighbors) => Ok(NeighborIterator { iter: neighbors.iter() }),
            None => Err(Error::UnknownNode)
        }
    }

    fn degree(&self, node: &N) -> Result<usize, Error> {
        match self.adjacency.get(node) {
            Some(neighbors) => Ok(neighbors.len()),
            None => Err(Error::UnknownNode)
        }
    }

    fn has_edge(&self, source: &N, target: &N) -> Result<bool, Error> {
        match self.adjacency.get(source) {
            Some(neighbors) => {
                if neighbors.iter().any(|edge| &edge.0 == target) {
                    Ok(true)
                } else {
                    if self.adjacency.contains_key(target) {
                        Ok(false)
                    } else {
                        Err(Error::UnknownNode)
                    }
                }
            },
            None => Err(Error::UnknownNode)
        }
    }

    fn edges(&'a self) -> Self::EdgeIterator {
        EdgeIterator { iter: self.edges.iter() }
    }
}

impl<'a, N: 'a+Eq+Hash, E: 'a> WeightedGraph<'a, N, E> for StableGraph<N, E> {
    fn weight(
        &self, source: &N, target: &N
    ) -> Result<Option<&E>, Error> {
        match self.adjacency.get(source) {
            Some(neighbors) => {
                match neighbors.iter().find(|(mate, _)| mate == target) {
                    Some((_, weight)) => Ok(Some(weight)),
                    None => {
                        if self.adjacency.contains_key(target) {
                            Ok(None)
                        } else {
                            Err(Error::UnknownNode)
                        }
                    }
                }
            },
            None => Err(Error::UnknownNode)
        }
    }
}

pub struct NeighborIterator<'a, N, E> {
    iter: std::slice::Iter<'a, (N, E)>
}

impl<'a, N, E> Iterator for NeighborIterator<'a, N, E> {
    type Item = &'a N;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|edge| &edge.0)
    }
}

pub struct EdgeIterator<'a, N> {
    iter: std::slice::Iter<'a, (N, N)>
}

impl<'a, N> Iterator for EdgeIterator<'a, N> {
    type Item = (&'a N, &'a N);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|edge| (&edge.0, &edge.1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Eq, Hash, PartialEq, Debug)]
    struct Node {
        value: u8
    }

    impl Node {
        fn new(value: u8) -> Self {
            Node { value }
        }
    }

    #[test]
    fn build_given_duplicate_node() {
        let n0 = Node::new(0);
        let n1 = Node::new(0);
        let graph = StableGraph::<_, ()>::build(vec![
            &n0, &n1
        ], vec![ ]);

        assert_eq!(graph.err(), Some(Error::DuplicateNode))
    }

    #[test]
    fn build_given_invalid_source() {
        let n0 = Node::new(0);
        let n1 = Node::new(1);
        let graph = StableGraph::build(vec![
            &n0
        ], vec![
            (&n1, &n0, ())
        ]);

        assert_eq!(graph.err(), Some(Error::UnknownNode));
    }

    #[test]
    fn build_given_invalid_target() {
        let n0 = Node::new(0);
        let n1 = Node::new(1);
        let graph = StableGraph::build(vec![
            &n0
        ], vec![
            (&n0, &n1, ())
        ]);

        assert_eq!(graph.err(), Some(Error::UnknownNode));
    }

    #[test]
    fn build_given_duplicate_edge() {
        let n0 = Node::new(0);
        let n1 = Node::new(1);
        let graph = StableGraph::build(vec![
            &n0, &n1
        ], vec![
            (&n0, &n1, ()),
            (&n0, &n1, ())
        ]);

        assert_eq!(graph.err(), Some(Error::DuplicateEdge));
    }

    #[test]
    fn build_given_duplicate_edge_reversed() {
        let n0 = Node::new(0);
        let n1 = Node::new(1);
        let graph = StableGraph::build(vec![
            &n0, &n1
        ], vec![
            (&n0, &n1, ()),
            (&n1, &n0, ())
        ]);

        assert_eq!(graph.err(), Some(Error::DuplicateEdge));
    }

    #[test]
    fn is_empty_given_empty() {
        let graph = StableGraph::<(), ()>::build(vec![ ], vec![ ]).unwrap();

        assert!(graph.is_empty());
    }

    #[test]
    fn is_empty_given_p1() {
        let n0 = Node::new(0);
        let graph = StableGraph::<_, ()>::build(vec![
            &n0
        ], vec![ ]).unwrap();

        assert!(!graph.is_empty());
    }

    #[test]
    fn order_given_empty() {
        let graph = StableGraph::<(), ()>::build(vec![ ], vec![ ]).unwrap();

        assert_eq!(graph.order(), 0);
    }

    #[test]
    fn order_given_p1() {
        let n0 = Node::new(0);
        let graph = StableGraph::<_, ()>::build(vec![
            &n0
        ], vec![ ]).unwrap();

        assert_eq!(graph.order(), 1);
    }

    #[test]
    fn size_given_empty() {
        let graph = StableGraph::<(), ()>::build(vec![ ], vec![ ]).unwrap();

        assert_eq!(graph.size(), 0);
    }

    #[test]
    fn size_given_p3() {
        let n0 = Node::new(0);
        let n1 = Node::new(1);
        let n2 = Node::new(2);
        let graph = StableGraph::build(vec![
            &n0, &n1, &n2
         ], vec![
            (&n0, &n1, ()),
            (&n1, &n2, ())
         ]).unwrap();

        assert_eq!(graph.size(), 2);
    }

    #[test]
    fn nodes_given_empty() {
        let graph = StableGraph::<(), ()>::build(vec![ ], vec![ ]).unwrap();
        let nodes = graph.nodes().collect::<Vec<_>>();

        assert_eq!(nodes.is_empty(), true);
    }

    #[test]
    fn nodes_given_p3() {
        let n0 = Node::new(0);
        let n1 = Node::new(1);
        let n2 = Node::new(2);
        let graph = StableGraph::build(vec![
            &n0, &n1, &n2
        ], vec![
            (&n0, &n1, ()),
            (&n1, &n2, ())
        ]).unwrap();
        let nodes = graph.nodes().collect::<Vec<_>>();

        assert_eq!(nodes, vec![
            &&n0, &&n1, &&n2
        ]);
    }

    #[test]
    fn has_node_given_outside() {
        let n0 = Node::new(0);
        let n1 = Node::new(1);
        let graph = StableGraph::<_, ()>::build(vec![
            &n0
        ], vec![

        ]).unwrap();

        assert_eq!(graph.has_node(&&n1), false);
    }

    #[test]
    fn has_node_given_inside() {
        let n0 = Node::new(0);
        let graph = StableGraph::<_, ()>::build(vec![
            &n0
        ], vec![

        ]).unwrap();

        assert_eq!(graph.has_node(&&n0), true);
    }

    #[test]
    fn neighbors_given_outside() {
        let n0 = Node::new(0);
        let n1 = Node::new(1);
        let graph = StableGraph::<_, ()>::build(vec![
            &n0
        ], vec![ ]).unwrap();
        let neighbors = graph.neighbors(&&n1);

        assert_eq!(neighbors.err(), Some(Error::UnknownNode));
    }

    #[test]
    fn neighbors_given_p1() {
        let n0 = Node::new(0);
        let graph = StableGraph::<_, ()>::build(vec![
            &n0
        ], vec![ ]).unwrap();
        let neighbors = graph.neighbors(&&n0).unwrap();

        assert!(neighbors.collect::<Vec<_>>().is_empty());
    }

    #[test]
    fn neighbors_given_p2() {
        let n0 = Node::new(0);
        let n1 = Node::new(1);
        let graph = StableGraph::build(vec![
            &n0, &n1
        ], vec![
            (&n0, &n1, ())
        ]).unwrap();
        let neighbors = graph.neighbors(&&n0).unwrap();

        assert_eq!(neighbors.collect::<Vec<_>>(), vec![ &&n1 ]);
    }

    #[test]
    fn neighbors_given_p3_secondary() {
        let n0 = Node::new(0);
        let n1 = Node::new(1);
        let n2 = Node::new(2);
        let graph = StableGraph::build(vec![
            &n0, &n1, &n2
        ], vec![
            (&n0, &n1, ()),
            (&n1, &n2, ())
        ]).unwrap();
        let neighbors = graph.neighbors(&&n1).unwrap();

        assert_eq!(neighbors.collect::<Vec<_>>(), vec![
            &&n0, &&n2
        ]);
    }

    #[test]
    fn degree_given_outside() {
        let n0 = Node::new(0);
        let n1 = Node::new(1);
        let graph = StableGraph::<_, ()>::build(vec![
            &n0
        ], vec![ ]).unwrap();
        let degree = graph.degree(&&n1);

        assert_eq!(degree.err(), Some(Error::UnknownNode));
    }

    #[test]
    fn degree_given_p3_secondary() {
        let n0 = Node::new(0);
        let n1 = Node::new(1);
        let n2 = Node::new(2);
        let graph = StableGraph::build(vec![
            &n0, &n1, &n2
        ], vec![
            (&n0, &n1, ()),
            (&n1, &n2, ())
        ]).unwrap();

        assert_eq!(graph.degree(&&n1), Ok(2));
    }

    #[test]
    fn edges_given_empty() {
        let graph = StableGraph::<(), ()>::build(vec![ ], vec![ ]).unwrap();
        let edges = graph.edges().collect::<Vec<_>>();

        assert!(edges.is_empty());
    }

    #[test]
    fn edges_given_p3_secondary() {
        let n0 = Node::new(0);
        let n1 = Node::new(1);
        let n2 = Node::new(2);
        let graph = StableGraph::build(vec![
            &n0, &n1, &n2
        ], vec![
            (&n0, &n1, ()),
            (&n1, &n2, ())
        ]).unwrap();
        let edges = graph.edges().collect::<Vec<_>>();

        assert_eq!(edges, vec![
            (&&n0, &&n1),
            (&&n1, &&n2)
        ]);
    }

    #[test]
    fn has_edge_given_outside_source() {
        let n0 = Node::new(0);
        let n1 = Node::new(1);
        let graph = StableGraph::<_, ()>::build(vec![
            &n0
        ], vec![ ]).unwrap();
        let result = graph.has_edge(&&n1, &&n0);

        assert_eq!(result.err(), Some(Error::UnknownNode));
    }

    #[test]
    fn has_edge_given_outside_target() {
        let n0 = Node::new(0);
        let n1 = Node::new(1);
        let graph = StableGraph::<_, ()>::build(vec![
            &n0
        ], vec![ ]).unwrap();
        let result = graph.has_edge(&&n0, &&n1);

        assert_eq!(result.err(), Some(Error::UnknownNode));
    }

    #[test]
    fn has_edge_given_unconnected() {
        let n0 = Node::new(0);
        let n1 = Node::new(1);
        let graph = StableGraph::<_, ()>::build(vec![
            &n0, &n1
        ], vec![ ]).unwrap();

        assert!(!graph.has_edge(&&n0, &&n1).unwrap());
    }

    #[test]
    fn has_edge_given_connected() {
        let n0 = Node::new(0);
        let n1 = Node::new(1);
        let graph = StableGraph::build(vec![
            &n0, &n1
        ], vec![
            (&n0, &n1, ())
        ]).unwrap();

        assert!(graph.has_edge(&&n0, &&n1).unwrap());
    }

    #[test]
    fn has_edge_given_connected_and_reversed() {
        let n0 = Node::new(0);
        let n1 = Node::new(1);
        let graph = StableGraph::build(vec![
            &n0, &n1
        ], vec![
            (&n0, &n1, ())
        ]).unwrap();

        assert!(graph.has_edge(&&n1, &&n0).unwrap());
    }

    #[test]
    fn weight_given_outside_source() {
        let n0 = Node::new(0);
        let n1 = Node::new(1);
        let graph = StableGraph::<_, ()>::build(vec![
            &n0
        ], vec![ ]).unwrap();
        let weight = graph.weight(&&n1, &&n0);

        assert_eq!(weight.err(), Some(Error::UnknownNode));
    }

    #[test]
    fn weight_given_outside_target() {
        let n0 = Node::new(0);
        let n1 = Node::new(1);
        let graph = StableGraph::<_, ()>::build(vec![
            &n0
        ], vec![ ]).unwrap();
        let weight = graph.weight(&&n0, &&n1);

        assert_eq!(weight.err(), Some(Error::UnknownNode));
    }

    #[test]
    fn weight_given_no_edge() {
        let n0 = Node::new(0);
        let n1 = Node::new(1);
        let graph = StableGraph::<_, ()>::build(vec![
            &n0, &n1
        ], vec![ ]).unwrap();
        let weight = graph.weight(&&n0, &&n1);

        assert_eq!(weight, Ok(None));
    }

    #[test]
    fn weight_given_edge() {
        let n0 = Node::new(0);
        let n1 = Node::new(1);
        let graph = StableGraph::build(vec![
            &n0, &n1
        ], vec![
            (&n0, &n1, 42)
        ]).unwrap();
        let weight = graph.weight(&&n0, &&n1);

        assert_eq!(weight, Ok(Some(&42)));
    }

    #[test]
    fn weight_given_edge_reversed() {
        let n0 = Node::new(0);
        let n1 = Node::new(1);
        let graph = StableGraph::build(vec![
            &n0, &n1
        ], vec![
            (&n0, &n1, 42)
        ]).unwrap();
        let weight = graph.weight(&&n1, &&n0);

        assert_eq!(weight, Ok(Some(&42)));
    }
}