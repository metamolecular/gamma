use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

use super::{ Graph, Error };

/// An undirected, node-labeled Graph. Node, and edge order are undefined.
/// Neighbor order remains stable. As such, HashGraph can be tought of as
/// the hashed node counterpart to IndexGraph.
/// 
/// Note that not only is the order of edges obtained from #edges undefined,
/// but the order in which nodes appear within each tuple is also undefined.
/// 
/// ```rust
/// use std::collections::HashMap;
/// 
/// use gamma::graph::{ Graph, HashGraph, Error };
/// 
/// fn main() -> Result<(), Error> {
///     let mut adjacency = HashMap::new();
/// 
///     adjacency.insert('A', vec![ 'B' ]);
///     adjacency.insert('B', vec![ 'A', 'C' ]);
///     adjacency.insert('C', vec![ 'B' ]);
/// 
///     let mut graph = HashGraph::build(adjacency)?;
/// 
///     assert_eq!(graph.degree(&'B'), Ok(2));
/// 
///     Ok(())
/// }
/// ```
pub struct HashGraph<N> {
    adjacency: HashMap<N, Vec<N>>,
    edges: HashSet<(N, N)>
}

impl<'a, N: 'a+Hash+Eq+Clone> HashGraph<N> {
    /// The elements of adjacency will be validated to ensure:
    /// 
    /// 1. If there is a forward edge, there is a matching back edge.
    /// 2. There are no duplicate edges.
    /// 3. All targets are present as keys.
    /// 4. No loops (self-edges) are present.
    /// 
    /// If any test fails, an error is returned.
    pub fn build(adjacency: HashMap<N, Vec<N>>) -> Result<Self, Error> {
        let mut directed = HashSet::new();

        for (source, source_neighbors) in adjacency.iter() {
            for target in source_neighbors {
                if target == source {
                    return Err(Error::InvalidEdge);
                } else if !adjacency.contains_key(target) {
                    return Err(Error::UnknownNode);
                }

                if !directed.insert((source, target)) {
                    return Err(Error::DuplicateEdge);
                }
            }
        }

        let mut undirected = HashSet::new();

        for (source, target) in directed {
            let mut edge = (target.clone(), source.clone());

            if !undirected.contains(&edge) {
                std::mem::swap(&mut edge.0, &mut edge.1);

                undirected.insert(edge);
            }
        }

        Ok(Self { adjacency, edges: undirected })
    }

    fn neighbors_for(&'a self, node: &N) -> Result<&'a Vec<N>, Error> {
        match self.adjacency.get(node) {
            Some(neighbors) => Ok(neighbors),
            None => Err(Error::UnknownNode)
        }
    }
}

impl<'a, N: 'a+Hash+Eq+Clone> Graph<'a, N> for HashGraph<N> {
    type NodeIterator = std::collections::hash_map::Keys<'a, N, Vec<N>>;
    type NeighborIterator = std::slice::Iter<'a, N>;
    type EdgeIterator = EdgeIterator<'a, N>;

    fn is_empty(&self) -> bool {
        self.adjacency.is_empty()
    }

    fn order(&self) -> usize {
        self.adjacency.len()
    }

    fn size(&self) -> usize {
        self.edges.len()
    }

    fn nodes(&'a self) -> Self::NodeIterator {
        self.adjacency.keys()
    }

    fn has_node(&self, node: &N) -> bool {
        self.adjacency.contains_key(node)
    }

    fn neighbors(&'a self, node: &N) -> Result<Self::NeighborIterator, Error> {
        Ok(self.neighbors_for(node)?.iter())
    }

    fn degree(&self, node: &N) -> Result<usize, Error> {
        Ok(self.neighbors_for(node)?.len())
    }

    fn edges(&'a self) -> Self::EdgeIterator {
        EdgeIterator { iter: self.edges.iter() }
    }

    fn has_edge(&self, source: &N, target: &N) -> Result<bool, Error> {
        if self.neighbors_for(source)?.contains(target) {
            Ok(true)
        } else {
            if self.adjacency.contains_key(target) {
                Ok(false)
            } else {
                Err(Error::UnknownNode)
            }
        }
    }
}

pub struct EdgeIterator<'a, N> {
    iter: std::collections::hash_set::Iter<'a, (N, N)>
}

