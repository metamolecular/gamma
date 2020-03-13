use std::collections::HashSet;
use std::hash::Hash;

use crate::graph::Graph;
use crate::graph::Error;

/// Implements a depth-first traversal as an edge Iterator. Reports cycle
/// closure edges.
/// 
/// ```
/// use gamma::graph::Graph;
/// use gamma::graph::HashGraph;
/// use gamma::traversal::depth_first;
/// 
/// let graph = HashGraph::build(vec![ 0, 1, 2 ], vec![
///     (&0, &1, ()),
///     (&1, &2, ()),
///     (&2, &0, ()),
/// ]).unwrap();
/// let traversal = depth_first(&graph, &0).unwrap();
/// 
/// assert_eq!(traversal.collect::<Vec<_>>(), vec![
///     (&0, &1, false),
///     (&1, &2, false),
///     (&2, &0, true)
/// ]);
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

impl<'a, N: std::fmt::Debug, G> Iterator for DepthFirst<'a, N, G>
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
    use crate::graph::HashGraph;

    #[test]
    fn returns_error_given_unknown_root() {
        let graph = HashGraph::<_, ()>::build(vec![ 0 ], vec![ ]).unwrap();

        assert_eq!(depth_first(&graph, &1).err().unwrap(), Error::UnknownNode);
    }

    #[test]
    fn walks_p1() {
        let graph = HashGraph::<_, ()>::build(vec![ 0 ], vec![ ]).unwrap();
        let traversal = depth_first(&graph, &0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![ ]);
    }

    #[test]
    fn walks_p2() {
        let graph = HashGraph::build(vec![ 0, 1 ], vec![
            (&0, &1, ())
        ]).unwrap();
        let traversal = depth_first(&graph, &0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            (&0, &1, false)
        ]);
    }

    #[test]
    fn walks_p3() {
        let graph = HashGraph::build(vec![ 0, 1, 2 ], vec![
            (&0, &1, ()),
            (&1, &2, ())
        ]).unwrap();
        let traversal = depth_first(&graph, &0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            (&0, &1, false),
            (&1, &2, false)
        ]);
    }

    #[test]
    fn walks_p3_from_inside() {
        let graph = HashGraph::build(vec![ 0, 1, 2 ], vec![
            (&0, &1, ()),
            (&1, &2, ())
        ]).unwrap();
        let traversal = depth_first(&graph, &1).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            (&1, &0, false),
            (&1, &2, false)
        ]);
    }

    #[test]
    fn walks_p4() {
        let graph = HashGraph::build(vec![ 0, 1, 2, 3 ], vec![
            (&0, &1, ()),
            (&1, &2, ()),
            (&2, &3, ())
        ]).unwrap();
        let traversal = depth_first(&graph, &0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            (&0, &1, false),
            (&1, &2, false),
            (&2, &3, false)
        ]);
    }

    #[test]
    fn walks_c3() {
        let graph = HashGraph::build(vec![ 0, 1, 2 ], vec![
            (&0, &1, ()),
            (&1, &2, ()),
            (&2, &0, ()),
        ]).unwrap();
        let traversal = depth_first(&graph, &0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            (&0, &1, false),
            (&1, &2, false),
            (&2, &0, true)
        ]);
    }

    #[test]
    fn walks_s3() {
        let graph = HashGraph::build(vec![ 0, 1, 2, 3 ], vec![
            (&0, &1, ()),
            (&1, &2, ()),
            (&1, &3, ())
        ]).unwrap();
        let traversal = depth_first(&graph, &0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            (&0, &1, false),
            (&1, &2, false),
            (&1, &3, false)
        ]);
    }

    #[test]
    fn walks_s3_from_inside() {
        let graph = HashGraph::build(vec![ 0, 1, 2, 3 ], vec![
            (&0, &1, ()),
            (&1, &2, ()),
            (&1, &3, ())
        ]).unwrap();
        let traversal = depth_first(&graph, &1).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            (&1, &0, false),
            (&1, &2, false),
            (&1, &3, false)
        ]);
    }

    #[test]
    fn walks_flower_from_stalk() {
        let graph = HashGraph::build(vec![ 0, 1, 2, 3 ], vec![
            (&0, &1, ()),
            (&1, &2, ()),
            (&2, &3, ()),
            (&3, &1, ())
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
    fn walks_flower_with_cut_in_branch() {
        let graph = HashGraph::build(vec![ 0, 1, 2, 3 ], vec![
            (&0, &1, ()),
            (&1, &2, ()),
            (&2, &0, ()),
            (&2, &3, ())
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
    fn walks_a_blocked_branched_path() {
        let graph = HashGraph::build(vec![ 0, 1, 2, 3, 4 ], vec![
            (&0, &1, ()),
            (&1, &2, ()),
            (&1, &3, ()),
            (&3, &4, ()),
            (&4, &1, ())
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
    fn walks_220_with_cut_on_second_branching() {
        let graph = HashGraph::build(vec![ 0, 1, 2, 3, 4, 5 ], vec![
            (&0, &1, ()),
            (&1, &2, ()),
            (&2, &5, ()),
            (&2, &3, ()),
            (&3, &4, ()),
            (&4, &5, ()),
            (&5, &0, ())
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
    fn walks_both_cycles_of_210() {
        let graph = HashGraph::build(vec![ 0, 1, 2, 3, 4 ], vec![
            (&0, &1, ()),
            (&1, &2, ()),
            (&2, &0, ()),
            (&2, &3, ()),
            (&3, &4, ()),
            (&4, &0, ())
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
    fn walks_cube() {
        let graph = HashGraph::build(vec![ 0, 1, 2, 3, 4, 5, 6, 7 ], vec![
            (&0, &1, ()), (&1, &2, ()), (&2, &3, ()), (&3, &0, ()),
            (&4, &5, ()), (&5, &6, ()), (&6, &7, ()), (&7, &4, ()),
            (&0, &4, ()), (&1, &5, ()), (&2, &6, ()), (&3, &7, ())
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