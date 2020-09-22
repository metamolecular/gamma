use std::collections::HashMap;
use std::collections::hash_map::Entry::{ Occupied, Vacant };

use super::Error;

#[derive(Debug,PartialEq)]
pub struct Forest {
    parents: HashMap<usize, Option<usize>>
}

impl Forest {
    pub fn new() -> Self {
        Self {
            parents: HashMap::new()
        }
    }

    pub fn add_root(&mut self, root: usize) -> Result<(), Error> {
        match self.parents.entry(root) {
            Vacant(entry) => {
                entry.insert(None);
            },
            Occupied(_) => return Err(Error::DuplicateRoot(root))
        }

        Ok(())
    }

    pub fn add_edge(&mut self, parent: usize, node: usize) -> Result<(), Error> {
        if !self.parents.contains_key(&parent) {
            return Err(Error::MissingNode(parent));
        }

        if self.parents.contains_key(&node) {
            return Err(Error::Duplicate(parent, node));
        }

        self.parents.insert(node, Some(parent));

        Ok(())
    }

    pub fn has_node(&self, node: usize) -> bool {
        self.parents.contains_key(&node)
    }

    pub fn path(&self, node: usize) -> Result<Vec<usize>, Error> {
        let mut parent = match self.parents.get(&node) {
            None => return Err(Error::MissingNode(node)),
            Some(parent) => parent
        };
        let mut result = vec![ node ];

        loop {
            match parent {
                None => {
                    result.reverse();

                    return Ok(result);
                },
                Some(id) => {
                    result.push(*id);

                    parent = match self.parents.get(id) {
                        None => return Err(Error::MissingNode(node)),
                        Some(parent) => parent
                    }
                }
            }
        }
    }

    pub fn even(&self, node: usize) -> Result<bool, Error> {
        let path = self.path(node)?;

        Ok(path.len() % 2 == 1)
    }

    pub fn odd(&self, node: usize) -> Result<bool, Error> {
        let path = self.path(node)?;

        Ok(path.len() % 2 == 0)
    }

    pub fn root(&self, node: usize) -> Result<usize, Error> {
        let path = self.path(node)?;

        Ok(path[0])
    }

    pub fn blossom<'a>(
        &self, v: usize, w: usize
    ) -> Result<Vec<usize>, Error> {
        let left = self.path(v)?;
        let mut right = self.path(w)?;

        if left.len() != right.len() {
            return Err(Error::MissingBlossom(v, w))
        }

        for i in 0..left.len() {
            if left[i] != right[i] {
                if i > 0 {
                    let mut result = left[i..].to_vec();

                    result.reverse();
                    result.push(left[i - 1]);
                    result.append(&mut right[i..].to_vec());

                    return Ok(result)
                } else {
                    return Err(Error::MissingBlossom(v, w))
                }
            }
        };

        panic!("invalid state")
    }

    pub fn path_from2<'a>(
        &self, v: usize, w: usize
    ) -> Result<Vec<usize>, Error> {
        let left = self.path(v)?;
        let mut right = self.path(w)?;
    
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
}

#[cfg(test)]
mod add_root {
    use super::*;

    #[test]
    fn duplicate() {
        let mut forest = Forest::new();

        assert_eq!(forest.add_root(0), Ok(()));
    }
}

#[cfg(test)]
mod add_edge {
    use super::*;

    #[test]
    fn parent_not_member() {
        let mut forest = Forest::new();

        assert_eq!(forest.add_edge(0, 1), Err(Error::MissingNode(0)))
    }

    #[test]
    fn node_is_member() {
        let mut forest = Forest::new();

        assert_eq!(forest.add_root(0), Ok(()));
        assert_eq!(forest.add_root(1), Ok(()));

        assert_eq!(forest.add_edge(0, 1), Err(Error::Duplicate(0, 1)))
    }
}

#[cfg(test)]
mod has_node {
    use super::*;

    #[test]
    fn outside() {
        let forest = Forest::new();

        assert_eq!(forest.has_node(0), false);
    }

    #[test]
    fn root() {
        let mut forest = Forest::new();

        assert_eq!(forest.add_root(0), Ok(()));

        assert_eq!(forest.has_node(0), true);
    }

    #[test]
    fn child() {
        let mut forest = Forest::new();

        assert_eq!(forest.add_root(0), Ok(()));
        assert_eq!(forest.add_edge(0, 1), Ok(()));

        assert_eq!(forest.has_node(1), true);
    }
}

