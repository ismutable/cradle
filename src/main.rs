use petgraph::{Graph, Undirected};
use petgraph::dot::{Dot, Config};

type Location = (u8, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Occupancy {
    Vacant,
    Village,
    City,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BoardNode {
    location: Location,
    occupancy: Occupancy,
}

impl BoardNode {
    fn new(location: Location) -> Self {
        BoardNode {
            location,
            occupancy: Occupancy::Vacant,
        }
    }
}

fn main() {
    let mut graph = Graph::<BoardNode,(), Undirected>::new_undirected();
    let node_a = graph.add_node(BoardNode::new((0, 2)));
    let node_b = graph.add_node(BoardNode::new((1, 1)));
    let node_c = graph.add_node(BoardNode::new((1, 3)));
    graph.add_edge(node_a, node_b, ());
    graph.add_edge(node_a, node_c, ());
    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    graph.node_weight_mut(node_a).unwrap().occupancy = Occupancy::Village;
    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
}
