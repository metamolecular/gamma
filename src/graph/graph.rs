use crate::graph::Error;

/// An undirected graph.
pub trait Graph<'a, N: 'a> {
    type NodeIterator: Iterator<Item=&'a N>;
    type NeighborIterator: Iterator<Item=&'a N>;
    type EdgeIterator: Iterator<Item=(&'a N, &'a N)>;

    /// Returns true if there are no nodes, or false otherwise.
    fn is_empty(&self) -> bool;

    // /// Returns the number of nodes in this graph.
    fn order(&self) -> usize;

    // /// Returns the number of edges in this graph.
    fn size(&self) -> usize;

    // /// Iterates the nodes of this graph
    fn nodes(&'a self) -> Self::NodeIterator;

    // /// Returns true if node is a member, or false otherwise. 
    fn has_node(&self, node: &N) -> bool;

    // /// Iterates the neighbors of node.
    fn neighbors(&'a self, node: &N) -> Result<Self::NeighborIterator, Error>;

    // /// Returns the number of neighbors connected to node.
    fn degree(&self, node: &N) -> Result<usize, Error>;

    // /// Iterates the edges of this graph.
    fn edges(&'a self) -> Self::EdgeIterator;
    
    // /// Returns true if an edge exists between source and target.
    fn has_edge(&self, source: &N, target: &N) -> Result<bool, Error>;
}