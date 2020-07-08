use super::graph::Graph;
use super::error::Error;

/// A Graph backed by an adjacency array. Nodes are generated and iterated
/// in sequence from 0 to order -1.
/// 
/// ```rust
/// // mandatory import of Graph
/// use gamma::graph::{ Graph, ArrayGraph, Error };
/// 
/// fn main() -> Result<(), Error> {
///     let c3 = ArrayGraph::from_adjacency(vec![
///         vec![ 1, 2 ],
///         vec![ 0, 2 ],
///         vec![ 1, 0 ]
///     ])?;
/// 
///     assert_eq!(c3.nodes().to_vec(), vec![ 0, 1, 2 ]);
/// 
///     let result = ArrayGraph::from_adjacency(vec![
///         vec![ 1 ]
///     ]);
/// 
///     assert_eq!(result, Err(Error::MissingNode(1)));
/// 
///     Ok(())
/// }
/// ```
#[derive(Debug, PartialEq)]
pub struct ArrayGraph {
    nodes: Vec<usize>,
    adjacency: Vec<Vec<usize>>,
    edges: Vec<(usize, usize)>
}

impl ArrayGraph {
    pub fn new() -> Self {
        ArrayGraph {
            adjacency: Vec::new(), nodes: Vec::new(), edges: Vec::new()
        }
    }

    /// Builds an ArrayGraph from an adjacency array. Each entry must list
    /// all neighbors.
    pub fn from_adjacency(entries: Vec<Vec<usize>>) -> Result<Self, Error> {
        let mut adjacency = Vec::new();
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        for sid in 0..entries.len() {
            let neighbors = &entries[sid];

            for index in 0..neighbors.len() {
                let tid = neighbors[index];

                if tid >= entries.len() {
                    return Err(Error::MissingNode(tid));
                } else if neighbors[index+1..].contains(&tid) {
                    return Err(Error::DuplicateEdge(sid, tid));
                } else if !entries[tid].contains(&sid) {
                    return Err(Error::MissingEdge(tid, sid));
                }

                if sid < tid {
                    edges.push((sid, tid));
                }
            }
            
            adjacency.push(neighbors.to_vec());
            nodes.push(sid);
        }
        
        Ok(ArrayGraph { adjacency, nodes, edges })
    }

    fn neighbors_at(&self, id: usize) -> Result<&[usize], Error> {
        match self.adjacency.get(id) {
            Some(neighbors) => Ok(neighbors),
            None => Err(Error::MissingNode(id))
        }
    }
}

impl Graph for ArrayGraph {
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
        self.neighbors_at(id)
    } 

    fn has_node(&self, id: usize) -> bool {
        id < self.nodes.len()
    }

    fn degree(&self, id: usize) -> Result<usize, Error> {
        Ok(self.neighbors_at(id)?.len())
    }

    fn edges(&self) -> &[(usize, usize)] {
        &self.edges[..]
    }

    fn has_edge(&self, sid: usize, tid: usize) -> Result<bool, Error> {
        if self.neighbors_at(sid)?.contains(&tid) {
            Ok(true)   
        } else if self.has_node(tid) {
            Ok(false)
        } else {
            Err(Error::MissingNode(tid))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_adjacency_given_missing_node() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ 1 ]
        ]);

        assert_eq!(graph, Err(Error::MissingNode(1)));
    }

    #[test]
    fn from_adjacency_given_missing_edge() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ 1 ],
            vec![ ]
        ]);

        assert_eq!(graph, Err(Error::MissingEdge(1, 0)));
    }

    #[test]
    fn from_adjacency_given_duplicate_edge() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ 1 ],
            vec![ 0, 0]
        ]);

        assert_eq!(graph, Err(Error::DuplicateEdge(1, 0)));
    }

    #[test]
    fn neighbors_given_p3_inner() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();

        assert_eq!(graph.neighbors(1).unwrap(), [ 0, 2 ]);
    }

    #[test]
    fn is_empty_given_empty() {
        let graph = ArrayGraph::new();

        assert_eq!(graph.is_empty(), true);
    }

    #[test]
    fn is_empty_given_p1() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ ]
        ]).unwrap();

        assert_eq!(graph.is_empty(), false);
    }

    #[test]
    fn order_given_empty() {
        let graph = ArrayGraph::new();

        assert_eq!(graph.order(), 0);
    }

    #[test]
    fn order_given_p1() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ ]
        ]).unwrap();

        assert_eq!(graph.order(), 1);
    }

    #[test]
    fn size_given_empty() {
        let graph = ArrayGraph::new();

        assert_eq!(graph.size(), 0);
    }

    #[test]
    fn size_given_p2() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();

        assert_eq!(graph.size(), 2);
    }

    #[test]
    fn nodes_given_empty() {
        let graph = ArrayGraph::new();

        assert_eq!(graph.nodes(), &[ ]);
    }

    #[test]
    fn nodes_given_p1() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ ]
        ]).unwrap();

        assert_eq!(graph.nodes(), &[ 0 ]);
    }

    #[test]
    fn has_node_given_outside() {
        let graph = ArrayGraph::new();

        assert_eq!(graph.has_node(0), false);
    }

    #[test]
    fn has_node_given_inside() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ ]
        ]).unwrap();

        assert_eq!(graph.has_node(0), true);
    }

    #[test]
    fn degree_given_outside() {
        let graph = ArrayGraph::new();

        assert_eq!(graph.degree(0), Err(Error::MissingNode(0)));
    }

    #[test]
    fn degree_given_inside() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();

        assert_eq!(graph.degree(1), Ok(2));
    }

    #[test]
    fn edges_given_empty() {
        let graph = ArrayGraph::new();

        assert_eq!(graph.edges(), &[ ]);
    }
    
    #[test]
    fn edges_give_p3() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();

        assert_eq!(graph.edges(), &[
            (0, 1),
            (1, 2)
        ]);
    }

    #[test]
    fn has_edge_given_sid_outside() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ ]
        ]).unwrap();

        assert_eq!(graph.has_edge(1, 0), Err(Error::MissingNode(1)));
    }
}
