use std::collections::HashSet;

use crate::graph::{ Graph, Step };
use crate::selection::components;
use crate::traversal::depth_first;

/// Returns a greedy matching over all componenents of the Graph. Bipartate
/// graphs always return a perfect Matching. Non-bipartate graphs yield either
/// maximal or maximum Matchings.
/// 
/// Because a greedy Matching can be used as a starting point to a more
/// sophisticated matching procedure (e.g., Edmund's Blossom), it usually
/// makes sense to try a greedy matching and only fall back to a more advanced
/// procedure if the matching isn't perfect.
/// 
/// For more on matching, see: *[The Maximum Matching Problem](https://depth-first.com/articles/2019/04/02/the-maximum-matching-problem/)*.
/// 
/// ```rust
/// use gamma::graph::{ Graph, Error, ArrayGraph };
/// use gamma::matching::greedy;
/// 
/// fn main() -> Result<(), Error> {
///     let graph = ArrayGraph::from_adjacency(vec![
///         vec![ 1 ],
///         vec![ 0, 2 ],
///         vec![ 1 ]
///     ])?;
///     let edges = greedy(&graph);
/// 
///     assert_eq!(edges, vec![
///         (0, 1)
///     ]);
///     
///     Ok(())
/// }
/// ```
/// 
pub fn greedy<G: Graph>(
    graph: &G
) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();
    let mut nodes = HashSet::new();

    for graph in components(graph) {
        let traversal = depth_first(&graph, graph.nodes()[0]).expect(
            "depth-first traversal failed"
        );

        for Step { sid, tid, cut: _ } in traversal {
            if nodes.insert(sid) && nodes.insert(tid) {
                edges.push((sid, tid));
            }
        }
    }

    edges
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::ArrayGraph;

    #[test]
    fn empty() {
        let graph = ArrayGraph::new();
        let result = greedy(&graph);

        assert_eq!(result.is_empty(), true);
    }

    #[test]
    fn p1() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ ]
        ]).unwrap();

        assert_eq!(greedy(&graph), vec![ ]);
    }

    #[test]
    fn p2() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ 1 ],
            vec![ 0 ]
        ]).unwrap();

        assert_eq!(greedy(&graph), vec![
            (0, 1)
        ]);
    }

    #[test]
    fn p2_p2() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ 1 ],
            vec![ 0 ],
            vec![ 3 ],
            vec![ 2 ]
        ]).unwrap();

        assert_eq!(greedy(&graph), vec![
            (0, 1),
            (2, 3)
        ]);
    }

    #[test]
    fn p3() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();
        let result = greedy(&graph);

        assert_eq!(result, vec![
            (0, 1)
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
        let result = greedy(&graph);

        assert_eq!(result, vec![
            (0, 1),
            (2, 3)
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

        assert_eq!(greedy(&graph), vec![
            (0, 1)
        ]);
    }

    #[test]
    fn c6() {
        let graph = ArrayGraph::from_adjacency(vec![
            vec![ 1, 5 ],
            vec![ 0, 2 ],
            vec![ 1, 3 ],
            vec![ 2, 4 ],
            vec![ 3, 5 ],
            vec![ 4, 0 ]
        ]).unwrap();

        assert_eq!(greedy(&graph), vec![
            (0, 1),
            (2, 3),
            (4, 5)
        ]);
    }
}