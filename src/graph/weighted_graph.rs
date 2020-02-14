use crate::graph::Error;
use crate::graph::Graph;

pub trait WeightedGraph<'a, N:'a, E> : Graph<'a, N> {
    /// Returns the weight between source and target.
    fn weight(&self, source: &'a N, target: &'a N) -> Result<Option<&E>, Error>;
}