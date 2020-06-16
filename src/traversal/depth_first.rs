use std::collections::HashSet;
use std::hash::Hash;

use crate::graph::{ Graph, Error };

/// Implements a depth-first traversal as an edge Iterator. Reports cycle
/// closure edges.
/// 
/// ```
/// use gamma::graph::{ Graph, IndexGraph, Error };
/// use gamma::traversal::depth_first;
/// 
/// fn main() -> Result<(), Error> {
///     let graph = IndexGraph::build(vec![
///         vec![ 1, 2 ],
///         vec![ 0, 2 ],
///         vec![ 1, 0 ]
///     ])?;
///     let traversal = depth_first(&graph, &0)?;
///
///     assert_eq!(traversal.collect::<Vec<_>>(), vec![
///         (&0, &1, false),
///         (&1, &2, false),
///         (&2, &0, true)
///     ]);
/// 
///     Ok(())
/// }
/// ```
pub fn depth_first<'a, N, G>(
    graph: &'a G, root: &'a N
) -> Result<DepthFirst<'a, N, G>, Error>
where G: Graph<'a, N>, N: 'a + Hash + Eq {
    let mut nodes = HashSet::new();
    let mut stack = Vec::new();

    for neighbor in graph.neighbors(root)? {
        stack.push((root, neighbor));
    }

    nodes.insert(root);
    stack.reverse();

    Ok(DepthFirst { nodes, stack, graph })
}

/// Iterates edges of graph in depth-first order. To perform a depth-first
/// search, use the `depth_first` function instead.
pub struct DepthFirst<'a, N, G> {
    nodes: HashSet<&'a N>,
    stack: Vec<(&'a N, &'a N)>,
    graph: &'a G
}

