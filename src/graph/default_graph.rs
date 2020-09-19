use std::convert::TryFrom;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use super::{ Graph, Error };
use crate::traversal::DepthFirst;

/// A Graph backed by an adjacency matrix. Nodes and neighbors are iterated in
/// the order in which they're added.
/// 
/// ```rust
/// use std::convert::TryFrom;
/// use gamma::graph::{ Graph, Error, DefaultGraph };
/// 
/// fn main() -> Result<(), Error> {
///     let mut c3 = DefaultGraph::try_from(vec![
///         vec![ 1 ],
///         vec![ 0, 2 ],
///         vec![ 1 ]
///     ])?;
/// 
///     assert_eq!(c3.nodes().to_vec(), vec![ 0, 1, 2 ]);
/// 
///     assert_eq!(c3.add_edge(0, 1), Err(Error::DuplicateEdge(0, 1)));
/// 
///     Ok(())
/// }
/// ```
#[derive(Debug,PartialEq)]
pub struct DefaultGraph {
    indices: HashMap<usize, usize>,
    adjacency: Vec<Vec<usize>>,
    nodes: Vec<usize>,
    edges: Vec<(usize, usize)>
}

impl DefaultGraph {
    pub fn new() -> Self {
        Self {
            indices: HashMap::new(),
            adjacency: Vec::new(),
            nodes: Vec::new(),
            edges: Vec::new()
        }
    }

    pub fn add_node(&mut self, id: usize) -> Result<(), Error> {
        match self.indices.entry(id) {
            Entry::Occupied(_) => return Err(Error::DuplicateNode(id)),
            Entry::Vacant(entry) => {
                entry.insert(self.nodes.len());
            }
        }

        self.nodes.push(id);
        self.adjacency.push(vec![ ]);

        Ok(())
    }

    pub fn add_edge(&mut self, sid: usize, tid: usize) -> Result<(), Error> {
        let &source_index = match self.indices.get(&sid) {
            Some(index) => index,
            None => unimplemented!()
        };
        let &target_index = match self.indices.get(&tid) {
            Some(index) => index,
            None => unimplemented!()
        };
        
        if self.adjacency[source_index].contains(&tid) {
            return Err(Error::DuplicateEdge(sid, tid));
        }
        
        self.adjacency[source_index].push(tid);
        self.adjacency[target_index].push(sid);
        self.edges.push((sid, tid));

        Ok(())
    }

    fn index_for(&self, id: usize) -> Result<usize, Error> {
        match self.indices.get(&id) {
            Some(index) => Ok(*index),
            None => Err(Error::MissingNode(id))
        }
    }
}

impl Graph for DefaultGraph {
    fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    fn order(&self) -> usize {
        self.nodes.len()
    }

    fn size(&self) -> usize {
        self.edges.len()
    }

    fn nodes(&self) -> &[usize] {
        &self.nodes[..]
    }

    fn neighbors(&self, id: usize) -> Result<&[usize], Error> {
        let index = self.index_for(id)?;

        Ok(&self.adjacency[index])
    }
    
    fn has_node(&self, id: usize) -> bool {
        self.indices.contains_key(&id)
    }

    fn degree(&self, id: usize) -> Result<usize, Error> {
        let index = self.index_for(id)?;

        Ok(self.adjacency[index].len())
    }

    fn edges(&self) -> &[(usize, usize)] {
        &self.edges[..]
    }

    fn has_edge(&self, sid: usize, tid: usize) -> Result<bool, Error> {
        let index = self.index_for(sid)?;

        if self.indices.contains_key(&tid) {
            Ok(self.adjacency[index].contains(&tid))
        } else {
            return Err(Error::MissingNode(tid));
        }
    }
}

impl TryFrom<Vec<Vec<usize>>> for DefaultGraph {
    type Error = Error;

    fn try_from(adjacency: Vec<Vec<usize>>) -> Result<Self, Self::Error> {
        let mut result = Self::new();

        for (sid, neighbors) in adjacency.iter().enumerate() {
            for (index, &tid) in neighbors.iter().enumerate() {
                if tid >= adjacency.len() {
                    return Err(Error::MissingNode(tid));
                } else if neighbors[index+1..].contains(&tid) {
                    return Err(Error::DuplicateEdge(sid, tid));
                } else if !adjacency[tid].contains(&sid) {
                    return Err(Error::MissingEdge(tid, sid));
                }

                if sid < tid {
                    result.edges.push((sid, tid));
                }
            }

            result.nodes.push(sid);
            result.indices.insert(sid, sid);
        }

        result.adjacency = adjacency;

        Ok(result)
    }
}

impl<'a, G: Graph> TryFrom<DepthFirst<'a, G>> for DefaultGraph {
    type Error = Error;

    fn try_from(traversal: DepthFirst<'a, G>) -> Result<Self, Self::Error> {
        let mut result = DefaultGraph::new();

        for step in traversal {
            if result.is_empty() {
                result.add_node(step.sid)?;
            }

            if !step.cut {
                result.add_node(step.tid)?;
            }

            result.add_edge(step.sid, step.tid)?;
        }

        Ok(result)
    }
}

#[cfg(test)]
mod try_from_adjacency {
    use super::*;