#[cfg(test)]
mod path {
    use super::*;

    #[test]
    fn outside() {
        let forest = Forest::new();

        assert_eq!(forest.path(0), Err(Error::MissingNode(0)));
    }

    #[test]
    fn root() {
        let mut forest = Forest::new();

        assert_eq!(forest.add_root(0), Ok(()));

        assert_eq!(forest.path(0), Ok(vec![ 0 ]))
    }

    #[test]
    fn child() {
        let mut forest = Forest::new();

        assert_eq!(forest.add_root(0), Ok(()));
        assert_eq!(forest.add_edge(0, 1), Ok(()));

        assert_eq!(forest.path(1), Ok(vec![ 0, 1 ]))
    }

    #[test]
    fn grandchild() {
        let mut forest = Forest::new();

        assert_eq!(forest.add_root(0), Ok(()));
        assert_eq!(forest.add_edge(0, 1), Ok(()));
        assert_eq!(forest.add_edge(1, 2), Ok(()));

        assert_eq!(forest.path(2), Ok(vec![ 0, 1, 2 ]))
    }

    #[test]
    fn grandchild_with_branching_before() {
        let mut forest = Forest::new();

        assert_eq!(forest.add_root(0), Ok(()));
        assert_eq!(forest.add_edge(0, 1), Ok(()));
        assert_eq!(forest.add_edge(0, 2), Ok(()));
        assert_eq!(forest.add_edge(0, 3), Ok(()));
        assert_eq!(forest.add_edge(1, 4), Ok(()));
        assert_eq!(forest.add_edge(2, 5), Ok(()));
        assert_eq!(forest.add_edge(3, 6), Ok(()));
        assert_eq!(forest.add_edge(5, 7), Ok(()));

        assert_eq!(forest.path(7), Ok(vec![ 0, 2, 5, 7 ]))
    }

    #[test]
    fn grandchild_and_other_path() {
        let mut forest = Forest::new();

        assert_eq!(forest.add_root(0), Ok(()));
        assert_eq!(forest.add_edge(0, 1), Ok(()));
        assert_eq!(forest.add_edge(1, 2), Ok(()));
        assert_eq!(forest.add_root(3), Ok(()));
        assert_eq!(forest.add_edge(3, 4), Ok(()));
        assert_eq!(forest.add_edge(3, 5), Ok(()));
        assert_eq!(forest.add_edge(5, 6), Ok(()));

        assert_eq!(forest.path(2), Ok(vec![ 0, 1, 2 ]))
    }
}

#[cfg(test)]
mod even {
    use super::*;

    #[test]
    fn outside() {
        let forest = Forest::new();

        assert_eq!(forest.even(0), Err(Error::MissingNode(0)))
    }

    #[test]
    fn root() {
        let mut forest = Forest::new();

        assert_eq!(forest.add_root(0), Ok(()));

        assert_eq!(forest.even(0), Ok(true))
    }

    #[test]
    fn child() {
        let mut forest = Forest::new();

        assert_eq!(forest.add_root(0), Ok(()));
        assert_eq!(forest.add_edge(0, 1), Ok(()));

        assert_eq!(forest.even(1), Ok(false))
    }

    #[test]
    fn grandchild() {
        let mut forest = Forest::new();

        assert_eq!(forest.add_root(0), Ok(()));
        assert_eq!(forest.add_edge(0, 1), Ok(()));
        assert_eq!(forest.add_edge(1, 2), Ok(()));

        assert_eq!(forest.even(2), Ok(true))
    }
}

#[cfg(test)]
mod odd {
    use super::*;

    #[test]
    fn outside() {
        let forest = Forest::new();

        assert_eq!(forest.odd(0), Err(Error::MissingNode(0)))
    }

    #[test]
    fn root() {
        let mut forest = Forest::new();

        assert_eq!(forest.add_root(0), Ok(()));

        assert_eq!(forest.odd(0), Ok(false))
    }

    #[test]
    fn child() {
        let mut forest = Forest::new();

        assert_eq!(forest.add_root(0), Ok(()));
        assert_eq!(forest.add_edge(0, 1), Ok(()));

        assert_eq!(forest.odd(1), Ok(true))
    }

