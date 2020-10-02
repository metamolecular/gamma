use std::collections::HashMap;
use std::collections::hash_map::Entry::{ Occupied, Vacant };

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

    pub fn order(&self) -> usize {
        self.pairs.len()
    }

    pub fn has_node(&self, id: usize) -> bool {
        self.pairs.contains_key(&id)
    }

    pub fn pair(&mut self, sid: usize, tid: usize) {
        self.insert(sid, tid);
        self.insert(tid, sid)
    }

    pub fn edges(&self) -> impl Iterator<Item=(usize, usize)> + '_ {
        self.pairs.iter()
            .filter(|pair| pair.0 < pair.1)
            .map(|pair| (*pair.0, *pair.1))
    }

    pub fn augment(&mut self, path: Vec<usize>) {
        if path.len() % 2 == 1 {
            panic!("even path augmentation");
        }

        for i in 0..path.len() {
            if i % 2 == 0 {
                self.pair(path[i], path[i + 1]);
            }
        }
    }

    pub fn mate(&self, id: usize) -> usize {
        match self.pairs.get(&id) {
            Some(&mate) => mate,
            None => panic!("missing node: {}", id)
        }
    }

    fn insert(&mut self, sid: usize, tid: usize) {
        match self.pairs.entry(sid) {
            Occupied(mut entry) => {
                if *entry.get() == tid {
                    // panic!("duplicate edge: ({},{})", sid, tid)
                } else {
                    let old = entry.insert(tid);

                    self.pairs.remove(&old);
                }
            },
            Vacant(entry) => {
                entry.insert(tid);
            }
        }
    }
}

#[cfg(test)]
mod order {
    use super::*;

    #[test]
    fn default() {
        let pairing = Pairing::new();

        assert_eq!(pairing.order(), 0);
    }

    #[test]
    fn three_pairs() {
        let mut pairing = Pairing::new();

        pairing.pair(0, 1);
        pairing.pair(2, 3);
        pairing.pair(4, 5);

        assert_eq!(pairing.order(), 6);
    }
}

#[cfg(test)]
mod edges {
    use super::*;

    #[test]
    fn default() {
        let pairing = Pairing::new();

        assert_eq!(pairing.pairs.is_empty(), true)
    }

    #[test]
    fn pair_unk_unk() {
        let mut pairing = Pairing::new();

        pairing.pair(0, 1);
        pairing.pair(2, 3);

        assert_eq!(
            pairing.pairs,
            [ (0, 1), (2, 3), (1, 0), (3, 2) ]
                .iter().cloned().collect::<HashMap<_,_>>()
        )
    }

    #[test]
    fn pair_unk_tid() {
        let mut pairing = Pairing::new();

        pairing.pair(1, 2);
        pairing.pair(0, 1);

        assert_eq!(
            pairing.pairs,
            [ (0, 1), (1, 0) ].iter().cloned().collect::<HashMap<_,_>>()
        )
    }

    #[test]
    fn pair_sid_unk() {
        let mut pairing = Pairing::new();

        pairing.pair(1, 2);
        pairing.pair(2, 3);

        assert_eq!(
            pairing.pairs,
            [ (2, 3), (3, 2) ].iter().cloned().collect::<HashMap<_,_>>()
        )
    }

    #[test]
    fn pair_sid_tid() {
        let mut pairing = Pairing::new();

        pairing.pair(0, 1);
        pairing.pair(2, 3);
        pairing.pair(1, 2);

        assert_eq!(
            pairing.pairs,
            [ (1, 2), (2, 1) ].iter().cloned().collect::<HashMap<_,_>>()
        )
    }

    #[test]
    fn augment_empty() {
        let mut pairing = Pairing::new();
        let path = vec![ 0, 1, 2, 3 ];

        pairing.augment(path);

        assert_eq!(
            pairing.pairs,
            [ (0, 1), (2, 3), (1, 0), (3, 2) ]
                .iter().cloned().collect::<HashMap<_,_>>()
        )
    }

    #[test]
    fn augment_inner() {
        let mut pairing = Pairing::new();
        let path = vec![ 0, 1, 2, 3 ];

        pairing.pair(1, 2);
        pairing.augment(path);

        assert_eq!(
            pairing.pairs,
            [ (0, 1), (2, 3), (1, 0), (3, 2) ]
                .iter().cloned().collect::<HashMap<_,_>>()
        )
    }
}

#[cfg(test)]
mod augment {
    use super::*;

    #[test]
    #[should_panic(expected="even path augmentation")]
    fn odd_path() {
        let mut pairing = Pairing::new();
        let path = vec![ 0, 1, 2, 3, 4 ];

        pairing.augment(path)
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
        let mut pairing = Pairing::new();

        pairing.pair(0, 1);

        assert_eq!(pairing.has_node(0), true)
    }

    #[test]
    fn given_target() {
        let mut pairing = Pairing::new();

        pairing.pair(0, 1);

        assert_eq!(pairing.has_node(1), true)
    }
}

#[cfg(test)]
mod mate {
    use super::*;

    #[test]
    #[should_panic(expected="missing node: 0")]
    fn outside() {
        let pairing = Pairing::new();

        pairing.mate(0);
    }

    #[test]
    fn sid() {
        let mut pairing = Pairing::new();

        pairing.pair(0, 1);

        assert_eq!(pairing.mate(0), 1)
    }

    #[test]
    fn tid() {
        let mut pairing = Pairing::new();

        pairing.pair(0, 1);

        assert_eq!(pairing.mate(1), 0)
    }
}