impl<'a, N: Eq+Hash+Clone> Iterator for EdgeIterator<'a, N> {
    type Item = (&'a N, &'a N);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|edge| (&edge.0, &edge.1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! map(
        { $($key:expr => $value:expr),+ } => {
            {
                let mut m = ::std::collections::HashMap::new();
                $(
                    m.insert($key, $value);
                )+
                m
            }
         };
    );

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
    fn build_given_unknown_target() {
        let nodes = vec![ Node::new(0), Node::new(1) ];
        let graph = HashGraph::build(map!{
            &nodes[0] => vec![ &nodes[1] ]
        });

        assert_eq!(graph.err(), Some(Error::UnknownNode));
    }

    #[test]
    fn build_given_self_target() {
        let nodes = vec![ Node::new(0) ];
        let graph = HashGraph::build(map!{
            &nodes[0] => vec![ &nodes[0] ]
        });

        assert_eq!(graph.err(), Some(Error::InvalidEdge));
    }

    #[test]
    fn build_given_duplicate_edge() {
        let nodes = vec![ Node::new(0), Node::new(1) ];
        let graph = HashGraph::build(map!{
            &nodes[0] => vec![ &nodes[1], &nodes[1] ],
            &nodes[1] => vec![ &nodes[0] ]
        });

        assert_eq!(graph.err(), Some(Error::DuplicateEdge));
    }

    #[test]
    fn is_empty_given_empty() {
        let graph = HashGraph::<()>::build(HashMap::new()).unwrap();

        assert!(graph.is_empty());
    }

    #[test]
    fn is_empty_given_p2() {
        let nodes = vec![ Node::new(0), Node::new(1) ];
        let graph = HashGraph::build(map!{
            &nodes[0] => vec![ &nodes[1] ],
            &nodes[1] => vec![ &nodes[0] ]
        }).unwrap();

        assert!(!graph.is_empty());
    }

    #[test]
    fn order_given_empty() {
        let graph = HashGraph::<()>::build(HashMap::new()).unwrap();

        assert_eq!(graph.order(), 0);
    }

    #[test]
    fn order_given_p1() {
        let nodes = vec![ Node::new(0) ];
        let graph = HashGraph::build(map!{
            &nodes[0] => vec![ ]
        }).unwrap();

        assert_eq!(graph.order(), 1);
    }

    #[test]
    fn size_given_empty() {
        let graph = HashGraph::<()>::build(HashMap::new()).unwrap();

        assert_eq!(graph.size(), 0);
    }

    #[test]
    fn size_given_p3() {
        let nodes = vec![ Node::new(0), Node::new(1), Node::new(2) ];
        let graph = HashGraph::build(map!{
            &nodes[0] => vec![ &nodes[1] ],
            &nodes[1] => vec![ &nodes[0], &nodes[2] ],
            &nodes[2] => vec![ &nodes[1] ]
        }).unwrap();

        assert_eq!(graph.size(), 2);
    }

    #[test]
    fn nodes_given_empty() {
        let graph = HashGraph::<()>::build(HashMap::new()).unwrap();
        let nodes = graph.nodes().collect::<Vec<_>>();

        assert!(nodes.is_empty());
    }

    #[test]
    fn nodes_given_p3() {
        let nodes = vec![ Node::new(0), Node::new(1), Node::new(2) ];
        let graph = HashGraph::build(map!{
            &nodes[0] => vec![ &nodes[1] ],
            &nodes[1] => vec![ &nodes[0], &nodes[2] ],
            &nodes[2] => vec![ &nodes[1] ]
        }).unwrap();

        assert_eq!(
            graph.nodes().cloned().collect::<HashSet<_>>(),
            nodes.iter().collect::<HashSet<_>>()
        )
    }

    #[test]
    fn has_node_given_outside() {
        let nodes = vec![ Node::new(0), Node::new(1) ];
        let graph = HashGraph::build(map!{
            &nodes[0] => vec![ ]
        }).unwrap();

        assert!(!graph.has_node(&&nodes[1]));
    }

    #[test]
    fn has_node_given_inside() {
        let nodes = vec![ Node::new(0)];
        let graph = HashGraph::build(map!{
            &nodes[0] => vec![ ]
        }).unwrap();

        assert!(graph.has_node(&&nodes[0]));
    }

    #[test]
    fn neighbors_given_outside() {
        let nodes = vec![ Node::new(0), Node::new(1) ];
        let graph = HashGraph::build(map!{
            &nodes[0] => vec![ ]
        }).unwrap();
        let result = graph.neighbors(&&nodes[1]);

        assert_eq!(result.err(), Some(Error::UnknownNode));
    }

    #[test]
    fn neighbors_given_p1() {
        let nodes = vec![ Node::new(0) ];
        let graph = HashGraph::build(map!{
            &nodes[0] => vec![ ]
        }).unwrap();
        let neighbors = graph.neighbors(&&nodes[0]).unwrap();

        assert!(neighbors.collect::<Vec<_>>().is_empty());
    }

    #[test]
    fn neighbors_given_p2() {
        let nodes = vec![ Node::new(0), Node::new(1) ];
        let graph = HashGraph::build(map!{
            &nodes[0] => vec![ &nodes[1] ],
            &nodes[1] => vec![ &nodes[0] ]
        }).unwrap();
        let neighbors = graph.neighbors(&&nodes[0]).unwrap();

        assert_eq!(neighbors.collect::<Vec<_>>(), vec![ &&nodes[1] ]);
    }

    #[test]
    fn neighbors_given_p3_secondary() {
        let nodes = vec![ Node::new(0), Node::new(1), Node::new(2) ];
        let graph = HashGraph::build(map!{
            &nodes[0] => vec![ &nodes[1] ],
            &nodes[1] => vec![ &nodes[0], &nodes[2] ],
            &nodes[2] => vec![ &nodes[1] ]
        }).unwrap();
        let neighbors = graph.neighbors(&&nodes[1]).unwrap();

        assert_eq!(neighbors.collect::<Vec<_>>(), vec![
            &&nodes[0], &&nodes[2]
        ]);
    }

    #[test]
    fn degree_given_outside() {
        let nodes = vec![ Node::new(0), Node::new(1) ];
        let graph = HashGraph::build(map!{
            &nodes[0] => vec![ ]
        }).unwrap();
        let result = graph.degree(&&nodes[1]);

        assert_eq!(result.err(), Some(Error::UnknownNode));
    }

    #[test]
    fn degree_given_p3_secondary() {
        let nodes = vec![ Node::new(0), Node::new(1), Node::new(2) ];
        let graph = HashGraph::build(map!{
            &nodes[0] => vec![ &nodes[1] ],
            &nodes[1] => vec![ &nodes[0], &nodes[2] ],
            &nodes[2] => vec![ &nodes[1] ]
        }).unwrap();

        assert_eq!(graph.degree(&&nodes[1]).unwrap(), 2);
    }

    #[test]
    fn edges_given_empty() {
        let graph = HashGraph::<()>::build(HashMap::new()).unwrap();
        let edges = graph.edges().collect::<Vec<_>>();

        assert!(edges.is_empty());
    }

    #[test]
    fn edges_given_p3_secondary() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);
        let n2 = &Node::new(2);
        let graph = HashGraph::build(map!{
            n0 => vec![ n1 ],
            n1 => vec![ n0, n2 ],
            n2 => vec![ n1 ]
        }).unwrap();
        let mut found = HashSet::new();
        let mut count = 0;

