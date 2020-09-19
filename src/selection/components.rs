use std::convert::TryFrom;
use std::collections::HashSet;

use crate::graph::{ Graph, DefaultGraph };
use crate::traversal::{ DepthFirst };

/// Returns the [connected components](https://en.wikipedia.org/wiki/Component_(graph_theory))
/// of a Graph as an Adjacency.
/// 
/// ```rust
/// use std::convert::TryFrom;
/// 
/// use gamma::graph::{ Graph, Error, DefaultGraph };
/// use gamma::selection::components;
/// 
/// fn main() -> Result<(), Error> {
///     let graph = DefaultGraph::try_from(vec![
///         vec![ 1 ],
///         vec![ 0 ],
///         vec![ ]
///     ])?;
///     let mut c1 = DefaultGraph::new();
///     let mut c2 = DefaultGraph::new();
/// 
///     c1.add_node(0)?;
///     c1.add_node(1)?;
///     c1.add_edge(0, 1)?;
/// 
///     c2.add_node(2)?;
/// 
///     assert_eq!(components(&graph).collect::<Vec<_>>(), vec![ c1, c2 ]);
/// 
///     Ok(())
/// }
/// ```
pub fn components<'a, G: Graph>(
    graph: &'a G
) -> Components<'a, G> {
    Components {
        visited: HashSet::new(),
        iter: graph.nodes().iter(),
        graph: graph
    }
}

pub struct Components<'a, G: Graph> {
    visited: HashSet<usize>,
    iter: std::slice::Iter<'a, usize>,
    graph: &'a G
}

impl<'a, G: Graph> Iterator for Components<'a, G> {
    type Item = DefaultGraph;

    fn next(&mut self) -> Option<Self::Item> {
        let root = loop {
            match self.iter.next() {
                Some(root) => {
                    if !self.visited.contains(root) {
                        break root;
                    }
                },
                None => return None
            }
        };

        self.visited.insert(*root);

        let traversal = DepthFirst::new(self.graph, *root).expect(
            "root not found"
        );
        let mut component = DefaultGraph::try_from(traversal).expect(
            "traversal error"
        );

        if component.is_empty() {
            component.add_node(*root).expect("add root to empty graph");
        } else {
            for id in component.nodes() {
                self.visited.insert(*id);
            }
        }

        Some(component)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let graph = DefaultGraph::try_from(vec![
            vec![ ]
        ]).unwrap();
        let components = components(&graph).collect::<Vec<_>>();

        assert_eq!(components, vec![ graph ])
    }

    #[test]
    fn p1_p1() {
        let graph = DefaultGraph::try_from(vec![
            vec![ ],
            vec![ ]
        ]).unwrap();
        let components = components(&graph).collect::<Vec<_>>();
        let mut c1 = DefaultGraph::new();
        let mut c2 = DefaultGraph::new();

        assert_eq!(c1.add_node(0), Ok(()));
        assert_eq!(c2.add_node(1), Ok(()));

        assert_eq!(components, vec![ c1, c2 ])
    }

    #[test]
    fn p2() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0 ]
        ]).unwrap();
        let components = components(&graph).collect::<Vec<_>>();
        
        assert_eq!(components, vec![ graph ])
    }

    #[test]
    fn p2_p1() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0 ],
            vec![ ],
        ]).unwrap();
        let components = components(&graph).collect::<Vec<_>>();
        let mut c1 = DefaultGraph::new();
        let mut c2 = DefaultGraph::new();

        assert_eq!(c1.add_node(0), Ok(()));
        assert_eq!(c1.add_node(1), Ok(()));
        assert_eq!(c1.add_edge(0, 1), Ok(()));

        assert_eq!(c2.add_node(2), Ok(()));

        assert_eq!(components, vec![c1, c2 ])
    }

    #[test]
    fn p2_p2_p2() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0 ],
            vec![ 3 ],
            vec![ 2 ],
            vec![ 5 ],
            vec![ 4 ]
        ]).unwrap();
        let components = components(&graph).collect::<Vec<_>>();
        let mut c1 = DefaultGraph::new();
        let mut c2 = DefaultGraph::new();
        let mut c3 = DefaultGraph::new();

        assert_eq!(c1.add_node(0), Ok(()));
        assert_eq!(c1.add_node(1), Ok(()));
        assert_eq!(c1.add_edge(0, 1), Ok(()));

        assert_eq!(c2.add_node(2), Ok(()));
        assert_eq!(c2.add_node(3), Ok(()));
        assert_eq!(c2.add_edge(2, 3), Ok(()));

        assert_eq!(c3.add_node(4), Ok(()));
        assert_eq!(c3.add_node(5), Ok(()));
        assert_eq!(c3.add_edge(4, 5), Ok(()));

        assert_eq!(components, vec![c1, c2, c3 ]);
    }

    #[test]
    fn c3_p2() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1, 2 ],
            vec![ 0, 2 ],
            vec![ 0, 1 ],
            vec![ 4 ],
            vec![ 3 ]
        ]).unwrap();
        let components = components(&graph).collect::<Vec<_>>();
        let mut c1 = DefaultGraph::new();
        let mut c2 = DefaultGraph::new();

        assert_eq!(c1.add_node(0), Ok(()));
        assert_eq!(c1.add_node(1), Ok(()));
        assert_eq!(c1.add_node(2), Ok(()));
        assert_eq!(c1.add_edge(0, 1), Ok(()));
        assert_eq!(c1.add_edge(1, 2), Ok(()));
        assert_eq!(c1.add_edge(2, 0), Ok(()));

        assert_eq!(c2.add_node(3), Ok(()));
        assert_eq!(c2.add_node(4), Ok(()));
        assert_eq!(c2.add_edge(3, 4), Ok(()));

        assert_eq!(components, vec![ c1, c2 ])
    }
}