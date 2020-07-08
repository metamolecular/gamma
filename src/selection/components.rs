use std::collections::HashSet;

use crate::graph::{ Graph, HashGraph };
use crate::traversal::depth_first;

/// Returns the [connected components](https://en.wikipedia.org/wiki/Component_(graph_theory))
/// of a Graph as an Adjacency.
/// 
/// ```rust
/// use gamma::graph::{ Graph, Error, ArrayGraph, HashGraph, Step };
/// use gamma::selection::components;
/// 
/// fn main() -> Result<(), Error> {
///     let graph = ArrayGraph::from_adjacency(vec![
///         vec![ 1 ],
///         vec![ 0 ],
///         vec![ ]
///     ])?;
/// 
///     assert_eq!(components(&graph).collect::<Vec<_>>(), vec![
///         HashGraph::from_traversal(0, vec![
///             Step::new(0, 1, false)
///         ])?,
///         HashGraph::from_traversal(2, vec![ ])?
///     ]);
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
    type Item = HashGraph;

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

        let mut steps = Vec::new();
        let traversal = depth_first(self.graph, *root).expect(
            "unexpected error traversing graph"
        );

        for step in traversal {
            if !step.cut {
                self.visited.insert(step.tid);
            }

            steps.push(step);
        }

        let result = HashGraph::from_traversal(*root, steps).expect(
            "unexpected error building HashGraph"
        );
        
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{ ArrayGraph, Step };

    #[test]
    fn p1() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ ]
        ]).unwrap();
        let components = components(&graph).collect::<Vec<_>>();

        assert_eq!(components, vec![
            HashGraph::from_traversal(0, vec![ ]).unwrap()
        ]);
    }

    #[test]
    fn p1_p1() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ ],
            vec![ ]
        ]).unwrap();
        let components = components(&graph).collect::<Vec<_>>();

        assert_eq!(components, vec![
            HashGraph::from_traversal(0, vec![ ]).unwrap(),
            HashGraph::from_traversal(1, vec![ ]).unwrap()
        ]);
    }

    #[test]
    fn p2() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ 1 ],
            vec![ 0 ]
        ]).unwrap();
        let components = components(&graph).collect::<Vec<_>>();
        
        assert_eq!(components, vec![
            HashGraph::from_traversal(0, vec![
                Step::new(0, 1, false)
            ]).unwrap(),
        ]);
    }

    #[test]
    fn p2_p1() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ 1 ],
            vec![ 0 ],
            vec![ ],
        ]).unwrap();
        let components = components(&graph).collect::<Vec<_>>();
        
        assert_eq!(components, vec![
            HashGraph::from_traversal(0, vec![
                Step::new(0, 1, false)
            ]).unwrap(),
            HashGraph::from_traversal(2, vec![ ]).unwrap()
        ]);
    }

    #[test]
    fn p2_p2_p2() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ 1 ],
            vec![ 0 ],
            vec![ 3 ],
            vec![ 2 ],
            vec![ 5 ],
            vec![ 4 ]
        ]).unwrap();
        let components = components(&graph).collect::<Vec<_>>();
        
        assert_eq!(components, vec![
            HashGraph::from_traversal(0, vec![
                Step::new(0, 1, false)
            ]).unwrap(),
            HashGraph::from_traversal(2, vec![
                Step::new(2, 3, false)
            ]).unwrap(),
            HashGraph::from_traversal(4, vec![
                Step::new(4, 5, false)
            ]).unwrap()
        ]);
    }

    #[test]
    fn c3_p2() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ 1, 2 ],
            vec![ 0, 2 ],
            vec![ 0, 1 ],
            vec![ 4 ],
            vec![ 3 ]
        ]).unwrap();
        let components = components(&graph).collect::<Vec<_>>();

        assert_eq!(components, vec![
            HashGraph::from_traversal(0, vec![
                Step::new(0, 1, false),
                Step::new(1, 2, false),
                Step::new(2, 0, true)
            ]).unwrap(),
            HashGraph::from_traversal(3, vec![
                Step::new(3, 4, false)
            ]).unwrap()
        ]);
    }
}