mod node;
mod edge;

// #[cfg(test)]
// mod tests;

use std::sync::Arc;
use std::iter::Iterator;
use std::collections::HashMap;
use itertools::Itertools;
use priority_queue::PriorityQueue;

use node::Node;
use edge::Edge;


pub struct Graph {
    pub edges: Vec<Edge>,
    pub start_edge_indices: Vec<usize>,
    pub next_edge_indices: Vec<Vec<usize>>,
    pub edge_distances: Vec<usize>,
}

impl Graph {
    pub fn build(word_list: &[&str]) -> Self {
        let start_edges = Edge::get_start_edges(word_list);
        let other_edges = Edge::get_other_edges(&start_edges, word_list);
        
        let node_distances = calculate_distances(&other_edges);
        
        let edges: Vec<_> = start_edges.into_iter().chain(other_edges)
            .filter(|edge| node_distances.contains_key(&edge.to_node))
            .collect();
        
        let indices_from_node = edges.iter().enumerate()
            .map(|(index, edge)| (Arc::clone(&edge.from_node), index))
            .into_group_map();
        
        let next_edge_indices: Vec<_> = edges.iter()
            .map(|edge| indices_from_node[&edge.to_node].clone())
            .collect();
        
        let start_edge_indices: Vec<_> = match indices_from_node.get(&Node::Start) {
            Some(indices) => indices.clone(),
            None => Vec::new(),
        };
        
        let edge_distances: Vec<_> = edges.iter()
            .map(|edge| node_distances[&edge.to_node])
            .collect();
        
        Graph {
            edges,
            start_edge_indices,
            next_edge_indices,
            edge_distances
        }
    }
}


fn calculate_distances(edges: &[Edge]) -> HashMap<Arc<Node>, usize> {
    let mut distances = HashMap::new();
    let mut queue = PriorityQueue::new();
    
    let from_nodes_by_to_node = edges.into_iter()
        .map(|edge| (Arc::clone(&edge.to_node), Arc::clone(&edge.from_node)))
        .into_group_map();
    
    let final_node = Arc::new(Node::Final);

    if from_nodes_by_to_node.contains_key(&final_node) {
        distances.insert(Arc::clone(&final_node), 0);
        queue.push(final_node, 0);
    }
    
    while let Some((node, priority)) = queue.pop() {
        let from_node_distance = priority + 1;
        
        for from_node in from_nodes_by_to_node[&node].iter() {
            match distances.get(from_node) {
                Some(&distance) if distance <= from_node_distance => { }
                _ => {
                    distances.insert(Arc::clone(&from_node), from_node_distance);
                    queue.push(Arc::clone(&from_node), from_node_distance);
                }
            }
        }
    }
    
    distances
}
