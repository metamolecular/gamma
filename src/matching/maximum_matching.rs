use crate::graph::{ Graph, DefaultGraph };
use super::error::Error;
use super::pairing::Pairing;
use super::forest::Forest;
use super::marker::Marker;

pub fn maximum_matching<'a, G: Graph>(
    graph: &'a G, pairing: &'a mut Pairing
) -> Result<(), Error> { // A1
    let path = find_augmenting_path(graph, pairing)?; // A2

    if path.len() > 0 { // A3
        pairing.augment(&path)?;
        maximum_matching(graph, pairing) // A4
    } else { // A5
        Ok(()) // A6
    }
}

fn find_augmenting_path<'a, G: Graph>(
    graph: &'a G, pairing: &'a mut Pairing
) -> Result<Vec<usize>, Error> { // B01
    let mut forest = Forest::new(); // B02
    let mut marker = Marker::new();

    for (sid, tid) in pairing.edges() { // B03
        // B04 missing
        marker.add_edge(sid, tid);  // B05
    } // B06

    for v in graph.nodes() {
        if !pairing.has_node(*v) { // B05
            forest.add_root(*v)?; // B06
        } // B07
    }

    loop {
        let v = match node_candidate(graph, &marker, &forest)? {
            Some(v) => v,
            None => break
        }; // B08

        // while there exists an umarked edge e = { v, w } do
        loop {
            let w = match edge_candidate(v, graph, &marker) {
                Some(w) => w,
                None => break
            }; // B09

            if !forest.has_node(w) { // B10
                let x = pairing.mate(w)?; // B11

                forest.add_edge(v, w)?; // B12
                forest.add_edge(w, x)?;
            } else { // B13
                if forest.odd(w)? { // B14
                    // do nothing
                } else { // B15
                    if forest.root(v) != forest.root(w) { // B16
                        let mut p1 = forest.path(v)?;
                        let mut p2 = forest.path(w)?;

                        p2.reverse();
                        p1.append(&mut p2); // B17

                        return Ok(p1); // B18
                    } else { // B19
                        // constract a blossom in G and look for the path in the
                        // contracted graph
                        let blossom = create_blossom(v, w, &forest)?;
                        println!("blossom path {:?}", blossom);
                        println!("matching {:?}", pairing.edges().collect::<Vec<_>>());
                        // let contracted_graph = contract_graph(&blossom, graph)?;
                        unimplemented!()
                    }
                }
            }

            marker.add_edge(v, w); // B28
        } // B29

        marker.add_node(v); // B30
    } // B31

    Ok(vec![ ]) // B32
} // B33

// while there is an unmarked vertex v in F with distance( v, root( v ) )
// even do
fn node_candidate<'a, G: Graph>(
    graph: &'a G, marker: &'a Marker, forest: &'a Forest
) -> Result<Option<usize>, Error> {
    for &v in graph.nodes() {
        if !marker.has_node(v) && forest.has_node(v) && forest.even(v)? {
            return Ok(Some(v));
        }
    }

    Ok(None)
}

fn edge_candidate<'a, G: Graph>(
    v: usize, graph: &'a G, marker: &Marker
) -> Option<usize> {
    for &w in graph.neighbors(v).unwrap() {
        if !marker.has_edge(v, w) {
            return Some(w);
        }
    }

    None
}

fn create_blossom<'a>(
    v: usize, w: usize, forest: &'a Forest
) -> Result<Vec<usize>, Error> {
    let left = forest.path(v)?;
    let mut right = forest.path(w)?;

    for i in 0..left.len().max(right.len()) {
        if i == left.len() {
            return Ok(right[(i - 1)..].to_vec());
        } else if i == right.len() {
            return Ok(left[(i - 1)..].to_vec());
        } else if left[i] != right[i] {
            let mut result = left[(i - 1)..].to_vec();

            right = right[(i..)].to_vec();

            right.reverse();
            result.append(&mut right);

            return Ok(result);
        }
    }

    panic!("invalid state");
}

fn contract_graph<'a, G: Graph>(
    blossom: &'a Vec<usize>, graph: &'a G
) -> Result<DefaultGraph, Error> {
    // need an id in result graph for blossom
    // return that id
    let mut result = DefaultGraph::new();

    for node in graph.nodes() {
        if !blossom.contains(node) {
            result.add_node(*node).unwrap();
        }
    }

    for (sid, tid) in graph.edges() {
        if blossom.contains(sid) {
            if !blossom.contains(tid) {
                // result.add_edge(blossom, target;)
            }
        } else if blossom.contains(tid) {

        } else {
            result.add_edge(*sid, *tid).unwrap();
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
    use super::*;

    #[test]
    fn empty() {
        let graph = DefaultGraph::new();
        let mut pairing = Pairing::new();
        
        assert_eq!(maximum_matching(&graph, &mut pairing), Ok(()));

        assert_eq!(pairing, Pairing::try_from(vec![ ]).unwrap())
    }

    #[test]
    fn p2() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0 ]
        ]).unwrap();
        let mut pairing = Pairing::new();

        assert_eq!(maximum_matching(&graph, &mut pairing), Ok(()));

        assert_eq!(pairing, Pairing::try_from(vec![ (0, 1) ]).unwrap())
    }

    #[test]
    fn p3() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1 ],
            vec![ 0, 2 ],
            vec![ 1 ]
        ]).unwrap();
        let mut pairing = Pairing::new();

        assert_eq!(maximum_matching(&graph, &mut pairing), Ok(()));

        assert_eq!(pairing, Pairing::try_from(vec![ (0, 1) ]).unwrap())
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

        assert_eq!(maximum_matching(&graph, &mut pairing), Ok(()));

        assert_eq!(pairing, Pairing::try_from(vec![
            (0, 1), (2, 3)
        ]).unwrap())
    }

    #[test]#[ignore]
    fn c5() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1, 4 ],
            vec![ 0, 2, 5 ],
            vec![ 1, 3 ],
            vec![ 2, 4 ],
            vec![ 3, 0 ],
            vec![ 1 ]
        ]).unwrap();
        let mut pairing = Pairing::new();

        assert_eq!(maximum_matching(&graph, &mut pairing), Ok(()));

        assert_eq!(pairing, Pairing::try_from(vec![
            (0, 1), (2, 3)
        ]).unwrap())
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

        assert_eq!(maximum_matching(&graph, &mut pairing), Ok(()));

        assert_eq!(pairing, Pairing::try_from(vec![
            (0, 1), (2, 3), (4, 5)
        ]).unwrap())
    }

    #[test]
    fn acenapthene() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1, 10 ],
            vec![ 2, 11, 0 ],
            vec![ 3, 1 ],
            vec![ 4, 2 ],
            vec![ 5, 3 ],
            vec![ 6, 11, 4 ],
            vec![ 7, 5 ],
            vec![ 8, 6 ],
            vec![ 9, 7 ],
            vec![ 10, 11, 8 ],
            vec![ 0, 9 ],
            vec![ 9, 5, 1 ]
        ]).unwrap();
        let mut pairing = Pairing::new();

        assert_eq!(maximum_matching(&graph, &mut pairing), Ok(()));

        assert_eq!(pairing, Pairing::try_from(vec![
            (0, 10), (1, 11), (2, 3), (4, 5), (6, 7), (8, 9)
        ]).unwrap())
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

        assert_eq!(maximum_matching(&graph, &mut pairing), Ok(()));

        assert_eq!(pairing, Pairing::try_from(vec![
            (0, 1), (2, 3)
        ]).unwrap())
    }
}