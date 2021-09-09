use std::collections::HashMap;
use std::hash::Hash;
use std::slice::Iter;

/// Very simple DiGraph implementation
///
/// T is the type of the nodes. No duplicate nodes may exist
/// in the graph.
///
/// Hopefully this implementation will be updated
/// as I create more graph algorithms and understand
/// what is important in a graph data structure.
pub struct DiGraph<T: Clone + Eq + Hash> {
    edge_map: HashMap<T, Vec<T>>
}

impl<T: Clone + Eq + Hash> Default for DiGraph<T> {
    fn default() -> Self {
        Self::new()
    }
}


impl<T: Clone + Eq + Hash> DiGraph<T> {
    pub fn new() -> DiGraph<T> {
        DiGraph {
            edge_map: HashMap::new()
        }
    }

    pub fn num_vertices(&self) -> usize {
        self.edge_map.len()
    }

    pub fn num_edges(&self) -> usize {
        self.edge_map.iter().fold(0, |acc, (_node, edges)| {
            acc + edges.len()
        })
    }

    pub fn add_vertex(&mut self, node: T) {
        self.edge_map.insert(node, Vec::new());
    }

    pub fn contains(&self, node: &T) -> bool {
        self.edge_map.contains_key(node)
    }

    pub fn are_neighbors(&self, u: &T, v: &T) -> bool {
        if let Some(edges) = self.edge_map.get(u) {
            edges.contains(v)
        } else {
            false
        }
    }

    pub fn neighbors_of(&self, node: &T) -> Option<Iter<T>> {
        self.edge_map.get(node).map(|edges| edges.iter())
    }

    pub fn out_degree(&self, node: &T) -> Option<usize> {
        self.edge_map.get(node).map(|edges| edges.len())
    }

    pub fn in_degree(&self, node: &T) -> Option<usize> {
        // If the target node isn't even in the graph, don't bother checking anything
        if self.edge_map.get(node).is_none() {
            None
        } else {
            // Otherwise, count up how many times the target node appears in all the edges
            Some(self.edge_map.iter().fold(0, |acc, (_node, edges)| {
                if edges.contains(node) {
                    acc + 1
                } else {
                    acc
                }
            }))
        }
    }

    // Ideally, use a proper std::Error implementing type
    // Or revert back to &'static str
    pub fn add_edge(&mut self, u: &T, v: T) -> Result<(), String> {
        if self.edge_map.contains_key(u) {
            // Notes:
            // 1. It is safe to unwrap these values because I'm directly checking
            //    the map contains the key `u`.
            // 2. This code could be cleaner, this was the first version I got
            //    working with the borrow checker. The issue is that I'm immutably
            //    checking the contents of the hashmap, then mutably borrowing it
            //    two different times and non of those references can overlap.
            if !self.edge_map.get(u).unwrap().contains(&v) {
                // We HAVE to clone here because both Vec#push
                // and add_vertex take ownership of the value.
                self.edge_map.insert(v.clone(), Vec::new());
            }

            let target_edges = self.edge_map.get_mut(u).unwrap();
            target_edges.push(v);
            Ok(())
        } else {
            Err(String::from("Start vertex of edge not present!"))
        }
    }

    fn remove_by_value(list: &mut Vec<T>, target: &T) {
        match list.iter().position(|i| target.eq(i)) {
            None => {}
            Some(idx) => {
                list.remove(idx);
            }
        }
    }

    // Not clear to me if a return value is worthwhile here
    pub fn remove_edge(&mut self, u: &T, v: &T) {
        if let Some(target_edges) = self.edge_map.get_mut(u) {
            DiGraph::remove_by_value(target_edges, v);
        }
    }

    // This is a relatively expensive operation: O(V + E)
    pub fn remove_vertex(&mut self, target: &T) {
        // First, remove all directed edges going TO the target
        for (_node, edges) in self.edge_map.iter_mut() {
            DiGraph::remove_by_value(edges, target)
        }
        // Then, remove the vertex and all of its outgoing edges
        self.edge_map.remove(target);
    }
}