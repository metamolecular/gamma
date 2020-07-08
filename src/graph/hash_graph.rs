use std::collections::HashMap;
use std::collections::hash_map::Entry;

use super::{ Graph, Error, Step };

/// A Graph backed by an adjacency map. Nodes will not necessarily be iterated
/// in numerical order, but all iteration orders are stable. As such,
/// HashGraph works well when extracting subgraphs from other graphs.
/// 
/// ```rust
/// use gamma::graph::{ Graph, HashGraph, Error, Step };
/// 
/// fn main() -> Result<(), Error> {
///     let c3 = HashGraph::from_traversal(0, vec![
///         Step::new(0, 1, false),
///         Step::new(1, 2, false),
///         Step::new(2, 0, true)
///     ])?;
/// 
///     assert_eq!(c3.nodes().to_vec(), vec![ 0, 1, 2 ]);
/// 
///     let result = HashGraph::from_traversal(0, vec![
///         Step::new(0, 1, false),
///         Step::new(1, 0, false)
///     ]);
/// 
///     assert_eq!(result, Err(Error::DuplicateEdge(1, 0)));
/// 
///     Ok(())
/// }
/// ```
/// 
#[derive(Debug, PartialEq)]
pub struct HashGraph {
    adjacency: HashMap<usize, Vec<usize>>,
    edges: Vec<(usize, usize)>,
    nodes: Vec<usize>
}

impl HashGraph {
    /// Builds from a traversal. Returns an error given:
    /// - a Step source has not been seen before
    /// - duplicate edge forward or reversed
    pub fn from_traversal(
        root: usize, steps: Vec<Step>
    ) -> Result<Self, Error> {
        let mut adjacency = HashMap::new();
        let mut edges = Vec::new();
        let mut nodes = Vec::new();

        adjacency.insert(root, vec![ ]);
        nodes.push(root);

        for step in steps {
            let Step { sid, tid, cut } = step;

            let neighbors = match adjacency.get_mut(&sid) {
                Some(neighbors) => neighbors,
                None => return Err(Error::MissingNode(sid))
            };

            neighbors.push(tid);

            match adjacency.entry(tid) {
                Entry::Occupied(occupied) => {
                    if cut {
                        if occupied.get().contains(&sid) {
                            return Err(Error::DuplicateEdge(sid, tid));
                        }
                    } else {
                        return Err(Error::DuplicateEdge(sid, tid));
                    }
                },
                Entry::Vacant(vacant) => {
                    vacant.insert(vec![ sid ]);
                }
            }

            edges.push((sid, tid));

            if !cut {
                nodes.push(tid);
            }
        }


        Ok(HashGraph { adjacency, edges, nodes })
    }

    /// Builds a node-induced subgraph from edges. Returns error given:
    /// - duplicate edge forward or reversed
    pub fn from_edges(
        edges: Vec<(usize, usize)>, singletons: Vec<usize>
    ) -> Result<Self, Error> {
        let mut nodes = Vec::new();
        let mut adjacency: HashMap<usize, Vec<usize>> = HashMap::new();

        for (sid, tid) in edges.iter() {
            match adjacency.entry(*sid) {
                Entry::Occupied(mut entry) => {
                    let neighbors = entry.get_mut();

                    if neighbors.contains(tid) {
                        return Err(Error::DuplicateEdge(*sid, *tid));
                    } else {
                        neighbors.push(*tid);
                    }
                },
                Entry::Vacant(entry) => {
                    entry.insert(vec![ *tid ]);
                    nodes.push(*sid);
                }
            }

            match adjacency.entry(*tid) {
                Entry::Occupied(mut entry) => {
                    let neighbors = entry.get_mut();

                    if neighbors.contains(sid) {
                        return Err(Error::DuplicateEdge(*sid, *tid));
                    } else {
                        neighbors.push(*sid);
                    }
                },
                Entry::Vacant(entry) => {
                        entry.insert(vec![ *sid ]);
                        nodes.push(*tid);
                }
            }
        }
        
        for singleton in singletons {
            match adjacency.entry(singleton) {
                Entry::Occupied(_) =>
                    return Err(Error::DuplicateNode(singleton)),
                Entry::Vacant(entry) => {
                    nodes.push(singleton);
                    entry.insert(vec![ ]);
                }
            }
        }

        Ok(HashGraph { nodes, edges, adjacency })
    }
}

impl Graph for HashGraph {
    fn is_empty(&self) -> bool {
        self.adjacency.is_empty()
    }

    fn order(&self) -> usize {
        self.adjacency.len()
    }

    fn size(&self) -> usize {
        self.edges.len()
    }

    fn nodes(&self) -> &[usize] {
        &self.nodes[..]
    }

    fn neighbors(&self, id: usize) -> Result<&[usize], Error> {
        match self.adjacency.get(&id) {
            Some(neighbors) => Ok(&neighbors[..]),
            None => Err(Error::MissingNode(id))
        }
    }
    