    #[test]
    fn grandchild() {
        let mut forest = Forest::new();

        assert_eq!(forest.add_root(0), Ok(()));
        assert_eq!(forest.add_edge(0, 1), Ok(()));
        assert_eq!(forest.add_edge(1, 2), Ok(()));

        assert_eq!(forest.odd(2), Ok(false))
    }
}

#[cfg(test)]
mod root {
    use super::*;

    #[test]
    fn outside() {
        let forest = Forest::new();

        assert_eq!(forest.root(0), Err(Error::MissingNode(0)))
    }

    #[test]
    fn root() {
        let mut forest = Forest::new();

        assert_eq!(forest.add_root(0), Ok(()));

        assert_eq!(forest.root(0), Ok(0))
    }

    #[test]
    fn child() {
        let mut forest = Forest::new();

        assert_eq!(forest.add_root(0), Ok(()));
        assert_eq!(forest.add_edge(0, 1), Ok(()));

        assert_eq!(forest.root(1), Ok(0))
    }

    #[test]
    fn grandchild() {
        let mut forest = Forest::new();

        assert_eq!(forest.add_root(0), Ok(()));
        assert_eq!(forest.add_edge(0, 1), Ok(()));
        assert_eq!(forest.add_edge(1, 2), Ok(()));

        assert_eq!(forest.root(2), Ok(0))
    }
}

#[cfg(test)]
mod blossom {
    use super::*;

    #[test]
    fn unconnected() {
        let mut forest = Forest::new();

        assert_eq!(forest.add_root(0), Ok(()));
        assert_eq!(forest.add_root(1), Ok(()));
        assert_eq!(forest.add_edge(0, 2), Ok(()));
        assert_eq!(forest.add_edge(1, 3), Ok(()));

        assert_eq!(forest.blossom(2, 3), Err(Error::MissingBlossom(2, 3)))
    }

    #[test]
    fn different_lengths() {
        let mut forest = Forest::new();

        assert_eq!(forest.add_root(0), Ok(()));
        assert_eq!(forest.add_edge(0, 1), Ok(()));
        assert_eq!(forest.add_edge(1, 2), Ok(()));
        assert_eq!(forest.add_edge(0, 3), Ok(()));
        assert_eq!(forest.add_edge(3, 4), Ok(()));
        assert_eq!(forest.add_edge(4, 5), Ok(()));

        assert_eq!(forest.blossom(5, 2), Err(Error::MissingBlossom(5, 2)))
    }

    #[test]
    fn inline() {
        let mut forest = Forest::new();

        assert_eq!(forest.add_root(0), Ok(()));
        assert_eq!(forest.add_edge(0, 1), Ok(()));
        assert_eq!(forest.add_edge(1, 2), Ok(()));
        assert_eq!(forest.add_edge(2, 3), Ok(()));

        assert_eq!(forest.blossom(3, 2), Err(Error::MissingBlossom(3, 2)))
    }

    #[test]
    fn c3_from_root() {
        let mut forest = Forest::new();

        assert_eq!(forest.add_root(0), Ok(()));
        assert_eq!(forest.add_edge(0, 1), Ok(()));
        assert_eq!(forest.add_edge(0, 2), Ok(()));

        assert_eq!(forest.blossom(1, 2), Ok(vec![ 1, 0, 2 ]))
    }

    #[test]
    fn c5_from_root() {
        let mut forest = Forest::new();

        assert_eq!(forest.add_root(0), Ok(()));
        assert_eq!(forest.add_edge(0, 1), Ok(()));
        assert_eq!(forest.add_edge(1, 2), Ok(()));
        assert_eq!(forest.add_edge(0, 3), Ok(()));
        assert_eq!(forest.add_edge(3, 4), Ok(()));

        assert_eq!(forest.blossom(2, 4), Ok(vec![ 2, 1, 0, 3, 4 ]))
    }

    #[test]
    fn c5_from_inside() {
        let mut forest = Forest::new();

        assert_eq!(forest.add_root(0), Ok(()));
        assert_eq!(forest.add_edge(0, 1), Ok(()));
        assert_eq!(forest.add_edge(1, 2), Ok(()));
        assert_eq!(forest.add_edge(2, 3), Ok(()));
        assert_eq!(forest.add_edge(3, 4), Ok(()));
        assert_eq!(forest.add_edge(2, 5), Ok(()));
        assert_eq!(forest.add_edge(5, 6), Ok(()));

        assert_eq!(forest.blossom(4, 6), Ok(vec![ 4, 3, 2, 5, 6 ]))
    }
}