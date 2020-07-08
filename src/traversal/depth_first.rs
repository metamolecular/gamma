use std::collections::HashSet;

use crate::graph::{ Graph, Error, Step };

/// Implements a depth-first traversal as a Step Iterator.
/// 
/// ```rust
/// use gamma::graph::{ Graph, Error, ArrayGraph, Step };
/// use gamma::traversal::depth_first;
/// 
/// fn main() -> Result<(), Error> {
///     let graph = ArrayGraph::from_adjacency(vec![
///         vec![ 1, 3 ],
///         vec![ 0, 2 ],
///         vec![ 1, 3 ],
///         vec![ 2, 0 ]
///     ])?;
///     let traversal = depth_first(&graph, 0)?;
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
pub fn depth_first<'a, G>(
    graph: &'a G, root: usize
) -> Result<DepthFirst<'a, G>, Error>
where G: Graph {
    let mut nodes = HashSet::new();
    let mut stack = Vec::new();

    for neighbor in graph.neighbors(root)? {
        stack.push((root, *neighbor));
    }

    nodes.insert(root);
    stack.reverse();

    Ok(DepthFirst { nodes, stack, graph })
}

/// Iterates edges of graph in depth-first order. To perform a depth-first
/// search, use the `depth_first` function instead.
pub struct DepthFirst<'a, G> {
    nodes: HashSet<usize>,
    stack: Vec<(usize, usize)>,
    graph: &'a G
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
                    // let neighbors = self.graph.neighbors(node).unwrap()
                        // .collect::<Vec<_>>();
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
mod tests {
    use super::*;
    use crate::graph::ArrayGraph;

    #[test]
    fn unknown_root() {
        let graph = ArrayGraph::new();

        assert_eq!(depth_first(&graph, 1).err().unwrap(), Error::MissingNode(1));
    }

    #[test]
    fn p1() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ ]
        ]).unwrap();
        let traversal = depth_first(&graph, 0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![ ]);
    }

    #[test]
    fn p2() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ 1 ],
            vec![ 0 ]
        ]).unwrap();
        let traversal = depth_first(&graph, 0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            Step::new(0, 1, false)
        ]);
    }

    #[test]
    fn p3() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();
        let traversal = depth_first(&graph, 0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            Step::new(0, 1, false),
            Step::new(1, 2, false)
        ]);
    }

    #[test]
    fn p3_inside() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();
        let traversal = depth_first(&graph, 1).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            Step::new(1, 0, false),
            Step::new(1, 2, false)
        ]);
    }

    #[test]
    fn p4() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1, 3 ],
            vec![ 2 ]
        ]).unwrap();
        let traversal = depth_first(&graph, 0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            Step::new(0, 1, false),
            Step::new(1, 2, false),
            Step::new(2, 3, false)
        ]);
    }

    #[test]
    fn c3() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ 1, 2 ],
            vec![ 0, 2 ],
            vec![ 1, 0 ]
        ]).unwrap();
        let traversal = depth_first(&graph, 0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            Step::new(0, 1, false),
            Step::new(1, 2, false),
            Step::new(2, 0, true)
        ]);
    }

    #[test]
    fn s3() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ 1 ],
            vec![ 0, 2, 3 ],
            vec![ 1 ],
            vec![ 1 ]
        ]).unwrap();
        let traversal = depth_first(&graph, 0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            Step::new(0, 1, false),
            Step::new(1, 2, false),
            Step::new(1, 3, false)
        ]);
    }

    #[test]
    fn s3_inside() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ 1 ],
            vec![ 0, 2, 3 ],
            vec![ 1 ],
            vec![ 1 ]
        ]).unwrap();
        let traversal = depth_first(&graph, 1).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            Step::new(1, 0, false),
            Step::new(1, 2, false),
            Step::new(1, 3, false)
        ]);
    }

    #[test]
    fn flower_from_stalk() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ 1 ],
            vec![ 0, 2, 3 ],
            vec![ 1, 3 ],
            vec![ 2, 1 ]
        ]).unwrap();
        let traversal = depth_first(&graph, 0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            Step::new(0, 1, false),
            Step::new(1, 2, false),
            Step::new(2, 3, false),
            Step::new(3, 1, true)
        ]);
    }

    #[test]
    fn flower_with_cut_in_branch() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ 1, 2 ],
            vec![ 0, 2 ],
            vec![ 1, 0, 3 ],
            vec![ 2 ]
        ]).unwrap();
        let traversal = depth_first(&graph, 0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            Step::new(0, 1, false),
            Step::new(1, 2, false),
            Step::new(2, 0, true),
            Step::new(2, 3, false)
        ]);
    }

    #[test]
    fn blocked_branched_path() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ 1 ],
            vec![ 0, 2, 3, 4 ],
            vec![ 1 ],
            vec![ 1, 4 ],
            vec![ 3, 1 ]
        ]).unwrap();
        let traversal = depth_first(&graph, 0).unwrap();

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
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ 1, 5 ],
            vec![ 0, 2 ],
            vec![ 1, 5, 3 ],
            vec![ 2, 4 ],
            vec![ 3, 5 ],
            vec![ 2, 4, 0 ]
        ]).unwrap();
        let traversal = depth_first(&graph, 0).unwrap();

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
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ 1, 2, 4 ],
            vec![ 0, 2 ],
            vec![ 1, 0, 3 ],
            vec![ 2, 4 ],
            vec![ 3, 0 ]
        ]).unwrap();
        let traversal = depth_first(&graph, 0).unwrap();

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
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ 1, 3, 4 ],
            vec![ 0, 2, 5 ],
            vec![ 1, 3, 6 ],
            vec![ 2, 0, 7 ],
            vec![ 5, 7, 0 ],
            vec![ 4, 6, 1 ],
            vec![ 5, 7, 2 ],
            vec![ 6, 4, 3 ]
        ]).unwrap();
        let traversal = depth_first(&graph, 0).unwrap();

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