        for (source, target) in graph.edges() {
            found.insert((source, target));
            assert!(graph.has_edge(source, target).unwrap());

            count += 1;
        }

        assert_eq!(count, 2);
        assert_eq!(found.len(), 2);
    }

    #[test]
    fn has_edge_given_outside_source() {
        let nodes = vec![ Node::new(0), Node::new(1) ];
        let graph = HashGraph::build(map!{
            &nodes[0] => vec![ ]
        }).unwrap();
        let result = graph.has_edge(&&nodes[1], &&nodes[0]);

        assert_eq!(result.err(), Some(Error::UnknownNode));
    }

    #[test]
    fn has_edge_given_outside_target() {
        let nodes = vec![ Node::new(0), Node::new(1) ];
        let graph = HashGraph::build(map!{
            &nodes[0] => vec![ ]
        }).unwrap();
        let result = graph.has_edge(&&nodes[0], &&nodes[1]);

        assert_eq!(result.err(), Some(Error::UnknownNode));
    }

    #[test]
    fn has_edge_given_unconnnected() {
        let nodes = vec![ Node::new(0), Node::new(1) ];
        let graph = HashGraph::build(map!{
            &nodes[0] => vec![ ],
            &nodes[1] => vec![ ]
        }).unwrap();

        assert!(!graph.has_edge(&&nodes[0], &&nodes[1]).unwrap());
    }

    #[test]
    fn has_edge_given_connnected() {
        let nodes = vec![ Node::new(0), Node::new(1) ];
        let graph = HashGraph::build(map!{
            &nodes[0] => vec![ &nodes[1] ],
            &nodes[1] => vec![ &nodes[0] ]
        }).unwrap();

        assert!(graph.has_edge(&&nodes[0], &&nodes[1]).unwrap());
    }

    #[test]
    fn has_edge_given_connnected_and_reversed() {
        let nodes = vec![ Node::new(0), Node::new(1) ];
        let graph = HashGraph::build(map!{
            &nodes[0] => vec![ &nodes[1] ],
            &nodes[1] => vec![ &nodes[0] ]
        }).unwrap();

        assert!(graph.has_edge(&&nodes[1], &&nodes[0]).unwrap());
    }
}