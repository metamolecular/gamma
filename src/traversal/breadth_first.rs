use std::collections::VecDeque;
use std::collections::HashSet;

use crate::graph::{ Graph, Error };
use super::Step;

/// Implements a breadth-first traversal as a Step Iterator.
/// 
/// ```rust
/// use std::convert::TryFrom;
/// use gamma::graph::{ Graph, Error, DefaultGraph };
/// use gamma::traversal::{ BreadthFirst, Step};
/// 
/// fn main() -> Result<(), Error> {
///     let graph = DefaultGraph::try_from(vec![
///         vec![ 1, 3 ],
///         vec![ 0, 2 ],
///         vec![ 1, 3 ],
///         vec![ 2, 0 ]
///     ])?;
///     let traversal = BreadthFirst::new(&graph, 0)?;
/// 
///     assert_eq!(traversal.collect::<Vec<_>>(), vec![
///         Step::new(0, 1, false),
///         Step::new(0, 3, false),
///         Step::new(1, 2, false),
///         Step::new(3, 2, true)
///     ]);
/// 
///     Ok(())
/// }
/// ```
// pub fn breadth_first<'a, G>(
//     graph: &'a G, root: usize
// ) -> Result<BreadthFirst<'a, G>, Error>
// where G: Graph {
//     let mut nodes = HashSet::new();
//     let mut queue = VecDeque::new();

//     for neighbor in graph.neighbors(root)? {
//         queue.push_front((root, *neighbor));
//     }

//     nodes.insert(root);

//     Ok(BreadthFirst { nodes, queue, graph })
// }

/// #[derive(Debug,PartialEq)]
pub struct BreadthFirst<'a, G> {
    nodes: HashSet<usize>,
    queue: VecDeque<(usize, usize)>,
    graph: &'a G
}

impl<'a, G: Graph> BreadthFirst<'a, G> {
    pub fn new(graph: &'a G, root: usize) -> Result<Self, Error> {
        let mut nodes = HashSet::new();
        let mut queue = VecDeque::new();
    
        for neighbor in graph.neighbors(root)? {
            queue.push_front((root, neighbor));
        }
    
        nodes.insert(root);
    
        Ok(Self { nodes, queue, graph })
    }
}

