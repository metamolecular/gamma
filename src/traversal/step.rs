#[derive(Eq,PartialEq, Hash, Debug)]
pub struct Step<N> {
    pub source: N,
    pub target: N,
    pub cut: bool
}

impl<N> Step<N> {
    pub fn new(source: N, target: N, cut: bool) -> Self {
        Step { source, target, cut }
    }
}