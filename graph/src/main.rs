use crate::graph::{Edge, Graph, Vertex};

pub mod dijkstra;
pub mod graph;
pub mod min_heap;
pub mod topological_order;

fn main() {
    let mut my_graph = graph::WGraph::new();

    for elem in 0..8 {
        let vertex = Vertex::new(elem);
        my_graph.graph.push(vertex)
    }
    my_graph.graph[0].out_edges.push_front(Edge::new(0, 1));
    my_graph.graph[0].out_edges.push_front(Edge::new(1, 3));
    my_graph.graph[0].out_edges.push_front(Edge::new(4, 5));
    my_graph.graph[0].out_edges.push_front(Edge::new(5, 10));

    my_graph.graph[1].out_edges.push_front(Edge::new(2, 2));

    my_graph.graph[2].out_edges.push_front(Edge::new(5, 1));

    my_graph.graph[3].out_edges.push_front(Edge::new(0, 1));

    my_graph.graph[4].out_edges.push_front(Edge::new(1, 7));
    my_graph.graph[4].out_edges.push_front(Edge::new(7, 10));
    
    my_graph.graph[5].out_edges.push_front(Edge::new(6, 8));
    
    my_graph.graph[6].out_edges.push_front(Edge::new(7, 4));

    println!("Path from 0 to 7 with topo: {:?}", topological_order::path(0, 7, &my_graph));
    println!("Path from 0 to 7 with Dijkstra: {}.", dijkstra::path(0, 7, &my_graph));
    println!("Path from 0 to 7 with trait path: {}.", my_graph.path(0, 7));
}
