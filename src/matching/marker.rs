use std::collections::{ HashMap, HashSet };
use std::collections::hash_map::Entry::{ Occupied, Vacant };

pub struct Marker {
    nodes: HashSet<usize>,
    edges: HashMap<usize, Vec<usize>>
}

impl Marker {
    pub fn new() -> Self {
        Self {
            nodes: HashSet::new(),
            edges: HashMap::new()
        }
    }

    pub fn mark_node(&mut self, id: usize) {
        if !self.nodes.insert(id) {
            panic!("node marked twice: {}", id)
        }
    }

    pub fn has_node(&self, id: usize) -> bool {
        self.nodes.contains(&id)
    }

    pub fn mark_edge(&mut self, sid: usize, tid: usize) {
        match self.edges.entry(sid) {
            Occupied(mut entry) => {
                if entry.get().contains(&tid) {
                    panic!("edge marked twice: ({},{})", sid, tid)
                } else {
                    entry.get_mut().push(tid)
                }
            },
            Vacant(entry) => {
                entry.insert(vec![ tid ]);
            }
        }

        match self.edges.entry(tid) {
            Occupied(mut entry) => {
                entry.get_mut().push(sid)
            },
            Vacant(entry) => {
                entry.insert(vec![ sid ]);
            }
        }
    }

    pub fn has_edge(&self, sid: usize, tid: usize) -> bool {
        match self.edges.get(&sid) {
            None => false,
            Some(neighbors) => neighbors.contains(&tid)
        }
    }
}

#[cfg(test)]
mod mark_node {
    use super::*;

    #[test]
    #[should_panic(expected="node marked twice: 0")]
    fn duplicate() {
        let mut marker = Marker::new();

        marker.mark_node(0);
        marker.mark_node(0);
    }
}

#[cfg(test)]
mod mark_edge {
    use super::*;

    #[test]
    #[should_panic(expected="edge marked twice: (0,1)")]
    fn duplicate() {
        let mut marker = Marker::new();

        marker.mark_edge(0, 1);
        marker.mark_edge(0, 1);
    }

    #[test]
    #[should_panic(expected="edge marked twice: (1,0)")]
    fn duplicate_reverse() {
        let mut marker = Marker::new();

        marker.mark_edge(0, 1);
        marker.mark_edge(1, 0);
    }
}

#[cfg(test)]
mod has_node {
    use super::*;

    #[test]
    fn outside() {
        let marker = Marker::new();

        assert_eq!(marker.has_node(0), false)
    }

    #[test]
    fn inside() {
        let mut marker = Marker::new();

        marker.mark_node(0);

        assert_eq!(marker.has_node(0), true)
    }
}

#[cfg(test)]
mod has_edge {
    use super::*;

    #[test]
    fn outside() {
        let marker = Marker::new();

        assert_eq!(marker.has_edge(0, 1), false);
    }

    #[test]
    fn inside() {
        let mut marker = Marker::new();

        marker.mark_edge(0, 1);

        assert_eq!(marker.has_edge(0, 1), true);
    }

    #[test]
    fn inside_reverse() {
        let mut marker = Marker::new();

        marker.mark_edge(0, 1);

        assert_eq!(marker.has_edge(1, 0), true);
    }
}