    #[test]
    fn missing_node() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ]
        ]);

        assert_eq!(graph, Err(Error::MissingNode(1)))
    }

    #[test]
    fn duplicate_edge() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1, 1 ],
            vec![ 0 ]
        ]);

        assert_eq!(graph, Err(Error::DuplicateEdge(0, 1)))
    }

    #[test]
    fn missing_edge() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ ]
        ]);

        assert_eq!(graph, Err(Error::MissingEdge(1, 0)))
    }
}

#[cfg(test)]
mod try_from_depth_first {
    use super::*;

    #[test]
    fn p3_internal() {
        let g1 = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();
        let traversal = DepthFirst::new(&g1, 1).unwrap();
        let g2 = DefaultGraph::try_from(traversal).unwrap();

        assert_eq!(g2.edges(), [ (1, 0), (1, 2) ])
    }

    #[test]
    fn c3() {
        let g1 = DefaultGraph::try_from(vec![
            vec![ 1, 2 ],
            vec![ 0, 2 ],
            vec![ 1, 0 ]
        ]).unwrap();
        let traversal = DepthFirst::new(&g1, 0).unwrap();
        let g2 = DefaultGraph::try_from(traversal).unwrap();

        assert_eq!(g2.edges(), [ (0, 1), (1, 2), (2, 0) ])
    }
}

#[cfg(test)]
mod add_node {
    use super::*;

    #[test]
    fn duplicate() {
        let mut graph = DefaultGraph::try_from(vec![
            vec![ ]
        ]).unwrap();

        assert_eq!(graph.add_node(0), Err(Error::DuplicateNode(0)))
    }
}

#[cfg(test)]
mod add_edge {
    use super::*;

    #[test]
    fn duplicate() {
        let mut graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0 ]
        ]).unwrap();

        assert_eq!(graph.add_edge(0, 1), Err(Error::DuplicateEdge(0, 1)))
    }

    #[test]
    fn duplicate_reverse() {
        let mut graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0 ]
        ]).unwrap();

        assert_eq!(graph.add_edge(1, 0), Err(Error::DuplicateEdge(1, 0)))
    }
}

#[cfg(test)]
mod is_empty {
    use super::*;

    #[test]
    fn p0() {
        let graph = DefaultGraph::new();

        assert_eq!(graph.is_empty(), true)
    }

    #[test]
    fn p1() {
        let graph = DefaultGraph::try_from(vec![
            vec![ ]
        ]).unwrap();

        assert_eq!(graph.is_empty(), false)
    }
}

#[cfg(test)]
mod order {
    use super::*;

    #[test]
    fn p0() {
        let graph = DefaultGraph::new();

        assert_eq!(graph.order(), 0)
    }

    #[test]
    fn p3() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();

        assert_eq!(graph.order(), 3)
    }
}

#[cfg(test)]
mod size {
    use super::*;

    #[test]
    fn p0() {
        let graph = DefaultGraph::new();

        assert_eq!(graph.size(), 0)
    }

    #[test]
    fn p3() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();

        assert_eq!(graph.size(), 2)
    }
}

#[cfg(test)]
mod nodes {
    use super::*;

    #[test]
    fn p0() {
        let graph = DefaultGraph::new();

        assert_eq!(graph.nodes(), [ ])
    }

    #[test]
    fn p3() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();

        assert_eq!(graph.nodes(), [ 0, 1, 2 ])
    }
}

#[cfg(test)]
mod neighbors {
    use super::*;

    #[test]
    fn given_outside() {
        let graph = DefaultGraph::new();

        assert_eq!(graph.neighbors(1), Err(Error::MissingNode(1)))
    }

    #[test]
    fn given_inside_p3() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();

        assert_eq!(graph.neighbors(1).unwrap(), [ 0, 2 ])
    }
}

#[cfg(test)]
mod has_node {
    use super::*;

    #[test]
    fn given_outside() {
        let graph = DefaultGraph::new();

        assert_eq!(graph.has_node(0), false)
    }

    #[test]
    fn given_inside_p1() {
        let graph = DefaultGraph::try_from(vec![
            vec![ ]
        ]).unwrap();

        assert_eq!(graph.has_node(0), true)
    }
}

#[cfg(test)]
mod degree {
    use super::*;

    #[test]
    fn given_outside() {
        let graph = DefaultGraph::new();

        assert_eq!(graph.degree(0), Err(Error::MissingNode(0)))
    }

    #[test]
    fn given_inside_p3() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();

        assert_eq!(graph.degree(1), Ok(2))
    }
}

#[cfg(test)]
mod edges {
    use super::*;

    #[test]
    fn p0() {
        let graph = DefaultGraph::new();

        assert_eq!(graph.edges().to_vec(), vec![ ])
    }

    #[test]
    fn p3() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();

        assert_eq!(graph.edges(), [ (0, 1), (1, 2) ])
    }
}

#[cfg(test)]
mod has_edge {
    use super::*;

    #[test]
    fn unk_unk() {
        let graph = DefaultGraph::new();

        assert_eq!(graph.has_edge(0, 1), Err(Error::MissingNode(0)))
    }

    #[test]
    fn sid_unk() {
        let graph = DefaultGraph::try_from(vec![
            vec![ ]
        ]).unwrap();

        assert_eq!(graph.has_edge(0, 1), Err(Error::MissingNode(1)))
    }

    #[test]
    fn sid_tid() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0 ]
        ]).unwrap();

        assert_eq!(graph.has_edge(0, 1), Ok(true))
    }

    #[test]
    fn tid_sid() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0 ]
        ]).unwrap();

        assert_eq!(graph.has_edge(1, 0), Ok(true))
    }
}