mod graph;
mod vec_ext;

#[cfg(test)]
mod tests;

use rayon::prelude::*;

use graph::Graph;
use vec_ext::VecExt;

pub struct PalindromeGenerator {
    graph: Graph
}

impl PalindromeGenerator {
    pub fn new(word_list: &[&str]) -> Self {
        Self { graph: Graph::build(word_list) }
    }
    
    pub fn generate(&self, max_word_count: usize) -> Vec<String> {
        self.graph.start_edge_indices.par_iter()
            .flat_map(|&start_index| get_palindromes_by_start_edge(&self.graph, start_index, max_word_count))
            .collect()
    }
}


fn get_palindromes_by_start_edge<'a> (graph: &Graph, start_index: usize, max_word_count: usize) -> Vec<String> {
    let mut palindromes = Vec::new();
    
    let mut stack: Vec<(usize, usize, Vec<&str>)> = vec![(
        start_index,
        max_word_count,
        vec![&graph.edges[start_index].word[..]],
    )];
    
    while let Some((index, word_count, fragment)) = stack.pop() {
        let &distance = &graph.edge_distances[index];
        
        if distance < word_count {
            if word_count > 1 {
                for &next_index in graph.next_edge_indices[index].iter() {
                    let next_edge = &graph.edges[next_index];
                    let word = &next_edge.word[..];
                    let new_fragment = if next_edge.is_prepending {
                        fragment.clone_and_prepend(word)
                    } else {
                        fragment.clone_and_append(word)
                    };
                    
                    stack.push((
                        next_index,
                        word_count - 1,
                        new_fragment
                    ));
                }
            }
            
            if distance == 0 {
                palindromes.push(fragment.join(" "));
            }
        }
    }
    
    palindromes
}
