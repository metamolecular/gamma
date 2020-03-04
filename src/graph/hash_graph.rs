use std::hash::Hash;

use indexmap::IndexMap;

use crate::graph::Error;
use crate::graph::Graph;
use crate::graph::WeightedGraph;

/// Implements an undirected, labeled, graph. Node, neighbor, and edge
/// iteration order are stable and set by build order.
/// 
/// ```
/// use graphcore::graph::Graph;
/// use graphcore::graph::HashGraph;
/// 
/// let mut graph = HashGraph::build(vec![ 0, 1, 2 ], vec![
///     (&0, &1, "a"),
///     (&1, &2, "b")
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
pub struct HashGraph<'a, N, E> {
    neighbors: IndexMap<N, Vec<&'a N>>,
    edges: Vec<(&'a N, &'a N)>,
    weights: IndexMap<(&'a N, &'a N), E>
}

impl<'a, N: Eq+Hash, E> HashGraph<'a, N, E> {
    pub fn new() -> Self {
        HashGraph {
            neighbors: IndexMap::new(),
            edges: Vec::new(),
            weights: IndexMap::new()
        }
    }

    pub fn build(nodes: Vec<N>, edges: Vec<(&'a N, &'a N, E)>) -> Result<Self, Error> {
        let mut result = HashGraph::new();

        for node in nodes {
            result.add_node(node)?;
        }

        for (source, target, weight) in edges {
            result.add_edge(source, target)?;
            result.weights.insert((source, target), weight);
        }

        Ok(result)
    }

    pub fn add_node(&mut self, node: N) -> Result<(), Error> {
        if self.neighbors.contains_key(&node) {
            Err(Error::DuplicateNode)
        } else {
            self.neighbors.insert(node, Vec::new());

            Ok(())
        }
    }

    pub fn add_edge(&mut self, source: &'a N, target: &'a N) -> Result<(), Error> {
        self.connect(source, target)?;
        self.connect(target, source)?;
        self.edges.push((source, target));

        Ok(())
    }

    fn connect(&mut self, source: &'a N, target: &'a N) -> Result<(), Error> {
        match self.neighbors.get_mut(source) {
            None => Err(Error::UnknownNode),
            Some(neighbors) => {
                if neighbors.contains(&target) {
                    Err(Error::DuplicateEdge)
                } else {
                    neighbors.push(target);
                    
                    Ok(())
                }
            }
        }
    }
}

impl<'a, N: Eq + Hash, E> Graph<'a, N> for HashGraph<'a, N, E> {
    type NodeIterator = indexmap::map::Keys<'a, N, Vec<&'a N>>;
    type NeighborIterator = std::iter::Cloned<std::slice::Iter<'a, &'a N>>;
    type EdgeIterator = std::iter::Cloned<std::slice::Iter<'a, (&'a N, &'a N)>>;

    fn is_empty(&self) -> bool {
        self.neighbors.is_empty()
    }

    fn order(&self) -> usize {
        self.neighbors.len()
    }
    
    fn size(&self) -> usize {
        self.edges.len()
    }
    
    fn nodes(&'a self) -> Self::NodeIterator {
        self.neighbors.keys()
    }

    fn has_node(&self, node: &N) -> bool {
        self.neighbors.contains_key(node)
    }

    fn neighbors(&'a self, node: &N) -> Result<Self::NeighborIterator, Error> {
        match self.neighbors.get(node) {
            None => Err(Error::UnknownNode),
            Some(neighbors) => Ok(neighbors.iter().cloned())
        }
    }

    fn degree(&self, node: &N) -> Result<usize, Error> {
        match self.neighbors.get(node) {
            None => Err(Error::UnknownNode),
            Some(neighbors) => Ok(neighbors.len())
        }
    }

    fn edges(&'a self) -> Self::EdgeIterator {
        self.edges.iter().cloned()
    }

    fn has_edge(&self, source: &N, target: &N) -> Result<bool, Error> {
        match self.neighbors.get(source) {
            None => Err(Error::UnknownNode),
            Some(neighbors) => {
                if neighbors.contains(&target) {
                    Ok(true)
                } else {
                    if self.neighbors.contains_key(target) {
                        Ok(false)
                    } else {
                        Err(Error::UnknownNode)
                    }
                }
            }
        }
    }
}

