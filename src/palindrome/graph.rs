pub mod node;
pub mod start_edge;
pub mod edge;

#[cfg(test)]
mod tests;

use std::sync::Arc;
use std::iter::Iterator;
use std::collections::HashMap;
use itertools::Itertools;
use priority_queue::PriorityQueue;

pub use node::Node;
pub use start_edge::StartEdge;
pub use edge::Edge;


pub struct Graph {
    pub start_edges: Vec<StartEdge>,
    pub edges_form_node: HashMap<Arc<Node>, Vec<Edge>>,
    pub node_distances: HashMap<Arc<Node>, usize>,
}

impl Graph {
    pub fn build(word_list: &[&str]) -> Self {
        let all_start_edges = StartEdge::get_all(word_list);
        let all_edges = Edge::get_all(&all_start_edges, word_list);
        
        let node_distances = calculate_distances(&all_edges);
        
        let start_edges = all_start_edges.into_iter()
            .filter(|start_edge| node_distances.contains_key(&start_edge.to_node))
            .collect();
        
        let edges_form_node = all_edges.into_iter()
            .filter(|edge| node_distances.contains_key(&edge.to_node))
            .into_group_map_by(|edge| Arc::clone(&edge.from_node));
        
        Graph {
            start_edges,
            edges_form_node,
            node_distances,
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
