#[derive(Debug, PartialEq)]
pub enum Error {
    UnknownNode,
    InvalidEdge,
    DuplicateEdge,
    DuplicateNode,
    UnknownIndex(usize),
    DuplicatePairing(usize, usize),
    MissingPairing(usize, usize)
}