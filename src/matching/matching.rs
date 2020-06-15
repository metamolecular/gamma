use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

use super::{ Error };
use crate::graph::{ Graph, Error as GraphError };

pub struct Matching<N> {
    nodes: HashSet<N>,
    edges: HashMap<N, N>
}

impl<N: Eq+Hash+Clone> Matching<N> {
    pub fn build(spec: Vec<(N, N)>) -> Result<Self, Error> {
        let mut nodes = HashSet::new();
        let mut edges = HashMap::new();

        for (source, target) in spec {
            if nodes.contains(&source) {
                return Err(Error::Foo);
            } else {
                nodes.insert(source.clone());
                nodes.insert(target.clone());
                edges.insert(source, target);
            }
        }

        Ok(Matching { nodes, edges })
    }
}

impl<'a, N: 'a+Eq+Hash> Graph<'a, N> for Matching<N> {
    type NodeIterator = std::collections::hash_set::Iter<'a, N>;
    type NeighborIterator = NeighborIterator<'a, N>;
    type EdgeIterator = std::collections::hash_map::Iter<'a, N, N>;

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
        self.nodes.contains(node)
    }

    fn neighbors(
        &'a self, node: &N
    ) -> Result<Self::NeighborIterator, GraphError> {
        match self.edges.get(node) {
            Some(node) => Ok(NeighborIterator {
                neighbor: node, done: false
            }),
            None => Err(GraphError::UnknownNode)
        }
    }

    fn degree(&self, node: &N) -> Result<usize, GraphError> {
        if self.nodes.contains(node) {
            Ok(1)
        } else {
            Err(GraphError::UnknownNode)
        }
    }

    fn edges(&'a self) -> Self::EdgeIterator {
        self.edges.iter()
    }

    fn has_edge(&self, source: &N, target: &N) -> Result<bool, GraphError> {
        match self.edges.get(source) {
            Some(mate) => {
                if mate == target {
                    Ok(true)
                } else if self.nodes.contains(target) {
                    Ok(false)
                } else {
                    Err(GraphError::UnknownNode)
                }
            },
            None => match self.edges.get(target) {
                Some(mate) => {
                    if mate == source {
                        Ok(true)
                    } else if self.nodes.contains(source) {
                        Ok(false)
                    } else {
                        Err(GraphError::UnknownNode)
                    }
                },
                None => Err(GraphError::UnknownNode)
            }
        }
    }
}

pub struct NeighborIterator<'a, N> {
    neighbor: &'a N,
    done: bool
}

impl<'a, N> Iterator for NeighborIterator<'a, N> {
    type Item = &'a N;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            self.done = true;

            Some(self.neighbor)
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

    macro_rules! set {
        ( $( $x:expr ), * ) => {
            {
                #[allow(unused_mut)]
                let mut set = HashSet::new();

                $(
                    set.insert($x);
                )*

                set
            }
        };
    }

    #[test]
    fn is_empty_given_empty() {
        let matching = Matching::<()>::build(vec![ ]).unwrap();

        assert!(matching.is_empty());
    }

    #[test]
    fn is_empty_given_pair() {
        let n0 = Node::new(0);
        let n1 = Node::new(1);

        let matching = Matching::build(vec![
            (&n0, &n1)
        ]).unwrap();

        assert!(!matching.is_empty());
    }

    #[test]
    fn order_given_empty() {
        let matching = Matching::<()>::build(vec![ ]).unwrap();

        assert_eq!(matching.order(), 0);
    }

    #[test]
    fn order_given_pair() {
        let n0 = Node::new(0);
        let n1 = Node::new(1);
        let matching = Matching::build(vec![
            (&n0, &n1)
        ]).unwrap();

        assert_eq!(matching.order(), 2);
    }

    #[test]
    fn size_given_empty() {
        let matching = Matching::<()>::build(vec![ ]).unwrap();

        assert_eq!(matching.size(), 0);
    }

