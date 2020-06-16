use super::{ Graph, Error };

/// An undirected graph whose edges are arbitrarily weighted.
pub trait WeightedGraph<'a, N:'a, E> : Graph<'a, N> {
    /// Returns the weight between source and target.
    fn weight(&self, source: &'a N, target: &'a N) -> Result<Option<&E>, Error>;
}