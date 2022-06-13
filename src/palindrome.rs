pub mod graph;

#[cfg(test)]
mod tests;

use std::sync::Arc;
use rayon::prelude::*;

use graph::{Graph, Node, Edge};



pub struct PalindromeGenerator {
    graph: Graph
}

impl PalindromeGenerator {
    pub fn new(word_list: &[&str]) -> Self {
        Self { graph: Graph::build(word_list) }
    }
    
    pub fn generate(&self, max_word_count: usize) -> Vec<String> {
        match self.graph.edges_form_node.get(&Arc::new(Node::Start)) {
            Some(start_edges) => start_edges.par_iter()
                .flat_map(|start_edge| get_palindromes_by_start_edge(&self.graph, &start_edge, max_word_count))
                .collect(),
            None => Vec::new(),
        }
    }
}


fn get_palindromes_by_start_edge(graph: &Graph, start_edge: &Edge, max_word_count: usize) -> Vec<String> {
    let mut palindromes = Vec::new();
    
    let mut stack = vec![(
        Arc::clone(&start_edge.to_node),
        max_word_count,
        String::from(&start_edge.word)
    )];
    
    while let Some((node, word_count, fragment)) = stack.pop() {
        match graph.node_distances.get(&node) {
            Some(&distance) if distance < word_count => {
                
                if word_count > 1 {
                    for edge in graph.edges_form_node[&node].iter() {
                        let word = &edge.word;
                        let new_fragment = match &*node {
                            Node::Tail(_) => format!("{word} {fragment}"),
                            _             => format!("{fragment} {word}"),
                        };
                        
                        stack.push((
                            Arc::clone(&edge.to_node),
                            word_count - 1,
                            new_fragment
                        ));
                    }
                }
                
                if distance == 0 {
                    palindromes.push(fragment);
                }
            },
            _ => { }
        };
    }
    
    palindromes
}
