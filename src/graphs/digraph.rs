use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::slice::Iter;
use std::fmt::Debug;

/// Very simple DiGraph implementation
///
/// T is the type of the nodes. No duplicate nodes may exist
/// in the graph.
///
/// Hopefully this implementation will be updated
/// as I create more graph algorithms and understand
/// what is important in a graph data structure.
/// Notably, this implementation is very space inefficient...
pub struct DiGraph<T: Clone + Eq + Hash> {
    // Consider making this <T, Vec<&T>> to save space?
    // Test first! Refactor later!
    edge_map: HashMap<T, Vec<T>>
}

impl<T: Clone + Eq + Hash> Default for DiGraph<T> {
    fn default() -> Self {
        Self::new()
    }
}


impl<T: Clone + Eq + Hash> DiGraph<T> {
    /// Constructs a new, empty DiGraph
    pub fn new() -> DiGraph<T> {
        DiGraph {
            edge_map: HashMap::new()
        }
    }

    /// Prints out the backing HashMap for the DiGraph
    /// Your node type T must also implement Debug in order to use this function
    pub fn debug_print(g: DiGraph<T>) where T: Debug {
        println!("{:#?}", g.edge_map);
    }

    /// Returns the number of vertices present in the graph
    pub fn num_vertices(&self) -> usize {
        self.edge_map.len()
    }

    /// Returns the number of edges present in the graph
    /// This is an `O(V)` operation
    pub fn num_edges(&self) -> usize {
        self.edge_map.iter().fold(0, |acc, (_node, edges)| {
            acc + edges.len()
        })
    }

    /// Adds a new, unconnected, vertex to the graph
    pub fn add_vertex(&mut self, node: T) -> Result<(), String> {
        if self.edge_map.contains_key(&node) {
            // again, proper err type would be better here
            Err(String::from("Attempted to insert duplicate node"))
        } else {
            self.edge_map.insert(node, Vec::new());
            Ok(())
        }
    }

    /// Returns `true` if the query vertex exists in the graph, `false` otherwise
    pub fn contains(&self, node: &T) -> bool {
        self.edge_map.contains_key(node)
    }

    /// Returns `true` if there is a directed edge from `u` to `v`, `false` otherwise
    pub fn are_neighbors(&self, u: &T, v: &T) -> bool {
        if let Some(edges) = self.edge_map.get(u) {
            edges.contains(v)
        } else {
            false
        }
    }

    /// Returns an iterator over the neighbors of a given vertex.
    /// Or `None` if the vertex does not exist in the graph.
    pub fn neighbors_of(&self, node: &T) -> Option<Iter<T>> {
        self.edge_map.get(node).map(|edges| edges.iter())
    }

    /// Returns the number of directed edges that start at the query vertex.
    /// Or `None` if the query vertex does not exist in the graph.
    /// This is an `O(1)` operation
    pub fn out_degree(&self, node: &T) -> Option<usize> {
        self.edge_map.get(node).map(|edges| edges.len())
    }

    /// Returns the number of directed edges that end at the query vertex.
    /// Or None if the query vertex does not exist in the graph.
    /// This is an `O(E)` operation
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

    /// Adds a directed edge between `u` and `v`. Returns `Ok(())` if the
    /// operation was successful, but Err if `u` does not exist in the graph.
    pub fn add_edge(&mut self, u: &T, v: &T) -> Result<(), String> {
        // Ideally, use a proper std::Error implementing type
        // Or revert back to &'static str
        if self.edge_map.contains_key(u) {
            // Notes:
            // 1. It is safe to unwrap these values because I'm directly checking
            //    the map contains the key `u`.
            // 2. This code could be cleaner, this was the first version I got
            //    working with the borrow checker. The issue is that I'm immutably
            //    checking the contents of the hashmap, then mutably borrowing it
            //    two different times and non of those references can overlap.
            if !self.edge_map.contains_key(v) {
                // We HAVE to clone here because both Vec#push
                // and add_vertex take ownership of the value.
                self.edge_map.insert(v.clone(), Vec::new());
            }

            let target_edges = self.edge_map.get_mut(u).unwrap();
            target_edges.push(v.clone());
            Ok(())
        } else {
            Err(String::from("Start vertex of edge not present!"))
        }
    }

    /// Helper function for removing the first occurrence of the
    /// value `target` in the provided `list`.
    fn remove_by_value(list: &mut Vec<T>, target: &T) {
        match list.iter().position(|i| target.eq(i)) {
            None => {}
            Some(idx) => {
                list.remove(idx);
            }
        }
    }

    /// Removes the directed edge between `u` and `v`, if it exists.
    /// If the edge does not exist, this operation is idempotent.
    pub fn remove_edge(&mut self, u: &T, v: &T) {
        // Not clear to me if a return value is worthwhile here
        if let Some(target_edges) = self.edge_map.get_mut(u) {
            DiGraph::remove_by_value(target_edges, v);
        }
    }

    /// Removes the target vertex and all of its incoming edges
    /// (all edges that end at the target vertex) from the graph.
    ///
    /// This is an `O(E)` operation.
    pub fn remove_vertex(&mut self, target: &T) {
        // This is a relatively expensive operation: O(V + E)
        // First, remove all directed edges going TO the target
        for (_node, edges) in self.edge_map.iter_mut() {
            DiGraph::remove_by_value(edges, target)
        }
        // Then, remove the vertex and all of its outgoing edges
        self.edge_map.remove(target);
    }

    /// Returns all of the vertices in the graph that have an
    /// in-degree of 0.
    ///
    /// This is an `O(V+E)` operation.
    pub fn get_source_vertices(&self) -> HashSet<&T> {
        // Magic generic method for converting iterators into collections
        // The type &T is significant because we don't want to copy all the keys,
        // nor do we want to take ownership of the keys in the hashmap.
        let mut vertices: HashSet<&T> = self.edge_map.keys().collect::<HashSet<_>>();
        for (_node, edges) in self.edge_map.iter() {
            for end_vertex in edges {
                // It is idempotent to remove something not in the set
                vertices.remove(end_vertex);
            }
        }
        vertices
    }
}