    fn has_node(&self, id: usize) -> bool {
        self.adjacency.contains_key(&id)
    }

    fn degree(&self, id: usize) -> Result<usize, Error> {
        Ok(self.neighbors(id)?.len())
    }

    fn edges(&self) -> &[(usize, usize)] {
        &self.edges[..]
    }

    fn has_edge(&self, sid: usize, tid: usize) -> Result<bool, Error> {
        let neighbors = self.neighbors(sid)?;

        if self.adjacency.contains_key(&tid) {
            Ok(neighbors.contains(&tid))
        } else {
            Err(Error::MissingNode(tid))
        }
    }
}

#[cfg(test)]
mod from_adjacency {
    use super::*;

    #[test]
    fn given_missing_source() {
        let graph = HashGraph::from_traversal(2, vec![
            Step::new(3, 4, false)
        ]);

        assert_eq!(graph, Err(Error::MissingNode(3)));
    }

    #[test]
    fn given_duplicate_target() {
        let graph = HashGraph::from_traversal(2, vec![
            Step::new(2, 5, false),
            Step::new(2, 5, false)
        ]);

        assert_eq!(graph, Err(Error::DuplicateEdge(2, 5)));
    }

    #[test]
    fn given_duplicate_target_reversed() {
        let graph = HashGraph::from_traversal(2, vec![
            Step::new(2, 5, false),
            Step::new(5, 2, false)
        ]);

        assert_eq!(graph, Err(Error::DuplicateEdge(5, 2)));
    }

    #[test]
    fn given_foo_back_edge_as_cut() {
        let graph = HashGraph::from_traversal(0, vec![
            Step::new(0, 1, false),
            Step::new(1, 0, true)
        ]);

        assert_eq!(graph, Err(Error::DuplicateEdge(1, 0)));
    }

    #[test]
    fn is_emtpy() {
        let graph = HashGraph::from_traversal(0, vec![ ]).unwrap();

        assert_eq!(graph.is_empty(), false);
    }

    #[test]
    fn order() {
        let graph = HashGraph::from_traversal(0, vec![
            Step::new(0, 1, false)
        ]).unwrap();

        assert_eq!(graph.order(), 2);
    }

    #[test]
    fn order_given_cut() {
        let graph = HashGraph::from_traversal(0, vec![
            Step::new(0, 1, false),
            Step::new(1, 2, false),
            Step::new(2, 0, true)
        ]).unwrap();

        assert_eq!(graph.order(), 3);
    }

    #[test]
    fn size() {
        let graph = HashGraph::from_traversal(0, vec![
            Step::new(0, 1, false),
            Step::new(0, 2, false)
        ]).unwrap();

        assert_eq!(graph.size(), 2);
    }

    #[test]
    fn size_given_cut() {
        let graph = HashGraph::from_traversal(0, vec![
            Step::new(0, 1, false),
            Step::new(1, 2, false),
            Step::new(2, 0, true)
        ]).unwrap();

        assert_eq!(graph.size(), 3);
    }

    #[test]
    fn nodes() {
        let graph = HashGraph::from_traversal(0, vec![
            Step::new(0, 1, false),
            Step::new(0, 2, false)
        ]).unwrap();

        assert_eq!(graph.nodes().to_vec(), vec![ 0, 1, 2 ]);
    }

    #[test]
    fn nodes_given_cut() {
        let graph = HashGraph::from_traversal(0, vec![
            Step::new(0, 1, false),
            Step::new(1, 2, false),
            Step::new(2, 0, true)
        ]).unwrap();

        assert_eq!(graph.nodes().to_vec(), vec![ 0, 1, 2 ]);
    }

    #[test]
    fn neighbors_given_outside() {
        let graph = HashGraph::from_traversal(0, vec![ ]).unwrap();

        assert_eq!(graph.neighbors(1), Err(Error::MissingNode(1)));
    }

    #[test]
    fn neighbors_given_p3_inner() {
        let graph = HashGraph::from_traversal(0, vec![
            Step::new(0, 1, false),
            Step::new(1, 2, false)
        ]).unwrap();

        assert_eq!(graph.neighbors(1).unwrap().to_vec(), vec![ 0, 2 ]);
    }

    #[test]
    fn has_node_given_outside() {
        let graph = HashGraph::from_traversal(0, vec![ ]).unwrap();

        assert_eq!(graph.has_node(1), false);
    }

    #[test]
    fn has_node_given_inside() {
        let graph = HashGraph::from_traversal(0, vec![ ]).unwrap();

        assert_eq!(graph.has_node(0), true);
    }

    #[test]
    fn degree_given_outside() {
        let graph = HashGraph::from_traversal(0, vec![ ]).unwrap();

        assert_eq!(graph.degree(1), Err(Error::MissingNode(1)));
    }

