use std::collections::HashSet;

use crate::graph::{ Graph, Error };
use super::Step;

/// Implements a depth-first traversal as a Step Iterator.
/// 
/// ```rust
/// use std::convert::TryFrom;
/// 
/// use gamma::graph::{ Graph, Error, DefaultGraph };
/// use gamma::traversal::{ DepthFirst, Step };
/// 
/// fn main() -> Result<(), Error> {
///     let graph = DefaultGraph::try_from(vec![
///         vec![ 1, 3 ],
///         vec![ 0, 2 ],
///         vec![ 1, 3 ],
///         vec![ 2, 0 ]
///     ])?;
///     let traversal = DepthFirst::new(&graph, 0)?;
/// 
///     assert_eq!(traversal.collect::<Vec<_>>(), vec![
///         Step::new(0, 1, false),
///         Step::new(1, 2, false),
///         Step::new(2, 3, false),
///         Step::new(3, 0, true)
///     ]);
/// 
///     Ok(())
/// }
/// ```


/// Iterates edges of graph in depth-first order. To perform a depth-first
/// search, use the `depth_first` function instead.
#[derive(Debug,PartialEq)]
pub struct DepthFirst<'a, G> {
    nodes: HashSet<usize>,
    stack: Vec<(usize, usize)>,
    graph: &'a G
}

impl<'a, G: Graph> DepthFirst<'a, G> {
    pub fn new(graph: &'a G, root: usize) -> Result<Self, Error> {
        let mut nodes = HashSet::new();
        let mut stack = Vec::new();
    
        for neighbor in graph.neighbors(root)? {
            stack.push((root, *neighbor));
        }
    
        nodes.insert(root);
        stack.reverse();
    
        Ok(Self { nodes, stack, graph })
    }

    pub fn into_table(self) -> (Vec<usize>, Vec<(usize, usize)>) {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        for step in self {
            if nodes.is_empty() {
                nodes.push(step.sid);
            }

            if !step.cut {
                nodes.push(step.tid)
            }

            edges.push((step.sid, step.tid));
        }

        (nodes, edges)
    }
}

impl<'a, G> Iterator for DepthFirst<'a, G>
    where G: Graph {
    type Item = Step;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            None => None,
            Some((parent, node)) => {
                if self.nodes.contains(&node) {
                    Some(Step::new(parent, node, true))
                } else {
                    let neighbors = self.graph.neighbors(node).unwrap().to_vec();

                    for neighbor in neighbors.into_iter().rev() {
                        if neighbor == parent {
                            continue;
                        }

                        if self.nodes.contains(&neighbor) {
                            self.stack.retain(
                                |edge| edge.0 != neighbor && edge.1 != node
                            );
                        }

                        self.stack.push((node, neighbor));
                    }
    
                    self.nodes.insert(node);
    
                    Some(Step::new(parent, node, false))
                }
            }
        }
    }
}

#[cfg(test)]
mod into_table {
    use super::*;
    use std::convert::TryFrom;
    use crate::graph::DefaultGraph;

    #[test]
    fn p3() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();
        let traversal = DepthFirst::new(&graph, 0).unwrap();

