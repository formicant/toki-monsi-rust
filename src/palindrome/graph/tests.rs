use test_case::test_case;
use std::sync::Arc;

use super::{Graph, Node, Node::*, StartEdge, Edge};


fn s(string: &str) -> String { String::from(string) }

fn a<T>(value: T) -> Arc<T> { Arc::new(value) }

fn are_equail_unordered<T: Eq>(actual: &[T], expected: &[T]) -> bool {
    actual.len() == expected.len() && actual.iter().all(|item| expected.contains(&item))
}


#[test_case(&[],           &[], &[], &[]; "empty word list")]
#[test_case(&["pu", "en"], &[], &[], &[]; "no palindrome possible")]
#[test_case(
    &["a", "ala", "alasa", "kala", "la", "pu"],  // word_list
    
    &[  // expected_start_edges
        StartEdge { word: s("a"    ), to_node: a(Final           ) },
        StartEdge { word: s("a"    ), to_node: a(Tail(s("a"    ))) },
        StartEdge { word: s("ala"  ), to_node: a(Final           ) },
        StartEdge { word: s("ala"  ), to_node: a(Head(s("al"   ))) },
        StartEdge { word: s("ala"  ), to_node: a(Tail(s("ala"  ))) },
        StartEdge { word: s("alasa"), to_node: a(Head(s("al"   ))) },
        StartEdge { word: s("alasa"), to_node: a(Tail(s("alasa"))) },
        StartEdge { word: s("kala" ), to_node: a(Head(s("k"    ))) },
        StartEdge { word: s("kala" ), to_node: a(Head(s("kal"  ))) },
        StartEdge { word: s("kala" ), to_node: a(Tail(s("ala"  ))) },
        StartEdge { word: s("la"   ), to_node: a(Head(s("l"    ))) },
        StartEdge { word: s("la"   ), to_node: a(Tail(s("a"    ))) },
    ],
    &[  // expected_edges
        Edge { from_node: a(Final           ), word: s("a"    ), to_node: a(Tail(s("a"    ))) },
        Edge { from_node: a(Final           ), word: s("ala"  ), to_node: a(Tail(s("ala"  ))) },
        Edge { from_node: a(Final           ), word: s("alasa"), to_node: a(Tail(s("alasa"))) },
        Edge { from_node: a(Head(s("al"   ))), word: s("la"   ), to_node: a(Final           ) },
        Edge { from_node: a(Head(s("k"    ))), word: s("kala" ), to_node: a(Tail(s("ala"  ))) },
        Edge { from_node: a(Head(s("kal"  ))), word: s("la"   ), to_node: a(Head(s("k"    ))) },
        Edge { from_node: a(Head(s("l"    ))), word: s("la"   ), to_node: a(Tail(s("a"    ))) },
        Edge { from_node: a(Tail(s("a"    ))), word: s("a"    ), to_node: a(Final           ) },
        Edge { from_node: a(Tail(s("a"    ))), word: s("ala"  ), to_node: a(Head(s("al"   ))) },
        Edge { from_node: a(Tail(s("a"    ))), word: s("kala" ), to_node: a(Head(s("kal"  ))) },
        Edge { from_node: a(Tail(s("a"    ))), word: s("la"   ), to_node: a(Head(s("l"    ))) },
        Edge { from_node: a(Tail(s("ala"  ))), word: s("ala"  ), to_node: a(Final           ) },
        Edge { from_node: a(Tail(s("ala"  ))), word: s("kala" ), to_node: a(Head(s("k"    ))) },
        Edge { from_node: a(Tail(s("ala"  ))), word: s("la"   ), to_node: a(Tail(s("a"    ))) },
        Edge { from_node: a(Tail(s("alasa"))), word: s("la"   ), to_node: a(Tail(s("asa"  ))) },
        Edge { from_node: a(Tail(s("asa"  ))), word: s("alasa"), to_node: a(Head(s("al"   ))) },
    ],
    &[  // expected_node_distances
        (a(Final           ), 0),
        (a(Head(s("al"   ))), 1),
        (a(Head(s("k"    ))), 2),
        (a(Head(s("kal"  ))), 3),
        (a(Head(s("l"    ))), 2),
        (a(Tail(s("a"    ))), 1),
        (a(Tail(s("ala"  ))), 1),
        (a(Tail(s("alasa"))), 3),
        (a(Tail(s("asa"  ))), 2),
    ];
    "small word list"
)]
fn test_graph(
    word_list: &[&str],
    expected_start_edges: &[StartEdge],
    expected_edges: &[Edge],
    expected_node_distances: &[(Arc<Node>, usize)]
) {
    let graph = Graph::build(word_list);
    
    let actual_start_edges = graph.start_edges;
    let are_edges_grouped_correctly = graph.edges_form_node.iter()
        .map(|(node, edges)| edges.iter().all(|edge| edge.from_node == *node))
        .all(|x| x);
    let actual_edges: Vec<_> = graph.edges_form_node.into_iter()
        .flat_map(|(_, edges)| edges)
        .collect();
    let actual_node_distances: Vec<_> = graph.node_distances.into_iter().collect();
    
    assert!(are_equail_unordered(&actual_start_edges, &expected_start_edges));
    assert!(are_edges_grouped_correctly);
    assert!(are_equail_unordered(&actual_edges, &expected_edges));
    assert!(are_equail_unordered(&actual_node_distances, &expected_node_distances));
}
