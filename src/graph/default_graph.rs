use std::convert::TryFrom;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::cmp::PartialEq;

use super::{ Graph, Error };
use crate::traversal::DepthFirst;

/// An undirected Graph backed by an adjacency matrix. Nodes and neighbors are
/// iterated in the order in which they're added.
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
///     assert_eq!(c3.ids().collect::<Vec<_>>(), vec![ 0, 1, 2 ]);
/// 
///     assert_eq!(c3.add_edge(0, 1), Err(Error::DuplicateEdge(0, 1)));
/// 
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct DefaultGraph {
    indices: HashMap<usize, usize>,
    adjacency: Vec<Vec<usize>>,
    ids: Vec<usize>,
    edges: Vec<(usize, usize)>
}

impl DefaultGraph {
    pub fn new() -> Self {
        Self {
            indices: HashMap::new(),
            adjacency: Vec::new(),
            ids: Vec::new(),
            edges: Vec::new()
        }
    }

    pub fn add_node(&mut self, id: usize) -> Result<(), Error> {
        match self.indices.entry(id) {
            Entry::Occupied(_) => return Err(Error::DuplicateId(id)),
            Entry::Vacant(entry) => {
                entry.insert(self.ids.len());
            }
        }

        self.ids.push(id);
        self.adjacency.push(vec![ ]);

        Ok(())
    }

    pub fn add_edge(&mut self, sid: usize, tid: usize) -> Result<(), Error> {
        let &source_index = match self.indices.get(&sid) {
            Some(index) => index,
            None => return Err(Error::UnknownId(sid))
        };
        let &target_index = match self.indices.get(&tid) {
            Some(index) => index,
            None => return Err(Error::UnknownId(tid))
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
            None => Err(Error::UnknownId(id))
        }
    }
}

impl Graph for DefaultGraph {
    fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    fn order(&self) -> usize {
        self.ids.len()
    }

    fn size(&self) -> usize {
        self.edges.len()
    }

    fn ids(&self) -> Box<dyn Iterator<Item=usize> + '_> {
        Box::new(self.ids.iter().cloned())
    }

    fn neighbors(
        &self, id: usize
    ) -> Result<Box<dyn Iterator<Item=usize> + '_>, Error> {
        let index = self.index_for(id)?;

        Ok(Box::new(self.adjacency[index].iter().cloned()))
    }
    
    fn has_id(&self, id: usize) -> bool {
        self.indices.contains_key(&id)
    }

    fn degree(&self, id: usize) -> Result<usize, Error> {
        let index = self.index_for(id)?;

        Ok(self.adjacency[index].len())
    }

    fn edges(&self) -> Box<dyn Iterator<Item=(usize, usize)> + '_> {
        Box::new(self.edges.iter().cloned())
    }

    fn has_edge(&self, sid: usize, tid: usize) -> Result<bool, Error> {
        let index = self.index_for(sid)?;

        if self.indices.contains_key(&tid) {
            Ok(self.adjacency[index].contains(&tid))
        } else {
            return Err(Error::UnknownId(tid));
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
                    return Err(Error::UnknownId(tid));
                } else if neighbors[index+1..].contains(&tid) {
                    return Err(Error::DuplicateEdge(sid, tid));
                } else if !adjacency[tid].contains(&sid) {
                    return Err(Error::MissingEdge(tid, sid));
                }

                if sid < tid {
                    result.edges.push((sid, tid));
                }
            }

            result.ids.push(sid);
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

impl TryFrom<Vec<(usize, usize)>> for DefaultGraph {
    type Error = Error;

    fn try_from(edges: Vec<(usize, usize)>) -> Result<Self, Self::Error> {
        let mut result = DefaultGraph::new();

        for (sid, tid) in edges {
            if !result.has_id(sid) {
                result.add_node(sid)?;
            }

            if !result.has_id(tid) {
                result.add_node(tid)?;
            }

            result.add_edge(sid, tid)?;
        }

        Ok(result)
    }
}

