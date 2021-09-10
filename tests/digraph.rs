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
    fn adding_duplicate_vertex_is_err() {
        let mut g: DiGraph<i64> = DiGraph::new();

        g.add_vertex(1).expect("Inserting a unique vertex should've worked");

        assert!(g.add_vertex(1).is_err());
        assert!(g.add_vertex(2).is_ok());
    }

    #[test]
    fn removing_nonexistent_vertex_idempotent() {
        let mut g: DiGraph<i64> = DiGraph::new();

        // Should behave when the graph contains edges and when it doesn't
        g.remove_vertex(&10);

        g.add_vertex(1).expect("Inserting a unique vertex should've worked!");

        g.remove_vertex(&10);

        assert_eq!(g.num_edges(), 0);
        assert_eq!(g.num_vertices(), 1);
    }

    #[test]
    fn contains_should_report_correctly() {
        let mut g: DiGraph<i64> = DiGraph::new();

        assert_eq!(g.contains(&5), false);

        g.add_vertex(5).expect("Inserting a unique vertex should've worked!");
        assert_eq!(g.contains(&5), true);

        g.remove_vertex(&5);
        assert_eq!(g.contains(&5), false);

        g.add_vertex(1000).expect("Inserting a unique vertex should've worked!");
        assert_eq!(g.contains(&42), false);
    }

    #[test]
    fn adding_edges_between_existing_nodes() {
        let mut g: DiGraph<i64> = DiGraph::new();

        g.add_vertex(1).expect("Inserting a unique vertex should've worked!");
        g.add_vertex(2).expect("Inserting a unique vertex should've worked!");
        g.add_vertex(3).expect("Inserting a unique vertex should've worked!");
        g.add_vertex(4).expect("Inserting a unique vertex should've worked!");

        g.add_edge(&1, &2).expect("Source vertex SHOULD exist here!");
        g.add_edge(&2, &3).expect("Source vertex SHOULD exist here!");
        g.add_edge(&3, &4).expect("Source vertex SHOULD exist here!");
        g.add_edge(&4, &1).expect("Source vertex SHOULD exist here!");

        g.add_edge(&1, &3).expect("Source vertex SHOULD exist here!");
        g.add_edge(&2, &4).expect("Source vertex SHOULD exist here!");

        // Sanity check the vertices are untouched
        assert_eq!(g.contains(&1), true);
        assert_eq!(g.contains(&2), true);
        assert_eq!(g.contains(&3), true);
        assert_eq!(g.contains(&4), true);

        // Check all the true relationships
        assert_eq!(g.are_neighbors(&1, &2), true);
        assert_eq!(g.are_neighbors(&2, &3), true);
        assert_eq!(g.are_neighbors(&3, &4), true);
        assert_eq!(g.are_neighbors(&4, &1), true);
        
        assert_eq!(g.are_neighbors(&1, &3), true);
        assert_eq!(g.are_neighbors(&2, &4), true);

        // Check the false ones as well
        assert_eq!(g.are_neighbors(&2, &1), false);
        assert_eq!(g.are_neighbors(&3, &1), false);
        assert_eq!(g.are_neighbors(&3, &2), false);
        assert_eq!(g.are_neighbors(&1, &4), false);
        assert_eq!(g.are_neighbors(&4, &2), false);
    }

    #[test]
    fn adding_edges_when_target_doesnt_exist() {
        let mut g: DiGraph<i64> = DiGraph::new();

        // As long as we make one vertex manually the rest can be created
        // implicitly, as long as it's done in the right order.
        g.add_vertex(1).expect("Inserting a unique vertex should've worked!");

        g.add_edge(&1, &2).expect("Source vertex SHOULD exist here!");
        g.add_edge(&2, &3).expect("Source vertex SHOULD exist here!");
        g.add_edge(&3, &4).expect("Source vertex SHOULD exist here!");
        g.add_edge(&4, &1).expect("Source vertex SHOULD exist here!");

        g.add_edge(&1, &3).expect("Source vertex SHOULD exist here!");
        g.add_edge(&2, &4).expect("Source vertex SHOULD exist here!");

        // Sanity check the vertices are untouched
        assert_eq!(g.contains(&1), true);
        assert_eq!(g.contains(&2), true);
        assert_eq!(g.contains(&3), true);
        assert_eq!(g.contains(&4), true);

        // Check all the true relationships
        assert_eq!(g.are_neighbors(&1, &2), true);
        assert_eq!(g.are_neighbors(&2, &3), true);
        assert_eq!(g.are_neighbors(&3, &4), true);
        assert_eq!(g.are_neighbors(&4, &1), true);
        
        assert_eq!(g.are_neighbors(&1, &3), true);
        assert_eq!(g.are_neighbors(&2, &4), true);

        // Check the false ones as well
        assert_eq!(g.are_neighbors(&2, &1), false);
        assert_eq!(g.are_neighbors(&3, &1), false);
        assert_eq!(g.are_neighbors(&3, &2), false);
        assert_eq!(g.are_neighbors(&1, &4), false);
        assert_eq!(g.are_neighbors(&4, &2), false);
    }

    #[test]
    fn adding_edge_with_bad_source_is_err() {
        let mut g: DiGraph<i64> = DiGraph::new();

        assert!(g.add_edge(&1, &2).is_err());
    }

    fn helper_make_graph() -> DiGraph<i64> {
        let mut g: DiGraph<i64> = DiGraph::new();

        g.add_vertex(1).expect("Inserting a unique vertex should've worked!");

        g.add_edge(&1, &2).expect("Source vertex SHOULD exist here!");
        g.add_edge(&2, &3).expect("Source vertex SHOULD exist here!");
        g.add_edge(&2, &4).expect("Source vertex SHOULD exist here!");
        g.add_edge(&2, &5).expect("Source vertex SHOULD exist here!");
        g.add_edge(&5, &6).expect("Source vertex SHOULD exist here!");
        g.add_edge(&5, &4).expect("Source vertex SHOULD exist here!");
        g.add_edge(&4, &6).expect("Source vertex SHOULD exist here!");
        g.add_edge(&4, &3).expect("Source vertex SHOULD exist here!");
        g.add_edge(&3, &1).expect("Source vertex SHOULD exist here!");
        g.add_edge(&1, &5).expect("Source vertex SHOULD exist here!");

        g
    }

    #[test]
    fn in_and_out_degrees_are_correct() {
        let g: DiGraph<i64> = helper_make_graph();

        // Check in-degree
        assert_eq!(g.in_degree(&1).unwrap(), 1);
        assert_eq!(g.in_degree(&2).unwrap(), 1);
        assert_eq!(g.in_degree(&3).unwrap(), 2);
        assert_eq!(g.in_degree(&4).unwrap(), 2);
        assert_eq!(g.in_degree(&5).unwrap(), 2);
        assert_eq!(g.in_degree(&6).unwrap(), 2);

        // Check out-degree
        assert_eq!(g.out_degree(&1).unwrap(), 2);
        assert_eq!(g.out_degree(&2).unwrap(), 3);
        assert_eq!(g.out_degree(&3).unwrap(), 1);
        assert_eq!(g.out_degree(&4).unwrap(), 2);
        assert_eq!(g.out_degree(&5).unwrap(), 2);
        assert_eq!(g.out_degree(&6).unwrap(), 0);
    }

    #[test]
    fn remove_edge_leaves_graph_consistent() {
        let mut g: DiGraph<i64> = helper_make_graph();

        g.remove_edge(&4, &6);
        assert_eq!(g.in_degree(&6).unwrap(), 1);
        assert_eq!(g.out_degree(&4).unwrap(), 1);
        assert_eq!(g.are_neighbors(&4, &6), false);

        g.remove_edge(&1, &2);
        assert_eq!(g.in_degree(&2).unwrap(), 0);
        assert_eq!(g.out_degree(&1).unwrap(), 1);
        assert_eq!(g.are_neighbors(&1, &2), false);

        // Sanity check that the vertices are the same
        assert_eq!(g.contains(&1), true);
        assert_eq!(g.contains(&2), true);
        assert_eq!(g.contains(&3), true);
        assert_eq!(g.contains(&4), true);
        assert_eq!(g.contains(&5), true);
        assert_eq!(g.contains(&6), true);
    }

    #[test]
    fn remove_vertex_leaves_graph_consistent() {
        let mut g: DiGraph<i64> = helper_make_graph();

        // Remove something with no out-going edges
        g.remove_vertex(&6);
        assert_eq!(g.are_neighbors(&4, &6), false);
        assert_eq!(g.are_neighbors(&5, &6), false);

        assert_eq!(g.in_degree(&4).unwrap(), 2);
        assert_eq!(g.out_degree(&4).unwrap(), 1);
         
        assert_eq!(g.in_degree(&5).unwrap(), 2);
        assert_eq!(g.out_degree(&5).unwrap(), 1);

        // Remove something with in and out-going edges
        g.remove_vertex(&1);
        assert_eq!(g.are_neighbors(&3, &1), false);

        assert_eq!(g.in_degree(&3).unwrap(), 2);
        assert_eq!(g.out_degree(&3).unwrap(), 0);

        assert_eq!(g.in_degree(&2).unwrap(), 0);
        assert_eq!(g.out_degree(&2).unwrap(), 3);

        assert_eq!(g.in_degree(&5).unwrap(), 1);
        assert_eq!(g.out_degree(&5).unwrap(), 1);

        // are_neighbors(u, v) where u doesn't exist is always false:
        assert_eq!(g.are_neighbors(&1, &3), false);
        assert_eq!(g.are_neighbors(&6, &3), false);
        assert_eq!(g.are_neighbors(&1, &6), false);
        assert_eq!(g.are_neighbors(&6, &1), false);
    }

    #[test]
    fn neighbors_iterator_should_return_correct_nodes() {
        let g: DiGraph<i64> = helper_make_graph();

        assert_eq!(g.neighbors_of(&2).unwrap().eq([3, 4, 5].iter()), true);
        assert_eq!(g.neighbors_of(&1).unwrap().eq([2, 5].iter()), true);
        assert_eq!(g.neighbors_of(&6).unwrap().eq([].iter()), true);
    }

    #[test]
    fn get_source_vertices_returns_correct_nodes() {
        let mut g: DiGraph<i64> = helper_make_graph();

        // By default the helper graph has no appropriate vertices
        assert_eq!(g.get_source_vertices().len(), 0);

        g.remove_edge(&1, &2);

        // After removing the edge from 1 -> 2 we have a single
        // source vertex: 2
        assert_eq!(g.get_source_vertices().len(), 1);
        assert_eq!(g.get_source_vertices().contains(&2), true);
    }
}