use crate::graph::{ Graph, DefaultGraph, Error };

pub fn contract<'a, G: Graph>(
    graph: &'a G, path: &Vec<usize>
) -> Result<DefaultGraph, Error> {
    let result = DefaultGraph::new();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
    use super::*;

    #[test]#[ignore]
    pub fn foo() {
        let graph = DefaultGraph::try_from(vec![
            vec![ 1, 4 ],
            vec![ 0, 2 ],
            vec![ 1, 3 ],
            vec![ 4, 2 ],
            vec![ 3, 0, 5 ],
            vec![ 4 ]
        ]).unwrap();
        let path = vec![ 0, 1, 2, 3, 4 ];
        let contracted = contract(&graph, &path).unwrap();

        assert_eq!(contracted, DefaultGraph::try_from(vec![
            (0, 5)
        ]).unwrap())
    }
}