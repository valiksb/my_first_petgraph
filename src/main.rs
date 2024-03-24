use petgraph::graph::Graph;
use petgraph::dot::Dot;

fn main() {
    let mut graph = Graph::new_undirected();
    let origin = graph.add_node("Denver");
    let destination_1 = graph.add_node("San Diego");
    let destination_2 = graph.add_node("New York");
    let cost_1 = graph.add_edge(origin, destination_1, 250);
    let cost_2 = graph.add_edge(origin, destination_2, 1099);

    assert_eq!(graph.edge_weight(cost_1).unwrap(), &250);
    assert_eq!(graph.edge_weight(cost_2).unwrap(), &1099);

    println!("{}", Dot::new(&graph));
}