use crate::graph::{ Graph, DefaultGraph, Error };
use super::pairing::Pairing;

#[derive(Debug,PartialEq)]
pub struct Blossom {
    id: usize,
    path: Vec<usize>
}

impl Blossom {
    pub fn new(
        id: usize, mut left: Vec<usize>, mut right: Vec<usize>
    ) -> Self {
        for i in 0..left.len() {
            for j in 0..right.len() {
                if left[i] == right[j] {
                    let root = left[i];
                    left = left[0..i].to_vec();
                    right = right[0..j].to_vec();

                    right.reverse();

                    left.push(root);
                    left.append(&mut right);

                    return Self { id, path: left }
                }
            }
        }
        
        panic!("blossom root not found")
    }

    pub fn contract_graph<'a, G: Graph>(
        &self, graph: &'a G
    ) -> Result<DefaultGraph, Error> {
        let mut result = DefaultGraph::new();

        result.add_node(self.id)?;
    
        for id in graph.nodes() {
            if !self.path.contains(&id) {
                result.add_node(id)?;
            }
        }

        for (sid, tid) in graph.edges() {
            if self.path.contains(&sid) {
                if !self.path.contains(&tid) {
                    if !result.has_edge(self.id, tid)? {
                        result.add_edge(self.id, tid)?;
                    }
                }
            } else if self.path.contains(&tid) {
                if !result.has_edge(sid, self.id)? {
                    result.add_edge(sid, self.id)?;
                }
            } else {
                result.add_edge(sid, tid)?;
            }
        }
    
        Ok(result)
    }

    pub fn contract_pairing(&self, pairing: &Pairing) -> Pairing {
        let mut result = Pairing::new();

        for (sid, tid) in pairing.edges() {
            if self.path.contains(&sid) {
                if !self.path.contains(&tid) {
                    result.pair(self.id, tid);
                }
            } else if self.path.contains(&tid) {
                result.pair(sid, self.id);
            } else {
                result.pair(sid, tid);
            }
        }

        result
    }

    pub fn lift<'a, G: Graph>(
        &self, path: Vec<usize>, graph: &'a G
    ) -> Vec<usize> {
        let index = match path.iter().position(|&pid| pid == self.id) {
            Some(index) => index,
            None => return path
            // None => panic!("path missing blossom id: {}", self.id)
        };
        let left = path[0..index].to_vec();
        let right = path[(index + 1)..].to_vec();

        if left.is_empty() && right.is_empty() {
            self.path.to_vec()
        } else if !left.is_empty() && right.is_empty() {
            self.lift_left_blossom(left, graph)
        } else if left.is_empty() && !right.is_empty() {
            self.lift_blossom_right(right, graph)
        } else {
            self.lift_left_blossom_right(left, right, graph)
        }
    }

    fn lift_left_blossom<'a, G: Graph>(
        &self, mut left: Vec<usize>, graph: &'a G
    ) -> Vec<usize> {
        let sid = left.last().unwrap();
        let mut copy = self.path.to_vec();

        while !graph.has_edge(*sid, copy[0]).unwrap() {
            copy.rotate_right(1);
        }

        left.append(&mut copy);

        left
    }

    fn lift_blossom_right<'a, G: Graph>(
        &self, mut right: Vec<usize>, graph: &'a G
    ) -> Vec<usize> {
        let tid = right[0];
        let mut copy = self.path.to_vec();

        while !graph.has_edge(*copy.last().unwrap(), tid).unwrap() {
            copy.rotate_right(1);
        }

        copy.append(&mut right);

        copy
    }

    fn lift_left_blossom_right<'a, G: Graph>(
        &self, left: Vec<usize>, right: Vec<usize>, graph: &'a G
    ) -> Vec<usize> {
        let &sid = left.last().unwrap();
        let mut forward_blossom = self.path.to_vec();
        let mut forward = left.to_vec();
        
        while !graph.has_edge(sid, forward_blossom[0]).unwrap() {
            forward_blossom.rotate_right(1);
        }
    
        let &tid = &right[0];

        for &bid in &forward_blossom {
            forward.push(bid);

            if graph.has_edge(bid, tid).unwrap() {
                break;
            }
        }

        forward.extend(right.iter());

        if forward.len() % 2 == 0 {
            return forward
        }

        let mut reverse = left.to_vec();
        let mut reverse_blossom = self.path.to_vec();

        reverse_blossom.reverse();

        while !graph.has_edge(sid, reverse_blossom[0]).unwrap() {
            reverse_blossom.rotate_right(1);
        }

        for &bid in &reverse_blossom {
            reverse.push(bid);

            if graph.has_edge(bid, tid).unwrap() {
                break;
            }
        }

        reverse.extend(right.iter());

        reverse
    }
}