impl<'a, N, G> Iterator for DepthFirst<'a, N, G>
    where N: Eq + Hash, G: Graph<'a, N> {
    type Item = (&'a N, &'a N, bool);

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            None => None,
            Some((parent, node)) => {
                if self.nodes.contains(node) {
                    Some((parent, node, true))
                } else {
                    let neighbors = self.graph.neighbors(node).unwrap()
                        .collect::<Vec<_>>();

                    for neighbor in neighbors.into_iter().rev() {
                        if neighbor == parent {
                            continue;
                        }

                        if self.nodes.contains(neighbor) {
                            self.stack.retain(
                                |edge| edge.0 != neighbor && edge.1 != node
                            );
                        }

                        self.stack.push((node, neighbor));
                    }
    
                    self.nodes.insert(node);
    
                    Some((parent, node, false))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::IndexGraph;

    #[test]
    fn unknown_root() {
        let graph = IndexGraph::build(vec![ ]).unwrap();

        assert_eq!(depth_first(&graph, &1).err().unwrap(), Error::UnknownNode);
    }

    #[test]
    fn p1() {
        let graph = IndexGraph::build(vec![
            vec![ ]
        ]).unwrap();
        let traversal = depth_first(&graph, &0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![ ]);
    }

    #[test]
    fn p2() {
        let graph = IndexGraph::build(vec![
            vec![ 1 ],
            vec![ 0 ]
        ]).unwrap();
        let traversal = depth_first(&graph, &0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            (&0, &1, false)
        ]);
    }

    #[test]
    fn p3() {
        let graph = IndexGraph::build(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();
        let traversal = depth_first(&graph, &0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            (&0, &1, false),
            (&1, &2, false)
        ]);
    }

    #[test]
    fn p3_inside() {
        let graph = IndexGraph::build(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();
        let traversal = depth_first(&graph, &1).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            (&1, &0, false),
            (&1, &2, false)
        ]);
    }

    #[test]
    fn p4() {
        let graph = IndexGraph::build(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1, 3 ],
            vec![ 2 ]
        ]).unwrap();
        let traversal = depth_first(&graph, &0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            (&0, &1, false),
            (&1, &2, false),
            (&2, &3, false)
        ]);
    }

    #[test]
    fn c3() {
        let graph = IndexGraph::build(vec![
            vec![ 1, 2 ],
            vec![ 0, 2 ],
            vec![ 1, 0 ]
        ]).unwrap();
        let traversal = depth_first(&graph, &0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            (&0, &1, false),
            (&1, &2, false),
            (&2, &0, true)
        ]);
    }

    #[test]
    fn s3() {
        let graph = IndexGraph::build(vec![
            vec![ 1 ],
            vec![ 0, 2, 3 ],
            vec![ 1 ],
            vec![ 1 ]
        ]).unwrap();
        let traversal = depth_first(&graph, &0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            (&0, &1, false),
            (&1, &2, false),
            (&1, &3, false)
        ]);
    }

    #[test]
    fn s3_inside() {
        let graph = IndexGraph::build(vec![
            vec![ 1 ],
            vec![ 0, 2, 3 ],
            vec![ 1 ],
            vec![ 1 ]
        ]).unwrap();
        let traversal = depth_first(&graph, &1).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            (&1, &0, false),
            (&1, &2, false),
            (&1, &3, false)
        ]);
    }

    #[test]
    fn flower_from_stalk() {
        let graph = IndexGraph::build(vec![
            vec![ 1 ],
            vec![ 0, 2, 3 ],
            vec![ 1, 3 ],
            vec![ 2, 1 ]
        ]).unwrap();
        let traversal = depth_first(&graph, &0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            (&0, &1, false),
            (&1, &2, false),
            (&2, &3, false),
            (&3, &1, true)
        ]);
    }

    #[test]
    fn flower_with_cut_in_branch() {
        let graph = IndexGraph::build(vec![
            vec![ 1, 2 ],
            vec![ 0, 2 ],
            vec![ 1, 0, 3 ],
            vec![ 2 ]
        ]).unwrap();
        let traversal = depth_first(&graph, &0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            (&0, &1, false),
            (&1, &2, false),
            (&2, &0, true),
            (&2, &3, false)
        ]);
    }

    #[test]
    fn blocked_branched_path() {
        let graph = IndexGraph::build(vec![
            vec![ 1 ],
            vec![ 0, 2, 3, 4 ],
            vec![ 1 ],
            vec![ 1, 4 ],
            vec![ 3, 1 ]
        ]).unwrap();
        let traversal = depth_first(&graph, &0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            (&0, &1, false),
            (&1, &2, false),
            (&1, &3, false),
            (&3, &4, false),
            (&4, &1, true)
        ]);
    }

    #[test]
    fn bicyclo_220_with_cut_on_second_branching() {
        let graph = IndexGraph::build(vec![
            vec![ 1, 5 ],
            vec![ 0, 2 ],
            vec![ 1, 5, 3 ],
            vec![ 2, 4 ],
            vec![ 3, 5 ],
            vec![ 2, 4, 0 ]
        ]).unwrap();
        let traversal = depth_first(&graph, &0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            (&0, &1, false),
            (&1, &2, false),
            (&2, &5, false),
            (&5, &4, false),
            (&4, &3, false),
            (&3, &2, true),
            (&5, &0, true)
        ]);
    }

    #[test]
    fn bicyclo_210() {
        let graph = IndexGraph::build(vec![
            vec![ 1, 2, 4 ],
            vec![ 0, 2 ],
            vec![ 1, 0, 3 ],
            vec![ 2, 4 ],
            vec![ 3, 0 ]
        ]).unwrap();
        let traversal = depth_first(&graph, &0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            (&0, &1, false),
            (&1, &2, false),
            (&2, &0, true),
            (&2, &3, false),
            (&3, &4, false),
            (&4, &0, true)
        ]);
    }

    #[test]
    fn cube() {
        let graph = IndexGraph::build(vec![
            vec![ 1, 3, 4 ],
            vec![ 0, 2, 5 ],
            vec![ 1, 3, 6 ],
            vec![ 2, 0, 7 ],
            vec![ 5, 7, 0 ],
            vec![ 4, 6, 1 ],
            vec![ 5, 7, 2 ],
            vec![ 6, 4, 3 ]
        ]).unwrap();
        let traversal = depth_first(&graph, &0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            (&0, &1, false),
            (&1, &2, false),
            (&2, &3, false),
            (&3, &0, true),
            (&3, &7, false),
            (&7, &6, false),
            (&6, &5, false),
            (&5, &4, false),
            (&4, &7, true),
            (&4, &0, true),
            (&5, &1, true),
            (&6, &2, true)
        ]);
    }
}