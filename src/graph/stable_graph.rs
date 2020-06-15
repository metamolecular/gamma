use std::collections::{ HashMap, HashSet };
use std::hash::Hash;
use std::ops::Deref;
use std::rc::Rc;

// TODO: remove the internal Rc. You don't need it.
// Add to documentation how to solve the problem with Rc.
// N must implement clone

use super::{ Graph, WeightedGraph, Error };

/// Implements an undirected, labeled, graph. Node, neighbor, and edge
/// iteration order are set by the build function and will remain stable.
/// 
/// ```
/// use gamma::graph::{ Graph, StableGraph};
/// 
/// let mut graph = StableGraph::build(vec![ 0, 1, 2 ], vec![
///     (0, 1, "a"),
///     (1, 2, "b")
/// ]
/// ).unwrap();
/// 
/// assert_eq!(graph.is_empty(), false);
/// assert_eq!(graph.order(), 3);
/// assert_eq!(graph.size(), 2);
/// assert_eq!(graph.nodes().collect::<Vec<_>>(), vec![ &0, &1, &2 ]);
/// assert_eq!(graph.has_node(&0), true);
/// assert_eq!(graph.neighbors(&1).unwrap().collect::<Vec<_>>(), vec![ &0, &2 ]);
/// assert_eq!(graph.degree(&1).unwrap(), 2);
/// assert_eq!(graph.edges().collect::<Vec<_>>(), vec![
///     (&0, &1), (&1, &2)
/// ]);
/// assert_eq!(graph.has_edge(&0, &1).unwrap(), true);
/// assert_eq!(graph.has_edge(&1, &0).unwrap(), true);
/// assert_eq!(graph.has_edge(&0, &2).unwrap(), false);
/// ```
pub struct StableGraph<N, E> {
    nodes: Vec<Rc<N>>,
    adjacency: HashMap<Rc<N>, Vec<(Rc<N>, Rc<E>)>>,
    edges: Vec<(Rc<N>, Rc<N>)>
}

impl<N: Eq+Hash, E> StableGraph<N, E> {
    pub fn build(
        node_list: Vec<N>,
        edge_list: Vec<(usize, usize, E)>
    ) -> Result<Self, Error> {
        let mut adjacency = HashMap::new();
        let mut nodes = vec![ ];
        let mut edges = vec![ ];

        for node in node_list {
            let rc = Rc::new(node);

            adjacency.insert(rc.clone(), Vec::new());
            nodes.push(rc);
        }

        let mut pairings = HashSet::new();

        for (sid, tid, weight) in edge_list {
            if pairings.contains(&(sid, tid)) {
                return Err(Error::DuplicatePairing(sid, tid));
            }

            let source = match nodes.get(sid) {
                Some(node) => node,
                None => return Err(Error::UnknownIndex(sid))
            };
            let target = match nodes.get(tid) {
                Some(node) => node,
                None => return Err(Error::UnknownIndex(tid))
            };
            let weight = Rc::new(weight);
        
            adjacency.get_mut(target).unwrap().push(
                (source.clone(), weight.clone()
            ));
            adjacency.get_mut(source).unwrap().push(
                (target.clone(), weight)
            );
            edges.push((source.clone(), target.clone()));
            pairings.insert((sid, tid));
            pairings.insert((tid, sid));
        }
            
        Ok(Self { nodes, adjacency, edges })
    }
}

impl<'a, N: 'a+Eq+Hash, E: 'a> Graph<'a, N> for StableGraph<N, E> {
    type NodeIterator = NodeIterator<'a, N>;
    type NeighborIterator = NeighborIterator<'a, N, E>;
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
        NodeIterator { inner: self.nodes.iter() }
    }

    fn has_node(&self, node: &N) -> bool {
        self.adjacency.contains_key(node)
    }

    fn neighbors(
        &'a self, node: &N
    ) -> Result<Self::NeighborIterator, Error> {
        match self.adjacency.get(node) {
            None => Err(Error::UnknownNode),
            Some(neighbors) => Ok(NeighborIterator {
                inner: neighbors.iter()
            })
        }
    }

    fn degree(&self, node: &N) -> Result<usize, Error> {
        match self.adjacency.get(node) {
            None => Err(Error::UnknownNode),
            Some(neighbors) => Ok(neighbors.len())
        }
    }

    fn has_edge(&self, source: &N, target: &N) -> Result<bool, Error> {
        match self.adjacency.get(source) {
            None => Err(Error::UnknownNode),
            Some(neighbors) => {
                let hit = neighbors.iter().find(|(neighbor, _)| {
                    Deref::deref(neighbor) == target
                });

                if hit.is_some() {
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
    }

    fn edges(&'a self) -> Self::EdgeIterator {
        EdgeIterator { inner: self.edges.iter() }
    }
}

impl<'a, N: 'a+Eq+Hash, E: 'a> WeightedGraph<'a, N, E> for StableGraph<N, E> {
    fn weight(
        &self, source: &'a N, target: &'a N
    ) -> Result<Option<&E>, Error> {
        match self.adjacency.get(source) {
            Some(neighbors) => {
                if self.adjacency.contains_key(target) {
                    for (neighbor, weight) in neighbors {
                        if Deref::deref(neighbor) == target {
                            return Ok(Some(Deref::deref(weight)));
                        }
                    }
    
                    Ok(None)
                } else {
                    Err(Error::UnknownNode)
                }
            },
            None => return Err(Error::UnknownNode)
        }
    }
}

pub struct NodeIterator<'a, N> {
    inner: std::slice::Iter<'a, std::rc::Rc<N>>
}

impl<'a, N> Iterator for NodeIterator<'a, N> {
    type Item = &'a N;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|rc| Deref::deref(rc))
    }
}

