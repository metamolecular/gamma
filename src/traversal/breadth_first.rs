use std::hash::Hash;
use std::collections::VecDeque;
use std::collections::HashSet;

use crate::graph::{ Graph, Error };

/// Implements a breadth-first traversal as an edge Iterator. Reports cycle
/// closure edges.
/// 
/// ```rust
/// use gamma::graph::{ Graph, IndexGraph, Error };
/// use gamma::traversal::breadth_first;
/// 
/// fn main() -> Result<(), Error> {
///     let graph = IndexGraph::build(vec![
///         vec![ 1, 3 ],
///         vec![ 0, 2 ],
///         vec![ 1, 3 ],
///         vec![ 2, 0 ]
///     ])?;
///     let traversal = breadth_first(&graph, &0)?;
///
///     assert_eq!(traversal.collect::<Vec<_>>(), vec![
///         (&0, &1, false),
///         (&0, &3, false),
///         (&1, &2, false),
///         (&3, &2, true)
///     ]);
/// 
///     Ok(())
/// }
/// ```
pub fn breadth_first<'a, N, G>(
    graph: &'a G, root: &'a N
) -> Result<BreadthFirst<'a, N, G>, Error>
where G: Graph<'a, N>, N: 'a + Hash + Eq {
    let mut nodes = HashSet::new();
    let mut queue = VecDeque::new();

    for neighbor in graph.neighbors(root)? {
        queue.push_front((root, neighbor));
    }

    nodes.insert(root);

    Ok(BreadthFirst { nodes, queue, graph })
}

/// Iterates edges of graph in breadth-first order. To perform a breadth-first
/// search, use the breadth_first function instead.
pub struct BreadthFirst<'a, N, G> {
    nodes: HashSet<&'a N>,
    queue: VecDeque<(&'a N, &'a N)>,
    graph: &'a G
}

