#[derive(Debug, PartialEq)]
pub enum Error {
    UnknownNode,
    InvalidEdge,
    UnknownIndex(usize),
    DuplicatePairing(usize, usize),
    MissingPairing(usize, usize)
}