use crate::graph::{ Graph };
use super::pairing::Pairing;
use super::forest::Forest;
use super::marker::Marker;
use super::blossom::Blossom;

/// Performs a maximum matching over the Graph.
/// 
/// A greedy matching can be used as a starting point to maximum matching, so
/// it may be helpful to try a greedy matching, falling back to maximum
/// matching if the matching isn't perfect.
/// 
/// For more on matching, see: *[The Maximum Matching Problem](https://depth-first.com/articles/2019/04/02/the-maximum-matching-problem/)*.
/// 
/// ```rust
/// use std::convert::TryFrom;
/// use std::collections::BTreeSet;
/// use gamma::graph::{ Error, DefaultGraph };
/// use gamma::matching::{ maximum_matching, Pairing };
/// 
/// fn main() -> Result<(), Error> {
///      let graph = DefaultGraph::try_from(vec![
///          (0, 1), (1, 2), (2, 3)
///      ]).unwrap();
///      let mut pairing = Pairing::new();
///
///      maximum_matching(&graph, &mut pairing);
///
///      assert_eq!(
///          pairing.edges().collect::<BTreeSet<_>>(),
///            [ (0, 1), (2, 3) ].iter().cloned().collect::<BTreeSet<_>>()
///      );
///
///     Ok(())
/// }
/// ```
pub fn maximum_matching<'a, G: Graph>(
    graph: &'a G, pairing: &'a mut Pairing
) {
    while let Some(path) = augmenting_path(graph, pairing) {
        pairing.augment(path);
        maximum_matching(graph, pairing);
    }
}

fn augmenting_path<'a, G: Graph>(
    graph: &'a G, pairing: &'a mut Pairing
) -> Option<Vec<usize>> {
    let mut forest = Forest::new();
    let mut marker = Marker::new();

    for (sid, tid) in pairing.edges() {
        marker.mark_edge(sid, tid);
    }

    for v in graph.nodes() {
        if !pairing.has_node(*v) {
            forest.add_root(*v);
        }
    }

    loop {
        let v = match some_v(&forest, &marker) {
            Some(node) => node,
            None => break
        };

        loop {
            let w = match some_w(v, graph, &marker) {
                Some(node) => node,
                None => break
            };
            
            match forest.path(w) {
                Some(path_w) => {
                    if path_w.len() % 2 == 1 {
                        return even_path(v, path_w, graph, &forest, pairing)
                    }
                },
                None => {
                    forest.add_edge(v, w);
                    forest.add_edge(w, pairing.mate(w));
                }
            }

            marker.mark_edge(v, w);
        }

        marker.mark_node(v);
    }

    None
}

fn some_v(forest: &Forest, marker: &Marker) -> Option<usize> {
    forest.even_nodes().find(|id| !marker.has_node(*id))
}

fn some_w<G: Graph>(v: usize, graph: &G, marker: &Marker) -> Option<usize> {
    graph.neighbors(v)
        .expect("neighbors of v").iter().cloned()
        .find(|&id| !marker.has_edge(v, id))
}

fn even_path<G: Graph>(
    v: usize,
    mut path_w: Vec<usize>,
    graph: &G,
    forest: &Forest,
    pairing: &Pairing
) -> Option<Vec<usize>> {
    let mut path_v = forest.path(v).expect("v not in forest");

    if path_v.last() == path_w.last() {
        process_blossom(path_v, path_w, graph, pairing)
    } else {
        path_v.reverse();
        path_v.append(&mut path_w);

        Some(path_v)
    }
}