impl<'a, N, G> Iterator for BreadthFirst<'a, N, G>
    where N: Eq + Hash, G: Graph<'a, N> {
    type Item = (&'a N, &'a N, bool);

    fn next(&mut self) -> Option<Self::Item> {
        match self.queue.pop_back() {
            None => None,
            Some((parent, node)) => {
                if self.nodes.contains(node) {
                    Some((parent, node, true))
                } else {
                    for neighbor in self.graph.neighbors(node).unwrap() {
                        if neighbor == parent || self.nodes.contains(neighbor) {
                            continue;
                        }
    
                        self.queue.push_front((node, neighbor));
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
    fn nonmember_root() {
        let graph = IndexGraph::build(vec![
            vec! [ ]
        ]).unwrap();
        let traversal = breadth_first(&graph, &0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![ ]);
    }

    #[test]
    fn p1() {
        let graph = IndexGraph::build(vec![
            vec![ ]
        ]).unwrap();
        let traversal = breadth_first(&graph, &0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![ ]);
    }

    #[test]
    fn p2() {
        let graph = IndexGraph::build(vec![
            vec![ 1 ],
            vec![ 0 ]
        ]).unwrap();
        let traversal = breadth_first(&graph, &0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            (&0, &1, false)
        ]);
    }

    #[test]
    fn p3_primary() {
        let graph = IndexGraph::build(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();
        let traversal = breadth_first(&graph, &0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            (&0, &1, false),
            (&1, &2, false)
        ]);
    }

    #[test]
    fn p3_secondary() {
        let graph = IndexGraph::build(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();
        let traversal = breadth_first(&graph, &1).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            (&1, &0, false),
            (&1, &2, false)
        ]);
    }

    #[test]
    fn p4_primary() {
        let graph = IndexGraph::build(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1, 3 ],
            vec![ 2 ]
        ]).unwrap();
        let traversal = breadth_first(&graph, &1).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            (&1, &0, false),
            (&1, &2, false),
            (&2, &3, false)
        ]);
    }

    #[test]
    fn s3_tertiary() {
        let graph = IndexGraph::build(vec![
            vec![ 1, 2, 3 ],
            vec![ 0 ],
            vec![ 0 ],
            vec![ 0 ]
        ]).unwrap();
        let traversal = breadth_first(&graph, &0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            (&0, &1, false),
            (&0, &2, false),
            (&0, &3, false)
        ]);
    }

    #[test]
    fn s3_primary() {
        let graph = IndexGraph::build(vec![
            vec![ 1, 2, 3 ],
            vec![ 0 ],
            vec![ 0 ],
            vec![ 0 ]
        ]).unwrap();
        let traversal = breadth_first(&graph, &1).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            (&1, &0, false),
            (&0, &2, false),
            (&0, &3, false)
        ]);
    }

    #[test]
    fn c3() {
        let graph = IndexGraph::build(vec![
            vec![ 1, 2 ],
            vec![ 0, 2 ],
            vec![ 1, 0 ]
        ]).unwrap();
        let traversal = breadth_first(&graph, &0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            (&0, &1, false),
            (&0, &2, false),
            (&1, &2, true)
        ]);
    }

    #[test]
    fn c4() {
        let graph = IndexGraph::build(vec![
            vec![ 1, 3 ],
            vec![ 0, 2 ],
            vec![ 1, 3 ],
            vec![ 2, 0 ]
        ]).unwrap();
        let traversal = breadth_first(&graph, &0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            (&0, &1, false),
            (&0, &3, false),
            (&1, &2, false),
            (&3, &2, true)
        ]);
    }

    #[test]
    fn diamond() {
        let graph = IndexGraph::build(vec![
            vec![ 1, 2, 3 ],
            vec![ 2, 0 ],
            vec![ 0, 1, 3 ],
            vec![ 0, 2 ]
        ]).unwrap();
        let bfs = breadth_first(&graph, &0).unwrap();

        assert_eq!(bfs.collect::<Vec<_>>(), vec![
            (&0, &1, false),
            (&0, &2, false),
            (&0, &3, false),
            (&1, &2, true),
            (&2, &3, true)
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
        let bfs = breadth_first(&graph, &0).unwrap();

        assert_eq!(bfs.collect::<Vec<_>>(), vec![
            (&0, &1, false),
            (&1, &2, false),
            (&1, &3, false),
            (&2, &3, true)
        ]);
    }

    #[test]
    fn t2_primary() {
        let graph = IndexGraph::build(vec![
            vec![ 1, 4 ],
            vec![ 0, 2, 3 ],
            vec![ 1, 5 ],
            vec![ 1, 6 ],
            vec![ 0 ],
            vec![ 2 ],
            vec![ 3 ]
        ]).unwrap();
        let bfs = breadth_first(&graph, &0).unwrap();

        assert_eq!(bfs.collect::<Vec<_>>(), vec![
            (&0, &1, false),
            (&0, &4, false),
            (&1, &2, false),
            (&1, &3, false),
            (&2, &5, false),
            (&3, &6, false)
        ]);
    }

    #[test]
    fn t2_tertiary() {
        let graph = IndexGraph::build(vec![
            vec![ 1, 4 ],
            vec![ 0, 2, 3 ],
            vec![ 1, 5 ],
            vec![ 1, 6 ],
            vec![ 0 ],
            vec![ 2 ],
            vec![ 3 ]
        ]).unwrap();
        let bfs = breadth_first(&graph, &1).unwrap();

        assert_eq!(bfs.collect::<Vec<_>>(), vec![
            (&1, &0, false),
            (&1, &2, false),
            (&1, &3, false),
            (&0, &4, false),
            (&2, &5, false),
            (&3, &6, false)
        ]);
    }

    #[test]
    fn bicyclo_111() {
        let graph = IndexGraph::build(vec![
            vec![ 1, 2, 3 ],
            vec![ 0, 4 ],
            vec![ 0, 4 ],
            vec![ 0, 4 ],
            vec![ 1, 2, 3 ]
        ]).unwrap();
        let bfs = breadth_first(&graph, &0).unwrap();

        assert_eq!(bfs.collect::<Vec<_>>(), vec![
            (&0, &1, false),
            (&0, &2, false),
            (&0, &3, false),
            (&1, &4, false),
            (&2, &4, true),
            (&3, &4, true)
        ]);
    }

    #[test]
    fn bicyclo_221() {
        let graph = IndexGraph::build(vec![
            vec![ 1, 5 ],
            vec![ 0, 2, 6 ],
            vec![ 1, 3 ],
            vec![ 2, 4 ],
            vec![ 3, 5, 6 ],
            vec![ 4, 0 ],
            vec![ 4, 1 ]
        ]).unwrap();
        let bfs = breadth_first(&graph, &0).unwrap();

        assert_eq!(bfs.collect::<Vec<_>>(), vec![
            (&0, &1, false),
            (&0, &5, false),
            (&1, &2, false),
            (&1, &6, false),
            (&5, &4, false),
            (&2, &3, false),
            (&6, &4, true),
            (&4, &3, true)
        ]);
    }

    #[test]
    fn butterfly() {
        let graph = IndexGraph::build(vec![
            vec![ 1, 2 ],
            vec![ 0, 2 ],
            vec![ 0, 1, 3, 4 ],
            vec![ 2, 4 ],
            vec![ 2, 3 ]
        ]).unwrap();
        let bfs = breadth_first(&graph, &0).unwrap();

        assert_eq!(bfs.collect::<Vec<_>>(), vec![
            (&0, &1, false),
            (&0, &2, false),
            (&1, &2, true),
            (&2, &3, false),
            (&2, &4, false),
            (&3, &4, true)
        ]);
    }
}