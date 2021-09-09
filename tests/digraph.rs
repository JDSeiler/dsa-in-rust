#[cfg(test)]
mod digraph {
    use dsa_in_rust::graphs::digraph::*;

    #[test]
    fn new_graph_has_no_edges_or_vertices() {
        let g: DiGraph<i64> = DiGraph::new();
        assert_eq!(g.num_edges(), 0);
        assert_eq!(g.num_vertices(), 0);
    }

    #[test]
    fn adding_vertices_modifies_vertices_only() {
        let mut g: DiGraph<i64> = DiGraph::new();

        g.add_vertex(1).expect("Inserting a unique vertex should've worked!");
        g.add_vertex(2).expect("Inserting a unique vertex should've worked!");
        g.add_vertex(3).expect("Inserting a unique vertex should've worked!");
        g.add_vertex(4).expect("Inserting a unique vertex should've worked!");

        assert_eq!(g.num_edges(), 0);
        assert_eq!(g.num_vertices(), 4);
    }

    #[test]
    fn adding_and_removing_vertices() {
        let mut g: DiGraph<i64> = DiGraph::new();

        g.add_vertex(1).expect("Inserting a unique vertex should've worked!");
        g.add_vertex(2).expect("Inserting a unique vertex should've worked!");
        g.add_vertex(3).expect("Inserting a unique vertex should've worked!");
        g.add_vertex(4).expect("Inserting a unique vertex should've worked!");

        assert_eq!(g.num_edges(), 0);
        assert_eq!(g.num_vertices(), 4);

        g.remove_vertex(&1);
        g.remove_vertex(&3); // Should be able to delete in any order
        g.remove_vertex(&4);
        g.remove_vertex(&2);

        assert_eq!(g.num_edges(), 0);
        assert_eq!(g.num_vertices(), 0);
    }

    #[test]
    fn removing_nonexistent_vertex_idempotent() {
        let mut g: DiGraph<i64> = DiGraph::new();

        // Should behave when the graph contains edges and when it doesn't
        g.remove_vertex(&10);

        g.add_vertex(1);

        g.remove_vertex(&10);

        assert_eq!(g.num_edges(), 0);
        assert_eq!(g.num_vertices(), 1);
    }

    /*
    List of test TODO:
    1. `contains` (false and true cases)
    2. adding vertices (existing and non-existing end vertices)
    3. are_neighbors (false and true cases)
    4. in_degree and out_degree
    5. remove_edge and remove_vertex, check consistency of graph after modification
    6. check neighbors iterator for correctness
    7. check "get_source_vertices" for correctness
    */
}