impl<'a, G> Iterator for BreadthFirst<'a, G>
    where G: Graph {
    type Item = Step;

    fn next(&mut self) -> Option<Self::Item> {
        match self.queue.pop_back() {
            None => None,
            Some((parent, node)) => {
                if self.nodes.contains(&node) {
                    Some(Step::new(parent, node, true))
                } else {
                    for neighbor in self.graph.neighbors(node).unwrap() {
                        if neighbor == parent || self.nodes.contains(&neighbor) {
                            continue;
                        }
    
                        self.queue.push_front((node, neighbor));
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
    use std::convert::TryFrom;
    use crate::graph::DefaultGraph;

    #[test]
    fn nonmember_root() {
        let graph = DefaultGraph::try_from(vec![
            vec! [ ]
        ]).unwrap();
        let traversal = BreadthFirst::new(&graph, 0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![ ]);
    }

    #[test]
    fn p1() {
        let graph = DefaultGraph::try_from(vec![
            vec![ ]
        ]).unwrap();
        let traversal = BreadthFirst::new(&graph, 0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![ ]);
    }

    #[test]
    fn p2() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0 ]
        ]).unwrap();
        let traversal = BreadthFirst::new(&graph, 0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            Step::new(0, 1, false)
        ]);
    }

    #[test]
    fn p3_primary() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();
        let traversal = BreadthFirst::new(&graph, 0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            Step::new(0, 1, false),
            Step::new(1, 2, false)
        ]);
    }

    #[test]
    fn p3_secondary() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();
        let traversal = BreadthFirst::new(&graph, 1).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            Step::new(1, 0, false),
            Step::new(1, 2, false)
        ]);
    }

    #[test]
    fn p4_primary() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1, 3 ],
            vec![ 2 ]
        ]).unwrap();
        let traversal = BreadthFirst::new(&graph, 1).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            Step::new(1, 0, false),
            Step::new(1, 2, false),
            Step::new(2, 3, false)
        ]);
    }

    #[test]
    fn s3_tertiary() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1, 2, 3 ],
            vec![ 0 ],
            vec![ 0 ],
            vec![ 0 ]
        ]).unwrap();
        let traversal = BreadthFirst::new(&graph, 0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            Step::new(0, 1, false),
            Step::new(0, 2, false),
            Step::new(0, 3, false)
        ]);
    }

    #[test]
    fn s3_primary() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1, 2, 3 ],
            vec![ 0 ],
            vec![ 0 ],
            vec![ 0 ]
        ]).unwrap();
        let traversal = BreadthFirst::new(&graph, 1).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            Step::new(1, 0, false),
            Step::new(0, 2, false),
            Step::new(0, 3, false)
        ]);
    }

    #[test]
    fn c3() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1, 2 ],
            vec![ 0, 2 ],
            vec![ 1, 0 ]
        ]).unwrap();
        let traversal = BreadthFirst::new(&graph, 0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            Step::new(0, 1, false),
            Step::new(0, 2, false),
            Step::new(1, 2, true)
        ]);
    }

    #[test]
    fn c4() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1, 3 ],
            vec![ 0, 2 ],
            vec![ 1, 3 ],
            vec![ 2, 0 ]
        ]).unwrap();
        let traversal = BreadthFirst::new(&graph, 0).unwrap();

        assert_eq!(traversal.collect::<Vec<_>>(), vec![
            Step::new(0, 1, false),
            Step::new(0, 3, false),
            Step::new(1, 2, false),
            Step::new(3, 2, true)
        ]);
    }

    #[test]
    fn diamond() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1, 2, 3 ],
            vec![ 2, 0 ],
            vec![ 0, 1, 3 ],
            vec![ 0, 2 ]
        ]).unwrap();
        let bfs = BreadthFirst::new(&graph, 0).unwrap();

        assert_eq!(bfs.collect::<Vec<_>>(), vec![
            Step::new(0, 1, false),
            Step::new(0, 2, false),
            Step::new(0, 3, false),
            Step::new(1, 2, true),
            Step::new(2, 3, true)
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
        let bfs = BreadthFirst::new(&graph, 0).unwrap();

        assert_eq!(bfs.collect::<Vec<_>>(), vec![
            Step::new(0, 1, false),
            Step::new(1, 2, false),
            Step::new(1, 3, false),
            Step::new(2, 3, true)
        ]);
    }

    #[test]
    fn t2_primary() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1, 4 ],
            vec![ 0, 2, 3 ],
            vec![ 1, 5 ],
            vec![ 1, 6 ],
            vec![ 0 ],
            vec![ 2 ],
            vec![ 3 ]
        ]).unwrap();
        let bfs = BreadthFirst::new(&graph, 0).unwrap();

        assert_eq!(bfs.collect::<Vec<_>>(), vec![
            Step::new(0, 1, false),
            Step::new(0, 4, false),
            Step::new(1, 2, false),
            Step::new(1, 3, false),
            Step::new(2, 5, false),
            Step::new(3, 6, false)
        ]);
    }

    #[test]
    fn t2_tertiary() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1, 4 ],
            vec![ 0, 2, 3 ],
            vec![ 1, 5 ],
            vec![ 1, 6 ],
            vec![ 0 ],
            vec![ 2 ],
            vec![ 3 ]
        ]).unwrap();
        let bfs = BreadthFirst::new(&graph, 1).unwrap();

        assert_eq!(bfs.collect::<Vec<_>>(), vec![
            Step::new(1, 0, false),
            Step::new(1, 2, false),
            Step::new(1, 3, false),
            Step::new(0, 4, false),
            Step::new(2, 5, false),
            Step::new(3, 6, false)
        ]);
    }

    #[test]
    fn bicyclo_111() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1, 2, 3 ],
            vec![ 0, 4 ],
            vec![ 0, 4 ],
            vec![ 0, 4 ],
            vec![ 1, 2, 3 ]
        ]).unwrap();
        let bfs = BreadthFirst::new(&graph, 0).unwrap();

        assert_eq!(bfs.collect::<Vec<_>>(), vec![
            Step::new(0, 1, false),
            Step::new(0, 2, false),
            Step::new(0, 3, false),
            Step::new(1, 4, false),
            Step::new(2, 4, true),
            Step::new(3, 4, true)
        ]);
    }

    #[test]
    fn bicyclo_221() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1, 5 ],
            vec![ 0, 2, 6 ],
            vec![ 1, 3 ],
            vec![ 2, 4 ],
            vec![ 3, 5, 6 ],
            vec![ 4, 0 ],
            vec![ 4, 1 ]
        ]).unwrap();
        let bfs = BreadthFirst::new(&graph, 0).unwrap();

        assert_eq!(bfs.collect::<Vec<_>>(), vec![
            Step::new(0, 1, false),
            Step::new(0, 5, false),
            Step::new(1, 2, false),
            Step::new(1, 6, false),
            Step::new(5, 4, false),
            Step::new(2, 3, false),
            Step::new(6, 4, true),
            Step::new(4, 3, true)
        ]);
    }

    #[test]
    fn butterfly() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1, 2 ],
            vec![ 0, 2 ],
            vec![ 0, 1, 3, 4 ],
            vec![ 2, 4 ],
            vec![ 2, 3 ]
        ]).unwrap();
        let bfs = BreadthFirst::new(&graph, 0).unwrap();

        assert_eq!(bfs.collect::<Vec<_>>(), vec![
            Step::new(0, 1, false),
            Step::new(0, 2, false),
            Step::new(1, 2, true),
            Step::new(2, 3, false),
            Step::new(2, 4, false),
            Step::new(3, 4, true)
        ]);
    }
}