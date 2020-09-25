use std::collections::HashSet;

use crate::graph::Graph;
use crate::selection::components;
use crate::traversal::{ DepthFirst, Step };
use super::pairing::Pairing;

/// Returns a greedy matching over all componenents of the Graph. Bipartate
/// graphs may return a perfect Matching. Non-bipartate graphs yield either
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
/// use std::convert::TryFrom;
/// use std::collections::BTreeSet;
/// use gamma::graph::{ Graph, Error, DefaultGraph };
/// use gamma::matching::greedy;
/// 
/// fn main() -> Result<(), Error> {
///     let graph = DefaultGraph::try_from(vec![
///         vec![ 1 ],
///         vec![ 0, 2 ],
///         vec![ 1 ]
///     ])?;
///     let edges = greedy(&graph);
/// 
//      assert_eq!(
//          pairing.edges().collect::<BTreeSet<_>>(),
//          [ (0, 1) ].iter().cloned().collect::<BTreeSet<_>>()
//      )
///     
///     Ok(())
/// }
/// ```
pub fn greedy<G: Graph>(graph: &G) -> Pairing {
    // let mut edges = Vec::new();
    let mut pairing = Pairing::new();
    let mut nodes = HashSet::new();

    for graph in components(graph) {
        let traversal = DepthFirst::new(&graph, graph.nodes()[0]).expect(
            "could not create depth-first traversal"
        );

        for Step { sid, tid, cut: _ } in traversal {
            if nodes.insert(sid) && nodes.insert(tid) {
                // edges.push((sid, tid));
                pairing.pair(sid, tid);
            }
        }
    }

    pairing
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
    use std::collections::BTreeSet;
    use super::*;
    use crate::graph::DefaultGraph;

    #[test]
    fn empty() {
        let graph = DefaultGraph::new();
        let pairing = greedy(&graph);

        assert_eq!(
            pairing.edges().collect::<BTreeSet<_>>(),
            [ ].iter().cloned().collect::<BTreeSet<_>>()
        )
    }

    #[test]
    fn p1() {
        let graph = DefaultGraph::try_from(vec![
            vec![ ]
        ]).unwrap();
        let pairing = greedy(&graph);

        assert_eq!(
            pairing.edges().collect::<BTreeSet<_>>(),
            [ ].iter().cloned().collect::<BTreeSet<_>>()
        )
    }

    #[test]
    fn p2() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0 ]
        ]).unwrap();
        let pairing = greedy(&graph);

        assert_eq!(
            pairing.edges().collect::<BTreeSet<_>>(),
            [ (0, 1) ].iter().cloned().collect::<BTreeSet<_>>()
        )
    }

    #[test]
    fn p2_p2() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0 ],
            vec![ 3 ],
            vec![ 2 ]
        ]).unwrap();
        let pairing = greedy(&graph);

        assert_eq!(pairing.edges().collect::<BTreeSet<_>>(),
            [ (0, 1), (2, 3) ].iter().cloned().collect::<BTreeSet<_>>()
        )
    }

    #[test]
    fn p3() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();
        let pairing = greedy(&graph);

        assert_eq!(pairing.edges().collect::<BTreeSet<_>>(),
            [ (0, 1) ].iter().cloned().collect::<BTreeSet<_>>()
        )
    }

    #[test]
    fn p4() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1, 3 ],
            vec![ 2 ]
        ]).unwrap();
        let pairing = greedy(&graph);

        assert_eq!(pairing.edges().collect::<BTreeSet<_>>(),
            [ (0, 1), (2, 3) ].iter().cloned().collect::<BTreeSet<_>>()
        )
    }

    #[test]
    fn s3() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0, 2, 3 ],
            vec![ 1 ],
            vec![ 1 ]
        ]).unwrap();
        let pairing = greedy(&graph);

        assert_eq!(pairing.edges().collect::<BTreeSet<_>>(),
            [ (0, 1) ].iter().cloned().collect::<BTreeSet<_>>()
        )
    }

    #[test]
    fn c6() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1, 5 ],
            vec![ 0, 2 ],
            vec![ 1, 3 ],
            vec![ 2, 4 ],
            vec![ 3, 5 ],
            vec![ 4, 0 ]
        ]).unwrap();
        let pairing = greedy(&graph);

        assert_eq!(pairing.edges().collect::<BTreeSet<_>>(),
            [ (0, 1), (2, 3), (4, 5) ].iter().cloned().collect::<BTreeSet<_>>()
        )
    }
}