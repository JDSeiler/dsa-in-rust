use super::digraph::DiGraph;
use std::{collections::VecDeque, hash::Hash};
use std::iter::FromIterator;
use std::fmt::Debug;

/// O(V*E) because of my implementation >:(
/// Not exactly sure how to make in-degree a linear time operation
pub fn topological_sort<T: Clone + Eq + Hash + Debug>(g: &mut DiGraph<T>) -> Option<Vec<T>> {
    let mut topological_order: Vec<T> = Vec::new();
    let mut candidates: VecDeque<T> = VecDeque::from_iter(g.get_source_vertices());

    while !candidates.is_empty() {
        let current = candidates.pop_front().unwrap();
        topological_order.push(current.clone());

        let neighbors: Vec<T> = g.neighbors_of(&current).unwrap().map(|k| k.to_owned()).collect();
        for v in neighbors {
            g.remove_edge(&current, &v);
            if g.in_degree(&v).unwrap() == 0 {
                candidates.push_back(v.clone());
            }
        }
    }

    if g.num_edges() > 0 {
        None
    } else {
        Some(topological_order)
    }
}