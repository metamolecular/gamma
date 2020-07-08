use gamma::graph::{ Graph, ArrayGraph, Error };

fn main() -> Result<(), Error> {
    let p3 = ArrayGraph::from_adjacency(vec![
        vec![ 1 ],
        vec![ 0, 2 ],
        vec![ 1 ]
    ])?;

    assert_eq!(p3.is_empty(), false);
    assert_eq!(p3.order(), 3);
    assert_eq!(p3.size(), 2);
    assert_eq!(p3.nodes().to_vec(), vec![ 0, 1, 2 ]);
    assert_eq!(p3.neighbors(1)?.to_vec(), vec![ 0, 2 ]);
    assert_eq!(p3.has_node(4), false);
    assert_eq!(p3.degree(0)?, 1);
    assert_eq!(p3.edges().to_vec(), vec![
        (0, 1),
        (1, 2)
    ]);
    assert_eq!(p3.has_edge(1, 2)?, true);

    let result = ArrayGraph::from_adjacency(vec![
        vec![ 1 ]
    ]);

    assert_eq!(result, Err(Error::MissingNode(1)));
    assert!(false);

    Ok(())
}