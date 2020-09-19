#[derive(Debug, PartialEq)]
pub enum Error {
    Duplicate(usize, usize),
    DuplicateRoot(usize),
    MissingNode(usize),
    OddPathAugmentation
}