        assert_eq!(traversal.into_table(), (vec![ 0, 1, 2 ], vec![
            (0, 1),
            (1, 2)
        ]))
    }

    #[test]
    fn c3() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1, 2 ],
            vec![ 0, 2 ],
            vec![ 1, 0 ]
        ]).unwrap();
        let traversal = DepthFirst::new(&graph, 0).unwrap();

        assert_eq!(traversal.into_table(), (vec![ 0, 1, 2 ], vec![
            (0, 1),
            (1, 2),
            (2, 0)
        ]))
    }

    #[test]
    fn s3() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0, 2, 3 ],
            vec![ 1 ],
            vec![ 1 ]
        ]).unwrap();
        let traversal = DepthFirst::new(&graph, 0).unwrap();

        assert_eq!(traversal.into_table(), (vec![ 0, 1, 2, 3 ], vec![
            (0, 1),
            (1, 2),
            (1, 3)
        ]));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use crate::graph::DefaultGraph;

    #[test]
    fn unknown_root() {
        let graph = DefaultGraph::new();
        let traversal = DepthFirst::new(&graph, 1);

        assert_eq!(traversal, Err(Error::MissingNode(1)));
    }

    #[test]
    fn p1() {
        let graph = DefaultGraph::try_from(vec![
            vec![ ]
        ]).unwrap();
        let traversal = DepthFirst::new(&graph, 0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![ ]);
    }

    #[test]
    fn p2() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0 ]
        ]).unwrap();
        let traversal = DepthFirst::new(&graph, 0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            Step::new(0, 1, false)
        ]);
    }

    #[test]
    fn p3() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();
        let traversal = DepthFirst::new(&graph, 0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            Step::new(0, 1, false),
            Step::new(1, 2, false)
        ]);
    }

    #[test]
    fn p3_inside() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();
        let traversal = DepthFirst::new(&graph, 1).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            Step::new(1, 0, false),
            Step::new(1, 2, false)
        ]);
    }

    #[test]
    fn p4() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1, 3 ],
            vec![ 2 ]
        ]).unwrap();
        let traversal = DepthFirst::new(&graph, 0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            Step::new(0, 1, false),
            Step::new(1, 2, false),
            Step::new(2, 3, false)
        ]);
    }

    #[test]
    fn c3() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1, 2 ],
            vec![ 0, 2 ],
            vec![ 1, 0 ]
        ]).unwrap();
        let traversal = DepthFirst::new(&graph, 0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            Step::new(0, 1, false),
            Step::new(1, 2, false),
            Step::new(2, 0, true)
        ]);
    }

    #[test]
    fn s3() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0, 2, 3 ],
            vec![ 1 ],
            vec![ 1 ]
        ]).unwrap();
        let traversal = DepthFirst::new(&graph, 0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            Step::new(0, 1, false),
            Step::new(1, 2, false),
            Step::new(1, 3, false)
        ]);
    }

    #[test]
    fn s3_inside() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0, 2, 3 ],
            vec![ 1 ],
            vec![ 1 ]
        ]).unwrap();
        let traversal = DepthFirst::new(&graph, 1).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            Step::new(1, 0, false),
            Step::new(1, 2, false),
            Step::new(1, 3, false)
        ]);
    }

    #[test]
    fn flower_from_stalk() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0, 2, 3 ],
            vec![ 1, 3 ],
            vec![ 2, 1 ]
        ]).unwrap();
        let traversal = DepthFirst::new(&graph, 0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            Step::new(0, 1, false),
            Step::new(1, 2, false),
            Step::new(2, 3, false),
            Step::new(3, 1, true)
        ]);
    }

    #[test]
    fn flower_with_cut_in_branch() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1, 2 ],
            vec![ 0, 2 ],
            vec![ 1, 0, 3 ],
            vec![ 2 ]
        ]).unwrap();
        let traversal = DepthFirst::new(&graph, 0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            Step::new(0, 1, false),
            Step::new(1, 2, false),
            Step::new(2, 0, true),
            Step::new(2, 3, false)
        ]);
    }

    #[test]
    fn blocked_branched_path() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0, 2, 3, 4 ],
            vec![ 1 ],
            vec![ 1, 4 ],
            vec![ 3, 1 ]
        ]).unwrap();
        let traversal = DepthFirst::new(&graph, 0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            Step::new(0, 1, false),
            Step::new(1, 2, false),
            Step::new(1, 3, false),
            Step::new(3, 4, false),
            Step::new(4, 1, true)
        ]);
    }

    #[test]
    fn bicyclo_220_with_cut_on_second_branching() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1, 5 ],
            vec![ 0, 2 ],
            vec![ 1, 5, 3 ],
            vec![ 2, 4 ],
            vec![ 3, 5 ],
            vec![ 2, 4, 0 ]
        ]).unwrap();
        let traversal = DepthFirst::new(&graph, 0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            Step::new(0, 1, false),
            Step::new(1, 2, false),
            Step::new(2, 5, false),
            Step::new(5, 4, false),
            Step::new(4, 3, false),
            Step::new(3, 2, true),
            Step::new(5, 0, true)
        ]);
    }

    #[test]
    fn bicyclo_210() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1, 2, 4 ],
            vec![ 0, 2 ],
            vec![ 1, 0, 3 ],
            vec![ 2, 4 ],
            vec![ 3, 0 ]
        ]).unwrap();
        let traversal = DepthFirst::new(&graph, 0).unwrap();
        
        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            Step::new(0, 1, false),
            Step::new(1, 2, false),
            Step::new(2, 0, true),
            Step::new(2, 3, false),
            Step::new(3, 4, false),
            Step::new(4, 0, true)
        ]);
    }

    #[test]
    fn cube() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1, 3, 4 ], // 0
            vec![ 0, 2, 5 ], // 1
            vec![ 1, 3, 6 ], // 2
            vec![ 2, 0, 7 ], // 3
            vec![ 5, 7, 0 ], // 4
            vec![ 4, 6, 1 ], // 5
            vec![ 5, 7, 2 ], // 6
            vec![ 6, 4, 3 ]  // 7
        ]).unwrap();
        let traversal = DepthFirst::new(&graph, 0).unwrap();
        
        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            Step::new(0, 1, false),
            Step::new(1, 2, false),
            Step::new(2, 3, false),
            Step::new(3, 0, true),
            Step::new(3, 7, false),
            Step::new(7, 6, false),
            Step::new(6, 5, false),
            Step::new(5, 4, false),
            Step::new(4, 7, true),
            Step::new(4, 0, true),
            Step::new(5, 1, true),
            Step::new(6, 2, true)
        ]);
    }
}