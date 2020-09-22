use std::collections::HashMap;
use std::convert::TryFrom;

use super::Error;

#[derive(Debug,PartialEq)]
pub struct Pairing {
    pairs: HashMap<usize, usize>
}

impl Pairing {
    pub fn new() -> Self {
        Self {
            pairs: HashMap::new()
        }
    }

    pub fn has_node(&self, id: usize) -> bool {
        self.pairs.contains_key(&id)
    }

    pub fn pair(&mut self, sid: usize, tid: usize) -> Result<(), Error> {
        if let Some(sid_mate) = self.pairs.get(&sid).cloned() {
            if sid_mate == tid {
                return Err(Error::Duplicate(sid, tid));
            } else {
                self.pairs.remove(&sid_mate);
            }
        }

        if let Some(tid_mate) = self.pairs.get(&tid).cloned() {
            if tid_mate == sid {
                return Err(Error::Duplicate(sid, tid));
            } else {
                self.pairs.remove(&tid_mate);
            }
        }

        self.pairs.insert(sid, tid);
        self.pairs.insert(tid, sid);
        
        Ok(())
    }

    pub fn augment(&mut self, path: &Vec<usize>) -> Result<(), Error> {
        if path.len() % 2 == 1 {
            return Err(Error::OddPathAugmentation);
        }

        for i in 0..path.len() {
            if i % 2 == 0 {
                self.pair(path[i], path[i + 1])?;
            }
        }

        Ok(())
    }

    pub fn edges(&self) -> impl Iterator<Item=(usize, usize)> + '_ {
        self.pairs.iter()
            .filter(|pair| pair.0 < pair.1)
            .map(|pair| (*pair.0, *pair.1))
    }

    pub fn mate(&self, id: usize) -> Result<usize, Error> {
        match self.pairs.get(&id) {
            Some(&mate) => Ok(mate),
            None => Err(Error::MissingNode(id))
        }
    }
}

impl TryFrom<Vec<(usize, usize)>> for Pairing {
    type Error = Error;

    fn try_from(edges: Vec<(usize, usize)>) -> Result<Self, Self::Error> {
        let mut result = Self::new();

        for (sid, tid) in edges {
            result.pair(sid, tid)?;
        }

        Ok(result)
    }
}

#[cfg(test)]
mod try_from {
    use super::*;

    #[test]
    fn duplicates() {
        let pairing = Pairing::try_from(vec![
            (0, 1),
            (0, 1)
        ]);

        assert_eq!(pairing, Err(Error::Duplicate(0, 1)))
    }

    #[test]
    fn duplicates_reversed() {
        let pairing = Pairing::try_from(vec![
            (0, 1),
            (1, 0)
        ]);

        assert_eq!(pairing, Err(Error::Duplicate(1, 0)))
    }

    #[test]
    fn one_edge() {
        let pairing1 = Pairing::try_from(vec![
            (0, 1)
        ]).unwrap();
        let mut pairing2 = Pairing::new();

        assert_eq!(pairing2.pair(0, 1), Ok(()));

        assert_eq!(pairing1, pairing2)
    }
}

#[cfg(test)]
mod pair {
    use super::*;

    #[test]
    fn default() {
        let mut pairing = Pairing::new();

        assert_eq!(pairing.pair(0, 1), Ok(()))
    }

    #[test]
    fn duplicate() {
        let mut pairing = Pairing::new();

        assert_eq!(pairing.pair(0, 1), Ok(()));

        assert_eq!(pairing.pair(0, 1), Err(Error::Duplicate(0, 1)))
    }

    #[test]
    fn duplicate_reversed() {
        let mut pairing = Pairing::new();

        assert_eq!(pairing.pair(0, 1), Ok(()));

        assert_eq!(pairing.pair(1, 0), Err(Error::Duplicate(1, 0)))
    }
}

#[cfg(test)]
mod edges {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn default() {
        let pairing = Pairing::new();

        assert_eq!(pairing.edges().collect::<Vec<_>>(), vec![ ])
    }

    #[test]
    fn pair_unk_unk() {
        let mut pairing = Pairing::try_from(vec![ (0, 1) ]).unwrap();

        assert_eq!(pairing.pair(2, 3), Ok(()));

        assert_eq!(
            pairing.edges().collect::<HashSet<_>>(),
            [ (0, 1), (2, 3) ].iter().cloned().collect::<HashSet<_>>()
        )
    }