    #[test]
    fn edges() {
        let graph = HashGraph::from_traversal(0, vec![
            Step::new(0, 1, false),
            Step::new(1, 2, false),
            Step::new(2, 0, true)
        ]).unwrap();

        assert_eq!(graph.edges().to_vec(), vec![
            (0, 1),
            (1, 2),
            (2, 0)
        ]);
    }

    #[test]
    fn has_edge_give_source_outside() {
        let graph = HashGraph::from_traversal(0, vec![ ]).unwrap();

        assert_eq!(graph.has_edge(1, 0), Err(Error::MissingNode(1)));
    }

    #[test]
    fn has_edge_give_target_outside() {
        let graph = HashGraph::from_traversal(0, vec![ ]).unwrap();

        assert_eq!(graph.has_edge(0, 1), Err(Error::MissingNode(1)));
    }

    #[test]
    fn has_edge_given_unconnected() {
        let graph = HashGraph::from_traversal(0, vec![
            Step::new(0, 1, false),
            Step::new(1, 2, false)
        ]).unwrap();

        assert_eq!(graph.has_edge(0, 2), Ok(false));
    }

    #[test]
    fn has_edge_given_connected() {
        let graph = HashGraph::from_traversal(0, vec![
            Step::new(0, 1, false)
        ]).unwrap();

        assert_eq!(graph.has_edge(0, 1), Ok(true));
    }
}

#[cfg(test)]
mod from_edges {
    use super::*;

    #[test]
    fn given_duplicate_edge() {
        let graph = HashGraph::from_edges(vec![
            (0, 1),
            (0, 1)
        ], vec![ ]);

        assert_eq!(graph, Err(Error::DuplicateEdge(0, 1)));
    }

    #[test]
    fn given_duplicate_edge_reversed() {
        let graph = HashGraph::from_edges(vec![
            (0, 1),
            (1, 0)
        ], vec![ ]);

        assert_eq!(graph, Err(Error::DuplicateEdge(1, 0)));
    }

    #[test]
    fn given_duplicate_singleton() {
        let graph = HashGraph::from_edges(vec![
            (0, 1)
        ], vec![ 1 ]);

        assert_eq!(graph, Err(Error::DuplicateNode(1)));
    }

    #[test]
    fn is_empty() {
        let graph = HashGraph::from_edges(vec![
            (0, 1)
        ], vec![ ]).unwrap();

        assert_eq!(graph.is_empty(), false);
    }

    #[test]
    fn order() {
        let graph = HashGraph::from_edges(vec![
            (0, 1),
            (1, 2),
            (2, 3)
        ], vec![ ]).unwrap();

        assert_eq!(graph.order(), 4);
    }

    #[test]
    fn size() {
        let graph = HashGraph::from_edges(vec![
            (0, 1),
            (1, 2),
            (2, 3)
        ], vec![ ]).unwrap();

        assert_eq!(graph.size(), 3);
    }
    
    #[test]
    fn nodes() {
        let graph = HashGraph::from_edges(vec![
            (0, 1),
            (1, 2),
            (2, 3)
        ], vec![ ]).unwrap();

        assert_eq!(graph.nodes(), &[ 0, 1, 2, 3 ]);
    }

    #[test]
    fn nodes_given_singleton() {
        let graph = HashGraph::from_edges(vec![
            (0, 1)
        ], vec![ 2 ]).unwrap();

        assert_eq!(graph.nodes(), &[ 0, 1, 2 ]);
    }

    #[test]
    fn neighbors() {
        let graph = HashGraph::from_edges(vec![
            (0, 1),
            (1, 2),
            (2, 3)
        ], vec![ ]).unwrap();

        assert_eq!(graph.neighbors(1).unwrap(), &[ 0, 2 ]);
    }

    #[test]
    fn has_node() {
        let graph = HashGraph::from_edges(vec![
            (0, 1),
            (1, 2),
            (2, 3)
        ], vec![ ]).unwrap();

        assert_eq!(graph.has_node(9), false);
    }

    #[test]
    fn degree() {
        let graph = HashGraph::from_edges(vec![
            (0, 1),
            (1, 2),
            (2, 3)
        ], vec![ ]).unwrap();

        assert_eq!(graph.degree(1), Ok(2));
    }

    #[test]
    fn edges() {
        let graph = HashGraph::from_edges(vec![
            (0, 1),
            (2, 3),
            (1, 2)
        ], vec![ ]).unwrap();

        assert_eq!(graph.edges(), &[
            (0, 1),
            (2, 3),
            (1, 2)
        ]);
    }

    #[test]
    fn has_edge() {
        let graph = HashGraph::from_edges(vec![
            (0, 1),
            (1, 2),
            (2, 3)
        ], vec![ ]).unwrap();

        assert_eq!(graph.has_edge(2, 1), Ok(true));
    }
}