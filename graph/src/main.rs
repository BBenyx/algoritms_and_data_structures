use crate::graph::{Edge, Graph, Vertex};

pub mod graph;

fn main() {
    let mut my_graph = graph::WGraph::new();

    for elem in 0..5 {
        let vertex = Vertex::new(elem);
        my_graph.graph.push(vertex)
    }
    my_graph.graph[0].out_edges.push_front(Edge::new(1, 1));
    my_graph.graph[0].out_edges.push_front(Edge::new(3, 1));

    my_graph.graph[1].out_edges.push_front(Edge::new(2, 1));
    my_graph.graph[1].out_edges.push_front(Edge::new(3, 1));

    my_graph.graph[2].out_edges.push_front(Edge::new(3, 1));

   my_graph.graph[3].out_edges.push_front(Edge::new(0, 1));

    my_graph.graph[4].out_edges.push_front(Edge::new(0, 1));

    println!("{:?}", my_graph.cyclic());
}
