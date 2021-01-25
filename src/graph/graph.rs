pub use super::error::Error;

/// An unweighted graph.
pub trait Graph {
    /// Returns true if there are no nodes, or false otherwise.
    fn is_empty(&self) -> bool;

    /// Returns the number of nodes in this graph.
    fn order(&self) -> usize;

    /// Returns the number of edges in this graph.
    fn size(&self) -> usize;

    /// Returns an Iterator over node identifiers.
    fn ids(&self) -> Box<dyn Iterator<Item=usize> + '_>;

    /// Returns an iterator over node identifiers for the neighbors at id,
    /// or Error if not found.
    fn neighbors(
        &self, id: usize
    ) -> Result<Box<dyn Iterator<Item=usize> + '_>, Error>;
    
    /// Returns true if id is a member, or false otherwise.
    fn has_id(&self, id: usize) -> bool;

    /// Returns the count of neighbors at id, or Error if id not found.
    fn degree(&self, id: usize) -> Result<usize, Error>;

    /// Returns an iterator over the edges of this graph.
    fn edges(&self) -> Box<dyn Iterator<Item=(usize, usize)> + '_>;

    /// Returns true if the edge (sid, tid) exists, or false otherwise.
    /// Returns Error if either sid or tid are not found.
    fn has_edge(&self, sid: usize, tid: usize) -> Result<bool, Error>;
}