fn process_blossom<G:Graph>(
    left: Vec<usize>, right: Vec<usize>, graph: &G, pairing: &Pairing
) -> Option<Vec<usize>> {
    let max_id = graph.nodes().iter().max().expect("no max id");
    let blossom =  Blossom::new(max_id + 1, left, right);
    let contracted_graph = blossom.contract_graph(graph).expect("bad graph");
    let mut contracted_pairing = blossom.contract_pairing(&pairing);

    match augmenting_path(&contracted_graph, &mut contracted_pairing) {
        Some(path) => Some(blossom.lift(path, graph)),
        None => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::convert::TryFrom;
    use crate::graph::DefaultGraph;

    #[test]
    fn empty() {
        let graph = DefaultGraph::new();
        let mut pairing = Pairing::new();
        
        maximum_matching(&graph, &mut pairing);

        assert_eq!(
            pairing.edges().collect::<HashMap<_,_>>(),
            HashMap::new()
        )
    }

    #[test]
    fn p2() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0 ]
        ]).unwrap();
        let mut pairing = Pairing::new();

        maximum_matching(&graph, &mut pairing);

        assert_eq!(
            pairing.edges().collect::<HashMap<_,_>>(),
            [ (0, 1) ].iter().cloned().collect::<HashMap<_,_>>()
        )
    }

    #[test]
    fn p3() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();
        let mut pairing = Pairing::new();

        maximum_matching(&graph, &mut pairing);

        assert_eq!(
            pairing.edges().collect::<HashMap<_,_>>(),
            [ (0, 1) ].iter().cloned().collect::<HashMap<_,_>>()
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
        let mut pairing = Pairing::new();

        maximum_matching(&graph, &mut pairing);

        assert_eq!(
            pairing.edges().collect::<HashMap<_,_>>(),
            [ (0, 1), (2, 3) ].iter().cloned().collect::<HashMap<_,_>>()
        )
    }

    #[test]
    fn c5() {
        let graph = DefaultGraph::try_from(vec![
            (0, 1), (1, 2), (2, 3), (3, 4), (4, 0)
        ]).unwrap();
        let mut pairing = Pairing::new();

        maximum_matching(&graph, &mut pairing);

        assert_eq!(
            pairing.edges().collect::<HashMap<_,_>>(),
            [ (0, 1), (2, 3) ].iter().cloned().collect::<HashMap<_,_>>()
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
        let mut pairing = Pairing::new();

        maximum_matching(&graph, &mut pairing);

        assert_eq!(
            pairing.edges().collect::<HashMap<_,_>>(),
            [ (0, 1), (2, 3), (4, 5) ].iter().cloned().collect::<HashMap<_,_>>()
        )
    }

    #[test]
    fn acenapthene() {
        let graph = DefaultGraph::try_from(vec![
            (0, 1), (1, 2), (2, 3), (3, 4), (4, 5),
            (5, 6), (6, 7), (7, 8), (8, 9), (9, 10),
            (10, 0), (11, 5), (11, 1), (11, 9)
        ]).unwrap();

        let mut pairing = Pairing::new();

        maximum_matching(&graph, &mut pairing);

        assert_eq!(
            pairing.edges().collect::<HashMap<_,_>>(),
            [ (0, 10), (1, 11), (2, 3), (4, 5), (6, 7), (8, 9) ]
                .iter().cloned().collect::<HashMap<_,_>>()
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
        let mut pairing = Pairing::new();

        maximum_matching(&graph, &mut pairing);

        assert_eq!(
            pairing.edges().collect::<HashMap<_,_>>(),
            [ (0, 1), (2, 3) ].iter().cloned().collect::<HashMap<_,_>>()
        )
    }

    #[test]
    fn path_through_5_blossom() {
        let graph = DefaultGraph::try_from(vec![
            (0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 1),
            (4, 6), (6, 7)
        ]).unwrap();
        let mut pairing = Pairing::new();

        pairing.pair(2, 3);
        pairing.pair(1, 5);
        pairing.pair(4, 6);

        maximum_matching(&graph, &mut pairing);

        assert_eq!(
            pairing.edges().collect::<HashMap<_,_>>(),
            [ (0, 1), (2, 3), (4, 5), (6, 7) ].iter().cloned().collect::<HashMap<_,_>>()
        )
    }

    // https://doi.org/10.2991/icieac-14.2014.14
    #[test]
    fn yu_zhonge() {
        let graph = DefaultGraph::try_from(vec![
            (1, 2), (2, 3),
            (3, 13), (13, 14), (14, 4), (4, 5), (5, 6), (6, 7),
            (7, 8), (8, 9), (9, 10), (10, 11), (11, 5),
            (10, 18), (18, 17), (17, 16), (16, 15), (15, 14),
            (18, 19), (19, 20), (20, 21), (21, 22), (22, 16),
            (4, 3), (13, 12)
        ]).unwrap();
        let mut pairing = Pairing::new();

        pairing.pair(1, 2);
        pairing.pair(3, 13);
        pairing.pair(4, 14);
        pairing.pair(5, 6);
        pairing.pair(7, 8);
        pairing.pair(9, 10);
        pairing.pair(15, 16);
        pairing.pair(17, 18);
        pairing.pair(19, 20);
        pairing.pair(21, 22);

        maximum_matching(&graph, &mut pairing);

        assert_eq!(
            pairing.edges().collect::<HashMap<_,_>>(), [
                (1, 2), (3, 4), (6, 7), (8, 9),
                (5, 11), (10, 18), (19, 20), (21, 22),
                (16, 17), (14, 15), (12, 13)
            ].iter().cloned().collect::<HashMap<_,_>>()
        )
    }

    #[test]
    fn coronene_6_5() {
        let graph = DefaultGraph::try_from(vec![
            (6, 7), (7, 8), (8, 9), (9, 10), (10, 11),
            (11, 12), (12, 13), (13, 14), (14, 15), (15, 16),
            (16, 17), (17, 6),
            (6, 0), (8, 1), (10, 2), (12, 3), (14, 4), (16, 5),
            (0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 8)
        ]).unwrap();
        let mut pairing = Pairing::new();

        maximum_matching(&graph, &mut pairing);

        assert_eq!(
            pairing.edges().collect::<HashMap<_,_>>(), [
                (0, 1), (2, 3), (4, 5), (6, 7), (8, 9),
                (10, 11), (12, 13), (14, 15), (16, 17)
            ].iter().cloned().collect::<HashMap<_,_>>()
        )
    }

    #[test]
    fn coronene_3_5() {
        let graph = DefaultGraph::try_from(vec![
            (4, 5), (5, 6), (6, 7), (7, 8), (8, 9), (9, 10),
            (10, 11), (11, 4),
            (0, 4), (1, 6), (2, 8), (3, 10),
            (0, 1), (1, 2), (2, 3), (3, 0)
        ]).unwrap();
        let mut pairing = Pairing::new();

        maximum_matching(&graph, &mut pairing);

        assert_eq!(
            pairing.edges().collect::<HashMap<_,_>>(), [
                (4, 5), (0, 1), (8, 9), (10, 11), (2, 3), (6, 7)
            ].iter().cloned().collect::<HashMap<_,_>>()
        )
    }

    #[test]
    fn c60() {
        let graph = DefaultGraph::try_from(vec![
            (29, 30), (30, 43), (43, 44), (44, 55), (55, 29),
            (29, 28), (31, 30), (43, 42), (44, 45), (55, 54),
            (28, 57), (57, 56), (56, 31), (31, 32), (32, 33),
            (33, 42), (42, 41), (41, 40), (40, 45), (45, 46),
            (46, 47), (47, 54), (54, 26), (26, 27), (27, 28),
            (57, 7),  (56, 4),  (32, 3),  (33, 34), (41, 36),
            (40, 39), (46, 51), (47, 48), (26, 25), (27, 8),
            (7, 6),   (6, 5),   (5, 4),   (4, 3),   (3, 2),
            (2, 35),  (35, 34), (34, 36), (36, 37), (37, 38),
            (38, 39), (39, 51), (51, 50), (50, 49), (49, 48),
            (48, 25), (25, 24), (24, 9),  (9, 8),   (8, 7),
            (6, 11),  (5, 0),   (2, 1),   (35, 16), (37, 17),
            (38, 53), (50, 52), (49, 22), (24, 23), (9, 10),
            (11, 12), (12, 0),  (0, 1),   (1, 15),  (15, 16),
            (16, 17), (17, 18), (18, 53), (53, 52), (52, 21),
            (21, 22), (22, 23), (23, 58), (58, 10), (10, 11),
            (12, 13), (15, 14), (18, 19), (21, 20), (58, 59),
            (13, 14), (14, 19), (19, 20), (20, 59), (59, 13)
        ]).unwrap();
        let mut pairing = Pairing::new();

        maximum_matching(&graph, &mut pairing);

        assert_eq!(
            pairing.edges().collect::<HashMap<_,_>>(), [
                (48, 49), (1, 2),   (17, 37), (6, 7),   (54, 55), (31, 56),
                (19, 20), (28, 57), (26, 27), (22, 23), (29, 30), (4, 5),
                (50, 51), (34, 35), (24, 25), (10, 11), (43, 44), (3, 32),
                (46, 47), (13, 14), (33, 42), (58, 59), (15, 16), (8, 9),
                (36, 41), (38, 39), (21, 52), (40, 45), (0, 12),  (18, 53)
            ].iter().cloned().collect::<HashMap<_,_>>()
        )
    }
}