    #[test]
    fn size_given_pair() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);
        let matching = Matching::build(vec![
            (n0, n1)
        ]).unwrap();

        assert_eq!(matching.size(), 1);
    }

    #[test]
    fn nodes_given_empty() {
        let matching = Matching::<()>::build(vec![ ]).unwrap();
        let nodes = matching.nodes().collect::<HashSet<_>>();

        assert_eq!(nodes, set![ ]);
    }

    #[test]
    fn nodes_given_pair() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);

        let matching = Matching::build(vec![
            (n0, n1)
        ]).unwrap();
        let nodes = matching.nodes().collect::<HashSet<_>>();

        assert_eq!(nodes, set![ &n0, &n1 ]);
    }

    #[test]
    fn has_node_given_outside() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);
        let n2 = &Node::new(2);

        let matching = Matching::build(vec![
            (n0, n1)
        ]).unwrap();

        assert!(!matching.has_node(&n2));
    }

    #[test]
    fn has_node_given_inside() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);

        let matching = Matching::build(vec![
            (n0, n1)
        ]).unwrap();

        assert!(matching.has_node(&n0));
    }

    #[test]
    fn neighbors_given_outside() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);
        let n2 = &Node::new(2);
        let matching = Matching::build(vec![
            (n0, n1)
        ]).unwrap();
        let neighbors = matching.neighbors(&n2);

        assert_eq!(neighbors.err(), Some(GraphError::UnknownNode));
    }

    #[test]
    fn neighbors_given_inside() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);
        let matching = Matching::build(vec![
            (n0, n1)
        ]).unwrap();
        let neighbors = matching.neighbors(&n0).unwrap();

        assert_eq!(neighbors.collect::<HashSet<_>>(), set![ &n1 ]);
    }

    #[test]
    fn degree_given_outside() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);
        let n2 = &Node::new(2);
        let matching = Matching::build(vec![
            (n0, n1)
        ]).unwrap();

        assert_eq!(matching.degree(&n2).err(), Some(GraphError::UnknownNode));
    }

    #[test]
    fn degree_given_inside() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);
        let matching = Matching::build(vec![
            (n0, n1)
        ]).unwrap();

        assert_eq!(matching.degree(&n0), Ok(1));
    }

    #[test]
    fn edges_given_empty() {
        let matching = Matching::<()>::build(vec![ ]).unwrap();
        let edges = matching.edges().collect::<HashSet<_>>();

        assert_eq!(edges, set![ ]);
    }

    #[test]
    fn edges_given_two() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);
        let n2 = &Node::new(2);
        let n3 = &Node::new(3);
        let matching = Matching::build(vec![
            (n0, n1),
            (n2, n3)
        ]).unwrap();
        let edges = matching.edges().collect::<HashSet<_>>();

        assert_eq!(edges, set![
            (&n0, &n1),
            (&n2, &n3)
        ]);
    }

    #[test]
    fn has_edge_given_outside_source() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);
        let n2 = &Node::new(2);
        let matching = Matching::build(vec![
            (n0, n1)
        ]).unwrap();
        let result = matching.has_edge(&n2, &n0);

        assert_eq!(result.err(), Some(GraphError::UnknownNode));
    }

    #[test]
    fn has_edge_given_outside_target() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);
        let n2 = &Node::new(2);
        let matching = Matching::build(vec![
            (n0, n1)
        ]).unwrap();
        let result = matching.has_edge(&n0, &n2);

        assert_eq!(result.err(), Some(GraphError::UnknownNode));
    }

    #[test]
    fn has_edge_given_unconnected() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);
        let n2 = &Node::new(2);
        let n3 = &Node::new(3);
        let matching = Matching::build(vec![
            (n0, n1),
            (n2, n3)
        ]).unwrap();

        assert_eq!(matching.has_edge(&n0, &n2), Ok(false));
    }

    #[test]
    fn has_edge_given_connected() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);
        let matching = Matching::build(vec![
            (n0, n1)
        ]).unwrap();

        assert_eq!(matching.has_edge(&n0, &n1), Ok(true));
    }

    #[test]
    fn has_edge_given_connected_reverse() {
        let n0 = &Node::new(0);
        let n1 = &Node::new(1);
        let matching = Matching::build(vec![
            (n0, n1)
        ]).unwrap();

        assert_eq!(matching.has_edge(&n1, &n0), Ok(true));
    }
}