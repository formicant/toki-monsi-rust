mod node;
mod edge;

#[cfg(test)]
mod tests;

use std::rc::Rc;
use std::iter::Iterator;
use std::collections::HashMap;
use itertools::Itertools;
use priority_queue::PriorityQueue;

pub use node::Node;
pub use edge::Edge;


pub struct Graph {
    pub start_edges: Vec<Edge>,
    pub other_edges: Vec<Edge>,
    pub node_distances: HashMap<Rc<Node>, usize>,
}

impl Graph {
    pub fn build(word_list: &[&str]) -> Self {
        let start_edges = Edge::get_start_edges(word_list);
        let other_edges = Edge::get_other_edges(&start_edges, word_list);
        
        let node_distances = calculate_distances(&other_edges);

        Graph {
            start_edges,
            other_edges,
            node_distances,
        }
    }
}


fn calculate_distances(edges: &[Edge]) -> HashMap<Rc<Node>, usize> {
    let mut distances = HashMap::new();
    let mut queue = PriorityQueue::new();
    
    let from_nodes_by_to_node = edges.into_iter()
        .map(|edge| (Rc::clone(&edge.to_node), Rc::clone(&edge.from_node)))
        .into_group_map();
    
    let final_node = Rc::new(Node::Final);

    if from_nodes_by_to_node.contains_key(&final_node) {
        distances.insert(Rc::clone(&final_node), 0);
        queue.push(final_node, 0);
    }
    
    while let Some((node, priority)) = queue.pop() {
        let from_node_distance = priority + 1;
        
        for from_node in from_nodes_by_to_node[&node].iter() {
            match distances.get(from_node) {
                Some(&distance) if distance <= from_node_distance => { }
                _ => {
                    distances.insert(Rc::clone(&from_node), from_node_distance);
                    queue.push(Rc::clone(&from_node), from_node_distance);
                }
            }
        }
    }
    
    distances
}
