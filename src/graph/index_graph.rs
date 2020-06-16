use super::{ Graph, Error };

/// Implements an undirected, unlabeled Graph. Node, neighbor, and edge
/// iterator order are set by the build function and will remain stable.
/// IndexGraph is intended for for use as a debugging tool where
/// stable ordering of nodes and especially neighbors/edges are helpful.
/// IndexGraph can also be used when precise neighbor iteraton order is
/// required. It may have other applications as well.
/// 
/// ```
/// use gamma::graph::{ Graph, IndexGraph, Error };
/// 
/// fn main() -> Result<(), Error> {
///     let mut graph = IndexGraph::build(vec![
///        vec![ 1 ],
///        vec![ 0, 2 ],
///        vec![ 1 ]
///     ])?;
/// 
///     assert_eq!(graph.degree(&1), Ok(2));
/// 
///     Ok(())
/// }
/// ```
pub struct IndexGraph {
    nodes: Vec<usize>,
    adjacency: Vec<Vec<usize>>,
    edges: Vec<(usize, usize)>
}

impl IndexGraph {
    /// Builds an IndexGraph from the supplied adjacency Vec. Each
    /// element of this Vec contains a Vec of neighbors. They will
    /// be iterated by IndexGraph#neighbors in the order given.
    /// 
    /// Errors will be returned given out-of-bounds indicies, duplicate
    /// edges, or missing back-edges (this is an indirected graph).
    pub fn build(
        adjacency: Vec<Vec<usize>>
    ) -> Result<Self, Error> {
        let nodes = (0..adjacency.len()).collect::<Vec<_>>();
        let mut edges = Vec::new();

        for (sid, tids) in adjacency.iter().enumerate() {            
            for (index, tid) in tids.iter().enumerate() {
                if *tid >= nodes.len() {
                    return Err(Error::UnknownIndex(*tid));
                } else if duplicate_after(tid, index, &tids) {
                    return Err(Error::DuplicatePairing(sid, *tid));
                }

                match adjacency.get(*tid) {
                    Some(sids) => {
                        if sids.contains(&sid) {
                            if sid < *tid {
                                edges.push((sid, *tid));
                            }
                        } else {
                            return Err(Error::MissingPairing(*tid, sid));
                        }
                    },
                    None => unimplemented!()
                }
            }
        }

        Ok(IndexGraph { nodes, adjacency, edges })
    }
}

impl<'a> Graph<'a, usize> for IndexGraph {
    type NodeIterator = std::slice::Iter<'a, usize>;
    type NeighborIterator = std::slice::Iter<'a, usize>;
    type EdgeIterator = EdgeIterator<'a>;

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

    fn has_node(&self, node: &usize) -> bool {
        self.nodes.len() > *node
    }

    fn neighbors(
        &'a self, node: &usize
    ) -> Result<Self::NeighborIterator, Error> {
        match self.adjacency.get(*node) {
            Some(neighbors) => Ok(neighbors.iter()),
            None => Err(Error::UnknownNode)
        }
    }

    fn degree(&self, node: &usize) -> Result<usize, Error> {
        match self.adjacency.get(*node) {
            Some(neighbors) => Ok(neighbors.len()),
            None => Err(Error::UnknownNode)
        }
    }

