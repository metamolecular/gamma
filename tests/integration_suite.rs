use gamma::graph::{ Graph, StableGraph };
use gamma::traversal::{ depth_first, breadth_first };

fn gimmie_graph<'a>() -> StableGraph<usize, &'a str> {
    StableGraph::build(vec![ 0, 1, 2 ], vec![
        (0, 1, "a"),
        (1, 2, "b")
    ]).unwrap()
}

#[test]
fn retured_graph() {
    let graph = gimmie_graph();

    assert_eq!(graph.order(), 3);
}

#[test]
fn stable_graph_api() {
    let graph = StableGraph::build(vec![ 0, 1, 2 ], vec![
        (0, 1, "a"),
        (1, 2, "b")
    ]
    ).unwrap();
    
    assert_eq!(graph.is_empty(), false);
    assert_eq!(graph.order(), 3);
    assert_eq!(graph.size(), 2);
    assert_eq!(graph.nodes().collect::<Vec<_>>(), vec![ &0, &1, &2 ]);
    assert_eq!(graph.has_node(&0), true);
    assert_eq!(graph.neighbors(&1).unwrap().collect::<Vec<_>>(), vec![ &0, &2 ]);
    assert_eq!(graph.degree(&1).unwrap(), 2);
    assert_eq!(graph.edges().collect::<Vec<_>>(), vec![
        (&0, &1), (&1, &2)
    ]);
    assert_eq!(graph.has_edge(&0, &1).unwrap(), true);
    assert_eq!(graph.has_edge(&1, &0).unwrap(), true);
    assert_eq!(graph.has_edge(&0, &2).unwrap(), false);
}

#[test]
fn depth_first_c3() {
    let graph = StableGraph::build(vec![ 0, 1, 2 ], vec![
        (0, 1, ()),
        (1, 2, ()),
        (2, 0, ()),
    ]).unwrap();
    let traversal = depth_first(&graph, &0).unwrap();
    
    assert_eq!(traversal.collect::<Vec<_>>(), vec![
        (&0, &1, false),
        (&1, &2, false),
        (&2, &0, true)
    ]);
}

#[test]
fn breadth_first_c3() {
    let graph = StableGraph::build(vec![ 0, 1, 2 ], vec![
        (0, 1, ()),
        (1, 2, ()),
        (2, 0, ()),
    ]).unwrap();
    let traversal = breadth_first(&graph, &0).unwrap();
    
    assert_eq!(traversal.collect::<Vec<_>>(), vec![
        (&0, &1, false),
        (&0, &2, false),
        (&1, &2, true)
    ]);
}
