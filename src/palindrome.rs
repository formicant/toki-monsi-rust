mod graph;
mod vec_ext;

#[cfg(test)]
mod tests;

use std::rc::Rc;
use itertools::Itertools;
use rayon::prelude::*;

use graph::{Graph, Edge};
use vec_ext::VecExt;


struct Step {
    pub word: String,
    pub is_prepending: bool,
    pub distance: usize,
    pub next_step_indices: Vec<usize>,
}

pub struct PalindromeGenerator {
    steps: Vec<Step>,
    start_step_count: usize,
}

impl PalindromeGenerator {
    pub fn new(word_list: &[&str]) -> Self {
        // Build the helper graph
        let graph = Graph::build(word_list);
        
        // An edge is useful if the final node is achievable from it
        let is_useful = |edge: &Edge| {
            graph.node_distances.contains_key(&edge.to_node)
        };
        
        // We take useful start edges first
        let mut useful_edges: Vec<_> = graph.start_edges.into_iter().filter(is_useful).collect();
        let start_step_count = useful_edges.len();
        // Then, we add other useful edges
        useful_edges.extend(graph.other_edges.into_iter().filter(is_useful));
        
        let indices_from_node = useful_edges.iter()
            .enumerate()
            .map(|(index, edge)| (Rc::clone(&edge.from_node), index))
            .into_group_map();
        
        let steps: Vec<_> = useful_edges.into_iter()
            .map(|edge| Step {
                is_prepending: edge.is_prepending(),
                word: edge.word,
                distance: graph.node_distances[&edge.to_node],
                next_step_indices: indices_from_node[&edge.to_node].clone(),
            })
            .collect();
        
        Self { steps, start_step_count }
    }
    
    pub fn generate(&self, max_word_count: usize) -> Vec<String> {
        (0..self.start_step_count).into_par_iter()
            .flat_map(|start_index| get_palindromes_by_start_step(&self.steps, start_index, max_word_count))
            .collect()
    }
}


fn get_palindromes_by_start_step (steps: &[Step], start_index: usize, max_word_count: usize) -> Vec<String> {
    let mut palindromes = Vec::new();
    
    let mut stack: Vec<(&Step, usize, Vec<&str>)> = vec![(
        &steps[start_index],
        max_word_count,
        vec![&steps[start_index].word[..]],
    )];
    
    while let Some((step, word_count, fragment)) = stack.pop() {
        let distance = step.distance;
        
        if distance < word_count {
            if word_count > 1 {
                for &next_step_index in step.next_step_indices.iter() {
                    let next_step = &steps[next_step_index];
                    let word = &next_step.word[..];
                    let new_fragment = if next_step.is_prepending {
                        fragment.clone_and_prepend(word)
                    } else {
                        fragment.clone_and_append(word)
                    };
                    
                    stack.push((
                        next_step,
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