    fn has_edge(&self, source: &usize, target: &usize) -> Result<bool, Error> {
        match self.adjacency.get(*source) {
            None => Err(Error::UnknownNode),
            Some(neighbors) => {
                if neighbors.contains(target) {
                    Ok(true)
                } else {
                    if *target < self.nodes.len() {
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

pub struct EdgeIterator<'a> {
    inner: std::slice::Iter<'a, (usize, usize)>
}

impl<'a> Iterator for EdgeIterator<'a> {
    type Item = (&'a usize, &'a usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(sid, tid)| (sid, tid))
    }
}

fn duplicate_after(item: &usize, index: usize, items: &Vec<usize>) -> bool {
    items[index + 1..].contains(item)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_given_unknown_tid() {
        let graph = IndexGraph::build(vec![
            vec![ 1 ]
        ]);

        assert_eq!(graph.err(), Some(Error::UnknownIndex(1)));
    }

    #[test]
    fn build_given_unmatched_sid() {
        let graph = IndexGraph::build(vec![
            vec![ ],
            vec![ 0 ]
        ]);
        
        assert_eq!(graph.err(), Some(Error::MissingPairing(0, 1)));
    }

    #[test]
    fn build_given_unmatched_tid() {
        let graph = IndexGraph::build(vec![
            vec![ 1 ],
            vec![ ]
        ]);
        
        assert_eq!(graph.err(), Some(Error::MissingPairing(1, 0)));
    }

    #[test]
    fn build_given_duplicate_pairing() {
        let graph = IndexGraph::build(vec![
            vec![ 1, 1 ],
            vec![ 0 ]
        ]);

        assert_eq!(graph.err(), Some(Error::DuplicatePairing(0, 1)));
    }

    #[test]
    fn is_empty_given_empty() {
        let graph = IndexGraph::build(vec![ ]).unwrap();

        assert!(graph.is_empty());
    }

    #[test]
    fn is_empty_given_p1() {
        let graph = IndexGraph::build(vec![
            vec![ ]
        ]).unwrap();

        assert!(!graph.is_empty());
    }

    #[test]
    fn order_given_empty() {
        let graph = IndexGraph::build(vec![ ]).unwrap();

        assert_eq!(graph.order(), 0);
    }

    #[test]
    fn order_given_p1() {
        let graph = IndexGraph::build(vec![
            vec![ ]
        ]).unwrap();

        assert_eq!(graph.order(), 1);
    }

    #[test]
    fn size_given_empty() {
        let graph = IndexGraph::build(vec![
            vec![ ]
        ]).unwrap();

        assert_eq!(graph.size(), 0);
    }

    #[test]
    fn size_given_p3() {
        let graph = IndexGraph::build(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();

        assert_eq!(graph.size(), 2);
    }

    #[test]
    fn nodes_given_empty() {
        let graph = IndexGraph::build(vec![ ]).unwrap();
        let nodes = graph.nodes().collect::<Vec<_>>();

        assert_eq!(nodes.is_empty(), true);
    }

    #[test]
    fn nodes_given_p3() {
        let graph = IndexGraph::build(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();
        let nodes = graph.nodes().collect::<Vec<_>>();

        assert_eq!(nodes, vec![ &0, &1, &2 ]);
    }

    #[test]
    fn has_node_given_outside() {
        let graph = IndexGraph::build(vec![ ]).unwrap();

        assert!(!graph.has_node(&1));
    }

    #[test]
    fn has_node_given_inside() {
        let graph = IndexGraph::build(vec![
            vec![ ]
        ]).unwrap();

        assert!(graph.has_node(&0));
    }

    #[test]
    fn neighbors_given_outside() {
        let graph = IndexGraph::build(vec![ ]).unwrap();
        let neighbors = graph.neighbors(&1);

        assert_eq!(neighbors.err(), Some(Error::UnknownNode));
    }

    #[test]
    fn neighbors_given_p1() {
        let graph = IndexGraph::build(vec![
            vec![ ]
        ]).unwrap();
        let neighbors = graph.neighbors(&0).unwrap();

        assert!(neighbors.collect::<Vec<_>>().is_empty());
    }

    #[test]
    fn neighbors_given_p2() {
        let graph = IndexGraph::build(vec![
            vec![ 1 ],
            vec![ 0 ]
        ]).unwrap();
        let neighbors = graph.neighbors(&0).unwrap();

        assert_eq!(neighbors.collect::<Vec<_>>(), vec![ &1 ]);
    }

    #[test]
    fn neighbors_given_p3_secondary() {
        let graph = IndexGraph::build(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();
        let neighbors = graph.neighbors(&1).unwrap();

        assert_eq!(neighbors.collect::<Vec<_>>(), vec![ &0, &2 ]);
    }

    #[test]
    fn degree_given_outside() {
        let graph = IndexGraph::build(vec![
            vec![ ]
        ]).unwrap();
        let degree = graph.degree(&1);

        assert_eq!(degree.err(), Some(Error::UnknownNode));
    }

    #[test]
    fn degree_given_p3_secondary() {
        let graph = IndexGraph::build(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();

        assert_eq!(graph.degree(&1), Ok(2));
    }

    #[test]
    fn edges_given_empty() {
        let graph = IndexGraph::build(vec![ ]).unwrap();
        let edges = graph.edges().collect::<Vec<_>>();

        assert!(edges.is_empty());
    }

    #[test]
    fn edges_given_p3_secondary() {
        let graph = IndexGraph::build(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();
        let edges = graph.edges().collect::<Vec<_>>();

        assert_eq!(edges, vec![
            (&0, &1),
            (&1, &2)
        ]);
    }

    #[test]
    fn has_edge_given_outside_source() {
        let graph = IndexGraph::build(vec![
            vec![ ]
        ]).unwrap();
        let result = graph.has_edge(&1, &0);

        assert_eq!(result.err(), Some(Error::UnknownNode));
    }

    #[test]
    fn has_edge_given_outside_target() {
        let graph = IndexGraph::build(vec![
            vec![ ]
        ]).unwrap();
        let result = graph.has_edge(&0, &1);

        assert_eq!(result.err(), Some(Error::UnknownNode));
    }

    #[test]
    fn has_edge_given_unconnected() {
        let graph = IndexGraph::build(vec![
            vec![ ],
            vec![ ]
        ]).unwrap();

        assert!(!graph.has_edge(&0, &1).unwrap());
    }

    #[test]
    fn has_edge_given_connected() {
        let graph = IndexGraph::build(vec![
            vec![ 1 ],
            vec![ 0 ]
        ]).unwrap();

        assert!(graph.has_edge(&0, &1).unwrap());
    }

    #[test]
    fn has_edge_given_connected_and_reversed() {
        let graph = IndexGraph::build(vec![
            vec![ 1 ],
            vec![ 0 ]
        ]).unwrap();

        assert!(graph.has_edge(&1, &0).unwrap());
    }
}