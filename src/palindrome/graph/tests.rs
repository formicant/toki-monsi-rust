use test_case::test_case;
use std::rc::Rc;

use super::{Graph, Node, Node::*, Edge};


fn s(string: &str) -> String { String::from(string) }

fn rc<T>(value: T) -> Rc<T> { Rc::new(value) }

fn are_equail_unordered<T: Eq>(actual: &[T], expected: &[T]) -> bool {
    actual.len() == expected.len() && actual.iter().all(|item| expected.contains(&item))
}


#[test_case(&[],           &[], &[], &[]; "empty word list")]
#[test_case(&["pu", "en"], &[], &[], &[]; "no palindrome possible")]
#[test_case(
    &["a", "ala", "alasa", "kala", "la", "pu"],  // word_list
    &[  // expected_useful_start_edges
        Edge { from_node: rc(Start), word: s("a"    ), to_node: rc(Final           ) },
        Edge { from_node: rc(Start), word: s("a"    ), to_node: rc(Tail(s("a"    ))) },
        Edge { from_node: rc(Start), word: s("ala"  ), to_node: rc(Final           ) },
        Edge { from_node: rc(Start), word: s("ala"  ), to_node: rc(Head(s("al"   ))) },
        Edge { from_node: rc(Start), word: s("ala"  ), to_node: rc(Tail(s("ala"  ))) },
        Edge { from_node: rc(Start), word: s("alasa"), to_node: rc(Head(s("al"   ))) },
        Edge { from_node: rc(Start), word: s("alasa"), to_node: rc(Tail(s("alasa"))) },
        Edge { from_node: rc(Start), word: s("kala" ), to_node: rc(Head(s("k"    ))) },
        Edge { from_node: rc(Start), word: s("kala" ), to_node: rc(Head(s("kal"  ))) },
        Edge { from_node: rc(Start), word: s("kala" ), to_node: rc(Tail(s("ala"  ))) },
        Edge { from_node: rc(Start), word: s("la"   ), to_node: rc(Head(s("l"    ))) },
        Edge { from_node: rc(Start), word: s("la"   ), to_node: rc(Tail(s("a"    ))) },
    ],
    &[  // expected_useful_other_edges
        Edge { from_node: rc(Head(s("al"   ))), word: s("la"   ), to_node: rc(Final           ) },
        Edge { from_node: rc(Head(s("k"    ))), word: s("kala" ), to_node: rc(Tail(s("ala"  ))) },
        Edge { from_node: rc(Head(s("kal"  ))), word: s("la"   ), to_node: rc(Head(s("k"    ))) },
        Edge { from_node: rc(Head(s("l"    ))), word: s("la"   ), to_node: rc(Tail(s("a"    ))) },
        Edge { from_node: rc(Tail(s("a"    ))), word: s("a"    ), to_node: rc(Final           ) },
        Edge { from_node: rc(Tail(s("a"    ))), word: s("ala"  ), to_node: rc(Head(s("al"   ))) },
        Edge { from_node: rc(Tail(s("a"    ))), word: s("kala" ), to_node: rc(Head(s("kal"  ))) },
        Edge { from_node: rc(Tail(s("a"    ))), word: s("la"   ), to_node: rc(Head(s("l"    ))) },
        Edge { from_node: rc(Tail(s("ala"  ))), word: s("ala"  ), to_node: rc(Final           ) },
        Edge { from_node: rc(Tail(s("ala"  ))), word: s("kala" ), to_node: rc(Head(s("k"    ))) },
        Edge { from_node: rc(Tail(s("ala"  ))), word: s("la"   ), to_node: rc(Tail(s("a"    ))) },
        Edge { from_node: rc(Tail(s("alasa"))), word: s("la"   ), to_node: rc(Tail(s("asa"  ))) },
        Edge { from_node: rc(Tail(s("asa"  ))), word: s("alasa"), to_node: rc(Head(s("al"   ))) },
        Edge { from_node: rc(Final           ), word: s("a"    ), to_node: rc(Tail(s("a"    ))) },
        Edge { from_node: rc(Final           ), word: s("ala"  ), to_node: rc(Tail(s("ala"  ))) },
        Edge { from_node: rc(Final           ), word: s("alasa"), to_node: rc(Tail(s("alasa"))) },
    ],
    &[  // expected_node_distances
        (rc(Final           ), 0),
        (rc(Head(s("al"   ))), 1),
        (rc(Head(s("k"    ))), 2),
        (rc(Head(s("kal"  ))), 3),
        (rc(Head(s("l"    ))), 2),
        (rc(Tail(s("a"    ))), 1),
        (rc(Tail(s("ala"  ))), 1),
        (rc(Tail(s("alasa"))), 3),
        (rc(Tail(s("asa"  ))), 2),
    ];
    "small word list"
)]
fn test_graph(
    word_list: &[&str],
    expected_useful_start_edges: &[Edge],
    expected_useful_other_edges: &[Edge],
    expected_node_distances: &[(Rc<Node>, usize)]
) {
    let graph = Graph::build(word_list);
    
    let is_useful = |edge: &Edge| {
        graph.node_distances.contains_key(&edge.to_node)
    };
    let actual_useful_start_edges: Vec<_> = graph.start_edges.into_iter().filter(is_useful).collect();
    let actual_useful_other_edges: Vec<_> = graph.other_edges.into_iter().filter(is_useful).collect();
    
    let actual_node_distances: Vec<_> = graph.node_distances.into_iter().collect();
    
    assert!(are_equail_unordered(&actual_useful_start_edges, &expected_useful_start_edges));
    assert!(are_equail_unordered(&actual_useful_other_edges, &expected_useful_other_edges));
    assert!(are_equail_unordered(&actual_node_distances, &expected_node_distances));
}