#[cfg(test)]
mod new {
    use super::*;

    #[test]
    #[should_panic(expected="blossom root not found")]
    fn different_roots() {
        Blossom::new(1, vec![ 2, 1, 0 ], vec![ 5, 4, 3 ]);
    }

    #[test]
    fn root_at_right() {
        let blossom = Blossom::new(1, vec![ 2, 1, 0 ], vec![ 5, 4, 0 ]);

        assert_eq!(blossom.path, vec![ 2, 1, 0, 4, 5 ])
    }

    #[test]
    fn root_inside() {
        let blossom = Blossom::new(1, vec![
            4, 3, 2, 1, 0
        ], vec![
            7, 6, 2, 1, 0
        ]);

        assert_eq!(blossom.path, vec![ 4, 3, 2, 6, 7 ])
    }
}

#[cfg(test)]
mod contract_graph {
    use std::convert::TryFrom;
    use super::*;

    #[test]
    fn butterfly_tid_inside() {
        let graph = DefaultGraph::try_from(vec![
            (0, 1), (1, 2), (2, 0), (3, 2), (3, 1)
        ]).unwrap();
        let blossom = Blossom::new(4, vec![0], vec![ 1, 2, 0 ]);
        let contracted = blossom.contract_graph(&graph);

        assert_eq!(contracted, DefaultGraph::try_from(vec![
            (3, 4)
        ]))
    }

    #[test]
    fn butterfly_sid_inside() {
        let graph = DefaultGraph::try_from(vec![
            (0, 1), (1, 2), (2, 0), (2, 3), (1, 3)
        ]).unwrap();
        let blossom = Blossom::new(4, vec![0], vec![ 1, 2, 0 ]);
        let contracted = blossom.contract_graph(&graph);

        assert_eq!(contracted, DefaultGraph::try_from(vec![
            (3, 4)
        ]))
    }

    #[test]
    fn sid_inside() {
        let graph = DefaultGraph::try_from(vec![
            (0, 1), (1, 2), (2, 3), (3, 4), (4, 0), (4, 5), (5, 6)
        ]).unwrap();
        let blossom = Blossom::new(7, vec![ 4, 0, 1 ], vec![ 3, 2, 1 ]);
        let contracted = blossom.contract_graph(&graph);

        assert_eq!(contracted, DefaultGraph::try_from(vec![
            (6, 5), (5, 7)
        ]))
    }

    #[test]
    fn tid_inside() {
        let graph = DefaultGraph::try_from(vec![
            (0, 1), (1, 2), (2, 3), (3, 4), (4, 0), (5, 4), (5, 6)
        ]).unwrap();
        let blossom = Blossom::new(7, vec![ 4, 0, 1 ], vec![ 3, 2, 1 ]);
        let contracted = blossom.contract_graph(&graph);

        assert_eq!(contracted, DefaultGraph::try_from(vec![
            (6, 5), (5, 7)
        ]))
    }

    #[test]
    fn example_causes_double_edge() {
        // one way to force a dobule-edge for contracted graph
        let graph = DefaultGraph::try_from(vec![
            (0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7), (7, 8),
            (8, 2), (6, 1)
        ]).unwrap();
        let blossom = Blossom::new(9, vec![ 8, 2, 3, 4 ], vec![ 7, 6, 5, 4 ]);
        let contracted = blossom.contract_graph(&graph);

        assert_eq!(contracted, DefaultGraph::try_from(vec![
            (0, 1), (1, 9)
        ]))
    }
}

#[cfg(test)]
mod contract_pairing {
    use super::*;

    #[test]
    fn sid_inside() {
        let blossom = Blossom::new(5, vec![ 2, 1, 0 ], vec![ 4, 3, 0 ]);
        let mut pairing = Pairing::new();

        pairing.pair(7, 8);
        pairing.pair(1, 6);

        let mut expected = Pairing::new();

        expected.pair(7, 8);
        expected.pair(5, 6);

        assert_eq!(blossom.contract_pairing(&pairing), expected);
    }