impl<'a, N: Eq + Hash, E> WeightedGraph<'a, N, E> for HashGraph<'a, N, E> {
    fn weight(
        &self, source: &'a N, target: &'a N
    ) -> Result<Option<&E>, Error> {
        match self.has_edge(source, target) {
            Err(error) => Err(error),
            Ok(flag) => {
                if flag {
                    match self.weights.get(&(source, target)) {
                        Some(weight) => Ok(Some(weight)),
                        None => {
                            match self.weights.get(&(target, source)) {
                                Some(weight) => Ok(Some(weight)),
                                None => { panic!("weight not found"); }
                            }
                        }
                    }
                } else {
                    Ok(None)
                }
            }
        }
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
    fn add_node_returns_error_given_member() {
        let n0 = &Node::new(0);
        let mut graph = HashGraph::<_, ()>::build(vec![ n0 ], vec![ ]).unwrap();

        assert_eq!(graph.add_node(n0), Err(Error::DuplicateNode));
    }

    #[test]
    fn add_edge_returns_error_given_unknown_source() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);
        let mut graph = HashGraph::<_, ()>::build(vec![ n1 ], vec![ ]).unwrap();

        assert_eq!(graph.add_edge(&n0, &n1), Err(Error::UnknownNode));
    }

    #[test]
    fn add_edge_returns_error_given_unknown_target() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);
        let mut graph = HashGraph::<_, ()>::build(vec![ n0 ], vec![ ]).unwrap();

        assert_eq!(graph.add_edge(&n0, &n1), Err(Error::UnknownNode));
    }

    #[test]
    fn is_empty_returns_true_by_default() {
        let graph = HashGraph::<(), ()>::new();

        assert_eq!(graph.is_empty(), true);
    }

    #[test]
    fn is_empty_returns_false_given_p1() {
        let n0 = &Node::new(0);
        let graph = HashGraph::<_, ()>::build(vec![ n0 ], vec![ ]).unwrap();

        assert_eq!(graph.is_empty(), false);
    }

    #[test]
    fn order_returns_zero_by_default() {
        let graph = HashGraph::<(), ()>::new();

        assert_eq!(graph.order(), 0);
    }

    #[test]
    fn size_returns_edge_count_given_p2() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);
        let graph = HashGraph::<_, ()>::build(vec![ n0, n1 ], vec![
            (&n0, &n1, ())
        ]).unwrap();


        assert_eq!(graph.size(), 1);
    }

    #[test]
    fn nodes_iterates_nothing_by_default() {
        let graph = HashGraph::<(), ()>::new();
        let nodes = graph.nodes().collect::<Vec<_>>();

        assert_eq!(nodes.is_empty(), true);
    }

    #[test]
    fn nodes_iterates_nodes_given_p3() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);
        let n2 = &Node::new(2);
        let graph = HashGraph::build(vec![ n0, n1, n2 ], vec![
            (&n0, &n1, ()),
            (&n1, &n2, ())
        ]).unwrap();
        let nodes = graph.nodes().collect::<Vec<_>>();

        assert_eq!(nodes, vec![ &n0, &n1, &n2 ]);
    }

    #[test]
    fn has_node_returns_false_given_unknown() {
        let n0 = &Node::new(0);
        let graph = HashGraph::<_, ()>::new();

        assert_eq!(graph.has_node(&n0), false);
    }

    #[test]
    fn has_node_returns_true_given_member() {
        let n0 = &Node::new(0);
        let graph = HashGraph::<_, ()>::build(vec![ &n0 ], vec![ ]).unwrap();

        assert_eq!(graph.has_node(&&n0), true);
    }

    #[test]
    fn neighbors_returns_error_given_unknown() {
        let n0 = &Node::new(0);
        let graph = HashGraph::<_, ()>::new();

        assert_eq!(graph.neighbors(&n0).err().unwrap(), Error::UnknownNode);
    }

    #[test]
    fn neighbors_iterates_nothing_given_p1() {
        let n0 = &Node::new(0);
        let graph = HashGraph::<_, ()>::build(vec![ n0 ], vec![ ]).unwrap();

        assert_eq!(graph.neighbors(&n0).unwrap().count(), 0);
    }

    #[test]
    fn neighbors_iterates_neighbor_given_p2() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);
        let graph = HashGraph::build(vec![ n0, n1 ], vec![
            (&n0, &n1, ())
        ]).unwrap();
        let neighbors = graph.neighbors(&n0).unwrap().collect::<Vec<_>>();

        assert_eq!(neighbors, vec![ &n1 ]);
    }

    #[test]
    fn neighbors_tierates_neighbors_given_p3_secondary() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);
        let n2 = &Node::new(2);
        let graph = HashGraph::build(vec![ n0, n1, n2 ], vec![
            (&n0, &n1, ()),
            (&n1, &n2, ())
        ]).unwrap();
        let neighbors = graph.neighbors(&n1).unwrap().collect::<Vec<_>>();

        assert_eq!(neighbors, vec![ &n0, &n2 ]);
    }

    #[test]
    fn degree_returns_error_given_unknown_node() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);
        let graph = HashGraph::<_, ()>::build(
            vec![ n0 ], vec![ ]
        ).unwrap();

        assert_eq!(graph.degree(&n1).err().unwrap(), Error::UnknownNode);
    }

    #[test]
    fn degree_returns_neighbor_count_given_p3_secondary() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);
        let n2 = &Node::new(2);
        let graph = HashGraph::build(vec![ n0, n1, n2 ], vec![
            (&n0, &n1, ()),
            (&n1, &n2, ())
        ]).unwrap();

        assert_eq!(graph.degree(&n1).unwrap(), 2);
    }

    #[test]
    fn edges_iterates_nothing_by_default() {
        let graph = HashGraph::<(), ()>::new();
        let edges = graph.edges().collect::<Vec<_>>();

        assert_eq!(edges.is_empty(), true);
    }

    #[test]
    fn edges_iterates_edges_given_s3() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);
        let n2 = &Node::new(2);
        let n3 = &Node::new(3);
        let graph = HashGraph::build(
            vec![ n0, n1, n2, n3 ], vec![
                (&n0, &n1, ()),
                (&n1, &n2, ()),
                (&n2, &n3, ())
            ]
        ).unwrap();
        let edges = graph.edges().collect::<Vec<_>>();

        assert_eq!(edges, vec![
            (&n0, &n1),
            (&n1, &n2),
            (&n2, &n3)
        ]);
    }

    #[test]
    fn has_edge_throws_given_unknown_source() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);
        let graph = HashGraph::<_, ()>::build(
            vec![ n1 ], vec![ ]
        ).unwrap();

        assert_eq!(graph.has_edge(&n0, &n1).err().unwrap(), Error::UnknownNode);
    }

    #[test]
    fn has_edge_throws_given_unknonw_target() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);
        let graph = HashGraph::<_, ()>::build(
            vec![ n0 ], vec![ ]
        ).unwrap();

        assert_eq!(graph.has_edge(&n0, &n1).err().unwrap(), Error::UnknownNode);
    }

    #[test]
    fn has_edge_returns_false_given_unconnected_members() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);
        let graph = HashGraph::<_, ()>::build(
            vec![ n0, n1 ], vec![ ]
        ).unwrap();

        assert_eq!(graph.has_edge(&n0, &n1).unwrap(), false);
    }

    #[test]
    fn has_edge_returns_true_given_connected_members() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);
        let graph = HashGraph::<_, ()>::build(
            vec![ n0, n1 ], vec![ (&n0, &n1, ()) ]
        ).unwrap();

        assert_eq!(graph.has_edge(&n0, &n1).unwrap(), true);
    }

    #[test]
    fn has_edge_returns_true_given_connected_and_reversed() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);
        let graph = HashGraph::<_, ()>::build(
            vec![ n0, n1 ], vec![ (&n0, &n1, ()) ]
        ).unwrap();

        assert_eq!(graph.has_edge(&n1, &n0).unwrap(), true);
    }

    #[test]
    fn weights_throws_given_unknown_source() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);
        let graph = HashGraph::<_, ()>::build(
            vec![ n1 ], vec![ ]
        ).unwrap();

        assert_eq!(graph.weight(&n0, &n1).err().unwrap(), Error::UnknownNode);
    }

    #[test]
    fn weights_throws_given_unknown_target() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);
        let graph = HashGraph::<_, ()>::build(
            vec![ n0 ], vec![ ]
        ).unwrap();

        assert_eq!(graph.weight(&n0, &n1).err().unwrap(), Error::UnknownNode);
    }

    #[test]
    fn weights_returns_none_given_no_edge() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);
        let graph = HashGraph::<_, ()>::build(
            vec![ n0, n1 ], vec![ ]
        ).unwrap();

        assert_eq!(graph.weight(&n0, &n1).unwrap(), None);
    }

    #[test]
    fn weights_returns_weight_given_edge() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);
        let graph = HashGraph::build(
            vec![ n0, n1 ], vec![ (&n0, &n1, 42) ]
        ).unwrap();

        assert_eq!(graph.weight(&n0, &n1).unwrap(), Some(&42));
    }

    #[test]
    fn weights_returns_weight_given_edge_reversed() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);
        let graph = HashGraph::build(
            vec![ n0, n1 ], vec![ (&n0, &n1, 42) ]
        ).unwrap();

        assert_eq!(graph.weight(&n1, &n0).unwrap(), Some(&42));
    }
}