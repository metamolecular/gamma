#[derive(Debug, PartialEq)]
pub enum Error {
    UnknownNode,
    UnknownIndex(usize),
    DuplicatePairing(usize, usize)
}