#[derive(Debug, PartialEq)]
pub enum Error {
    UnknownNode,
    DuplicateNode,
    DuplicateEdge
}