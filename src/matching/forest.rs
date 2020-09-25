use std::collections::HashMap;
use std::collections::hash_map::Entry::{ Occupied, Vacant };

#[derive(Debug,PartialEq)]
pub struct Forest {
    parents: HashMap<usize, Entry>,
    nodes: Vec<usize>
}

impl Forest {
    pub fn new() -> Self {
        Self {
            parents: HashMap::new(),
            nodes: Vec::new()
        }
    }

    pub fn add_root(&mut self, root: usize) {
        match self.parents.entry(root) {
            Vacant(entry) => {
                entry.insert(Entry { parent: None, parity: Parity::Even });
                self.nodes.push(root);
            },
            Occupied(_) => panic!("duplicate node: {}", root)
        }
    }

    pub fn add_edge(&mut self, parent: usize, node: usize) {
        let parity = match self.parents.get(&parent) {
            Some(entry) => entry.parity.invert(),
            None => panic!("missing parent: {}", parent)
        };

        match self.parents.entry(node) {
            Vacant(entry) => {
                entry.insert(Entry { parent: Some(parent), parity: parity });
                self.nodes.push(node);
            },
            Occupied(_) => panic!("duplicate node: {}", node)
        }
    }

    pub fn even_nodes(&self) -> impl Iterator<Item=usize> + '_ {
        self.nodes.iter().filter(move |node| {
            self.parents.get(node).unwrap().parity == Parity::Even
        }).cloned()
    }

    pub fn path(&self, node: usize) -> Option<Vec<usize>> {
        let mut parent = match self.parents.get(&node) {
            None => return None,
            Some(parent) => parent.parent
        };
        let mut result = vec![ node ];

        loop {
            match parent {
                None => {
                    return Some(result)
                },
                Some(id) => {
                    result.push(id);

                    parent = match self.parents.get(&id) {
                        None => panic!("missing parent: {}", id),
                        Some(parent) => parent.parent
                    }
                }
            }
        }
    }
}

#[derive(Debug,PartialEq)]
struct Entry {
    parent: Option<usize>,
    parity: Parity
}

#[derive(Debug,PartialEq)]
enum Parity {
    Even,
    Odd
}

impl Parity {
    fn invert(&self) -> Self {
        match self {
            Parity::Even => Parity::Odd,
            Parity::Odd => Parity::Even
        }
    }
}

#[cfg(test)]
mod add_root {
    use super::*;

    #[test]
    #[should_panic(expected="duplicate node: 0")]
    fn duplicate() {
        let mut forest = Forest::new();

        forest.add_root(0);
        forest.add_root(0);
    }
}

#[cfg(test)]
mod add_edge {
    use super::*;

    #[test]
    #[should_panic(expected="missing parent: 0")]
    fn parent_outside() {
        let mut forest = Forest::new();

        forest.add_edge(0, 1);
    }

    #[test]
    #[should_panic(expected="duplicate node: 1")]
    fn duplicate_node() {
        let mut forest = Forest::new();

        forest.add_root(0);
        forest.add_root(1);

        forest.add_edge(0, 1);
    }
}

#[cfg(test)]
mod path {
    use super::*;

    #[test]
    fn outside() {
        let forest = Forest::new();

        assert_eq!(forest.path(0), None)
    }

    #[test]
    fn root() {
        let mut forest = Forest::new();

        forest.add_root(0);

        assert_eq!(forest.path(0), Some(vec![ 0 ]))
    }

    #[test]
    fn child() {
        let mut forest = Forest::new();

        forest.add_root(0);
        forest.add_edge(0, 1);

        assert_eq!(forest.path(1), Some(vec![ 1, 0 ]))
    }

    #[test]
    fn grandchild() {
        let mut forest = Forest::new();

        forest.add_root(0);
        forest.add_edge(0, 1);
        forest.add_edge(1, 2);

        assert_eq!(forest.path(2), Some(vec![ 2, 1, 0 ]))
    }

    #[test]
    fn grandchild_with_branching_before() {
        let mut forest = Forest::new();

        forest.add_root(0);
        forest.add_edge(0, 1);
        forest.add_edge(0, 2);
        forest.add_edge(0, 3);
        forest.add_edge(1, 4);
        forest.add_edge(2, 5);
        forest.add_edge(3, 6);
        forest.add_edge(5, 7);

        assert_eq!(forest.path(7), Some(vec![ 7, 5, 2, 0 ]))
    }

    #[test]
    fn grandchild_and_other_path() {
        let mut forest = Forest::new();

        forest.add_root(0);
        forest.add_edge(0, 1);
        forest.add_edge(1, 2);
        forest.add_root(3);
        forest.add_edge(3, 4);
        forest.add_edge(3, 5);
        forest.add_edge(5, 6);

        assert_eq!(forest.path(2), Some(vec![ 2, 1, 0 ]))
    }
}

#[cfg(test)]
mod even_nodes {
    use std::collections::HashSet;
    use std::iter::FromIterator;
    use super::*;

    #[test]
    fn empty() {
        let forest = Forest::new();

        assert_eq!(
            forest.even_nodes().collect::<HashSet<_>>(),
            HashSet::from_iter([ ].iter().cloned())
        )
    }

    #[test]
    fn two_root() {
        let mut forest = Forest::new();

        forest.add_root(0);
        forest.add_root(1);

        assert_eq!(
            forest.even_nodes().collect::<HashSet<_>>(),
            HashSet::from_iter([ 0, 1 ].iter().cloned())
        )
    }

    #[test]
    fn complex_tree() {
        let mut forest = Forest::new();

        forest.add_root(0);
        forest.add_edge(0, 1);
        forest.add_edge(1, 2);
        forest.add_root(3);
        forest.add_edge(3, 4);
        forest.add_edge(4, 5);
        forest.add_edge(4, 6);

        assert_eq!(
            forest.even_nodes().collect::<HashSet<_>>(),
            HashSet::from_iter([ 0, 3, 2, 5, 6 ].iter().cloned())
        )
    }
}