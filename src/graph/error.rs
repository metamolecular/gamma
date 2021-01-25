#[derive(Debug,PartialEq,Eq)]
pub enum Error {
    UnknownId(usize),
    DuplicateId(usize),
    MissingEdge(usize, usize),
    DuplicateEdge(usize, usize)
}