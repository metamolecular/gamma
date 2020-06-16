use std::hash::Hash;
use std::collections::HashSet;

use crate::graph::Graph;
use crate::matching::Matching;
use crate::traversal::depth_first;

/// Attempts a greedy matching. The Graph is traversed in depth-first order
/// from the first node it iterates. Edges are added to the result if neither
/// terminal has been added already. Bipartate graphs always
/// return a perfect Matching. Non-bipartate graphs yield either maximal
/// or maximum Matchings.
/// 
/// Because a greedy Matching can be used as a starting point to a more
/// sophisticated matching procedure (e.g., Edmund's Blossom), it usually
/// makes sense to try a greedy matching and only fall back to a more advanced
/// procedure if the Matching isn't perfect.
/// 
/// For more on matching, see: *[The Maximum Matching Problem](https://depth-first.com/articles/2019/04/02/the-maximum-matching-problem/)*.
/// 
/// ```rust
/// use std::collections::HashSet;
/// use gamma::graph::{Graph, IndexGraph, Error};
/// use gamma::matching::greedy;
/// 
/// fn main() -> Result<(), Error> {
///     let graph = IndexGraph::build(vec![
///         vec![ 1, 5 ],
///         vec![ 0, 2 ],
///         vec![ 1, 3 ],
///         vec![ 2, 4 ],
///         vec![ 3, 5 ],
///         vec![ 4, 0 ]
///     ])?;
///     let matching = greedy(&graph);
///     let mut edges = HashSet::new();
///
///     edges.insert((&0, &1));
///     edges.insert((&2, &3));
///     edges.insert((&4, &5));
///     assert_eq!(matching.edges().collect::<HashSet<_>>(), edges);
/// 
///     Ok(())
/// }
/// ```
pub fn greedy<'a, N: 'a+Clone+Eq+Hash>(
    graph: &'a impl Graph<'a, N>
) -> Matching<N> {
    let root = match graph.nodes().next() {
        Some(root) => root,
        None => return Matching::build(vec![ ]).unwrap()
    };

    let mut nodes = HashSet::new();
    let mut edges = Vec::new();

    for (source, target, _) in depth_first(graph, root).unwrap() {
        if !nodes.contains(source) && !nodes.contains(target) {
            edges.push((source.clone(), target.clone()));
            nodes.insert(source);
            nodes.insert(target);
        }
    }

    Matching::build(edges).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::IndexGraph;

    macro_rules! set {
        ( $( $x:expr ), * ) => {
            {
                #[allow(unused_mut)]
                let mut set = HashSet::new();

                $(
                    set.insert($x);
                )*

                set
            }
        };
    }

    #[test]
    fn empty() {
        let graph = IndexGraph::build(vec![ ]).unwrap();
        let matching = greedy(&graph);

        assert!(matching.is_empty());
    }

    #[test]
    fn p1() {
        let graph = IndexGraph::build(vec![
            vec! [ ]
        ]).unwrap();
        let matching = greedy(&graph);

        assert!(matching.is_empty());
    }

    #[test]
    fn p2() {
        let graph = IndexGraph::build(vec![
            vec![ 1 ],
            vec![ 0 ]
        ]).unwrap();
        let matching = greedy(&graph);

        assert_eq!(matching.edges().collect::<HashSet<_>>(), set![
            (&0, &1)
        ]);
    }

    #[test]
    fn p3() {
        let graph = IndexGraph::build(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();
        let matching = greedy(&graph);

        assert_eq!(matching.edges().collect::<HashSet<_>>(), set![
            (&0, &1)
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
        let matching = greedy(&graph);

        assert_eq!(matching.edges().collect::<HashSet<_>>(), set![
            (&0, &1),
            (&2, &3)
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
        let matching = greedy(&graph);

        assert_eq!(matching.edges().collect::<HashSet<_>>(), set![
            (&0, &1)
        ]);
    }

    #[test]
    fn c6() {
        let graph = IndexGraph::build(vec![
            vec![ 1, 5 ],
            vec![ 0, 2 ],
            vec![ 1, 3 ],
            vec![ 2, 4 ],
            vec![ 3, 5 ],
            vec![ 4, 0 ]
        ]).unwrap();
        let matching = greedy(&graph);

        assert_eq!(matching.edges().collect::<HashSet<_>>(), set![
            (&0, &1),
            (&2, &3),
            (&4, &5)
        ]);
    }
}