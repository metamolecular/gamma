#[derive(Debug,PartialEq,Eq)]
pub enum Error {
    MissingNode(usize),
    DuplicateNode(usize),
    MissingEdge(usize, usize),
    DuplicateEdge(usize, usize)
}