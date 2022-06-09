use std::rc::Rc;
use std::fmt;
use std::collections::HashSet;
use caseless::default_case_fold_str;

use super::{Node, StartEdge};


#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct Edge {
    pub from_node: Rc<Node>,
    pub word: String,
    pub to_node: Rc<Node>,
}

impl<'a> fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})→ {}", self.from_node, self.word, self.to_node)
    }
}

impl Edge {
    pub fn get_all(start_edges: &[StartEdge], word_list: &[&str]) -> Vec<Self> {
        let mut edges = Vec::new();
        
        let mut visited_nodes: HashSet<_> = start_edges.iter().map(|edge| Rc::clone(&edge.to_node)).collect();
        let mut stack: Vec<_> = visited_nodes.iter().map(|node| Rc::clone(&node)).collect();
        
        while let Some(node) = stack.pop() {
            let from_node = node;
            
            for &word in word_list {
                let caseless_word = default_case_fold_str(word);
                
                let possible_to_node = match &*from_node {
                    Node::Final      => Node::try_create(&caseless_word, ""),
                    Node::Head(head) => Node::try_create(&caseless_word, &head),
                    Node::Tail(tail) => Node::try_create(&tail, &caseless_word),
                };
                if let Some(to_node) = possible_to_node {
                    edges.push(Self {
                        from_node: Rc::clone(&from_node),
                        word: String::from(word),
                        to_node: Rc::clone(&to_node),
                    });
                    
                    if !visited_nodes.contains(&to_node) {
                        visited_nodes.insert(Rc::clone(&to_node));
                        stack.push(to_node);
                    }
                }
            }
        }
        
        edges
    }
}