    #[test]
    fn tid_inside() {
        let blossom = Blossom::new(5, vec![ 2, 1, 0 ], vec![ 4, 3, 0 ]);
        let mut pairing = Pairing::new();

        pairing.pair(7, 8);
        pairing.pair(6, 1);

        let mut expected = Pairing::new();

        expected.pair(7, 8);
        expected.pair(6, 5);

        assert_eq!(blossom.contract_pairing(&pairing), expected);
    }

    #[test]
    fn sid_tid_inside() {
        let blossom = Blossom::new(5, vec![ 2, 1, 0 ], vec![ 4, 3, 0 ]);
        let mut pairing = Pairing::new();

        pairing.pair(7, 8);
        pairing.pair(2, 1);

        let mut expected = Pairing::new();

        expected.pair(7, 8);

        assert_eq!(blossom.contract_pairing(&pairing), expected);
    }
}

#[cfg(test)]
mod lift {
    use::std::convert::TryFrom;
    use super::*;

    #[test]
    fn missing_blossom_id() {
        let graph = DefaultGraph::try_from(vec![
            (1, 2), (2, 3), (3, 4), (4, 5), (5, 1)
        ]).unwrap();
        let blossom = Blossom::new(5, vec![ 2, 1, 0 ], vec![ 4, 3, 0 ]);
        let path = vec![ 8, 9, 10, 11 ];

        assert_eq!(blossom.lift(path, &graph), vec![
            8, 9, 10, 11
        ])
    }

    #[test]
    fn none_blossom_none() {
        let graph = DefaultGraph::try_from(vec![
            (0, 1), (1, 2), (2, 3), (3, 4), (4, 0)
        ]).unwrap();
        let blossom = Blossom::new(5, vec![ 2, 1, 0 ], vec![ 4, 3, 0 ]);
        let path = vec![ 5 ];

        assert_eq!(blossom.lift(path, &graph), vec![
            2, 1, 0, 3, 4
        ])
    }

    #[test]
    fn left_blossom_none() {
        let graph = DefaultGraph::try_from(vec![
            (0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 1)
        ]).unwrap();
        let blossom = Blossom::new(6, vec![ 1, 2, 3 ], vec![ 5, 4, 3 ]);
        let path = vec![ 0, 6 ];

        assert_eq!(blossom.lift(path, &graph), vec![
            0, 1, 2, 3, 4, 5
        ])
    }

    #[test]
    fn left_blossom_none_rotated_twice() {
        let graph = DefaultGraph::try_from(vec![
            (0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 1)
        ]).unwrap();
        let blossom = Blossom::new(6, vec![ 2, 3, 4 ], vec![ 1, 5, 4 ]);
        let path = vec![ 0, 6 ];

        assert_eq!(blossom.lift(path, &graph), vec![
            0, 1, 2, 3, 4, 5
        ])
    }

    #[test]
    fn none_blossom_right() {
        let graph = DefaultGraph::try_from(vec![
            (0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 1)
        ]).unwrap();
        let blossom = Blossom::new(6, vec![ 2, 3, 4 ], vec![ 1, 5, 4 ]);
        let path = vec![ 6, 0 ];

        assert_eq!(blossom.lift(path, &graph), vec![
            2, 3, 4, 5, 1, 0
        ])
    }

    #[test]
    fn left_blossom_right() {
        let graph = DefaultGraph::try_from(vec![
            (0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 1), (3, 6)
        ]).unwrap();
        let blossom = Blossom::new(7, vec![ 2, 3, 4 ], vec![ 1, 5, 4 ]);
        let path = vec![ 0, 7, 6 ];

        assert_eq!(blossom.lift(path, &graph), vec![
            0, 1, 5, 4, 3, 6
        ])
    }

    #[test]
    fn left_blossom_right_shifted() {
        let graph = DefaultGraph::try_from(vec![
            (0, 5), (5, 1), (1, 2), (2, 3), (3, 4), (4, 5), (3, 6)
        ]).unwrap();
        let blossom = Blossom::new(7, vec![ 2, 3, 4 ], vec![ 1, 5, 4 ]);
        let path = vec![ 0, 7, 6 ];

        assert_eq!(blossom.lift(path, &graph), vec![
            0, 5, 1, 2, 3, 6
        ])
    }
}