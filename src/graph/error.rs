#[derive(Debug, PartialEq)]
pub enum Error {
    UnknownNode,
    InvalidEdge,
    DuplicateEdge,
    DuplicateNode,
    MissingEdge
}