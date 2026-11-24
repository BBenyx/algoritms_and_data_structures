use crate::graph::{VertexId, WGraph, GraphError};

pub fn path(from: VertexId, to: VertexId, graph: &WGraph) -> Result<bool, GraphError> {
    let mut dist: Vec<isize> = vec![isize::MAX; graph.graph.len()];
    let sorted = match graph.topo_sort_from(from) {
        Ok(list) => list,
        Err(e) => return Err(e),
    };
    dist[sorted[0]] = 0;

    for v in sorted {
        if v == to {
            return Ok(true);
        };

        for edge in &graph.graph[v].out_edges {
            dist[edge.to] = std::cmp::min(dist[edge.to], dist[v] + edge.weight);
        }
    }

    Ok(false)
}