pub struct NeighborIterator<'a, N, E> {
    inner: std::slice::Iter<'a, (std::rc::Rc<N>, std::rc::Rc<E>)>
}

impl<'a, N, E> Iterator for NeighborIterator<'a, N, E> {
    type Item = &'a N;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(rc, _)| Deref::deref(rc))
    }
}

pub struct EdgeIterator<'a, N> {
    inner: std::slice::Iter<'a, (std::rc::Rc<N>, std::rc::Rc<N>)>
}

impl<'a, N: 'a> Iterator for EdgeIterator<'a, N> {
    type Item = (&'a N, &'a N);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(source, target)| {
            (Deref::deref(source), Deref::deref(target))
        })
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
    fn build_given_invalid_sid() {
        let graph = StableGraph::build(vec![
            Node::new(0)
        ], vec![
            (1, 0, ())
        ]);

        assert_eq!(graph.err(), Some(Error::UnknownIndex(1)));
    }

    #[test]
    fn build_given_invalid_tid() {
        let graph = StableGraph::build(vec![
            Node::new(0)
        ], vec![
            (0, 1, ())
        ]);

        assert_eq!(graph.err(), Some(Error::UnknownIndex(1)));
    }

    #[test]
    fn build_given_duplicate_pairing() {
        let graph = StableGraph::build(vec![
            Node::new(0), Node::new(1)
        ], vec![
            (0, 1, ()),
            (0, 1, ())
        ]);

        assert_eq!(graph.err(), Some(Error::DuplicatePairing(0, 1)));
    }

    #[test]
    fn build_given_duplicate_pairing_reversed() {
        let graph = StableGraph::build(vec![
            Node::new(0), Node::new(1)
        ], vec![
            (0, 1, ()),
            (1, 0, ())
        ]);

        assert_eq!(graph.err(), Some(Error::DuplicatePairing(1, 0)));
    }

    #[test]
    fn is_empty_given_empty() {
        let graph = StableGraph::<(), ()>::build(vec![ ], vec![ ]).unwrap();

        assert!(graph.is_empty());
    }

    #[test]
    fn is_empty_given_p1() {
        let graph = StableGraph::<_, ()>::build(vec![
            Node::new(0)
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
        let graph = StableGraph::<_, ()>::build(vec![
            Node::new(0)
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
        let graph = StableGraph::build(vec![
            Node::new(0), Node::new(1), Node::new(2)
         ], vec![
            (0, 1, ()),
            (1, 2, ())
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
        let graph = StableGraph::build(vec![
            Node::new(0), Node::new(1), Node::new(2)
        ], vec![
            (0, 1, ()),
            (1, 2, ())
        ]).unwrap();
        let nodes = graph.nodes().collect::<Vec<_>>();

        assert_eq!(nodes, vec![
            &Node::new(0), &Node::new(1), &Node::new(2)
        ]);
    }

    #[test]
    fn has_node_given_outside() {
        let graph = StableGraph::<_, ()>::build(vec![
            Node::new(0)
        ], vec![

        ]).unwrap();

        assert_eq!(graph.has_node(&Node::new(1)), false);
    }

    #[test]
    fn has_node_given_inside() {
        let graph = StableGraph::<_, ()>::build(vec![
            Node::new(0)
        ], vec![

        ]).unwrap();

        assert_eq!(graph.has_node(&Node::new(0)), true);
    }

    #[test]
    fn neighbors_given_outside() {
        let graph = StableGraph::<_, ()>::build(vec![
            Node::new(0)
        ], vec![ ]).unwrap();
        let neighbors = graph.neighbors(&Node::new(1));

        assert_eq!(neighbors.err(), Some(Error::UnknownNode));
    }

    #[test]
    fn neighbors_given_p1() {
        let graph = StableGraph::<_, ()>::build(vec![
            Node::new(0)
        ], vec![ ]).unwrap();
        let neighbors = graph.neighbors(&Node::new(0)).unwrap();

        assert!(neighbors.collect::<Vec<_>>().is_empty());
    }

    #[test]
    fn neighbors_given_p2() {
        let graph = StableGraph::build(vec![
            Node::new(0), Node::new(1)
        ], vec![
            (0, 1, ())
        ]).unwrap();
        let neighbors = graph.neighbors(&Node::new(0)).unwrap();

        assert_eq!(neighbors.collect::<Vec<_>>(), vec![ &Node::new(1) ]);
    }

    #[test]
    fn neighbors_given_p3_secondary() {
        let graph = StableGraph::build(vec![
            Node::new(0), Node::new(1), Node::new(2)
        ], vec![
            (0, 1, ()), (1, 2, ())
        ]).unwrap();
        let neighbors = graph.neighbors(&Node::new(1)).unwrap();

        assert_eq!(neighbors.collect::<Vec<_>>(), vec![
            &Node::new(0), &Node::new(2)
        ]);
    }

    #[test]
    fn degree_given_outside() {
        let graph = StableGraph::<_, ()>::build(vec![
            Node::new(0)
        ], vec![ ]).unwrap();
        let degree = graph.degree(&Node::new(1));

        assert_eq!(degree.err(), Some(Error::UnknownNode));
    }

    #[test]
    fn degree_given_p3_secondary() {
        let graph = StableGraph::build(vec![
            Node::new(0), Node::new(1), Node::new(2)
        ], vec![
            (0, 1, ()), (1, 2, ())
        ]).unwrap();

        assert_eq!(graph.degree(&Node::new(1)), Ok(2));
    }

    #[test]
    fn edges_given_empty() {
        let graph = StableGraph::<(), ()>::build(vec![ ], vec![ ]).unwrap();
        let edges = graph.edges().collect::<Vec<_>>();

        assert!(edges.is_empty());
    }

    #[test]
    fn edges_given_p3_secondary() {
        let graph = StableGraph::build(vec![
            Node::new(0), Node::new(1), Node::new(2)
        ], vec![
            (0, 1, ()), (1, 2, ())
        ]).unwrap();
        let edges = graph.edges().collect::<Vec<_>>();

        assert_eq!(edges, vec![
            (&Node::new(0), &Node::new(1)),
            (&Node::new(1), &Node::new(2))
        ]);
    }

    #[test]
    fn has_edge_given_outside_source() {
        let graph = StableGraph::<_, ()>::build(vec![
            Node::new(0)
        ], vec![ ]).unwrap();
        let result = graph.has_edge(&Node::new(2), &Node::new(0));

        assert_eq!(result.err(), Some(Error::UnknownNode));
    }

    #[test]
    fn has_edge_given_outside_target() {
        let graph = StableGraph::<_, ()>::build(vec![
            Node::new(0)
        ], vec![ ]).unwrap();
        let result = graph.has_edge(&Node::new(0), &Node::new(2));

        assert_eq!(result.err(), Some(Error::UnknownNode));
    }

    #[test]
    fn has_edge_given_unconnected() {
        let graph = StableGraph::<_, ()>::build(vec![
            Node::new(0), Node::new(1)
        ], vec![ ]).unwrap();

        assert!(!graph.has_edge(&Node::new(0), &Node::new(1)).unwrap());
    }

    #[test]
    fn has_edge_given_connected() {
        let graph = StableGraph::build(vec![
            Node::new(0), Node::new(1)
        ], vec![
            (0, 1, ())
        ]).unwrap();

        assert!(graph.has_edge(&Node::new(0), &Node::new(1)).unwrap());
    }

    #[test]
    fn has_edge_given_connected_and_reversed() {
        let graph = StableGraph::build(vec![
            Node::new(0), Node::new(1)
        ], vec![
            (0, 1, ())
        ]).unwrap();

        assert!(graph.has_edge(&Node::new(1), &Node::new(0)).unwrap());
    }

    #[test]
    fn weight_given_outside_source() {
        let graph = StableGraph::<_, ()>::build(vec![
            Node::new(0), Node::new(1)
        ], vec![ ]).unwrap();
        let weight = graph.weight(&Node::new(2), &Node::new(0));

        assert_eq!(weight.err(), Some(Error::UnknownNode));
    }

    #[test]
    fn weight_given_outside_target() {
        let graph = StableGraph::<_, ()>::build(vec![
            Node::new(0), Node::new(1)
        ], vec![ ]).unwrap();
        let weight = graph.weight(&Node::new(0), &Node::new(2));

        assert_eq!(weight.err(), Some(Error::UnknownNode));
    }

    #[test]
    fn weight_given_no_edge() {
        let graph = StableGraph::<_, ()>::build(vec![
            Node::new(0), Node::new(1)
        ], vec![ ]).unwrap();
        let weight = graph.weight(&Node::new(0), &Node::new(1));

        assert_eq!(weight, Ok(None));
    }

    #[test]
    fn weight_given_edge() {
        let graph = StableGraph::build(vec![
            Node::new(0), Node::new(1)
        ], vec![
            (0, 1, 42)
        ]).unwrap();
        let weight = graph.weight(&Node::new(0), &Node::new(1));

        assert_eq!(weight, Ok(Some(&42)));
    }

    #[test]
    fn weight_given_edge_reversed() {
        let graph = StableGraph::build(vec![
            Node::new(0), Node::new(1)
        ], vec![
            (0, 1, 42)
        ]).unwrap();
        let weight = graph.weight(&Node::new(1), &Node::new(0));

        assert_eq!(weight, Ok(Some(&42)));
    }
}