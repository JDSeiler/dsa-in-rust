
#[cfg(test)]
mod topological_sorting {
    use dsa_in_rust::graphs::digraph::DiGraph;
    use dsa_in_rust::graphs::topological_sorting::topological_sort;

    #[test]
    fn produces_correct_result_on_acyclic_graph() {
        let mut g: DiGraph<i64> = DiGraph::new();

        g.add_vertex(1).expect("Adding a unique vertex should've worked!");
        g.add_edge(&1, &2).expect("Adding this edge should've worked!");

        g.add_edge(&2, &3).expect("Adding this edge should've worked!");
        g.add_edge(&2, &4).expect("Adding this edge should've worked!");
        g.add_edge(&2, &5).expect("Adding this edge should've worked!");

        g.add_edge(&4, &5).expect("Adding this edge should've worked!");
        g.add_edge(&4, &6).expect("Adding this edge should've worked!");

        g.add_edge(&5, &7).expect("Adding this edge should've worked!");
        g.add_edge(&6, &7).expect("Adding this edge should've worked!");

        let sorted = topological_sort(&mut g).expect("This graph is acylic!");
        assert!(sorted.eq(&[1,2,3,4,5,6,7]));
    }

    #[test]
    fn produces_correct_result_on_cyclic_graph() {
        let mut g: DiGraph<i64> = DiGraph::new();

        g.add_vertex(1).expect("Adding a unique vertex should've worked!");

        g.add_edge(&1, &2).expect("Adding this edge should've worked!");
        g.add_edge(&2, &3).expect("Adding this edge should've worked!");
        g.add_edge(&3, &4).expect("Adding this edge should've worked!");
        g.add_edge(&4, &1).expect("Adding this edge should've worked!");

        let sorted = topological_sort(&mut g);
        assert!(sorted.is_none());
    }

    #[test]
    fn produces_correct_result_on_wikipedia_example() {
        // See: https://en.wikipedia.org/wiki/Topological_sorting
        let mut g: DiGraph<i64> = DiGraph::new();

        g.add_vertex(5).expect("Adding a unique vertex should've worked!");
        g.add_vertex(7).expect("Adding a unique vertex should've worked!");
        g.add_vertex(3).expect("Adding a unique vertex should've worked!");
        g.add_vertex(8).expect("Adding a unique vertex should've worked");
        g.add_vertex(2).expect("Adding a unique vertex should've worked");
        g.add_vertex(9).expect("Adding a unique vertex should've worked");
        g.add_vertex(11).expect("Adding a unique vertex should've worked");
        g.add_vertex(10).expect("Adding a unique vertex should've worked");

        g.add_edge(&5, &11).expect("Adding this edge should've worked!");

        g.add_edge(&7, &11).expect("Adding this edge should've worked!");
        g.add_edge(&7, &8).expect("Adding this edge should've worked!");

        g.add_edge(&3, &8).expect("Adding this edge should've worked!");
        g.add_edge(&3, &10).expect("Adding this edge should've worked!");

        g.add_edge(&11, &2).expect("Adding this edge should've worked!");
        g.add_edge(&11, &9).expect("Adding this edge should've worked!");
        g.add_edge(&11, &10).expect("Adding this edge should've worked!");

        g.add_edge(&8, &9).expect("Adding this edge should've worked!");

        let sorted = topological_sort(&mut g).expect("This graph is acyclic!");
        // The result is non-deterministic because of the backing hashmap and there
        // being 3 different valid start locations
        println!("Topological ordering is: {:?}", sorted);
    }
}