impl PartialEq for DefaultGraph {
    fn eq(&self, other: &Self) -> bool {
        if self.size() != other.size() {
            return false;
        } else if self.order() != other.order() {
            return false;
        }

        for id in self.ids() {
            if !other.has_id(id) {
                return false;
            }
        }

        for (sid, tid) in self.edges() {
            match other.has_edge(sid, tid) {
                Ok(result) => {
                    if !result {
                        return false
                    }
                }, Err(_) => return false
            }
        }

        true
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

        assert_eq!(graph, Err(Error::UnknownId(1)))
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
mod try_from_edges {
    use super::*;

    #[test]
    fn duplicate_edge() {
        let graph = DefaultGraph::try_from(vec![
            (0, 1),
            (0, 1)
        ]);

        assert_eq!(graph, Err(Error::DuplicateEdge(0, 1)))
    }

    #[test]
    fn duplicate_edge_reverse() {
        let graph = DefaultGraph::try_from(vec![
            (0, 1),
            (1, 0)
        ]);

        assert_eq!(graph, Err(Error::DuplicateEdge(1, 0)))
    }

    #[test]
    fn valid() {
        let graph = DefaultGraph::try_from(vec![
            (0, 1),
            (1, 2),
            (3, 4)
        ]).unwrap();
        let mut expected = DefaultGraph::new();

        assert_eq!(expected.add_node(0), Ok(()));
        assert_eq!(expected.add_node(1), Ok(()));
        assert_eq!(expected.add_node(2), Ok(()));
        assert_eq!(expected.add_node(3), Ok(()));
        assert_eq!(expected.add_node(4), Ok(()));
        assert_eq!(expected.add_edge(0, 1), Ok(()));
        assert_eq!(expected.add_edge(1, 2), Ok(()));
        assert_eq!(expected.add_edge(3, 4), Ok(()));

        assert_eq!(graph, expected);
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

        assert_eq!(g2.edges().collect::<Vec<_>>(), [ (1, 0), (1, 2) ])
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

        assert_eq!(g2.edges().collect::<Vec<_>>(), [ (0, 1), (1, 2), (2, 0) ])
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

        assert_eq!(graph.add_node(0), Err(Error::DuplicateId(0)))
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

    #[test]
    fn missing_sid() {
        let mut graph = DefaultGraph::try_from(vec![
            vec![ ]
        ]).unwrap();

        assert_eq!(graph.add_edge(1, 0), Err(Error::UnknownId(1)))
    }

    #[test]
    fn missing_tid() {
        let mut graph = DefaultGraph::try_from(vec![
            vec![ ]
        ]).unwrap();

        assert_eq!(graph.add_edge(0, 1), Err(Error::UnknownId(1)))
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

        assert_eq!(graph.ids().collect::<Vec<_>>(), [ ])
    }

    #[test]
    fn p3() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();

        assert_eq!(graph.ids().collect::<Vec<_>>(), [ 0, 1, 2 ])
    }
}

#[cfg(test)]
mod neighbors {
    use super::*;

    #[test]
    fn given_outside() {
        let graph = DefaultGraph::new();

        assert_eq!(graph.neighbors(1).err(), Some(Error::UnknownId(1)))
    }

    #[test]
    fn given_inside_p3() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();

        assert_eq!(graph.neighbors(1).unwrap().collect::<Vec<_>>(), [ 0, 2 ])
    }
}

#[cfg(test)]
mod has_node {
    use super::*;

    #[test]
    fn given_outside() {
        let graph = DefaultGraph::new();

        assert_eq!(graph.has_id(0), false)
    }

    #[test]
    fn given_inside_p1() {
        let graph = DefaultGraph::try_from(vec![
            vec![ ]
        ]).unwrap();

        assert_eq!(graph.has_id(0), true)
    }
}

#[cfg(test)]
mod degree {
    use super::*;

    #[test]
    fn given_outside() {
        let graph = DefaultGraph::new();

        assert_eq!(graph.degree(0), Err(Error::UnknownId(0)))
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

        assert_eq!(graph.edges().collect::<Vec<_>>(), vec![ ])
    }

    #[test]
    fn p3() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();

        assert_eq!(graph.edges().collect::<Vec<_>>(), [ (0, 1), (1, 2) ])
    }
}

#[cfg(test)]
mod has_edge {
    use super::*;

    #[test]
    fn unk_unk() {
        let graph = DefaultGraph::new();

        assert_eq!(graph.has_edge(0, 1), Err(Error::UnknownId(0)))
    }

    #[test]
    fn sid_unk() {
        let graph = DefaultGraph::try_from(vec![
            vec![ ]
        ]).unwrap();

        assert_eq!(graph.has_edge(0, 1), Err(Error::UnknownId(1)))
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

#[cfg(test)]
mod eq {
    use super::*;

    #[test]
    fn c3_and_p3() {
        let c3 = DefaultGraph::try_from(vec![
            vec![ 1, 2 ],
            vec![ 0, 2 ],
            vec![ 1, 0 ]
        ]).unwrap();
        let p3 = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();

        assert_eq!(c3 == p3, false)
    }

    #[test]
    fn p2_and_p2_p1() {
        let p2 = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0 ]
        ]).unwrap();
        let p2_p1 = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0 ],
            vec![ ],
        ]).unwrap();

        assert_eq!(p2 == p2_p1, false)
    }

    #[test]
    fn p2_and_p2_reverse() {
        let g1 = DefaultGraph::try_from(vec![
            (0, 1)
        ]).unwrap();
        let g2 = DefaultGraph::try_from(vec![
            (1, 0)
        ]).unwrap();

        assert_eq!(g1 == g2, true)
    }

    #[test]
    fn p2_and_p2_different_ids() {
        let g1 = DefaultGraph::try_from(vec![
            (0, 1)
        ]).unwrap();
        let g2 = DefaultGraph::try_from(vec![
            (0, 2)
        ]).unwrap();

        assert_eq!(g1 == g2, false)
    }
}