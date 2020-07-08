/// A single traversal step comprised of source and target nodes, and a
/// boolean flag indicating whether a cycle cut is present.
#[derive(Eq,PartialEq,Hash,Debug)]
pub struct Step {
    pub sid: usize,
    pub tid: usize,
    pub cut: bool
}

impl Step {
    pub fn new(sid: usize, tid: usize, cut: bool) -> Self {
        Step { sid, tid, cut }
    }
}