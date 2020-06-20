use std::hash::Hash;
use std::collections::HashSet;

use crate::traversal::{ depth_first, to_adjacency };
use crate::graph::{ HashGraph, Graph };

pub fn components<'a, N: 'a+Clone+Hash+Eq, G: Graph<'a, N>>(
    graph: &'a G
) -> Components<N, G> {
    Components {
        visited: HashSet::new(),
        iter: graph.nodes(),
        graph: graph
    }
}

pub struct Components<'a, N: 'a, G: Graph<'a, N>> {
    visited: HashSet<&'a N>,
    iter: <G as Graph<'a, N>>::NodeIterator,
    graph: &'a G
}

impl<'a, N: Hash+Eq, G: Graph<'a, N>> Iterator for Components<'a, N, G> {
    type Item = HashGraph<&'a N>;

    fn next(&mut self) -> Option<Self::Item> {
        let root = loop {
            match self.iter.next() {
                Some(root) => {
                    if self.visited.contains(root) {
                        continue;
                    } else {
                        break Some(root);
                    }
                },
                None => break None
            }
        };

        match root {
            Some(root) => {
                let traversal = depth_first(self.graph, root).unwrap();
                let adjacency = to_adjacency(traversal).unwrap();
                let subgraph = HashGraph::build(adjacency).unwrap();

                for node in subgraph.nodes() {
                    self.visited.insert(node);
                }

                Some(subgraph)
            },
            None => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::IndexGraph;

    #[test]
    fn p1() {
        let graph = IndexGraph::build(vec![
            vec![ ]
        ]).unwrap();
        let components = components(&graph).collect::<Vec<_>>();
        
        assert_eq!(components.len(), 1);
    }

    #[test]
    fn p1_p1() {
        let graph = IndexGraph::build(vec![
            vec![ ],
            vec![ ]
        ]).unwrap();
        let components = components(&graph).collect::<Vec<_>>();

        assert_eq!(components.len(), 2);
    }

    #[test]
    fn p2() {
        let graph = IndexGraph::build(vec![
            vec![ 1 ],
            vec![ 0 ]
        ]).unwrap();
        let components = components(&graph).collect::<Vec<_>>();
        
        assert_eq!(components.len(), 1);
    }

    #[test]
    fn p2_p1() {
        let graph = IndexGraph::build(vec![
            vec![ 1 ],
            vec![ 0 ],
            vec![ ],
        ]).unwrap();
        let components = components(&graph).collect::<Vec<_>>();
        
        assert_eq!(components.len(), 2);
    }

    #[test]
    fn p2_p2_p2() {
        let graph = IndexGraph::build(vec![
            vec![ 1 ],
            vec![ 0 ],
            vec![ 3 ],
            vec![ 2 ],
            vec![ 5 ],
            vec![ 4 ]
        ]).unwrap();
        let components = components(&graph).collect::<Vec<_>>();
        
        assert_eq!(components.len(), 3);
    }
}