    #[test]
    fn pair_unk_tid() {
        let mut pairing = Pairing::try_from(vec![ (1, 2) ]).unwrap();

        assert_eq!(pairing.pair(0, 1), Ok(()));

        assert_eq!(
            pairing.edges().collect::<HashSet<_>>(),
            [ (0, 1) ].iter().cloned().collect::<HashSet<_>>()
        )
    }

    #[test]
    fn pair_sid_unk() {
        let mut pairing = Pairing::try_from(vec![ (1, 2) ]).unwrap();

        assert_eq!(pairing.pair(2, 3), Ok(()));

        assert_eq!(
            pairing.edges().collect::<HashSet<_>>(),
            [ (2, 3) ].iter().cloned().collect::<HashSet<_>>()
        )
    }

    #[test]
    fn pair_sid_tid() {
        let mut pairing = Pairing::new();

        assert_eq!(pairing.pair(0, 1), Ok(()));
        assert_eq!(pairing.pair(2, 3), Ok(()));
        assert_eq!(pairing.pair(1, 2), Ok(()));

        assert_eq!(
            pairing.edges().collect::<HashSet<_>>(),
            [ (1, 2) ].iter().cloned().collect::<HashSet<_>>()
        )
    }

    #[test]
    fn augment_empty() {
        let mut pairing = Pairing::new();
        let path = vec![ 0, 1, 2, 3 ];

        assert_eq!(pairing.augment(&path), Ok(()));

        assert_eq!(
            pairing.edges().collect::<HashSet<_>>(),
            [ (0, 1), (2, 3) ].iter().cloned().collect::<HashSet<_>>()
        )
    }

    #[test]
    fn augment_inner() {
        let mut pairing = Pairing::new();
        let path = vec![ 0, 1, 2, 3 ];

        assert_eq!(pairing.pair(1, 2), Ok(()));
        assert_eq!(pairing.augment(&path), Ok(()));

        assert_eq!(
            pairing.edges().collect::<HashSet<_>>(),
            [ (0, 1), (2, 3) ].iter().cloned().collect::<HashSet<_>>()
        )
    }
}

#[cfg(test)]
mod augment {
    use super::*;

    #[test]
    fn odd_path() {
        let mut pairing = Pairing::new();
        let path = vec![ 0, 1, 2 ];

        assert_eq!(pairing.augment(&path), Err(Error::OddPathAugmentation))
    }

    #[test]
    fn empty_with_p2() {
        let mut pairing = Pairing::new();
        let path = vec![ 0, 1 ];

        assert_eq!(pairing.augment(&path), Ok(()))
    }

    #[test]
    fn p2_with_orthogonal_p2() {
        let mut pairing = Pairing::try_from(vec![ (0, 1) ]).unwrap();

        assert_eq!(pairing.augment(&vec![ 2, 3 ]), Ok(()));

        assert_eq!(pairing, Pairing::try_from(vec![
            (0, 1), (2, 3)
        ]).unwrap())
    }

    #[test]
    fn p2_with_overlapping_p4() {
        let mut pairing = Pairing::try_from(vec![ (1, 2) ]).unwrap();
        
        assert_eq!(pairing.augment(&vec![ 0, 1 ]), Ok(()));
        assert_eq!(pairing, Pairing::try_from(vec![
            (0, 1)
        ]).unwrap())
    }
}

#[cfg(test)]
mod has_node {
    use super::*;

    #[test]
    fn default() {
        let pairing = Pairing::new();

        assert_eq!(pairing.has_node(0), false)
    }

    #[test]
    fn given_source() {
        let pairing = Pairing::try_from(vec![ (0, 1) ]).unwrap();

        assert_eq!(pairing.has_node(0), true)
    }

    #[test]
    fn given_target() {
        let pairing = Pairing::try_from(vec![ (0, 1) ]).unwrap();

        assert_eq!(pairing.has_node(1), true)
    }
}

#[cfg(test)]
mod mate {
    use super::*;

    #[test]
    fn outside() {
        let pairing = Pairing::new();

        assert_eq!(pairing.mate(0), Err(Error::MissingNode(0)))
    }

    #[test]
    fn sid() {
        let pairing = Pairing::try_from(vec![ (0, 1) ]).unwrap();

        assert_eq!(pairing.mate(0), Ok(1))
    }

    #[test]
    fn tid() {
        let pairing = Pairing::try_from(vec![ (0, 1) ]).unwrap();

        assert_eq!(pairing.mate(1), Ok(0))
    }
}