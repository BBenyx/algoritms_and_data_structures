use crate::graph::{Color, VertexId, WGraph};
use crate::min_heap::GrayHeap;

pub fn path(from: VertexId, to: VertexId, graph: &WGraph) -> bool {
    let mut states = vec![(0, Color::White); graph.graph.len()];
    let mut gray_heap = GrayHeap::new();

    states[from].1 = Color::Gray;
    gray_heap.push(from, 0);

    while !gray_heap.is_empty() {
        let smallest = gray_heap.pop_min().unwrap();

        for edge in &graph.graph[smallest.0].out_edges {
            match &mut states[edge.to] {
                (w, col) if *col == Color::White => {
                    if edge.to == to {
                        return true;
                    }
                    *col = Color::Gray;
                    *w = smallest.1 + edge.weight;
                    gray_heap.push(edge.to, *w);
                }

                (w, col) if *col == Color::Gray && *w > smallest.1 + edge.weight => {
                    gray_heap.change_weight(edge.to, smallest.1 + edge.weight)
                }
                _ => (),
            }
        }
        states[smallest.0].1 = Color::Black;
    }

    false
}
